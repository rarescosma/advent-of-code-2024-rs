//! # Red-Nosed Reports
//!
//! Part 1: check if adjacent level are all-increasing or all-decreasing and
//! that the delta is at least 1 and at most 3.
//!
//! Part 2: we can make a level safe by removing _at most_ one element.
//! Figure out which additional levels become safe.
use aoc_prelude::Itertools;

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/02.in")
        .lines()
        .map(|line| line.split_whitespace().map(|el| el.parse().unwrap()).collect_vec())
        .collect_vec();

    let p1 = input.clone().into_iter().filter(is_valid).count();

    let p2 = input
        .into_iter()
        .filter(|row| is_valid(row) || permute(row).any(|del_row| is_valid(&del_row)))
        .count();

    (p1, p2)
}

fn permute(row: &[i32]) -> impl Iterator<Item = Vec<i32>> + '_ {
    (0..row.len()).map(|idx| {
        let mut cp = row.to_owned();
        cp.remove(idx);
        cp
    })
}

fn is_valid(row: &Vec<i32>) -> bool {
    let rev = row.clone().into_iter().rev().collect_vec();

    let mut sorted = row.clone();
    sorted.sort_unstable();

    sorted.iter().zip(sorted.iter().skip(1)).all(|(left, right)| (1..=3).contains(&(right - left)))
        && (row == &sorted || rev == sorted)
}

aoc_2024::main! {
    solve()
}
