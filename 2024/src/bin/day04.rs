//! # Ceres Search
//!
//! Part 1: we only implement searching in the left direction, but accumulate
//! the counts for all 8 rotations of the map (45 and 90 degrees increments).
//!
//! Part 2: check for the diagonal neighbors of an `A` to match the [`PAT`].
//! Then do the same for the other three 90 degrees rotations.
use aoc_2dmap::prelude::*;
use aoc_prelude::Itertools;

const PAT: [Option<char>; 4] = [Some('S'), Some('S'), Some('M'), Some('M')];

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/04.in").lines().collect_vec();

    let mut map = Map::new((input[0].len(), input.len()), input.join("").chars());

    let mut p1 = find_p1(&map);
    let mut p2 = find_p2(&map);

    for _ in 0..3 {
        map = rotate_90(&map);
        p1 += find_p1(&map);
        p2 += find_p2(&map);
    }

    (p1, p2)
}

fn find_p1(map: &Map<char>) -> usize {
    find_str(map, "XMAS") + find_str(&rotate_45(map, '.'), "XMAS")
}

fn find_str(map: &Map<char>, what: &str) -> usize {
    let win_size = what.len();

    (0..map.size.y)
        .map(|r| {
            let row = map.get_row(r).join("");
            row.as_bytes().windows(win_size).filter(|window| *window == what.as_bytes()).count()
        })
        .sum()
}

fn find_p2(map: &Map<char>) -> usize {
    map.iter()
        .filter(|pos| {
            map.get_unchecked(pos) == 'A'
                && pos
                    .neighbors_only_diag()
                    .map(|n| map.get(n))
                    .enumerate()
                    .all(|(idx, res)| res == PAT[idx])
        })
        .count()
}

fn rotate_90<T: Clone>(map: &Map<T>) -> Map<T> {
    Map::new(
        (map.size.y, map.size.x),
        (1..=map.size.x).flat_map(|col_idx| map.get_col(map.size.x - col_idx)),
    )
}

fn rotate_45<T: Clone>(map: &Map<T>, empty: T) -> Map<T> {
    let row_size = map.size.x;
    let mut new_rows = Vec::new();

    // up to and including main diagonal
    for y in 0..map.size.y {
        let mut new_row =
            diag_iter((0, y).into(), map.size).map(|pos| map.get_unchecked(pos)).collect_vec();
        new_row.resize(row_size as usize, empty.clone());
        new_rows.push(new_row);
    }

    // from main diagonal to bottom-right corner
    for x in 1..map.size.x {
        let mut new_row = diag_iter((x, map.size.y - 1).into(), map.size)
            .map(|pos| map.get_unchecked(pos))
            .collect_vec();
        new_row.resize(row_size as usize, empty.clone());
        new_rows.push(new_row);
    }

    Map::new((row_size, new_rows.len()), new_rows.into_iter().flatten())
}

fn diag_iter(p: Pos, map_s: MapSize) -> impl Iterator<Item = Pos> {
    // increase x, decrease y
    (p.x..map_s.x).zip(0..=p.y).map(move |(x, dec_y)| (x, p.y - dec_y).into())
}

aoc_2024::main! {
    solve()
}
