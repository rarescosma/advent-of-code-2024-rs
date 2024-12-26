//! # Guard Gallivant
//!
//! Brute-force, unfortunately. Could be sped up by pre-computing a "jump table" so we teleport
//! the guard when hitting an obstacle instead of incrementing its position 1 by 1.
//!
//! Part 2: the big win is only trying to put obstacles in the positions walked by the guard
//! during part 1.
//!
//! Gotcha for part 2: when using a Vec as boolean array, you gotta `.fill(false)` instead of
//! `.clear()` it.
//!
//! Optimized to use teleport maps.

use std::thread::available_parallelism;

use aoc_2dmap::prelude::*;
use aoc_prelude::{num_integer::Integer, Itertools};
use rayon::prelude::*;

type Teleport = Vec<Pos>;

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/06.in");
    let map_size = Pos::from((
        input.bytes().position(|x| x == b'\n').unwrap(),
        input.bytes().filter(|x| *x == b'\n').count(),
    ));

    let map = Map::new(map_size, input.bytes().filter(|&c| c != b'\n'));
    let start = find_tile(&map, b'^');
    let teleport = make_teleport(&map);

    let mut pos = start;
    let mut dir = 0;

    let mut visited = vec![false; (map.size.x * map.size.y) as usize];
    while map.within(pos + ORTHOGONAL[dir]) {
        let new_pos = teleport[key(pos, dir, map.size)];

        for x in pos.x.min(new_pos.x)..=new_pos.x.max(pos.x) {
            for y in pos.y.min(new_pos.y)..=new_pos.y.max(pos.y) {
                if map.within(Pos::new(x, y)) {
                    visited[(x * map.size.y + y) as usize] = true;
                }
            }
        }

        pos = new_pos;
        dir = turn_right(dir);
    }

    let p1 = visited.iter().filter(|&x| *x).count();

    let obstacles = visited
        .iter()
        .enumerate()
        .filter(|x| *x.1)
        .map(|(hash, _)| {
            let (x, y) = hash.div_rem(&(map.size.y as usize));
            Pos::from((x, y))
        })
        .collect_vec();

    let p2 = obstacles
        .chunks(obstacles.len() / available_parallelism().unwrap().get())
        .par_bridge()
        .map(|chunk| {
            let mut seen = vec![false; (map.size.x * map.size.y) as usize * 4];
            chunk
                .iter()
                .map(|obs| has_cycle(&map, start, *obs, &teleport, &mut seen) as usize)
                .sum::<usize>()
        })
        .sum();

    (p1, p2)
}

fn make_teleport(map: &Map<u8>) -> Teleport {
    let mut teleport = vec![Pos::new(0, 0); (map.size.x * map.size.y) as usize * 4];
    // Initially, all points teleport outside the map
    for x in 0..map.size.x {
        for y in 0..map.size.y {
            for dir in 0..4 {
                let out = match ORTHOGONAL[dir] {
                    NORTH => Pos::new(x, -1),
                    WEST => Pos::new(-1, y),
                    SOUTH => Pos::new(x, map.size.y),
                    EAST => Pos::new(map.size.x, y),
                    _ => unreachable!(),
                };
                teleport[key(Pos::new(x, y), dir, map.size)] = out;
            }
        }
    }

    // Every obstacle acts as a "black hole", pulling guards towards it
    for (obs, _) in map.iter().map(|p| (p, map[p])).filter(|(_, c)| *c == b'#') {
        for dir in 0..4 {
            // We're moving *away* from the obstacle, so the teleport key is for the opposite
            let mut cur = obs + ORTHOGONAL[dir];
            while map.within(cur) && map[cur] != b'#' {
                teleport[key(cur, turn_back(dir), map.size)] = obs + ORTHOGONAL[dir];
                cur += ORTHOGONAL[dir];
            }
        }
    }
    teleport
}

fn has_cycle(map: &Map<u8>, start: Pos, obs: Pos, teleport: &Teleport, seen: &mut [bool]) -> bool {
    seen.fill(false);
    let mut cur = start;
    let mut dir = 0;

    while map.within(cur) {
        let k_ = key(cur, dir, map.size);
        if seen[k_] {
            return true;
        } else {
            seen[k_] = true;
        }

        let mut next = teleport[k_];

        // 😱
        if (cur.x == next.x
            && next.x == obs.x
            && ((cur.y < obs.y && obs.y <= next.y) || (cur.y > obs.y && obs.y >= next.y)))
            || (cur.y == next.y
                && next.y == obs.y
                && ((cur.x < obs.x && obs.x <= next.x) || (cur.x > obs.x && obs.x >= next.x)))
        {
            next = obs - ORTHOGONAL[dir];
        }

        dir = turn_right(dir);
        cur = next;
    }
    false
}

#[inline]
fn find_tile(map: &Map<u8>, tile: u8) -> Pos { map.iter().find(|pos| map[pos] == tile).unwrap() }

#[inline]
fn key(p: Pos, dir: usize, map_size: Pos) -> usize { ((p.x * map_size.y + p.y) * 4) as usize + dir }

#[inline]
fn turn_right(dir: usize) -> usize { (dir + 1) % 4 }

#[inline]
fn turn_back(dir: usize) -> usize { (dir + 2) % 4 }

aoc_2024::main! {
    solve()
}
