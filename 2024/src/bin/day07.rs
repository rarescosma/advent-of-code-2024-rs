use aoc_prelude::num_integer::Integer;

type Int = u64;
type Op = u8;

const ADD: Op = 0;
const MUL: Op = 1;
const CAT: Op = 2;

fn solve() -> (Int, Int) {
    let mut operands = Vec::with_capacity(20);

    let mut p1 = 0;
    let mut p2 = 0;

    include_str!("../../inputs/07.in").lines().for_each(|line| {
        let (first, rest) = line.split_once(":").unwrap();
        let expected = first.parse::<Int>().unwrap();
        operands.clear();
        operands.extend(
            rest.split_ascii_whitespace()
                .filter_map(|el| el.parse::<Int>().ok()),
        );

        if check::<MUL>(expected, &operands) {
            p1 += expected;
            p2 += expected
        } else if check::<CAT>(expected, &operands) {
            p2 += expected;
        }
    });

    (p1, p2)
}

#[inline(never)]
fn check<const O: Op>(expected: Int, operands: &[Int]) -> bool {
    match operands {
        [] => false,
        [last] => expected == *last,
        [rest @ .., last] => (0..=O).any(|op| {
            can_proceed(op, expected, *last)
                .is_some_and(|prev_expected| check::<O>(prev_expected, rest))
        }),
    }
}

fn can_proceed(op: Op, exp: Int, operand: Int) -> Option<Int> {
    match op {
        ADD => (exp >= operand).then(|| exp - operand),
        MUL => (exp % operand == 0).then(|| exp / operand),
        CAT => {
            let (d, r) = (exp - operand).div_rem(&grade(operand));
            (r == 0).then_some(d)
        }
        _ => unreachable!(),
    }
}

#[inline]
fn grade(what: Int) -> Int {
    if what < 10 {
        10
    } else if what < 100 {
        100
    } else {
        1000
    }
}

aoc_2024::main! {
    solve()
}
