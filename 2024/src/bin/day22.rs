//! # Monkey Market
//!
//! Fancy brute-force with rayon parallelism + updating the chunk tally as
//! we go. Hashmaps replaced with flat arrays, indexed by four -9 <-> +9
//! integers.

use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
    thread::available_parallelism,
};

use aoc_2024::extract_nums;
use aoc_prelude::Itertools;
use rayon::prelude::*;

const MOD: Int = (1 << 24) - 1;
const NUM_KEYS: usize = 19usize.pow(4);

type Int = u64;
type Key = [i8; 4];
type Map = [u16; NUM_KEYS];

fn solve() -> (Int, u16) {
    let input = include_str!("../../inputs/22.in");
    let nums = input.lines().filter_map(|l| extract_nums(l).next()).collect_vec();

    let tally = Mutex::new([0u16; NUM_KEYS]);
    let p1 = AtomicU64::new(0);

    nums.chunks((nums.len() / available_parallelism().unwrap().get()) + 1)
        .par_bridge()
        .map(process_chunk)
        .for_each(|(total, res)| {
            let mut tally = tally.lock().unwrap();
            for (idx, el) in tally.iter_mut().enumerate() {
                *el += res[idx];
            }
            p1.fetch_add(total, Ordering::Relaxed);
        });

    (p1.load(Ordering::Relaxed), tally.into_inner().unwrap().into_iter().max().unwrap())
}

fn hash(n: Int) -> Int {
    let mut n = n;

    n = (n ^ n << 6) & MOD;
    n = (n ^ n >> 5) & MOD;
    (n ^ n << 11) & MOD
}

fn process_chunk(chunk: &[Int]) -> (Int, Map) {
    let mut total = 0;
    let mut res = [0u16; NUM_KEYS];
    let mut seen = [u16::MAX; NUM_KEYS];

    for (buyer_id, initial) in chunk.iter().enumerate() {
        let buyer_id = buyer_id as u16;

        let mut p = initial % 10;
        let mut n = *initial;

        let mut key = [0; 4];

        for j in 0..2000 {
            n = hash(n);
            let new_p = n % 10;
            let delta = new_p - p;
            if j < 4 {
                key[j] = delta as i8;
            } else {
                (key[0], key[1], key[2]) = (key[1], key[2], key[3]);
                key[3] = delta as i8;
                let idx = index(key);
                if seen[idx] != buyer_id {
                    seen[idx] = buyer_id;
                    res[idx] += new_p as u16;
                }
            }
            p = new_p;
        }
        total += n;
    }
    (total, res)
}

fn index(key: Key) -> usize {
    let mut idx = (key[3] + 9) as usize;
    idx = 19 * idx + (key[2] + 9) as usize;
    idx = 19 * idx + (key[1] + 9) as usize;
    19 * idx + (key[0] + 9) as usize
}

aoc_2024::main! {
    solve()
}
