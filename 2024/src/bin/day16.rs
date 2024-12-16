//! # Reindeer Maze
//!
//! Smells like Dijsktra.

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    hash::Hash,
};

use aoc_2dmap::prelude::{Map, Pos, ORTHOGONAL};
use aoc_prelude::{HashMap, HashSet, Itertools};

const TURN_COST: usize = 1000;

#[derive(Copy, Clone, Hash, Debug)]
enum Move {
    Adv,
    Right,
    Left,
}

impl Move {
    fn change_dir(&self, d: Pos) -> Pos {
        match self {
            Move::Adv => d,
            Move::Right => d.clockwise(),
            Move::Left => d.anticlockwise(),
        }
    }

    fn cost(&self) -> usize {
        match self {
            Move::Adv => 1,
            Move::Left | Move::Right => TURN_COST + 1,
        }
    }

    fn transform(&self, old_state: &State) -> State {
        let mut new_state = *old_state;
        new_state.dir = self.change_dir(old_state.dir);
        new_state.pos += new_state.dir;
        new_state
    }
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Hash, Clone, Copy, Debug)]
struct State {
    pos: Pos,
    dir: Pos,
}

impl State {
    fn steps<'a>(&'a self, map: &'a Map<char>) -> impl Iterator<Item = Move> + 'a {
        [Move::Right, Move::Left, Move::Adv].into_iter().filter(|mv| {
            let tile = map.get(self.pos + mv.change_dir(self.dir));
            tile == Some('.') || tile == Some('E')
        })
    }
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/16.in");
    let map_size = Pos::from((
        input.chars().position(|x| x == '\n').unwrap(),
        input.chars().filter(|x| *x == '\n').count(),
    ));
    let map = Map::new(map_size, input.chars().filter(|&c| c != '\n'));

    let start = find_tile(&map, 'S');
    let goal = find_tile(&map, 'E');

    let mut p1 = usize::MAX;
    let mut costs = HashMap::<State, usize>::with_capacity(1000);
    let mut paths = HashMap::<State, Vec<State>>::with_capacity(1000);
    let mut pq = BinaryHeap::with_capacity(1024);
    pq.push((Reverse(0), State { pos: start, dir: Pos::new(1, 0) }));

    while let Some((Reverse(cost), state)) = pq.pop() {
        if cost > *costs.get(&state).unwrap_or(&usize::MAX) {
            continue;
        }
        if state.pos == goal {
            p1 = cost;
            break;
        }
        for step in state.steps(&map) {
            let new_cost = cost + step.cost();
            let new_state = step.transform(&state);

            let cost = *costs.get(&new_state).unwrap_or(&usize::MAX);
            if new_cost > cost {
                continue;
            }
            if new_cost < cost {
                paths.insert(new_state, Vec::new());
                costs.insert(new_state, new_cost);
            }
            paths.entry(new_state).or_default().push(state);
            pq.push((Reverse(new_cost), new_state));
        }
    }

    let mut q = VecDeque::from(ORTHOGONAL.map(|dir| State { pos: goal, dir }));
    let mut seen = HashSet::with_capacity(200);
    seen.extend(q.iter().copied());

    while let Some(cur_node) = q.pop_front() {
        if let Some(prev_nodes) = paths.get(&cur_node) {
            for state in prev_nodes {
                if seen.contains(state) {
                    continue;
                }
                seen.insert(*state);
                q.push_back(*state);
            }
        }
    }

    let p2 = seen.iter().map(|state| state.pos).unique().count();

    (p1, p2)
}

fn find_tile(map: &Map<char>, tile: char) -> Pos {
    map.iter().find(|pos| map.get_unchecked(pos) == tile).unwrap()
}

aoc_2024::main! {
    solve()
}
