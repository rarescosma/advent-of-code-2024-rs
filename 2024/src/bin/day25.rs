//! # Code Chronicle
//!
//! Thunk about something fancy involving hashed differences, but the brute
//! runs in microseconds, so there we go.
//!
//! Happy solstice! 🎄

use std::iter::once;

use aoc_prelude::Itertools;

fn solve() -> (usize, &'static str) {
    let input = include_str!("../../inputs/25.in");

    let mut tumblers = Vec::with_capacity(300);
    let mut keys = Vec::with_capacity(300);

    let mut buf = [-1i8; 5];

    input.split("\n\n").for_each(|pat| {
        let mut lines = pat.lines();
        let first = lines.next().unwrap();
        let is_tumbler = first.chars().all(|c| c == '#');

        buf.fill(-1i8);

        lines.chain(once(first)).for_each(|l| {
            l.chars().enumerate().for_each(|(idx, c)| {
                if c == '#' {
                    buf[idx] += 1;
                }
            });
        });

        if is_tumbler {
            tumblers.push(buf)
        } else {
            keys.push(buf);
        }
    });

    let p1 = tumblers
        .iter()
        .cartesian_product(keys.iter())
        .filter(|&(t, k)| t.iter().zip(k).all(|(t, k)| t + k <= 5))
        .count();

    (p1, "💚")
}

aoc_2024::main! {
    solve()
}
