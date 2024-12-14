//! # Restroom Redoubt
//!
//! Part 1: simple modulo arithmetics.
//!
//! Part 2: detected a cycle, inspected the output manually, and noticed
//! the Pine Tree is surrounded by a frame of bots.
//!
//! Used a `u16` bit set to store the robot positions, and on each store
//! we check if the shard is equal to `u16::MAX` which means we have
//! 16 bots in a row, which means we found the frame.
//!
//! Rayon + atomics to for parallel search.

use aoc_2dmap::prelude::Pos;
use aoc_prelude::num_integer::Integer;
use aoc_prelude::Itertools;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread::available_parallelism;

const MAP_SIZE: Pos = Pos::c_new(101, 103);
const MID: Pos = Pos::c_new(50, 51);
const CYCLE_LEN: usize = 101 * 103;
const SET_SIZE: usize = ((MAP_SIZE.x * MAP_SIZE.y) as usize / 16) + 1;

struct BitSet<const N: usize> {
    inner: [u16; N],
}

impl<const N: usize> Default for BitSet<N> {
    fn default() -> Self {
        Self { inner: [0; N] }
    }
}

impl<const N: usize> BitSet<N> {
    // returns true if the shard becomes all ones
    fn set(&mut self, p: &Pos) -> bool {
        let idx = (p.y * MAP_SIZE.x + p.x) as usize;
        let (shard, shift) = idx.div_rem(&16);
        let word = &mut self.inner[shard];
        *word |= 1 << shift;
        *word == u16::MAX
    }

    fn clear(&mut self) {
        self.inner.fill(0);
    }
}

fn solve() -> (usize, usize) {
    let mut robots = Vec::with_capacity(500);
    let mut speeds = Vec::with_capacity(500);

    include_str!("../../inputs/14.in")
        .lines()
        .flat_map(|line| {
            line.split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .flat_map(str::parse::<i32>)
                .collect_tuple::<(_, _, _, _)>()
                .map(|(px, py, vx, vy)| (Pos::from((px, py)), Pos::from((vx, vy))))
        })
        .for_each(|(robot, speed)| {
            robots.push(robot);
            speeds.push(speed);
        });

    let p1 = into_quadrants(
        robots
            .iter()
            .enumerate()
            .map(|(idx, pos)| fast_forward_pos(*pos, speeds[idx], 100)),
    );

    let num_threads = available_parallelism().unwrap().get().max(32);
    let found = AtomicBool::new(false);
    let p2 = AtomicUsize::new(0);

    (0..num_threads).into_par_iter().for_each(|offset| {
        let mut bit_set = BitSet::<SET_SIZE>::default();
        let mut i = 0;
        loop {
            let check = i * num_threads + offset;
            if check > CYCLE_LEN || found.load(Ordering::Relaxed) {
                return;
            }
            for (rob_idx, rob) in robots.iter().enumerate() {
                if bit_set.set(&fast_forward_pos(*rob, speeds[rob_idx], check as i32)) {
                    p2.store(check, Ordering::Relaxed);
                    found.store(true, Ordering::Relaxed);
                    return;
                }
            }
            i += 1;
            bit_set.clear();
        }
    });

    (p1, p2.load(Ordering::Relaxed))
}

fn into_quadrants(robots: impl Iterator<Item = Pos>) -> usize {
    let (up, down): (Vec<Pos>, Vec<Pos>) = robots.partition(|robot| robot.y < MID.y);
    let (first, second): (Vec<Pos>, Vec<Pos>) = up.iter().partition(|robot| robot.x < MID.x);
    let (third, fourth): (Vec<Pos>, Vec<Pos>) = down.iter().partition(|robot| robot.x < MID.x);

    first.len() * second.len() * third.len() * fourth.len()
}

fn fast_forward_pos(pos: Pos, speed: Pos, turns: i32) -> Pos {
    let px = fast_forward_coord(pos.x, speed.x, turns, MAP_SIZE.x);
    let py = fast_forward_coord(pos.y, speed.y, turns, MAP_SIZE.y);
    Pos::new(px, py)
}

fn fast_forward_coord(initial: i32, speed: i32, turns: i32, map_size: i32) -> i32 {
    let mut res = (initial + speed * turns) % map_size;
    if res < 0 {
        res += map_size;
    }
    res
}

aoc_2024::main! {
    solve()
}
