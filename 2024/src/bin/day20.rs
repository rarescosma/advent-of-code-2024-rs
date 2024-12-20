//! # Race Condition
//!
//! Brute force: we compute a "blast radius" for each tile on the path and
//! check if a tile within this radius is a valid cheat.
//!
//! If the costs associated with taking the cheat (Manhattan distance) plus
//! the cost of the shunted tile is less than the cost of the current tile =>
//! it's a valid cheat.

use std::collections::VecDeque;

use aoc_2dmap::prelude::*;
use rayon::prelude::*;

const MAP_SIZE: usize = 141;
const MAX_CHEAT: usize = 20;
const MIN_SAVING: u32 = 100;

struct Buf {
    costs: Vec<u32>,
    queue: VecDeque<(u32, Pos)>,
    path: Vec<Pos>,
    on_path: Vec<bool>,
}

impl Default for Buf {
    fn default() -> Self {
        Self {
            costs: vec![u32::MAX; MAP_SIZE * MAP_SIZE],
            queue: VecDeque::with_capacity(1024),
            path: Vec::with_capacity(10000),
            on_path: vec![false; MAP_SIZE * MAP_SIZE],
        }
    }
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/20.in");
    let map_size = Pos::from((
        input.chars().position(|x| x == '\n').unwrap(),
        input.chars().filter(|x| *x == '\n').count(),
    ));
    let mut map = Map::new(map_size, input.chars().filter(|&c| c != '\n'));

    let goal = find_tile(&map, 'E');
    let start = find_tile(&map, 'S');
    map.set(goal, '.');
    map.set(start, '.');

    let mut buf = Buf::default();

    dfs(&map, &mut buf, start, goal);
    find_shunts(buf)
}

fn find_shunts(buf: Buf) -> (usize, usize) {
    buf.path
        .into_par_iter()
        .map(|pos| {
            let mut p2 = 0;
            let mut p1 = 0;
            for x_off in 1..=MAX_CHEAT {
                for y_off in 0..=(MAX_CHEAT - x_off) {
                    if x_off + y_off <= 1 {
                        continue;
                    }
                    let mut offset = Pos::new(x_off, y_off);
                    for _ in 0..4 {
                        let new_pos = pos + offset;
                        if new_pos.x > 0
                            && new_pos.y > 0
                            && new_pos.x < (MAP_SIZE as i32)
                            && new_pos.y < (MAP_SIZE as i32)
                        {
                            let idx = index(new_pos);
                            let dist = (x_off + y_off) as u32;
                            if buf.on_path[idx]
                                && buf.costs[idx] + dist + MIN_SAVING <= buf.costs[index(pos)]
                            {
                                p2 += 1;
                                if dist == 2 {
                                    p1 += 1;
                                }
                            }
                        }
                        offset = offset.clockwise();
                    }
                }
            }
            (p1, p2)
        })
        .reduce(|| (0, 0), |acc, val| (acc.0 + val.0, acc.1 + val.1))
}

fn dfs(map: &Map<char>, buf: &mut Buf, start: Pos, goal: Pos) {
    buf.queue.push_back((0, start));

    while let Some((cost, cur)) = buf.queue.pop_back() {
        buf.costs[index(cur)] = cost;
        buf.path.push(cur);
        buf.on_path[index(cur)] = true;
        if cur == goal {
            return;
        }
        for step in ORTHOGONAL {
            let next = cur + step;
            if map.get(next) != Some('.') {
                continue;
            }

            let idx = index(next);
            if buf.costs[idx] == u32::MAX {
                buf.queue.push_back((cost + 1, next));
            }
        }
    }
}

#[inline(always)]
fn index(p: Pos) -> usize { (p.y as usize) * MAP_SIZE + (p.x as usize) }

#[inline(always)]
fn find_tile(map: &Map<char>, tile: char) -> Pos {
    map.iter().find(|pos| map[pos] == tile).unwrap()
}

aoc_2024::main! {
    solve()
}
