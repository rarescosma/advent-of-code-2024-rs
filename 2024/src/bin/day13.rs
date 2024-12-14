//! # Claw Contraption
//!
//! Friday difficulty: solve systems of two equations using substitution.

type Int = i64;
type Pair = (Int, Int);

use aoc_prelude::num_integer::Integer;

fn solve() -> (Int, Int) {
    let (p1, p2) = include_str!("../../inputs/13.in")
        .split("\n\n")
        .filter_map(|lines| {
            let mut lines = lines.lines();
            let e0 = extract_nums(lines.next()?)?;
            let e1 = extract_nums(lines.next()?)?;
            let r = extract_nums(lines.next()?)?;
            Some((
                solve_eq(e0, e1, r).map(token_total),
                solve_eq(e0, e1, (r.0 + 10000000000000, r.1 + 10000000000000)).map(token_total),
            ))
        })
        .fold((0, 0), |acc, res| {
            (acc.0 + res.0.unwrap_or(0), acc.1 + res.1.unwrap_or(0))
        });

    (p1, p2)
}

#[inline]
fn extract_nums(s: &str) -> Option<Pair> {
    let mut it = s
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .flat_map(str::parse::<Int>);
    Some((it.next()?, it.next()?))
}

#[inline]
fn solve_eq(e0: Pair, e1: Pair, r: Pair) -> Option<Pair> {
    let (a, rem) = (e1.1 * r.0 - e1.0 * r.1).div_rem(&(e1.1 * e0.0 - e0.1 * e1.0));
    (rem == 0).then(|| (a, (r.0 - e0.0 * a) / e1.0))
}

#[inline]
fn token_total((a, b): Pair) -> Int {
    a * 3 + b
}

aoc_2024::main! {
    solve()
}
