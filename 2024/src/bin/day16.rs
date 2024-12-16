//! # Reindeer Maze
//!
//! Smells like Dijsktra.

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    hash::Hash,
};

use aoc_2dmap::prelude::{Map, Pos, ORTHOGONAL};
use aoc_dijsktra::hash::KeyMap;
use aoc_prelude::{Entry, HashMap, HashSet};

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

    fn transform(&self, game_state: &State) -> State {
        let mut new_state = *game_state;
        new_state.dir = self.change_dir(game_state.dir);
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
    let map = include_str!("../../inputs/16.in");
    let map_size = Pos::from((
        map.chars().position(|x| x == '\n').unwrap(),
        map.chars().filter(|x| *x == '\n').count(),
    ));
    let map = Map::new(map_size, map.chars().filter(|&c| c != '\n'));

    let start = find_tile(&map, 'S');
    let goal = find_tile(&map, 'E');

    let state = State { pos: start, dir: Pos::new(1, 0) };

    let mut p1 = usize::MAX;

    let mut known = KeyMap::default();

    let mut pq = BinaryHeap::with_capacity(1024);
    pq.push((Reverse(0), state));

    let mut paths = HashMap::<Pos, HashSet<(State, usize)>>::new();

    while let Some((Reverse(cost), state)) = pq.pop() {
        if state.pos == goal {
            p1 = cost;
            break;
        }
        for step in state.steps(&map) {
            let new_cost = cost + step.cost();
            let new_state = step.transform(&state);

            match known.entry(&new_state) {
                // Update if there's a less costly way to get to a known state...
                Entry::Occupied(mut entry) if new_cost <= *entry.get() => {
                    entry.insert(new_cost);
                    pq.push((Reverse(new_cost), new_state));
                    paths.entry(new_state.pos).or_default().insert((state, new_cost));
                }
                // ...or if the state is unknown.
                Entry::Vacant(entry) => {
                    entry.insert(new_cost);
                    pq.push((Reverse(new_cost), new_state));
                    paths.entry(new_state.pos).or_default().insert((state, new_cost));
                }
                _ => {}
            }
        }
    }

    let mut q = VecDeque::from(ORTHOGONAL.map(|dir| State { pos: goal, dir }));
    let mut next = HashMap::with_capacity(200);

    let mut good_tiles = HashSet::new();

    while let Some(start_node) = q.pop_front() {
        good_tiles.insert(start_node.pos);

        if let Some(back_links) = paths.get(&start_node.pos) {
            next.clear();
            let mut min_cost = usize::MAX;

            for &(key, val) in back_links {
                if val > p1 {
                    continue;
                }
                let mut cost = val;
                if key.dir != start_node.dir {
                    cost += 1000;
                }
                min_cost = min_cost.min(cost);
                next.entry(cost).or_insert(Vec::new()).push(key);
            }
            if min_cost != usize::MAX {
                q.extend(&next[&min_cost]);
            }
        }
    }
    let p2 = good_tiles.len();

    (p1, p2)
}

fn find_tile(map: &Map<char>, tile: char) -> Pos {
    map.iter().find(|pos| map.get_unchecked(pos) == tile).unwrap()
}

aoc_2024::main! {
    solve()
}
