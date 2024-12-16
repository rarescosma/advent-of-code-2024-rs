//! # Resonant Collinearity
//!
//! Brute force all the way, but fast enough for a Sunday problem, especially
//! after switching to boolean arrays instead of our fancy `Map`.
use aoc_2dmap::prelude::*;
use aoc_prelude::{BTreeMap, Itertools};

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/08.in").lines().collect_vec();
    let map = Map::new((input[0].len(), input.len()), input.join("").chars());

    let mut h_map = BTreeMap::<char, Vec<Pos>>::new();
    for pos in map.iter() {
        let ch = map.get_unchecked(pos);
        if ch != '.' {
            h_map.entry(ch).or_default().push(pos);
        }
    }

    let mut p1_vec = vec![false; (map.size.x * map.size.y) as usize];
    let mut p2_vec = vec![false; (map.size.x * map.size.y) as usize];

    for antennas in h_map.values() {
        antennas.iter().tuple_combinations().for_each(|(p1, p2)| {
            antinodes(p1, p2, &map.size, &mut p1_vec, &mut p2_vec);
            antinodes(p2, p1, &map.size, &mut p1_vec, &mut p2_vec);
        });
    }

    let p1 = p1_vec.into_iter().filter(|&x| x).count();
    let p2 = p2_vec.into_iter().filter(|&x| x).count();

    (p1, p2)
}

fn antinodes(a1: &Pos, a2: &Pos, map_size: &MapSize, p1_buf: &mut [bool], p2_buf: &mut [bool]) {
    let dist = *a2 - *a1;

    let mut cand = *a2;
    let mut i = 0;

    while within(&cand, map_size) {
        let idx = index(&cand, map_size);
        p2_buf[idx] = true;
        if i == 1 {
            p1_buf[idx] = true;
        }
        cand += dist;
        i += 1;
    }
}

fn within(p: &Pos, map_size: &MapSize) -> bool {
    p.x >= 0 && p.x < map_size.x && p.y >= 0 && p.y < map_size.y
}

fn index(p: &Pos, map_size: &MapSize) -> usize { (p.y * map_size.x + p.x) as usize }

aoc_2024::main! {
    solve()
}
