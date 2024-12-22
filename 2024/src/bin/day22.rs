//! # Monkey Market
//!
//! Description.

use aoc_prelude::{HashMap, HashSet, Itertools};
use aoc_2024::extract_nums;
use rayon::prelude::*;

type Int = i64;
type Key = [i8; 4];
type Map = HashMap<Key, Int>;

const MOD: Int = (1 << 24) - 1;

fn solve() -> (Int, Int) {
    let input = include_str!("../../inputs/22.in");
    let nums = input.lines().filter_map(|l| extract_nums(l).next()).collect_vec();

    let p1 = nums
        .iter()
        .map(|n| {
            let mut n = *n;
            for _ in 0..2000 {
                n = hash(n);
            }
            n
        })
        .sum();

    let maps = nums.iter().map(|n| make_map(*n)).collect_vec();

    let all_keys = maps.iter().flat_map(|m| m.keys()).collect::<HashSet<_>>();

    let p2 = all_keys
        .into_iter()
        .collect_vec()
        .into_par_iter()
        .map(|k| maps.iter().filter_map(|m| m.get(k)).sum::<Int>())
        .max()
        .unwrap();

    (p1, p2)
}

fn hash(n: Int) -> Int {
    let mut n = n;

    n = (n ^ n << 6) & MOD;
    n = (n ^ n >> 5) & MOD;
    (n ^ n << 11) & MOD
}

fn make_map(initial: Int) -> Map {
    let mut p = initial % 10;
    let mut n = initial;

    let mut key = Key::default();

    let mut res = HashMap::new();

    for j in 0..2000 {
        n = hash(n);
        let new_p = n % 10;
        let delta = new_p - p;
        p = new_p;
        if j < 4 {
            key[j] = delta as i8;
        } else {
            key[0] = key[1];
            key[1] = key[2];
            key[2] = key[3];
            key[3] = delta as i8;
            if !res.contains_key(&key) {
                res.insert(key, new_p);
            }
        }
    }
    res
}

aoc_2024::main! {
    solve()
}
