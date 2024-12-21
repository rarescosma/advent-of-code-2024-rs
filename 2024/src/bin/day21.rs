//! # Keypad Conundrum
//!
//! Brute-forced p1, no idea how to do p2, still thinking.

use std::{iter::once, mem};

use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::Itertools;

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

    fn as_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.offsets()
            .map(|o| match (o.x, o.y) {
                (1, 0) => '>',
                (-1, 0) => '<',
                (0, 1) => 'v',
                (0, -1) => '^',
                _ => panic!(),
            })
            .chain(once('A'))
    }

    fn offsets(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..self.num_moves).map(|n| {
            let base_idx = (((self.sequence >> n) & 1) == 0) as usize;
            self.bases[base_idx]
        })
    }
}

fn num_repr(c: char) -> usize {
    if c == 'A' {
        return 10;
    }
    (c as u8 - b'0') as _
}

fn arrow_repr(c: char) -> usize {
    match c {
        '^' => 0,
        'v' => 1,
        '>' => 2,
        '<' => 3,
        'A' => 4,
        _ => panic!(),
    }
}

type TrMap = Vec<Vec<Vec<Transition>>>;

fn make_tr_map<F: Fn(char) -> usize>(pad: &Map<char>, repr_fn: F) -> TrMap {
    let mut transitions = vec![vec![Vec::new(); 11]; 11];

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
                    let transition = Transition {
                        bases: [Pos::new(bases.x, 0), Pos::new(0, bases.y)],
                        num_moves: num_moves as _,
                        sequence,
                    };
                    if transition.is_valid(pad, from_pos) {
                        transitions[repr_fn(from_ch)][repr_fn(to_ch)].push(transition)
                    }
                }
            }
        }
    }
    transitions
}

fn sequence<F: Fn(char) -> usize>(tr_map: &TrMap, repr_fn: F, goal: &[char]) -> Vec<String> {
    let mut goal_i = 0;
    let mut paths = Vec::new();
    let mut new_paths = Vec::new();
    let mut cur_key = 'A';
    while goal_i < goal.len() {
        let transitions = &tr_map[repr_fn(cur_key)][repr_fn(goal[goal_i])];
        if paths.is_empty() {
            paths.extend(transitions.iter().map(|t| t.as_chars().join("")))
        } else {
            new_paths.clear();
            let min_moves = transitions.iter().map(|t| t.num_moves).min().unwrap();
            for transition in transitions {
                if transition.num_moves > min_moves {
                    continue;
                }
                for path in &paths {
                    let mut new_path = path.clone();
                    new_path.push_str(&transition.as_chars().join(""));
                    new_paths.push(new_path);
                }
            }
            mem::swap(&mut paths, &mut new_paths);
        }
        cur_key = goal[goal_i];
        goal_i += 1;
    }
    paths
}

fn solve() -> (u32, usize) {
    let num_map = Map::new((3, 4), "789456123.0A".chars());
    let arrow_map = Map::new((3, 2), ".^A<v>".chars());

    let num_transitions = make_tr_map(&num_map, num_repr);
    let arrow_transitions = make_tr_map(&arrow_map, arrow_repr);

    let p1 = include_str!("../../inputs/21.in")
        .lines()
        .map(|line| {
            let num = extract_nums(line).next().unwrap();
            let goal = line.chars().collect_vec();

            let min_len = sequence(&num_transitions, num_repr, &goal)
                .iter()
                .flat_map(|s1| sequence(&arrow_transitions, arrow_repr, &s1.chars().collect_vec()))
                .flat_map(|s2| sequence(&arrow_transitions, arrow_repr, &s2.chars().collect_vec()))
                .map(|res| res.len())
                .min()
                .unwrap();

            num * (min_len as u32)
        })
        .sum();

    (p1, 0)
}

#[inline]
fn extract_nums(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.split(|c: char| !c.is_ascii_digit()).filter(|s| !s.is_empty()).flat_map(str::parse::<u32>)
}

aoc_2024::main! {
    solve()
}
