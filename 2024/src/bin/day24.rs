//! # Crossed Wires
//!
//!  Finally settled on a generic solution for Part 2 that I'm happy with:
//!
//!   - Doesn't make any assumptions about the structure of the adders.
//!   - Uses truth tables to find swap candidates: when checking a particular bit position we cycle
//!     through all relevant values for x, y in `bit_pos` and `bit_pos - 1` (to account for carry)
//!     and compare the output truth table with the expected one.
//!     If not matching, but we've got other nodes in the  circuit that do match,
//!     we attempt to swap the output with them.
//!   - Mini brute-force when the above heuristic fails - we only take nodes that are locally close
//!     to the issue and account for cycles + check if we haven't disturbed the truth tables
//!     for `bit_pos - 1` and `bit_pos + 1` just to be sure.

use std::{
    collections::VecDeque,
    fmt::Display,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};

use aoc_2024::{extract_nums, reverse, BitSet};
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

type Gates = [Gate; MAX_NODES];
type Signals = [Signal; MAX_NODES];
type SigRef = usize;

struct State {
    gates: Gates,
    signals: Signals,
    inputs: [SigRef; MAX_NODES],
    size: SigRef,
}

impl Default for State {
    fn default() -> Self {
        Self {
            gates: [Gate::default(); MAX_NODES],
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
            out_ref: SigRef,
            seen: BitSet<{ MAX_NODES / 128 + 1 }>,
            cache: &mut [u8],
        ) -> Option<u8> {
            if cache[out_ref] != u8::MAX {
                return Some(cache[out_ref]);
            }

            let gate = this.gates[out_ref];

            // This indicates a cycle, we propagate the condition through the use of Option
            if seen.contains(gate.left) || seen.contains(gate.right) {
                return None;
            }

            let mut collapse = |sig_ref: SigRef| {
                if this.signals[sig_ref].is_input {
                    Some(this.signals[sig_ref].val)
                } else {
                    let mut new_seen = seen;
                    new_seen.set(sig_ref);
                    inner(this, sig_ref, new_seen, cache)
                }
            };
            let left = collapse(gate.left)?;
            let right = collapse(gate.right)?;

            let ret = gate.kind.eval(left, right);
            cache[out_ref] = ret;
            Some(ret)
        }

        let mut cache = [u8::MAX; MAX_NODES];
        for out_ref in 0..self.size {
            if !self.signals[out_ref].is_input {
                self.signals[out_ref].val = inner(self, out_ref, Default::default(), &mut cache)?;
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

    fn eval(&self, left: u8, right: u8) -> u8 {
        match self {
            GateKind::Or => left | right,
            GateKind::And => left & right,
            GateKind::Xor => left ^ right,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Gate {
    left: SigRef,
    right: SigRef,
    kind: GateKind,
}

impl Gate {
    fn splat(&mut self, other: Gate) {
        self.left = other.left;
        self.right = other.right;
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

            let left = register_signal(parts.next()?, &mut state.signals, &mut state.inputs);
            let kind = GateKind::parse(parts.next()?);
            let right = register_signal(parts.next()?, &mut state.signals, &mut state.inputs);
            let _ = parts.next()?;
            let out_ref = register_signal(parts.next()?, &mut state.signals, &mut state.inputs);

            Some((out_ref, Gate { left, right, kind }))
        })
        .for_each(|(out_ref, gate)| {
            state.gates[out_ref] = gate;
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
    let rev_idx = reverse(idx_cache);
    state.evaluate();

    let mut p1 = 0u64;
    for sig_ref in (0..state.size)
        .filter(|n| state.signals[*n].is_output)
        .sorted_by_key(|sig_ref| &rev_idx[sig_ref])
        .rev()
    {
        p1 = (p1 << 1) | (state.signals[sig_ref].val as u64);
    }

    let mut p2 = Vec::new();
    for bit_pos in 0..MAX_BITS {
        p2.extend(fix_bit(bit_pos, &mut state).into_iter().map(|sig_ref| &rev_idx[&sig_ref]));
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
            let gate = state.gates[sig_ref];
            q.push_back((depth + 1, gate.left));
            q.push_back((depth + 1, gate.right));
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
    let (sig_ref, signal) = Signal::from_str(sig);
    if signal.is_input {
        let mut idx = extract_nums::<usize>(sig).next().unwrap();
        if sig.starts_with("y") {
            idx += MAX_BITS as usize;
        }
        inputs[idx] = sig_ref;
    }
    signals[sig_ref] = signal;
    sig_ref
}

fn swap(state: &mut State, a: SigRef, b: SigRef) {
    let (l, r) = (state.gates[a], state.gates[b]);
    state.gates[a].splat(r);
    state.gates[b].splat(l);
}

fn make_id<I: PrimInt + Display>(prefix: &str, bit_pos: I) -> SigRef {
    idx(format!("{}{:0>2}", prefix, bit_pos))
}

aoc_2024::main! {
    solve()
}
