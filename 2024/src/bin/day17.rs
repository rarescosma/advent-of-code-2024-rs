//! # Chronospatial Computer
//!
//! Part 1: code-monkey.
//!
//! Part 2: the "A-ha" moment was realising a digit is only determined
//! by 3 bits of the "a" number and no matter how far left those digits get
//! shifted, the output digit stays the same.
//!
//! At each step we produce candidates that output the required digit,
//! then use these as the new "initial" values for the next iteration.

use std::mem;

use aoc_prelude::Itertools;

type Int = u64;

fn eval(a: Int, b: Int, c: Int, ix: &[Int]) -> Int {
    let (mut a, mut b, mut c) = (a, b, c);

    let mut ip = 0;

    let mut output = 0;

    loop {
        let (it, op) = (ix[ip], ix[ip + 1]);

        let combo = |op: Int| match op {
            0..=3 => op,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!(),
        };

        match it {
            0 => a >>= combo(op),
            1 => b ^= op,
            2 => b = combo(op) % 8,
            3 => {
                if a == 0 {
                    return output;
                } else {
                    ip = op as usize;
                }
            }
            4 => b ^= c,
            5 => output = output * 10 + combo(op) % 8,
            6 => b = a >> combo(op),
            7 => c = a >> combo(op),
            _ => unreachable!(),
        }

        if it != 3 {
            ip += 2;
        }
    }
}

fn solve() -> (String, Int) {
    let (regs, ix) = include_str!("../../inputs/17.in").split_once("\n\n").unwrap();

    let [a, b, c]: [Int; 3] =
        regs.lines().filter_map(|x| extract_nums(x).next()).collect_vec().try_into().unwrap();

    let ix = extract_nums(ix).collect_vec();

    let output = eval(a, b, c, &ix);
    let p1 = format!("{}", output).chars().join(",");

    let target = ix.iter().fold(0, |acc, digit| acc * 10 + digit);

    let mut initials = vec![0];
    let mut new_initials = Vec::with_capacity(16);
    for digit in 0..=target.ilog10() {
        let look_for = target % 10u64.pow(digit + 1);
        new_initials.clear();
        for initial in initials.iter() {
            let shifted = *initial << 3;
            new_initials.extend(
                (0..8)
                    .map(|offset| shifted + offset)
                    .filter(|&cand| eval(cand, b, c, &ix) == look_for),
            );
        }
        mem::swap(&mut initials, &mut new_initials);
    }
    let p2 = initials.into_iter().min().expect("no solution?!");
    (p1, p2)
}

#[inline]
fn extract_nums(s: &str) -> impl Iterator<Item = Int> + '_ {
    s.split(|c: char| !c.is_ascii_digit()).filter(|s| !s.is_empty()).flat_map(str::parse::<Int>)
}

aoc_2024::main! {
    solve()
}
