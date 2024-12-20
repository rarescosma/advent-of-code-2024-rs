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
    path: Vec<Pos>,
    on_path: Vec<bool>,
}

impl Default for Buf {
    fn default() -> Self {
        Self {
            costs: vec![u32::MAX; MAP_SIZE * MAP_SIZE],
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

    let buf = dfs(&map, start, goal);
    buf.path
        .into_par_iter()
        .map(|pos| find_cheats(pos, &buf.costs, &buf.on_path))
        .reduce(|| (0, 0), |acc, val| (acc.0 + val.0, acc.1 + val.1))
}

fn dfs(map: &Map<char>, start: Pos, goal: Pos) -> Buf {
    let mut buf = Buf::default();
    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back((0, start));

    while let Some((cost, cur)) = queue.pop_back() {
        buf.costs[index(cur)] = cost;
        buf.path.push(cur);
        buf.on_path[index(cur)] = true;

        if cur == goal {
            return buf;
        }

        for step in ORTHOGONAL {
            let next = cur + step;

            if map.get(next) != Some('.') {
                continue;
            }

            let idx = index(next);

            if buf.costs[idx] == u32::MAX {
                queue.push_back((cost + 1, next));
            }
        }
    }
    buf
}

fn find_cheats(pos: Pos, costs: &[u32], on_path: &[bool]) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    let cur_idx = index(pos);

    for x_off in 1..=MAX_CHEAT {
        for y_off in 0..=(MAX_CHEAT - x_off) {
            let dist = (x_off + y_off) as u32;
            if dist < 2 {
                continue;
            }

            let mut offset = Pos::new(x_off, y_off);

            for _ in 0..4 {
                let new_pos = pos + offset;

                if in_bounds(new_pos) {
                    let new_idx = index(new_pos);

                    if on_path[new_idx] && costs[new_idx] + dist + MIN_SAVING <= costs[cur_idx] {
                        if dist == 2 {
                            p1 += 1;
                        }
                        p2 += 1;
                    }
                }

                offset = offset.clockwise();
            }
        }
    }
    (p1, p2)
}

#[inline(always)]
fn in_bounds(pos: Pos) -> bool {
    pos.x > 0 && pos.y > 0 && pos.x < (MAP_SIZE as i32) && pos.y < (MAP_SIZE as i32)
}

#[inline(always)]
fn index(pos: Pos) -> usize { (pos.y as usize) * MAP_SIZE + (pos.x as usize) }

#[inline(always)]
fn find_tile(map: &Map<char>, tile: char) -> Pos {
    map.iter().find(|pos| map[pos] == tile).unwrap()
}

aoc_2024::main! {
    solve()
}
