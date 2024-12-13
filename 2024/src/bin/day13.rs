//! # Claw Contraption
//!
//! Friday difficulty: solve systems of two equations using substitution.

type Int = i64;

use aoc_prelude::num_integer::Integer;

fn solve() -> (Int, Int) {
    let (p1, p2) = include_str!("../../inputs/13.in")
        .split("\n\n")
        .filter_map(|lines| {
            let mut lines = lines.lines();
            let e1 = extract_nums(lines.next()?)?;
            let e2 = extract_nums(lines.next()?)?;
            let r = extract_nums(lines.next()?)?;
            Some((
                solve_eq(e1, e2, r).map(token_total),
                solve_eq(e1, e2, [r[0] + 10000000000000, r[1] + 10000000000000]).map(token_total),
            ))
        })
        .fold((0, 0), |acc, res| {
            (acc.0 + res.0.unwrap_or(0), acc.1 + res.1.unwrap_or(0))
        });

    (p1, p2)
}

#[inline]
fn extract_nums(s: &str) -> Option<[Int; 2]> {
    let mut it = s
        .split_ascii_whitespace()
        .flat_map(|part| part.split("+"))
        .flat_map(|part| part.split(","))
        .flat_map(|part| part.split("="))
        .flat_map(str::parse::<Int>);
    Some([it.next()?, it.next()?])
}

#[inline]
fn solve_eq(e1: [Int; 2], e2: [Int; 2], r: [Int; 2]) -> Option<(Int, Int)> {
    let up = e2[1] * r[0] - e2[0] * r[1];
    let down = e2[1] * e1[0] - e1[1] * e2[0];
    let (a, rem) = up.div_rem(&down);
    (rem == 0).then(|| (a, (r[0] - e1[0] * a) / e2[0]))
}

#[inline]
fn token_total(ab: (Int, Int)) -> Int {
    ab.0 * 3 + ab.1
}

aoc_2024::main! {
    solve()
}
