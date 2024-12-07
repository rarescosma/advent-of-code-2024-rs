use aoc_prelude::Itertools;

#[derive(Copy, Clone, Debug)]
enum Ops {
    Add,
    Mul,
    Concat,
}

impl From<u32> for Ops {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Add,
            1 => Self::Mul,
            2 => Self::Concat,
            _ => unreachable!(),
        }
    }
}

fn ops_iter<const B: u32>(ops: u32, operands_len: usize) -> impl Iterator<Item = Ops> {
    let mut sr = ops;

    (0..operands_len - 1).map(move |_| {
        if B == 2 {
            let op = sr & 0b1;
            sr >>= 1;
            op.into()
        } else {
            let op = sr % B;
            sr /= B;
            op.into()
        }
    })
}

fn solve() -> (u128, u128) {
    let mut results = Vec::new();
    let mut operands = Vec::new();

    include_str!("../../inputs/07.in").lines().for_each(|line| {
        let (first, rest) = line.split_once(":").unwrap();
        results.push(first.parse::<u128>().unwrap());
        operands.push(
            rest.split_whitespace()
                .filter_map(|el| el.parse::<u128>().ok())
                .collect_vec(),
        )
    });

    let p1 = results
        .iter()
        .enumerate()
        .filter_map(|(idx, &exp)| {
            if op_loop::<2>(exp, &operands[idx]) {
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
            if op_loop::<3>(exp, &operands[idx]) {
                Some(exp)
            } else {
                None
            }
        })
        .sum();

    (p1, p2)
}

fn op_loop<const B: u32>(expected: u128, operands: &[u128]) -> bool {
    for ops in 0..=((B.pow(operands.len() as u32 - 1)) - 1) {
        if op_test::<B>(operands, ops) == expected {
            return true;
        }
    }
    false
}

fn op_test<const B: u32>(operands: &[u128], ops: u32) -> u128 {
    let mut res = operands[0];
    for (idx, op) in ops_iter::<B>(ops, operands.len()).enumerate() {
        match op {
            Ops::Add => res += operands[idx + 1],
            Ops::Mul => res *= operands[idx + 1],
            Ops::Concat => res = res * 10u128.pow(grade(operands[idx + 1])) + operands[idx + 1],
        }
    }
    res
}

fn grade(what: u128) -> u32 {
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
