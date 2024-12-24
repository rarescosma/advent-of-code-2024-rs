//! # Crossed Wires
//!
//! Still gotta come up with a generic solution for Part 2.
//!
//! But it's Christmas so visual inspection will have to do for now.

use std::collections::{BTreeMap, VecDeque};

use aoc_2024::extract_nums;
use aoc_prelude::Itertools;

type Circuit<'a> = BTreeMap<&'a str, Gate<'a>>;

#[derive(Debug, Clone, Copy)]
enum GateKind {
    Or,
    And,
    Xor,
}

impl GateKind {
    fn parse(from: &str) -> Self {
        match from {
            "OR" => Self::Or,
            "AND" => Self::And,
            "XOR" => Self::Xor,
            _ => panic!(),
        }
    }

    fn eval(&self, in0: u8, in1: u8) -> u8 {
        match self {
            GateKind::Or => in0 | in1,
            GateKind::And => in0 & in1,
            GateKind::Xor => in0 ^ in1,
        }
    }

    fn repr(&self) -> String {
        match self {
            GateKind::Or => "OR",
            GateKind::And => "AND",
            GateKind::Xor => "XOR",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone, Copy)]
struct Gate<'a> {
    in0: &'a str,
    in1: &'a str,
    out: &'a str,
    kind: GateKind,
}

impl<'a> Gate<'a> {
    fn parse(from: &'a str) -> Option<Self> {
        let mut parts = from.split_ascii_whitespace();
        let in0 = parts.next()?;
        let kind = GateKind::parse(parts.next()?);
        let in1 = parts.next()?;
        let _ = parts.next()?;
        let out = parts.next()?;
        Some(Self { in0, in1, out, kind })
    }

    fn eval(&self, signals: &BTreeMap<&str, u8>) -> Option<u8> {
        let in0 = signals.get(self.in0)?;
        let in1 = signals.get(self.in1)?;

        Some(self.kind.eval(*in0, *in1))
    }

    fn splat(&mut self, other: Gate<'a>) {
        self.in0 = other.in0;
        self.in1 = other.in1;
        self.kind = other.kind;
    }

    fn repr(&self, out_to_gate: &BTreeMap<&str, Gate>) -> String {
        let in0 = if self.in0.starts_with("x") || self.in0.starts_with("y") {
            self.in0
        } else {
            &out_to_gate[self.in0].repr(out_to_gate)
        };
        let in1 = if self.in1.starts_with("x") || self.in1.starts_with("y") {
            self.in1
        } else {
            &out_to_gate[self.in1].repr(out_to_gate)
        };
        let ord = if in0 > in1 { [in0, in1] } else { [in1, in0] };
        format!("({} {} {})", ord[0], self.kind.repr(), ord[1])
    }
}

fn solve() -> (u64, String) {
    let (inputs, gates) = include_str!("../../inputs/24.in").split_once("\n\n").unwrap();

    let mut signals = inputs
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(name, val)| (name, extract_nums::<u8>(val).next().unwrap()))
        .collect::<BTreeMap<_, _>>();

    let gates = gates.lines().filter_map(Gate::parse).collect_vec();

    let mut circuit: Circuit = gates.iter().copied().map(|g| (g.out, g)).collect();

    let mut stack = VecDeque::from_iter(gates.into_iter().filter(|g| g.out.starts_with("z")));

    while let Some(out) = stack.pop_front() {
        match out.eval(&signals) {
            None => {
                stack.push_front(out);
                stack.push_front(circuit[out.in0]);
                stack.push_front(circuit[out.in1]);
            }
            Some(signal) => {
                signals.insert(out.out, signal);
            }
        }
    }

    let mut p1 = 0u64;
    for (name, sig) in signals.iter().sorted().rev() {
        if name.starts_with("z") {
            p1 = (p1 << 1) | (*sig as u64);
        }
    }

    let mut swaps = Vec::new();
    swap(&mut circuit, "vss", "z14", &mut swaps);
    swap(&mut circuit, "z31", "kpp", &mut swaps);
    swap(&mut circuit, "z35", "sgj", &mut swaps);
    swap(&mut circuit, "hjf", "kdh", &mut swaps);

    let p2 = swaps.iter().sorted().join(",");

    (p1, p2)
}

fn swap<'b, 'a: 'b>(circuit: &mut Circuit, a: &'a str, b: &'a str, swaps: &'b mut Vec<&'a str>) {
    let (l, r) = (*circuit.get(a).unwrap(), *circuit.get(b).unwrap());
    circuit.get_mut(a).unwrap().splat(r);
    circuit.get_mut(b).unwrap().splat(l);
    swaps.push(a);
    swaps.push(b);
}

#[allow(dead_code)]
fn is_valid(otg: &BTreeMap<&str, Gate>) -> bool {
    otg.keys().clone().filter(|k| k.starts_with("z")).sorted_unstable().all(|k| {
        let z_index = extract_nums::<u8>(k).next().unwrap();
        let formula = otg[k].repr(otg);
        let does_not_depend_on_higher_bits =
            (z_index + 1..45).all(|higher| !formula.contains(&format!("{:02}", higher)));
        let depends_on_all_lower_bits =
            (0..=z_index.min(44)).all(|lower| formula.contains(&format!("{:02}", lower)));
        does_not_depend_on_higher_bits & depends_on_all_lower_bits
    })
}

aoc_2024::main! {
    solve()
}
