//! # Reindeer Maze
//!
//! Lesson learned: do not try to adapt library code that doesn't fit the
//! problem statement.
//!
//! After many tries ended up storing a backtracking HashMap for *states*
//! rather than *positions* along with the associated costs.
//!
//! Then DFSed through it, always picking from the key with the minimum
//! cost.
//!
//! Simplified it in a second iteration to let the Dijkstra loop take care
//! of resetting the path set whenever a lower cost is found.
//!
//! Optimization #2: use two queues instead of a heap to turn Dijsktra into
//! a glorified BFS that always prefers shooting straight instead of turning.
//!
//! Inspiration: https://github.com/maneatingape/advent-of-code-rust/blob/0834bd10ef57be8ed8436d11171d0e9f9c52a1c9/src/year2024/day16.rs

use std::{collections::VecDeque, hash::Hash};

use aoc_2dmap::prelude::*;
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
    let mut costs = HashMap::<State, usize>::with_capacity(1024);
    let mut paths = HashMap::<State, Vec<State>>::with_capacity(1024);

    let mut q_one = VecDeque::with_capacity(512);
    let mut q_two = VecDeque::with_capacity(512);
    q_one.push_back((State { pos: start, dir: EAST }, 0));

    while !q_one.is_empty() {
        while let Some((state, cost)) = q_one.pop_front() {
            if state.pos == goal {
                p1 = cost;
                break;
            }
            for step in state.steps(&map) {
                let new_cost = cost + step.cost();
                let new_state = step.transform(&state);

                let lowest = *costs.get(&new_state).unwrap_or(&usize::MAX);
                if new_cost > lowest {
                    continue;
                }
                if new_cost < lowest {
                    paths.insert(new_state, Vec::new());
                    costs.insert(new_state, new_cost);
                }
                paths.entry(new_state).or_default().push(state);

                // prefer shooting straight than turning
                if new_state.dir == state.dir {
                    q_one.push_back((new_state, new_cost));
                } else {
                    q_two.push_back((new_state, new_cost));
                }
            }
        }
        (q_one, q_two) = (q_two, q_one);
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
    map.iter().find(|pos| map[pos] == tile).unwrap()
}

aoc_2024::main! {
    solve()
}
