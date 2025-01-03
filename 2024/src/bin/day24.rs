//! # Crossed Wires
//!
//! Finally settled on a generic solution for Part 2 that I'm happy with:
//!
//! - Doesn't make any assumptions about the structure of the adders.
//! - Uses truth tables to find swap candidates: when checking a particular bit position we cycle
//!   through all relevant values for x, y in `bit_pos` and `bit_pos - 1` (to account for carry)
//!   and compare the output truth table with the expected one.
//!   If not matching, but we've got other nodes in the  circuit that do match,
//!   we attempt to swap the output with them.
//! - Mini brute-force when the above heuristic fails - we only take nodes that are locally close
//!   to the issue and account for cycles + check if we haven't disturbed the truth tables
//!   for `bit_pos - 1` and `bit_pos + 1` just to be sure.

use std::{
    collections::VecDeque,
    fmt::Display,
    sync::{
        atomic::{AtomicUsize, Ordering},
        RwLock,
    },
};

use aoc_2024::{extract_nums, reverse, BitSet};
use aoc_prelude::{lazy_static, Entry, HashMap, HashSet, Itertools, PrimInt};

const MAX_BITS: i8 = 45;
const MAX_NODES: usize = 400;
const BF_MAX_DEPTH: usize = 6;
const HALF_ADDER: [u8; 16] = [0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0];
const FULL_ADDER: [u8; 16] = [0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1];
const LAST_CARRY: [u8; 16] = [0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1];

lazy_static! {
    static ref COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref IDX_CACHE: RwLock<HashMap<String, SigRef>> = RwLock::new(HashMap::new());
}

type Gates = [Gate; MAX_NODES];
type Signals = [Signal; MAX_NODES];
type SigRef = usize;
type GateKind = u8;

const OR: GateKind = 1;
const AND: GateKind = 2;
const XOR: GateKind = 3;

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
    fn register_signal(&mut self, sig: &str) -> SigRef {
        let (sig_ref, signal) = Signal::parse(sig);
        if signal.is_input {
            let mut idx = extract_nums::<usize>(sig).next().expect("valid identifier");
            if sig.starts_with("y") {
                idx += MAX_BITS as usize;
            }
            self.inputs[idx] = sig_ref;
        }
        self.signals[sig_ref] = signal;
        sig_ref
    }

    fn set_size(&mut self, size: usize) { self.size = size; }

    fn run(&mut self, x: u64, y: u64) -> Option<()> {
        self.zero_signals();

        let mut fill = |mut val: u64, offset: usize| {
            let mut bit_pos = 0;
            while val > 0 {
                let idx = self.inputs[bit_pos + offset];
                self.signals[idx].val = (val & 1) as u8;
                val >>= 1;
                bit_pos += 1;
            }
        };

        fill(x, 0);
        fill(y, MAX_BITS as _);

        self.evaluate()
    }

    fn zero_signals(&mut self) { self.signals.iter_mut().for_each(|s| s.val = 0); }

    fn evaluate(&mut self) -> Option<()> {
        fn inner(
            this: &mut State,
            out_ref: SigRef,
            seen: BitSet<{ MAX_NODES.div_ceil(128) }>,
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
                let sig = this.signals[sig_ref];
                match sig.is_input {
                    true => Some(sig.val),
                    false => inner(this, sig_ref, seen | sig_ref, cache),
                }
            };
            let left = collapse(gate.left)?;
            let right = collapse(gate.right)?;

            let ret = gate.eval(left, right);
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

    fn swap(&mut self, a: SigRef, b: SigRef) {
        let (a_gate, b_gate) = (self.gates[a], self.gates[b]);
        self.gates[a].splat(b_gate);
        self.gates[b].splat(a_gate);
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Signal {
    is_input: bool,
    is_output: bool,
    val: u8,
}

impl Signal {
    fn parse(name: &str) -> (SigRef, Self) {
        let is_input = name.starts_with("x") || name.starts_with("y");
        let is_output = name.starts_with("z");
        let sig_ref = idx(name.to_owned());
        (sig_ref, Self { is_input, is_output, val: 0 })
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Gate {
    left: SigRef,
    right: SigRef,
    kind: GateKind,
}

impl Gate {
    fn parse_kind(from: &str) -> GateKind {
        match from {
            "OR" => OR,
            "AND" => AND,
            "XOR" => XOR,
            _ => panic!(),
        }
    }

    fn splat(&mut self, other: Gate) {
        self.left = other.left;
        self.right = other.right;
        self.kind = other.kind;
    }

    fn eval(&self, left: u8, right: u8) -> u8 {
        match self.kind {
            OR => left | right,
            AND => left & right,
            XOR => left ^ right,
            _ => panic!(),
        }
    }
}

fn solve() -> Option<(u64, String)> {
    let (inputs, gates) = include_str!("../../inputs/24.in").split_once("\n\n")?;

    let mut state = State::default();

    for line in gates.lines() {
        let mut parts = line.split_ascii_whitespace();

        let left = state.register_signal(parts.next()?);
        let kind = Gate::parse_kind(parts.next()?);
        let right = state.register_signal(parts.next()?);
        let _ = parts.next()?;
        let out_ref = state.register_signal(parts.next()?);

        state.gates[out_ref] = Gate { left, right, kind };
    }

    for line in inputs.lines() {
        let (name, val) = line.split_once(": ")?;
        let sig_ref = state.register_signal(name);

        state.signals[sig_ref].val = extract_nums::<u8>(val).next()?;
    }

    let idx_cache = IDX_CACHE.read().unwrap().clone();
    state.set_size(idx_cache.len());
    let rev_idx = reverse(idx_cache);
    state.evaluate();

    let mut p1 = 0u64;
    for sig_ref in (0..state.size)
        .filter(|&sig_ref| state.signals[sig_ref].is_output)
        .sorted_by_key(|sig_ref| &rev_idx[sig_ref])
        .rev()
    {
        p1 = (p1 << 1) | (state.signals[sig_ref].val as u64);
    }

    let mut p2 = (0..MAX_BITS)
        .filter_map(|bit_pos| fix_bit(bit_pos, &mut state))
        .flatten()
        .map(|swap| &rev_idx[&swap])
        .collect_vec();
    p2.sort_unstable();

    Some((p1, p2.into_iter().join(",")))
}

fn fix_bit(bit_pos: i8, state: &mut State) -> Option<[SigRef; 2]> {
    let (ok, matching_signals) = mini_test(bit_pos, state).expect("no cycles");

    if ok {
        return None;
    }

    let out_ref = make_id("z", bit_pos);

    if let Some(sig_ref) = matching_signals.into_iter().next() {
        state.swap(out_ref, sig_ref);
        debug_assert!(matches!(mini_test(bit_pos, state), Some((true, _))));
        return Some([out_ref, sig_ref]);
    } else {
        let bf_signals = bf_cands(make_id("z", bit_pos + 1), state);
        for i in 0..bf_signals.len() - 1 {
            for j in i + 1..bf_signals.len() {
                state.swap(bf_signals[i], bf_signals[j]);
                if (-1..=1).all(|off| mini_test(bit_pos + off, state).is_some_and(|(ok, _)| ok)) {
                    return Some([bf_signals[i], bf_signals[j]]);
                }
                state.swap(bf_signals[i], bf_signals[j]);
            }
        }
    }
    None
}

fn mini_test(bit_pos: i8, state: &mut State) -> Option<(bool, HashSet<SigRef>)> {
    let mut truth_tables = [[0u8; 16]; MAX_NODES];

    let mut idx = 0;

    for x in 0..4 {
        let x = x << (bit_pos - 1).max(0);
        for y in 0..4 {
            let y = y << (bit_pos - 1).max(0);
            state.run(x, y)?;
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
        0 => HALF_ADDER,
        MAX_BITS => LAST_CARRY,
        _ => FULL_ADDER,
    };

    let mut matching_signals: HashSet<SigRef> = truth_tables
        .iter()
        .enumerate()
        .filter(|&(_, table)| table == &check_table)
        .map(|(sig_ref, _)| sig_ref)
        .collect();

    let ok = matching_signals.contains(&make_id("z", bit_pos));
    matching_signals.retain(|&sig_ref| !state.signals[sig_ref].is_output);

    Some((ok, matching_signals))
}

fn bf_cands(start_ref: SigRef, state: &State) -> Vec<SigRef> {
    let mut q = VecDeque::from([(0, start_ref)]);

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

fn idx(name: String) -> SigRef {
    let mut cache = IDX_CACHE.write().unwrap();
    match cache.entry(name) {
        Entry::Occupied(idx) => *idx.get(),
        Entry::Vacant(entry) => {
            let id = COUNTER.fetch_add(1, Ordering::Relaxed);
            entry.insert(id);
            id
        }
    }
}

fn make_id<I: PrimInt + Display>(prefix: &str, bit_pos: I) -> SigRef {
    idx(format!("{}{:0>2}", prefix, bit_pos))
}

aoc_2024::main! {
    solve().unwrap()
}
