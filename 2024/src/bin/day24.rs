//! # Crossed Wires
//!
//! Still gotta come up with a generic solution for Part 2.
//!
//! But it's Christmas so visual inspection will have to do for now.

use std::{
    collections::VecDeque,
    fmt::Display,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};

use aoc_2024::{extract_nums, reverse};
use aoc_prelude::{lazy_static, Entry, HashMap, HashSet, Itertools, PrimInt};

const MAX_BITS: i8 = 45;
const MAX_NODES: usize = 400;
const BF_MAX_DEPTH: usize = 6;
const HALF_ADDER_TT: [u8; 16] = [0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0];
const FULL_ADDER_TT: [u8; 16] = [0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1];
const LAST_BIT_TT: [u8; 16] = [0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1];

lazy_static! {
    static ref COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref IDX_CACHE: Mutex<HashMap<String, SigRef>> = Mutex::new(HashMap::new());
}

type Circuit = [Gate; MAX_NODES];
type Signals = [Signal; MAX_NODES];
type SigRef = usize;

struct State {
    circuit: Circuit,
    signals: Signals,
    inputs: [SigRef; MAX_NODES],
    size: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            circuit: [Gate::default(); MAX_NODES],
            signals: [Signal::default(); MAX_NODES],
            inputs: [usize::MAX; MAX_NODES],
            size: 0,
        }
    }
}

impl State {
    fn zero_signals(&mut self) { self.signals.iter_mut().for_each(|s| s.val = 0); }

    fn set_size(&mut self, size: usize) { self.size = size; }

    fn evaluate(&mut self) -> Option<()> {
        fn inner(
            this: &mut State,
            node: SigRef,
            seen: [bool; MAX_NODES],
            cache: &mut [u8],
        ) -> Option<u8> {
            if cache[node] != u8::MAX {
                return Some(cache[node]);
            }

            let gate = this.circuit[node];
            if seen[gate.in0] || seen[gate.in1] {
                return None;
            }

            let left = if this.signals[gate.in0].is_input {
                this.signals[gate.in0].val
            } else {
                let mut new_seen = seen;
                new_seen[gate.in0] = true;
                inner(this, gate.in0, new_seen, cache)?
            };

            let right = if this.signals[gate.in1].is_input {
                this.signals[gate.in1].val
            } else {
                let mut new_seen = seen;
                new_seen[gate.in1] = true;
                inner(this, gate.in1, new_seen, cache)?
            };

            let ret = gate.kind.eval(left, right);
            cache[node] = ret;
            Some(ret)
        }

        let mut cache = [u8::MAX; MAX_NODES];
        for n_ref in 0..self.size {
            if !self.signals[n_ref].is_input {
                self.signals[n_ref].val = inner(self, n_ref, [false; MAX_NODES], &mut cache)?;
            }
        }
        Some(())
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Signal {
    is_input: bool,
    is_output: bool,
    val: u8,
}

impl Signal {
    fn from_str(name: &str) -> (SigRef, Self) {
        let is_input = name.starts_with("x") || name.starts_with("y");
        let is_output = name.starts_with("z");
        let sig_ref = idx(name.to_owned());
        (sig_ref, Self { is_input, is_output, val: 0 })
    }
}

#[derive(Copy, Clone, Debug, Default)]
enum GateKind {
    #[default]
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

#[derive(Debug, Clone, Copy, Default)]
struct Gate {
    in0: SigRef,
    in1: SigRef,
    kind: GateKind,
}

impl Gate {
    fn splat(&mut self, other: Gate) {
        self.in0 = other.in0;
        self.in1 = other.in1;
        self.kind = other.kind;
    }
}

fn solve() -> (u64, String) {
    let (inputs, gates) = include_str!("../../inputs/24.in").split_once("\n\n").unwrap();

    let mut state = State::default();

    gates
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_ascii_whitespace();

            let in0 = register_signal(parts.next()?, &mut state.signals, &mut state.inputs);
            let kind = GateKind::parse(parts.next()?);
            let in1 = register_signal(parts.next()?, &mut state.signals, &mut state.inputs);
            let _ = parts.next()?;
            let out_ref = register_signal(parts.next()?, &mut state.signals, &mut state.inputs);

            Some((out_ref, Gate { in0, in1, kind }))
        })
        .for_each(|(id, gate)| {
            state.circuit[id] = gate;
        });

    inputs.lines().for_each(|line| {
        if let Some((sig_ref, val)) = (|| {
            let (name, val) = line.split_once(": ")?;
            let sig_ref = register_signal(name, &mut state.signals, &mut state.inputs);
            Some((sig_ref, extract_nums::<u8>(val).next()?))
        })() {
            state.signals[sig_ref].val = val;
        }
    });

    let idx_cache = IDX_CACHE.lock().unwrap().clone();
    state.set_size(idx_cache.len());
    state.evaluate();

    let rev_idx = reverse(idx_cache);

    let mut p1 = 0u64;
    for k in
        (0..state.size).filter(|n| state.signals[*n].is_output).sorted_by_key(|n| &rev_idx[n]).rev()
    {
        p1 = (p1 << 1) | (state.signals[k].val as u64);
    }

    let mut p2 = Vec::new();
    for bit in 0..MAX_BITS {
        p2.extend(fix_bit(bit, &mut state).into_iter().map(|n| &rev_idx[&n]));
    }
    p2.sort_unstable();

    (p1, p2.into_iter().join(","))
}

fn fix_bit(bit_pos: i8, state: &mut State) -> Vec<SigRef> {
    let (ok, matching_nodes) = mini_test(bit_pos, state).unwrap();

    if ok {
        return Vec::new();
    }

    let output_id = make_id("z", bit_pos);

    if let Some(node_id) = matching_nodes.into_iter().next() {
        swap(state, output_id, node_id);
        assert!(matches!(mini_test(bit_pos, state), Some((true, _))));
        return vec![output_id, node_id];
    } else {
        let bf_nodes = bf_cands(make_id("z", bit_pos + 1), state);
        for i in 0..bf_nodes.len() - 1 {
            for j in i + 1..bf_nodes.len() {
                swap(state, bf_nodes[i], bf_nodes[j]);
                if (-1..=1).all(|off| mini_test(bit_pos + off, state).is_some_and(|r| r.0)) {
                    return vec![bf_nodes[i], bf_nodes[j]];
                }
                swap(state, bf_nodes[i], bf_nodes[j]);
            }
        }
    }
    Vec::new()
}

fn mini_test(bit_pos: i8, state: &mut State) -> Option<(bool, HashSet<SigRef>)> {
    let mut truth_tables = [[0u8; 16]; MAX_NODES];

    let mut idx = 0;

    for x in 0..4 {
        let x = x << (bit_pos - 1).max(0);
        for y in 0..4 {
            let y = y << (bit_pos - 1).max(0);
            run(x, y, state)?;
            truth_tables.iter_mut().enumerate().take(state.size).for_each(|(sig_ref, table)| {
                let sig = state.signals[sig_ref];
                if !sig.is_input {
                    table[idx] = sig.val;
                }
            });
            idx += 1;
        }
    }

    let check_table = match bit_pos {
        0 => HALF_ADDER_TT,
        MAX_BITS => LAST_BIT_TT,
        _ => FULL_ADDER_TT,
    };

    let mut matching_nodes: HashSet<SigRef> = truth_tables
        .iter()
        .enumerate()
        .filter(|&(_, tt)| tt == &check_table)
        .map(|(n, _)| n)
        .collect();

    let ok = matching_nodes.contains(&make_id("z", bit_pos));
    matching_nodes.retain(|n| !state.signals[*n].is_output);

    Some((ok, matching_nodes))
}

fn run(x: u64, y: u64, state: &mut State) -> Option<()> {
    state.zero_signals();

    fn fill(is_y: bool, mut val: u64, signals: &mut Signals, inputs: &mut [SigRef; MAX_NODES]) {
        let mut bit_pos = 0;
        while val > 0 {
            let idx = if is_y { inputs[bit_pos + (MAX_BITS as usize)] } else { inputs[bit_pos] };
            signals[idx].val = (val & 1) as u8;
            val >>= 1;
            bit_pos += 1;
        }
    }

    fill(false, x, &mut state.signals, &mut state.inputs);
    fill(true, y, &mut state.signals, &mut state.inputs);

    state.evaluate()
}

fn bf_cands(start_node: SigRef, state: &State) -> Vec<SigRef> {
    let mut q = VecDeque::from([(0, start_node)]);

    let mut ret = Vec::new();

    while let Some((depth, sig_ref)) = q.pop_front() {
        let sig = state.signals[sig_ref];
        if depth < BF_MAX_DEPTH && !sig.is_input {
            ret.push(sig_ref);
            let gate = state.circuit[sig_ref];
            q.push_back((depth + 1, gate.in0));
            q.push_back((depth + 1, gate.in1));
        }
    }
    ret
}

fn idx(node: String) -> SigRef {
    let mut cache = IDX_CACHE.lock().unwrap();
    match cache.entry(node) {
        Entry::Occupied(idx) => *idx.get(),
        Entry::Vacant(entry) => {
            let id = COUNTER.fetch_add(1, Ordering::Relaxed);
            entry.insert(id);
            id
        }
    }
}

fn register_signal(sig: &str, signals: &mut Signals, inputs: &mut [SigRef; MAX_NODES]) -> SigRef {
    let (ref0, in0) = Signal::from_str(sig);
    if in0.is_input {
        let mut idx = extract_nums::<usize>(sig).next().unwrap();
        if sig.starts_with("y") {
            idx += MAX_BITS as usize;
        }
        inputs[idx] = ref0;
    }
    signals[ref0] = in0;
    ref0
}

fn swap(state: &mut State, a: SigRef, b: SigRef) {
    let (l, r) = (state.circuit[a], state.circuit[b]);
    state.circuit[a].splat(r);
    state.circuit[b].splat(l);
}

fn make_id<I: PrimInt + Display>(prefix: &str, bit_pos: I) -> SigRef {
    idx(format!("{}{:0>2}", prefix, bit_pos))
}

aoc_2024::main! {
    solve()
}
