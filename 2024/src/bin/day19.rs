//! # Linen Layout
//!
//! Prefix match + dp counting.
//!
//! Prefix match is slow => rayon.
//!
//! Have to come up with a better way to tally, but that's not actually
//! the bulk of the run time.

use std::collections::VecDeque;

use aoc_prelude::Itertools;
use rayon::prelude::*;

const SET_SIZE: usize = 64;

#[derive(Debug)]
struct BitSet {
    buckets: [u64; SET_SIZE],
}

impl Default for BitSet {
    fn default() -> Self { Self { buckets: [0; SET_SIZE] } }
}

impl BitSet {
    fn set(&mut self, p: (usize, usize)) { self.buckets[p.0] |= 1 << p.1; }
    fn get(&self, p: (usize, usize)) -> bool { (self.buckets[p.0] >> p.1) & 1 == 1 }
    fn clear(&mut self) { self.buckets.fill(0) }
}

fn solve() -> (usize, usize) {
    let (parts, towels) = include_str!("../../inputs/19.in").split_once("\n\n").unwrap();

    let parts = parts.split(", ").collect_vec();

    towels
        .lines()
        .collect_vec()
        .into_par_iter()
        .map(|towel| {
            let mut bit_set = BitSet::default();
            let mut cache = [None; SET_SIZE];

            let ans = num_arrangements(towel, &parts, &mut bit_set, &mut cache);
            ((ans > 0) as usize, ans)
        })
        .reduce(|| (0, 0), |acc, cur| (acc.0 + cur.0, acc.1 + cur.1))
}

fn num_arrangements(
    t: &str,
    px: &[&str],
    bit_set: &mut BitSet,
    cache: &mut [Option<usize>; SET_SIZE],
) -> usize {
    let t_len = t.len();
    bit_set.clear();
    cache.fill(None);

    let mut q = VecDeque::from([0]);
    while let Some(slice_start) = q.pop_front() {
        for &prefix in px {
            let slice_end = slice_start + prefix.len();
            if slice_end <= t_len && &t[slice_start..slice_end] == prefix {
                if cache[slice_end].is_none() {
                    q.push_back(slice_end);
                    cache[slice_end] = Some(1);
                }
                bit_set.set((slice_start, slice_end - 1));
            }
        }
    }

    cache.fill(None);
    tally(0, t_len, bit_set, cache)
}

fn tally(
    skip_rows: usize,
    t_len: usize,
    dp: &BitSet,
    cache: &mut [Option<usize>; SET_SIZE],
) -> usize {
    if let Some(cached) = cache[skip_rows] {
        return cached;
    }
    let mut ans = dp.get((skip_rows, t_len - 1)) as usize;
    for col_idx in skip_rows..=t_len {
        if dp.get((skip_rows, col_idx)) {
            ans += tally(col_idx + 1, t_len, dp, cache);
        }
    }
    cache[skip_rows] = Some(ans);
    ans
}

aoc_2024::main! {
    solve()
}
