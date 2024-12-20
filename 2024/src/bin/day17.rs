//! # Chronospatial Computer
//!
//! Part 1: code-monkey.
//!
//! Part 2: the "A-ha" moment was realising an output digit is only determined
//! by 3 bits of the "A" registry and no matter how far left those digits get
//! shifted, the output digit stays the same.
//!
//! At each step we produce candidates that output the required digit,
//! then use these as the new "initial" values for the next iteration, in
//! which they'll get shifted 3 positions to the left.
//!
//! So, if A = 0b111_011_100_101
//!             |-+-|-+-|
//!               |   |
//!               |   |
//! last digit <--+   |
//! next to last <----+

use std::mem;

use aoc_prelude::Itertools;

type Int = u64;

const BLOCK_SIZE: Int = 3;

fn solve() -> (String, Int) {
    let (reg_lines, program_lines) = include_str!("../../inputs/17.in").split_once("\n\n").unwrap();

    let mut regs = [0; 3];
    reg_lines
        .lines()
        .filter_map(|line| extract_nums(line).next())
        .enumerate()
        .for_each(|(idx, el)| regs[idx] = el);
    let [a, b, c] = regs;

    let program = extract_nums(program_lines).collect_vec();

    let p1 = format!("{}", eval(a, b, c, &program)).chars().join(",");

    let target = program.iter().fold(0, |acc, digit| acc * 10 + digit);

    let mut a_candidates = vec![0];
    let mut next_a_candidates = Vec::with_capacity(16);

    for digit in 0..=target.ilog10() {
        let look_for = target % Int::pow(10, digit + 1);
        next_a_candidates.clear();
        for msbs in a_candidates.iter().map(|bits| *bits << BLOCK_SIZE) {
            next_a_candidates.extend(
                (0..1 << BLOCK_SIZE)
                    .map(|lsbs| msbs + lsbs)
                    .filter(|&a| eval(a, b, c, &program) == look_for),
            );
        }
        mem::swap(&mut a_candidates, &mut next_a_candidates);
    }

    let p2 = a_candidates.into_iter().min().expect("no solution?!");

    (p1, p2)
}

fn eval(a: Int, b: Int, c: Int, program: &[Int]) -> Int {
    let (mut a, mut b, mut c) = (a, b, c);

    let mut ip = 0;

    let mut output = 0;

    loop {
        let (it, op) = (program[ip], program[ip + 1]);

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

#[inline]
fn extract_nums(s: &str) -> impl Iterator<Item = Int> + '_ {
    s.split(|c: char| !c.is_ascii_digit()).filter(|s| !s.is_empty()).flat_map(str::parse::<Int>)
}

aoc_2024::main! {
    solve()
}
