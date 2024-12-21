//! # Keypad Conundrum
//!
//! Brute-forced Part 1.
//!
//! For Part 2 we observe that going from the "string" domain to the "string length"
//! domain is beneficial. Namely, we can recursively compute the length at a given
//! depth for a given `&[char]` sequence by checking the transition map directly
//! (if the depth is 1) or by taking all possible transition sequences and passing
//! them one level down, then taking the minimum.

use std::{iter::once, mem};

use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{HashMap, Itertools};

type TrMap = Vec<Vec<Transition>>;

const MAX_KEYS: u8 = 11;
const UP: u8 = 1;
const DOWN: u8 = 2;
const RIGHT: u8 = 3;
const LEFT: u8 = 4;
const ENTER: u8 = 5;

#[derive(Copy, Clone, Debug)]
struct Transition {
    bases: [Pos; 2],
    num_moves: u8,
    sequence: u8,
}

impl Transition {
    fn is_valid(&self, map: &Map<char>, from_pos: Pos) -> bool {
        let mut cur = from_pos;
        for off in self.offsets() {
            cur += off;
            if map.get(cur) == Some('.') {
                return false;
            }
        }
        true
    }

    fn as_bytes(&self) -> impl Iterator<Item = u8> + '_ {
        self.offsets()
            .map(|o| match (o.x, o.y) {
                (0, -1) => UP,
                (0, 1) => DOWN,
                (1, 0) => RIGHT,
                (-1, 0) => LEFT,
                _ => panic!(),
            })
            .chain(once(ENTER))
    }

    fn offsets(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..self.num_moves).map(|n| {
            let base_idx = (((self.sequence >> n) & 1) == 0) as usize;
            self.bases[base_idx]
        })
    }
}

fn solve() -> (u64, u64) {
    let num_transitions = make_tr_map(&Map::new((3, 4), "789456123.0A".chars()), num_repr);
    let arrow_transitions = make_tr_map(&Map::new((3, 2), ".^A<v>".chars()), arrow_repr);

    let mut cache = HashMap::new();
    let (p1, p2) = include_str!("../../inputs/21.in")
        .lines()
        .map(|line| {
            let num = extract_nums(line).next().unwrap() as u64;
            let goal = line.chars().collect_vec();
            let sequences = possible_sequences(&goal, &num_transitions, num_repr);

            let min_len_2 = sequences
                .iter()
                .map(|s| sequence_length(s, 2, &arrow_transitions, &mut cache))
                .min()
                .unwrap();

            let min_len_26 = sequences
                .iter()
                .map(|s| sequence_length(s, 25, &arrow_transitions, &mut cache))
                .min()
                .unwrap();

            (num * min_len_2, num * min_len_26)
        })
        .fold((0, 0), |acc, cur| (acc.0 + cur.0, acc.1 + cur.1));

    (p1, p2)
}

fn make_tr_map<F: Fn(char) -> u8>(pad: &Map<char>, repr_fn: F) -> TrMap {
    let mut transitions = vec![Vec::new(); (MAX_KEYS * MAX_KEYS) as usize];

    for from_pos in pad.iter() {
        let from_ch = pad[from_pos];
        if from_ch == '.' {
            continue;
        }
        for to_pos in pad.iter() {
            let to_ch = pad[to_pos];
            if to_ch == '.' {
                continue;
            }

            let bases = (to_pos - from_pos).signum();
            let num_moves = (to_pos.x - from_pos.x).abs() + (to_pos.y - from_pos.y).abs();
            let num_x = (to_pos.x - from_pos.x).unsigned_abs();

            for sequence in 0..=((1u8 << num_moves) - 1) {
                if sequence.count_ones() == num_x {
                    // We have a number whose 1 bits indicate a move on the X axis
                    // and whose 0 bits indicate a move on the Y axis
                    let transition = Transition {
                        bases: [Pos::new(bases.x, 0), Pos::new(0, bases.y)],
                        num_moves: num_moves as _,
                        sequence,
                    };
                    if transition.is_valid(pad, from_pos) {
                        transitions[(repr_fn(from_ch) * MAX_KEYS + repr_fn(to_ch)) as usize]
                            .push(transition)
                    }
                }
            }
        }
    }
    transitions
}

fn possible_sequences<F: Fn(char) -> u8>(
    goal: &[char],
    tr_map: &TrMap,
    repr_fn: F,
) -> Vec<Vec<u8>> {
    let mut goal_i = 0;
    let mut paths = vec![Vec::new()];
    let mut new_paths = vec![Vec::new()];
    let mut cur_key = 'A';
    while goal_i < goal.len() {
        let key = (repr_fn(cur_key) * MAX_KEYS + repr_fn(goal[goal_i])) as usize;
        let transitions = &tr_map[key];
        new_paths.clear();
        for transition in transitions {
            for path in &paths {
                let mut new_path = path.clone();
                new_path.extend(transition.as_bytes());
                new_paths.push(new_path);
            }
        }
        mem::swap(&mut paths, &mut new_paths);
        cur_key = goal[goal_i];
        goal_i += 1;
    }
    paths
}

fn sequence_length(
    seq: &[u8],
    depth: u64,
    tr_map: &TrMap,
    cache: &mut HashMap<(u64, u64), u64>,
) -> u64 {
    let mut key: u64 = 0;
    // We've got maximum 14 arrow + 'A' key presses after the first stage
    // and each key is representable on 4 bits.
    for ch in seq {
        key = (key << 4) ^ (*ch as u64)
    }

    if cache.contains_key(&(key, depth)) {
        return cache[&(key, depth)];
    }

    // Robots start on the 'A' (ENTER) key
    let ret = once(ENTER)
        .chain(seq.iter().copied())
        .zip(seq)
        .map(|(from_ch, &to_ch)| {
            let tx = &tr_map[(from_ch * MAX_KEYS + to_ch) as usize];
            if depth == 1 {
                (tx[0].num_moves as u64) + 1
            } else {
                tx.iter()
                    .map(|t| sequence_length(&t.as_bytes().collect_vec(), depth - 1, tr_map, cache))
                    .min()
                    .unwrap()
            }
        })
        .sum();

    cache.insert((key, depth), ret);
    ret
}

#[inline]
fn num_repr(c: char) -> u8 {
    if c == 'A' {
        return 10;
    }
    (c as u8 - b'0') as _
}

#[inline]
fn arrow_repr(c: char) -> u8 {
    match c {
        '^' => UP,
        'v' => DOWN,
        '>' => RIGHT,
        '<' => LEFT,
        'A' => ENTER,
        _ => panic!(),
    }
}

#[inline]
fn extract_nums(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.split(|c: char| !c.is_ascii_digit()).filter(|s| !s.is_empty()).flat_map(str::parse::<u32>)
}

aoc_2024::main! {
    solve()
}
