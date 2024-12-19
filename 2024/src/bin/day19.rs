//! # Linen Layout
//!
//! Prefix match + dp counting.
//!
//! Prefix match is slow => rayon.
//!
//! Have to come up with a better way to tally, but that's not actually
//! the bulk of the run time.
//!
//! Optimization #1: sets for the win! Instead of iterating through patterns
//! take slices of the remaining string and check if they're in the pattern
//! set.

use std::collections::VecDeque;

use aoc_prelude::{HashSet, Itertools};
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

struct Counter<'a> {
    patterns: &'a HashSet<&'a str>,
    max_len: usize,
    cache: [Option<usize>; SET_SIZE],
    bit_set: BitSet,
}

impl<'a> Counter<'a> {
    fn new(patterns: &'a HashSet<&'a str>, max_len: usize) -> Self {
        Self { patterns, max_len, cache: [None; SET_SIZE], bit_set: Default::default() }
    }

    fn num_arrangements(&mut self, t: &str) -> usize {
        let t_len = t.len();
        self.bit_set.clear();
        self.cache.fill(None);

        let mut q = VecDeque::from([0]);
        while let Some(slice_start) = q.pop_front() {
            for pat_len in 1..=self.max_len.min(t_len - slice_start) {
                let slice_end = slice_start + pat_len;
                let slice = &t[slice_start..slice_end];
                if self.patterns.contains(slice) {
                    if self.cache[slice_end].is_none() {
                        q.push_back(slice_end);
                        self.cache[slice_end] = Some(1);
                    }
                    self.bit_set.set((slice_start, slice_end - 1));
                }
            }
        }

        self.cache.fill(None);
        self.tally(0, t_len)
    }

    fn tally(&mut self, skip_rows: usize, t_len: usize) -> usize {
        if let Some(cached) = self.cache[skip_rows] {
            return cached;
        }
        let mut ans = self.bit_set.get((skip_rows, t_len - 1)) as usize;
        for col_idx in skip_rows..=t_len {
            if self.bit_set.get((skip_rows, col_idx)) {
                ans += self.tally(col_idx + 1, t_len);
            }
        }
        self.cache[skip_rows] = Some(ans);
        ans
    }
}

fn solve() -> (usize, usize) {
    let (parts, towels) = include_str!("../../inputs/19.in").split_once("\n\n").unwrap();

    let parts: HashSet<_> = parts.split(", ").collect();
    let max_len = parts.iter().map(|p| p.len()).max().unwrap();

    towels
        .lines()
        .collect_vec()
        .into_par_iter()
        .map(|towel| {
            let ans = Counter::new(&parts, max_len).num_arrangements(towel);
            ((ans > 0) as usize, ans)
        })
        .reduce(|| (0, 0), |acc, cur| (acc.0 + cur.0, acc.1 + cur.1))
}

aoc_2024::main! {
    solve()
}
