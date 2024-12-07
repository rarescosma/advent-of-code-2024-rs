use aoc_prelude::num_integer::Integer;
use aoc_prelude::Itertools;

type Int = u64;
type Base = u8;
const TEN: Int = 10;

#[derive(Copy, Clone, Debug)]
enum Ops {
    Add,
    Mul,
    Concat,
}

impl From<Base> for Ops {
    fn from(value: Base) -> Self {
        match value {
            0 => Self::Add,
            1 => Self::Mul,
            2 => Self::Concat,
            _ => unreachable!(),
        }
    }
}

impl Ops {
    fn can_proceed(self, exp: Int, operand: Int) -> Option<Int> {
        match self {
            Ops::Add => (exp >= operand).then(|| exp - operand),
            Ops::Mul => (exp % operand == 0).then(|| exp / operand),
            Ops::Concat => {
                let (d, r) = (exp - operand).div_rem(&TEN.pow(grade(operand)));
                (r == 0).then_some(d)
            }
        }
    }
}

fn solve() -> (Int, Int) {
    let mut results = Vec::new();
    let mut operands = Vec::new();

    include_str!("../../inputs/07.in").lines().for_each(|line| {
        let (first, rest) = line.split_once(":").unwrap();
        results.push(first.parse::<Int>().unwrap());
        operands.push(
            rest.split_whitespace()
                .filter_map(|el| el.parse::<Int>().ok())
                .collect_vec(),
        )
    });

    let p1 = results
        .iter()
        .enumerate()
        .filter_map(|(idx, &exp)| {
            if op_loop_cons::<2>(exp, &operands[idx]) {
                Some(exp)
            } else {
                None
            }
        })
        .sum();

    let p2 = results
        .into_iter()
        .enumerate()
        .filter_map(|(idx, exp)| {
            if op_loop_cons::<3>(exp, &operands[idx]) {
                Some(exp)
            } else {
                None
            }
        })
        .sum();

    (p1, p2)
}

fn op_loop_cons<const B: Base>(expected: Int, operands: &[Int]) -> bool {
    match operands {
        [] => false,
        [last] => expected == *last,
        [rest @ .., last] => (0..B).map(Ops::from).any(|op| {
            op.can_proceed(expected, *last)
                .is_some_and(|prev_expected| op_loop_cons::<B>(prev_expected, rest))
        }),
    }
}

fn grade(what: Int) -> u32 {
    let mut sr = what;
    let mut ans = 1;
    while sr >= 10 {
        ans += 1;
        sr /= 10;
    }
    ans
}

aoc_2024::main! {
    solve()
}
