//! # Plutonian Pebbles
//!
//! Shove the pebble tally in a HashMap and accumulate changes in a Vec, then
//! apply them sequentially.

use aoc_prelude::num_integer::Integer;
use aoc_prelude::{ArrayVec, HashMap};

type Int = i64;
const TENS: [Int; 10] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
];

fn solve() -> (Int, Int) {
    let mut tally: HashMap<Int, Int> = include_str!("../../inputs/11.in")
        .trim()
        .split_ascii_whitespace()
        .filter_map(|el| el.parse::<Int>().ok().map(|key| (key, 1)))
        .collect();

    let mut changes = ArrayVec::<(Int, Int), 16384>::new();

    for _ in 0..25 {
        epoch(&mut tally, &mut changes);
    }
    let p1: Int = tally.values().sum();

    for _ in 0..50 {
        epoch(&mut tally, &mut changes);
    }
    let p2: Int = tally.values().sum();

    (p1, p2)
}

fn num_digits(num: Int) -> u32 {
    num.checked_ilog10().unwrap_or(0) + 1
}

fn split(num: Int) -> Option<(Int, Int)> {
    let digits = num_digits(num);
    let (q, r) = digits.div_rem(&2);
    (r == 0).then(|| num.div_rem(&TENS[q as usize]))
}

fn epoch(tally: &mut HashMap<Int, Int>, changes: &mut ArrayVec<(Int, Int), 16384>) {
    changes.clear();

    // all zeroes become ones
    if let Some(num_zeroes) = tally.remove(&0) {
        changes.push((1, num_zeroes));
    }

    for (&key, &val) in tally.iter() {
        if val > 0 {
            changes.push((key, -val));

            if let Some((left, right)) = split(key) {
                changes.push((left, val));
                changes.push((right, val));
            } else {
                changes.push((key * 2024, val));
            }
        }
    }

    for (k, delta) in changes {
        *tally.entry(*k).or_insert(0) += *delta;
    }
}

aoc_2024::main! {
    solve()
}
