//! # Historian Hysteria
//!
//! Part 1: Pairing up numbers from the left/right lists in increasing order
//! and accumulating the differences.
//!
//! Part 2: How often does each number from the left list appear in the right
//! list? Accumulate the frequencies.
use aoc_prelude::HashMap;
use std::ops::Sub;

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();

    include_str!("../../inputs/01.in").lines().for_each(|line| {
        let mut it = line.split_whitespace();

        l1.push(it.next().unwrap().parse().unwrap());
        l2.push(it.next().unwrap().parse().unwrap());
    });

    (l1, l2)
}

fn solve() -> (i32, i32) {
    let (mut left, mut right) = read_input();
    left.sort_unstable();
    right.sort_unstable();

    let p1 = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, e| acc + e.0.sub(e.1).abs());

    let mut freq = HashMap::new();
    for e in right {
        *freq.entry(e).or_insert(0) += 1;
    }

    let p2 = left
        .iter()
        .fold(0, |acc, e| acc + e * freq.get(e).unwrap_or(&0));

    (p1, p2)
}

aoc_2024::main! {
    solve()
}
