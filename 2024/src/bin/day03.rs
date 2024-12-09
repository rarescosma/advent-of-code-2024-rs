//! # Mull It Over
//!
//! Solves both parts simultaneously using
//! [regex](https://en.wikipedia.org/wiki/Regular_expression).
//!
//! Uses a state machine to tell if we're actively computing for Part 2.
use aoc_prelude::*;
use regex::{Captures, Regex};

lazy_static! {
    static ref INSTR_REGEX: Regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
}

enum Instr {
    Enable,
    Disable,
    Mul(usize, usize),
    Unknown,
}

impl From<Captures<'_>> for Instr {
    fn from(cap: Captures) -> Self {
        match cap.get(0) {
            Some(group) if group.as_str() == "do()" => Self::Enable,
            Some(group) if group.as_str() == "don't()" => Self::Disable,
            _ => (|| {
                let a: usize = cap.get(1)?.as_str().parse().ok()?;
                let b: usize = cap.get(2)?.as_str().parse().ok()?;
                Some(Self::Mul(a, b))
            })()
            .unwrap_or(Self::Unknown),
        }
    }
}

struct State {
    enabled: bool,
    p1: usize,
    p2: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            enabled: true,
            p1: 0,
            p2: 0,
        }
    }
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/03.in");
    let state =
        INSTR_REGEX
            .captures_iter(input)
            .map(Instr::from)
            .fold(State::default(), |acc, instr| match instr {
                Instr::Enable => State {
                    enabled: true,
                    ..acc
                },
                Instr::Disable => State {
                    enabled: false,
                    ..acc
                },
                Instr::Mul(a, b) => {
                    let res = a * b;
                    State {
                        p1: acc.p1 + res,
                        p2: acc.p2 + res * usize::from(acc.enabled),
                        enabled: acc.enabled,
                    }
                }
                Instr::Unknown => acc,
            });

    (state.p1, state.p2)
}

aoc_2024::main! {
    solve()
}
