//! # Print Queue
//!
//! The input is constructed so that each possible pair that occurs in a row has a defined
//! precedence that enables sorting with a custom `.sort_by` closure. Numbers are always
//! 2 digits so storing the `before_set` in a fixed size 100 x 100 array is faster than
//! using a `HashMap`.
use std::cmp::Ordering;

fn make_map<const N: usize>() -> [[bool; N]; N] {
    [[false; N]; N]
}

fn solve() -> (usize, usize) {
    let (ord, rep) = include_str!("../../inputs/05.in")
        .split_once("\n\n")
        .unwrap();

    let mut before_set = make_map::<100>();

    ord.lines().for_each(|line| {
        let (left, right) = line.split_once('|').unwrap();
        let r_num = right.parse::<usize>().unwrap();
        let l_num = left.parse::<usize>().unwrap();
        before_set[l_num][r_num] = true;
    });

    let mut p1 = 0;
    let mut p2 = 0;

    let mut rep_buf = Vec::with_capacity(100);

    rep.lines().for_each(|line| {
        rep_buf.clear();
        rep_buf.extend(line.split(',').filter_map(|n| n.parse::<usize>().ok()));

        let mut is_valid = true;

        rep_buf.sort_by(|&left, &right| {
            if before_set[left][right] {
                is_valid = false;
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        let mid = rep_buf[rep_buf.len() / 2];
        if is_valid {
            p1 += mid;
        } else {
            p2 += mid;
        }
    });

    (p1, p2)
}

aoc_2024::main! {
    solve()
}
