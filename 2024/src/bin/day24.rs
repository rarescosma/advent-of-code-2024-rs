//! # Crossed Wires
//!
//! Still gotta come up with a generic solution for Part 2.
//!
//! But it's Christmas so visual inspection will have to do for now.

use std::{
    collections::VecDeque,
    fmt::Display,
    sync::atomic::{AtomicUsize, Ordering},
};

use aoc_2024::{extract_nums, reverse};
use aoc_prelude::{lazy_static, Entry, HashMap, HashSet, Itertools, PrimInt};

const MAX_BITS: i8 = 45;
const BF_MAX_DEPTH: usize = 6;
const HALF_ADDER_TT: [u8; 16] = [0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0];
const FULL_ADDER_TT: [u8; 16] = [0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1];
const LAST_BIT_TT: [u8; 16] = [0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1];

type Circuit = HashMap<NodeRef, Gate>;

type Signals = HashMap<NodeRef, u8>;

type IdxCache = HashMap<String, NodeRef>;

lazy_static! {
    static ref COUNTER: AtomicUsize = AtomicUsize::new(0);
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NodeRef {
    id: usize,
    is_input: bool,
    is_output: bool,
}

fn idx(node: String, cache: &mut IdxCache) -> NodeRef {
    let node_c = node.clone();
    match cache.entry(node) {
        Entry::Occupied(idx) => *idx.get(),
        Entry::Vacant(entry) => {
            let id = COUNTER.fetch_add(1, Ordering::Relaxed);
            let nr = NodeRef {
                id,
                is_input: node_c.starts_with("x") || node_c.starts_with("y"),
                is_output: node_c.starts_with("z"),
            };
            entry.insert(nr);
            nr
        }
    }
}

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
}

#[derive(Debug, Clone, Copy)]
struct Gate {
    in0: NodeRef,
    in1: NodeRef,
    kind: GateKind,
}

impl Gate {
    fn parse(from: &str, idx_cache: &mut IdxCache) -> Option<(NodeRef, Self)> {
        let mut parts = from.split_ascii_whitespace();
        let in0 = idx(parts.next()?.to_owned(), idx_cache);
        let kind = GateKind::parse(parts.next()?);
        let in1 = idx(parts.next()?.to_owned(), idx_cache);
        let _ = parts.next()?;
        let out = idx(parts.next()?.to_owned(), idx_cache);
        Some((out, Self { in0, in1, kind }))
    }

    fn splat(&mut self, other: Gate) {
        self.in0 = other.in0;
        self.in1 = other.in1;
        self.kind = other.kind;
    }
}

fn solve() -> (u64, String) {
    let (inputs, gates) = include_str!("../../inputs/24.in").split_once("\n\n").unwrap();

    let mut idx_cache = HashMap::new();

    let mut p1_signals: Signals = inputs
        .lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(name, val)| {
            Some((idx(name.to_owned(), &mut idx_cache), extract_nums::<u8>(val).next()?))
        })
        .collect();

    let mut circuit: Circuit =
        gates.lines().filter_map(|line| Gate::parse(line, &mut idx_cache)).collect();

    let rev_idx = reverse(idx_cache.clone());

    evaluate(&circuit, &mut p1_signals);
    let mut p1 = 0u64;
    for k in p1_signals.keys().filter(|k| k.is_output).sorted_by_key(|n| &rev_idx[*n]).rev() {
        p1 = (p1 << 1) | (p1_signals[k] as u64);
    }

    let mut p2 = Vec::new();
    for bit in 0..MAX_BITS {
        p2.extend(fix_bit(bit, &mut circuit, &mut idx_cache).into_iter().map(|n| &rev_idx[&n]));
    }
    p2.sort_unstable();

    (p1, p2.into_iter().join(","))
}

fn fix_bit(bit_pos: i8, circuit: &mut Circuit, idx_cache: &mut IdxCache) -> Vec<NodeRef> {
    let (ok, matching_nodes) = mini_test(bit_pos, circuit, idx_cache).unwrap();

    if ok {
        return Vec::new();
    }

    let output_id = make_id("z", bit_pos, idx_cache);

    if let Some(node_id) = matching_nodes.into_iter().next() {
        swap(circuit, output_id, node_id);
        assert!(matches!(mini_test(bit_pos, circuit, idx_cache), Some((true, _))));
        return vec![output_id, node_id];
    } else {
        let bf_nodes = bf_cands(make_id("z", bit_pos + 1, idx_cache), circuit);
        for i in 0..bf_nodes.len() - 1 {
            for j in i + 1..bf_nodes.len() {
                swap(circuit, bf_nodes[i], bf_nodes[j]);
                if (-1..=1)
                    .all(|off| mini_test(bit_pos + off, circuit, idx_cache).is_some_and(|r| r.0))
                {
                    return vec![bf_nodes[i], bf_nodes[j]];
                }
                swap(circuit, bf_nodes[i], bf_nodes[j]);
            }
        }
    }
    Vec::new()
}

fn mini_test(
    bit_pos: i8,
    circuit: &Circuit,
    idx_cache: &mut IdxCache,
) -> Option<(bool, HashSet<NodeRef>)> {
    let mut truth_tables = HashMap::new();

    let empty = || [0u8; 16];

    let mut idx = 0;

    for x in 0..4 {
        let x = x << (bit_pos - 1).max(0);
        for y in 0..4 {
            let y = y << (bit_pos - 1).max(0);
            for (k, v) in run(x, y, circuit, idx_cache)? {
                if !k.is_input {
                    truth_tables.entry(k).or_insert_with(empty)[idx] = v;
                }
            }
            idx += 1;
        }
    }

    let check_table = match bit_pos {
        0 => HALF_ADDER_TT,
        MAX_BITS => LAST_BIT_TT,
        _ => FULL_ADDER_TT,
    };

    let mut matching_nodes: HashSet<NodeRef> =
        truth_tables.iter().filter(|&(_, tt)| tt == &check_table).map(|(n, _)| *n).collect();

    let ok = matching_nodes.contains(&make_id("z", bit_pos, idx_cache));
    matching_nodes.retain(|n| !n.is_output);

    Some((ok, matching_nodes))
}

fn run(x: u64, y: u64, circuit: &Circuit, idx_cache: &mut IdxCache) -> Option<Signals> {
    let mut signals = Signals::new();

    fn fill(prefix: &str, mut val: u64, signals: &mut Signals, idx_cache: &mut IdxCache) {
        let mut bit_pos = 0;
        while val > 0 {
            signals.insert(make_id(prefix, bit_pos, idx_cache), (val & 1) as u8);
            val >>= 1;
            bit_pos += 1;
        }
        for empty in bit_pos..MAX_BITS {
            signals.insert(make_id(prefix, empty, idx_cache), 0);
        }
    }

    fill("x", x, &mut signals, idx_cache);
    fill("y", y, &mut signals, idx_cache);

    evaluate(circuit, &mut signals)?;
    Some(signals)
}

fn evaluate(circuit: &Circuit, signals: &mut Signals) -> Option<()> {
    fn inner<'a: 'b, 'b>(
        circuit: &'a Circuit,
        node: &NodeRef,
        signals: &Signals,
        seen: HashSet<NodeRef>,
        cache: &mut HashMap<NodeRef, u8>,
    ) -> Option<u8> {
        if cache.contains_key(node) {
            return Some(cache[node]);
        }

        let gate = circuit[node];
        if seen.contains(&gate.in0) || seen.contains(&gate.in1) {
            return None;
        }

        let left = if gate.in0.is_input {
            signals[&gate.in0]
        } else {
            let mut new_seen = seen.clone();
            new_seen.insert(gate.in0);
            inner(circuit, &gate.in0, signals, new_seen, cache)?
        };

        let right = if gate.in1.is_input {
            signals[&gate.in1]
        } else {
            let mut new_seen = seen.clone();
            new_seen.insert(gate.in0);
            inner(circuit, &gate.in1, signals, new_seen, cache)?
        };

        let ret = gate.kind.eval(left, right);
        cache.insert(*node, ret);
        Some(ret)
    }

    let mut cache = HashMap::new();
    for node in circuit.keys() {
        if !node.is_input {
            signals.insert(*node, inner(circuit, node, signals, HashSet::new(), &mut cache)?);
        }
    }
    Some(())
}

fn bf_cands(start_node: NodeRef, circuit: &Circuit) -> Vec<NodeRef> {
    let mut q = VecDeque::from([(0, start_node)]);

    let mut ret = Vec::new();

    while let Some((depth, node)) = q.pop_front() {
        if depth < BF_MAX_DEPTH && !node.is_input && circuit.contains_key(&node) {
            ret.push(node);
            let gate = circuit[&node];
            q.push_back((depth + 1, gate.in0));
            q.push_back((depth + 1, gate.in1));
        }
    }
    ret
}

fn swap(circuit: &mut Circuit, a: NodeRef, b: NodeRef) {
    let (l, r) = (*circuit.get(&a).unwrap(), *circuit.get(&b).unwrap());
    circuit.get_mut(&a).unwrap().splat(r);
    circuit.get_mut(&b).unwrap().splat(l);
}

fn make_id<I: PrimInt + Display>(prefix: &str, bit_pos: I, idx_cache: &mut IdxCache) -> NodeRef {
    idx(format!("{}{:0>2}", prefix, bit_pos), idx_cache)
}

aoc_2024::main! {
    solve()
}
