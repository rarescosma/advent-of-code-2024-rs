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
const MAX_CHEAT: i32 = 20;
const MIN_SAVING: i32 = 100;

struct Buf {
    path: Vec<Pos>,
    costs: Vec<i32>,
}

impl Default for Buf {
    fn default() -> Self {
        Self { path: Vec::with_capacity(10000), costs: vec![-1; MAP_SIZE * MAP_SIZE] }
    }
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/20.in");

    let map_size = Pos::from((
        input.chars().position(|x| x == '\n').unwrap(),
        input.chars().filter(|x| *x == '\n').count(),
    ));
    let mut map = Map::new(map_size, input.chars().filter(|&c| c != '\n'));

    let start = find_tile(&map, 'S');
    let goal = find_tile(&map, 'E');
    map[start] = '.';
    map[goal] = '.';

    let buf = dfs(&map, start, goal).expect("no path!?");
    buf.path
        .into_par_iter()
        .map(|pos| find_cheats(pos, &buf.costs))
        .reduce(|| (0, 0), |acc, val| (acc.0 + val.0, acc.1 + val.1))
}

fn dfs(map: &Map<char>, start: Pos, goal: Pos) -> Option<Buf> {
    let mut buf = Buf::default();
    let mut queue = VecDeque::with_capacity(1024);
    queue.push_back((0, start));

    while let Some((cost, cur)) = queue.pop_back() {
        buf.path.push(cur);
        buf.costs[index(&cur)] = cost;

        if cur == goal {
            return Some(buf);
        }

        for step in ORTHOGONAL {
            let next = cur + step;

            if map.get(next) == Some('.') {
                let idx = index(&next);

                if buf.costs[idx] == -1 {
                    queue.push_back((cost + 1, next));
                }
            }
        }
    }
    None
}

fn find_cheats(pos: Pos, costs: &[i32]) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    let cur_idx = index(&pos);

    // generate the Manhattan rhomboid of radius MAX_CHEAT around `pos`
    for x_off in 1..=MAX_CHEAT {
        for y_off in 0..=(MAX_CHEAT - x_off) {
            let dist = x_off + y_off;

            // prune the first layer - cheat size of 1 is not really a cheat,
            // the DFS will go there anyway
            if dist < 2 {
                continue;
            }

            for offset in rotations(x_off, y_off) {
                let new_pos = pos + offset;

                if in_bounds(&new_pos) {
                    let new_idx = index(&new_pos);

                    if costs[new_idx] != -1 && costs[new_idx] + dist + MIN_SAVING <= costs[cur_idx]
                    {
                        if dist == 2 {
                            p1 += 1;
                        }
                        p2 += 1;
                    }
                }
            }
        }
    }
    (p1, p2)
}

#[inline(always)]
fn find_tile(map: &Map<char>, tile: char) -> Pos {
    map.iter().find(|pos| map[pos] == tile).unwrap()
}

#[inline(always)]
fn index(pos: &Pos) -> usize { (pos.y as usize) * MAP_SIZE + (pos.x as usize) }

#[inline(always)]
fn rotations(x: i32, y: i32) -> [Pos; 4] {
    [Pos::new(x, y), Pos::new(-y, x), Pos::new(-x, -y), Pos::new(y, -x)]
}

#[inline(always)]
fn in_bounds(pos: &Pos) -> bool {
    pos.x > 0 && pos.y > 0 && pos.x < (MAP_SIZE as i32) && pos.y < (MAP_SIZE as i32)
}

aoc_2024::main! {
    solve()
}
