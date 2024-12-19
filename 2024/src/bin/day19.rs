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
//! Optimization #2: don't need no queue to recurse, we can simply walk the design
//! from start to end and build the `tally` array as we go.

use aoc_prelude::HashSet;

const MAX_SIZE: usize = 64;

type PatternSet<'a> = HashSet<&'a [u8]>;

struct Counter<'a> {
    patterns: PatternSet<'a>,
    max_len: usize,
    tally: [u64; MAX_SIZE],
}

impl<'a> Counter<'a> {
    fn new(patterns: PatternSet<'a>) -> Self {
        let max_len = patterns.iter().map(|p| p.len()).max().expect("no patterns");
        Self { patterns, max_len, tally: [0; MAX_SIZE] }
    }

    fn count_ways(&mut self, design: &[u8]) -> u64 {
        let size = design.len();
        let tally = &mut self.tally;
        tally.fill(0);
        tally[0] = 1;

        for start in 0..size {
            if tally[start] > 0 {
                for pat_len in 1..=self.max_len.min(size - start) {
                    let end = start + pat_len;
                    if self.patterns.contains(&design[start..end]) {
                        tally[end] += tally[start];
                    }
                }
            }
        }

        tally[size]
    }
}

fn solve() -> (u64, u64) {
    let (patterns, designs) = include_str!("../../inputs/19.in").split_once("\n\n").unwrap();

    let mut counter = Counter::new(patterns.split(", ").map(str::as_bytes).collect());

    designs
        .lines()
        .map(|towel| {
            let ans = counter.count_ways(towel.as_bytes());
            ((ans > 0) as u64, ans)
        })
        .fold((0, 0), |acc, cur| (acc.0 + cur.0, acc.1 + cur.1))
}

aoc_2024::main! {
    solve()
}
