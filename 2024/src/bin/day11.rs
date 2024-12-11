//! # Plutonian Pebbles
//!
//! Shove the pebble tally in a HashMap and accumulate changes in a Vec, then
//! apply them sequentially.

use aoc_prelude::num_integer::Integer;
use aoc_prelude::HashMap;

type Int = u64;
const TEN: Int = 10u64;

fn solve() -> (Int, Int) {
    let mut tally: HashMap<Int, Int> = include_str!("../../inputs/11.in")
        .trim()
        .split_ascii_whitespace()
        .filter_map(|el| el.parse::<Int>().ok().map(|key| (key, 1)))
        .collect();

    let mut changes = Vec::with_capacity(16384);

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

enum Change {
    Decr(Int, Int),
    Incr(Int, Int),
}

impl Change {
    fn apply(&self, tally: &mut HashMap<Int, Int>) {
        match self {
            Change::Decr(k, delta) => {
                *tally.get_mut(k).unwrap() -= delta;
            }
            Change::Incr(k, delta) => {
                *tally.entry(*k).or_insert(0) += delta;
            }
        }
    }
}

fn num_digits(num: Int) -> u32 {
    num.checked_ilog10().unwrap_or(0) + 1
}

fn split(num: Int) -> Option<(Int, Int)> {
    let digits = num_digits(num);
    let (q, r) = digits.div_rem(&2);
    (r == 0).then(|| num.div_rem(&TEN.pow(q)))
}

fn epoch(tally: &mut HashMap<Int, Int>, changes: &mut Vec<Change>) {
    changes.clear();

    // all zeroes become ones
    if let Some(num_zeroes) = tally.remove(&0) {
        changes.push(Change::Incr(1, num_zeroes));
    }

    for (&key, &val) in tally.iter() {
        if val > 0 {
            changes.push(Change::Decr(key, val));

            if let Some((left, right)) = split(key) {
                changes.push(Change::Incr(left, val));
                changes.push(Change::Incr(right, val));
            } else {
                changes.push(Change::Incr(key * 2024, val));
            }
        }
    }

    for change in changes {
        change.apply(tally);
    }
}

aoc_2024::main! {
    solve()
}
