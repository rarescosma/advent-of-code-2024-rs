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
//!
//! Optimization #2: replace Dijsktra with DFS since all edges have equal cost.

use std::collections::VecDeque;

use aoc_2dmap::prelude::{Map, Pos, ORTHOGONAL};
use aoc_prelude::{HashMap, HashSet, Itertools};
use aoc_2024::extract_nums;

const MAP_SIZE: i32 = 71;
const INIT_BLOCKS: usize = 1024;
const START: Pos = Pos::c_new(0, 0);
const GOAL: Pos = Pos::c_new(MAP_SIZE - 1, MAP_SIZE - 1);

struct Buf {
    costs: HashMap<Pos, usize>,
    queue: VecDeque<(usize, Pos)>,
    edges: HashMap<Pos, Pos>,
    path: HashSet<Pos>,
}

impl Buf {
    fn default() -> Self {
        Self {
            costs: HashMap::with_capacity(1024),
            queue: VecDeque::with_capacity(1024),
            edges: HashMap::with_capacity(1024),
            path: HashSet::with_capacity(1024),
        }
    }

    fn clear(&mut self) {
        self.costs.clear();
        self.queue.clear();
        self.edges.clear();
    }
}

fn solve() -> (usize, String) {
    let mut map = Map::<char>::fill((MAP_SIZE, MAP_SIZE), '.');

    let blocks = include_str!("../../inputs/18.in")
        .lines()
        .filter_map(|line| {
            let mut nums = extract_nums::<i32>(line);
            Some(Pos::new(nums.next()?, nums.next()?))
        })
        .collect_vec();

    for block in blocks.iter().take(INIT_BLOCKS) {
        map[block] = '#';
    }

    let mut buf = Buf::default();

    let p1 = dfs(&map, &mut buf).expect("no path!?");

    let mut choke = None;
    for block in blocks.iter().skip(INIT_BLOCKS) {
        map[block] = '#';
        if buf.path.contains(block) && dfs(&map, &mut buf).is_none() {
            choke = Some(block);
            break;
        }
    }

    let p2 = choke.map(|pos| format!("{},{}", pos.x, pos.y)).expect("no block!?");

    (p1, p2)
}

fn dfs(map: &Map<char>, buf: &mut Buf) -> Option<usize> {
    buf.clear();
    buf.queue.push_back((0, START));

    let mut p1 = usize::MAX;

    while let Some((cost, cur)) = buf.queue.pop_back() {
        if cur == GOAL {
            p1 = cost;
            break;
        }
        for step in ORTHOGONAL {
            let next = cur + step;
            if map.get(next) != Some('.') {
                continue;
            }

            let cost = cost + 1;
            let lowest = *buf.costs.get(&next).unwrap_or(&usize::MAX);

            if cost < lowest {
                buf.edges.insert(next, cur);
                buf.costs.insert(next, cost);
                buf.queue.push_back((cost, next));
            }
        }
    }

    if p1 == usize::MAX {
        return None;
    }

    backtrack(buf);
    Some(p1)
}

// updates buf.path to the shortest path
fn backtrack(buf: &mut Buf) {
    let mut cur = GOAL;
    buf.path.clear();
    while let Some(prev) = buf.edges.get(&cur) {
        buf.path.insert(*prev);
        if *prev == START {
            return;
        }
        cur = *prev;
    }
}

aoc_2024::main! {
    solve()
}
