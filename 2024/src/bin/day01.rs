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
    l1.sort();
    l2.sort();

    (l1, l2)
}

fn solve() -> (i32, i32) {
    let (left, right) = read_input();

    let p1 = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, e| acc + e.0.sub(e.1).abs());

    let mut freq = HashMap::new();
    right.into_iter().for_each(|e| {
        *freq.entry(e).or_insert(0) += 1;
    });

    let p2 = left
        .iter()
        .fold(0, |acc, e| acc + e * freq.get(e).unwrap_or(&0));

    (p1, p2)
}

aoc_2024::main! {
    solve()
}
