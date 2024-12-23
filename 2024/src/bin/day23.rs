//! # LAN Party
//!
//! Brute force is still a thing.

use std::{collections::BTreeSet, sync::Mutex};

use aoc_prelude::{lazy_static, Entry, HashMap, HashSet, Itertools};
use rayon::prelude::*;

lazy_static! {
    static ref COUNTER: Mutex<u16> = Mutex::new(0);
}

fn idx<'a>(node: &'a str, cache: &mut HashMap<&'a str, u16>) -> u16 {
    match cache.entry(node) {
        Entry::Occupied(idx) => *idx.get(),
        Entry::Vacant(entry) => {
            let mut cnt = COUNTER.lock().unwrap();
            let idx = *cnt;
            entry.insert(idx);
            *cnt += 1;
            idx
        }
    }
}

fn solve() -> (usize, String) {
    let mut idx_cache = HashMap::new();

    let mut tee_nodes = HashSet::new();

    let mut ix = |s| idx(s, &mut idx_cache);

    let lines = include_str!("../../inputs/23.in").lines().collect_vec();
    lines.iter().for_each(|line| {
        let (fr, to) = line.split_once("-").unwrap();
        ix(fr);
        ix(to);
    });

    let num_nodes = *COUNTER.lock().unwrap() as usize;
    let mut adj = vec![vec![false; num_nodes]; num_nodes];

    lines.into_iter().for_each(|l| {
        let (fr, to) = l.split_once("-").unwrap();
        let fr_idx = ix(fr) as usize;
        let to_idx = ix(to) as usize;
        adj[fr_idx][to_idx] = true;
        adj[to_idx][fr_idx] = true;

        if fr.starts_with("t") {
            tee_nodes.insert(fr_idx);
        }
        if to.starts_with("t") {
            tee_nodes.insert(to_idx);
        }
    });

    let mut init = HashSet::new();
    for i in 0..num_nodes {
        for j in i + 1..num_nodes {
            for k in j + 1..num_nodes {
                if adj[i][j] && adj[i][k] && adj[j][k] {
                    init.insert(BTreeSet::from([i, j, k]));
                }
            }
        }
    }
    let p1 = init.iter().filter(|sg| sg.iter().any(|n| tee_nodes.contains(n))).collect_vec().len();

    loop {
        let new = init
            .iter()
            .par_bridge()
            .flat_map(|comp| {
                let mut next = Vec::new();

                for node in 0..num_nodes {
                    if comp.contains(&node) {
                        continue;
                    }
                    if comp.iter().all(|&n| adj[n][node]) {
                        let mut next_comp = comp.clone();
                        next_comp.insert(node);
                        next.push(next_comp);
                    }
                }
                next
            })
            .collect::<Vec<_>>();

        if new.is_empty() {
            break;
        }
        init = HashSet::from_iter(new);
    }

    let idx_to_name =
        idx_cache.clone().into_iter().map(|(k, v)| (v as usize, k)).collect::<HashMap<_, _>>();

    let p2 = init.into_iter().next().unwrap().iter().map(|idx| idx_to_name[idx]).sorted().join(",");

    (p1, p2)
}

aoc_2024::main! {
    solve()
}
