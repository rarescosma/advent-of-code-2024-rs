//! # Linen Layout
//!
//! Prefix match + dp counting.
//!
//! Prefix match is slow => rayon.
//!
//! Have to come up with a better way to tally, but that's not actually
//! the bulk of the run time.
//!
//! Optimization #1: sets for the win! Instead of iterating through patterns,
//! take slices of the remaining string and check if they're in the pattern
//! set.
//!
//! Optimization #2: don't need no queue to recurse, we can simply walk the towel
//! from start to end and build the `ways` array as we go.

use aoc_prelude::HashSet;

const MAX_SIZE: usize = 64;

struct Counter<'a> {
    patterns: &'a HashSet<&'a str>,
    max_len: usize,
    cache: [usize; MAX_SIZE],
}

impl<'a> Counter<'a> {
    fn new(patterns: &'a HashSet<&'a str>) -> Self {
        let max_len = patterns.iter().map(|p| p.len()).max().expect("no patterns");
        Self { patterns, max_len, cache: [0; MAX_SIZE] }
    }

    fn num_arrangements(&mut self, towel: &str) -> usize {
        let size = towel.len();
        let ways = &mut self.cache;
        ways.fill(0);
        ways[0] = 1;

        for start in 0..size {
            if ways[start] > 0 {
                for pat_len in 1..=self.max_len.min(size - start) {
                    let end = start + pat_len;
                    if self.patterns.contains(&towel[start..end]) {
                        ways[end] += ways[start];
                    }
                }
            }
        }

        ways[size]
    }
}

fn solve() -> (usize, usize) {
    let (parts, towels) = include_str!("../../inputs/19.in").split_once("\n\n").unwrap();

    let parts: HashSet<_> = parts.split(", ").collect();
    let mut counter = Counter::new(&parts);

    towels
        .lines()
        .map(|towel| {
            let ans = counter.num_arrangements(towel);
            ((ans > 0) as usize, ans)
        })
        .fold((0, 0), |acc, cur| (acc.0 + cur.0, acc.1 + cur.1))
}

aoc_2024::main! {
    solve()
}
