//! # RAM Run
//!
//! Part 1: Dijsktra again.
//!
//! Part 2: Brute forced the placement of blocks until Dijsktra didn't return
//! a path anymore, but that took 500ms to run: the number of Dijsktra runs
//! equals the number of placed blocks, and it's a fairly expensive algo.
//!
//! Re-implemented Dijsktra to backtrack the shortest path whenever a solution
//! is found, then kept placing blocks _until_ one of them intersects the
//! shortest path, and _only then_ re-perform the Dijsktra. This got the number
//! of Dijsktra runs down to ~35.

use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_2dmap::prelude::{Map, Pos, ORTHOGONAL};
use aoc_dijsktra::Transform;
use aoc_prelude::{HashMap, HashSet, Itertools};

const MAP_SIZE: i32 = 71;
const INIT_BLOCKS: usize = 1024;
const START: Pos = Pos::c_new(0, 0);
const GOAL: Pos = Pos::c_new(MAP_SIZE - 1, MAP_SIZE - 1);

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct State {
    pos: Pos,
}

impl Transform<State> for Pos {
    fn cost(&self) -> usize { 1 }

    fn transform(&self, game_state: &State) -> State { State { pos: game_state.pos + *self } }
}

impl State {
    fn steps<'a>(&'a self, ctx: &'a Map<char>) -> impl Iterator<Item = Pos> + 'a {
        ORTHOGONAL.into_iter().filter(|dir| ctx.get(self.pos + *dir) == Some('.'))
    }
}

struct Buf {
    costs: HashMap<State, usize>,
    pq: BinaryHeap<(Reverse<usize>, State)>,
    edges: HashMap<Pos, Pos>,
    path: HashSet<Pos>,
}

impl Buf {
    fn default() -> Self {
        Self {
            costs: HashMap::with_capacity(1024),
            pq: BinaryHeap::with_capacity(1024),
            edges: HashMap::with_capacity(1024),
            path: HashSet::with_capacity(1024),
        }
    }

    fn clear(&mut self) {
        self.costs.clear();
        self.pq.clear();
        self.edges.clear();
    }
}

fn dijsktra(map: &Map<char>, buf: &mut Buf) -> Option<usize> {
    buf.clear();
    buf.pq.push((Reverse(0), State { pos: START }));

    let mut p1 = usize::MAX;

    while let Some((Reverse(cost), state)) = buf.pq.pop() {
        if state.pos == GOAL {
            p1 = cost;
            break;
        }
        for step in state.steps(map) {
            let new_cost = cost + step.cost();
            let new_state = step.transform(&state);

            let lowest = *buf.costs.get(&new_state).unwrap_or(&usize::MAX);
            if new_cost > lowest {
                continue;
            }
            if new_cost < lowest {
                buf.edges.insert(new_state.pos, state.pos);
                buf.costs.insert(new_state, new_cost);
                buf.pq.push((Reverse(new_cost), new_state));
            }
        }
    }

    if p1 == usize::MAX {
        return None;
    }

    backtrack(buf);
    Some(p1)
}

// Some if there's a path
fn backtrack(buf: &mut Buf) {
    let mut cur = GOAL;
    buf.path.clear();
    buf.path.insert(cur);
    while let Some(prev) = buf.edges.get(&cur) {
        buf.path.insert(*prev);
        if *prev == START {
            return;
        }
        cur = *prev;
    }
}

fn solve() -> (usize, String) {
    let mut map = Map::<char>::fill((MAP_SIZE, MAP_SIZE), '.');

    let blocks = include_str!("../../inputs/18.in")
        .lines()
        .map(|line| {
            let mut nums = extract_nums(line);
            let x = nums.next().unwrap();
            let y = nums.next().unwrap();
            Pos::new(x, y)
        })
        .collect_vec();

    for block in blocks.iter().take(INIT_BLOCKS) {
        map.set(block, '#');
    }

    let mut buf = Buf::default();

    let p1 = dijsktra(&map, &mut buf).expect("no path!?");

    let mut ans = Pos::new(0, 0);
    for block in blocks.iter().skip(INIT_BLOCKS) {
        map.set(block, '#');
        if buf.path.contains(block) && dijsktra(&map, &mut buf).is_none() {
            ans = *block;
            break;
        }
    }

    let p2 = format!("{},{}", ans.x, ans.y);

    (p1, p2)
}

#[inline]
fn extract_nums(s: &str) -> impl Iterator<Item = i32> + '_ {
    s.split(|c: char| !c.is_ascii_digit()).filter(|s| !s.is_empty()).flat_map(str::parse::<i32>)
}

aoc_2024::main! {
    solve()
}
