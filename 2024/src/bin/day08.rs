use aoc_2dmap::prelude::*;
use aoc_prelude::{BTreeMap, HashSet, Itertools};
use std::iter::once;

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

    let mut p1_nodes = HashSet::new();
    let mut p2_nodes = HashSet::<Pos>::new();
    let mut buf = Vec::with_capacity(256);

    for antennas in h_map.values() {
        antennas.iter().tuple_combinations().for_each(|(p1, p2)| {
            p1_nodes.extend(antinodes(p1, p2, &map.size));
            antinodes_p2(p1, p2, &map.size, &mut buf);
            p2_nodes.extend(buf.iter());
        });
    }

    (p1_nodes.len(), p2_nodes.len())
}

fn within(p: &Pos, map_size: &MapSize) -> bool {
    p.x >= 0 && p.x < map_size.x && p.y >= 0 && p.y < map_size.y
}

fn antinodes<'a>(a1: &Pos, a2: &Pos, map_size: &'a MapSize) -> impl Iterator<Item = Pos> + 'a {
    let dist = *a2 - *a1;
    let cand1 = *a2 + dist;
    let cand2 = *a1 + (-dist.x, -dist.y).into();
    once(cand1)
        .chain(once(cand2))
        .filter(|p| within(p, map_size))
}

fn antinodes_p2(a1: &Pos, a2: &Pos, map_size: &MapSize, buf: &mut Vec<Pos>) {
    buf.clear();
    let dist = *a2 - *a1;
    let a_dist: Pos = (-dist.x, -dist.y).into();

    let mut cand = *a2;
    while within(&cand, map_size) {
        buf.push(cand);
        cand += dist;
    }

    cand = *a1;
    while within(&cand, map_size) {
        buf.push(cand);
        cand += a_dist;
    }
}

aoc_2024::main! {
    solve()
}
