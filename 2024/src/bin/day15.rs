//! # Warehouse Woes
//!
//! Part 2: Look at a window of 4 tiles above or below the starting position
//! and check whether we need to add any boxes to the push set.

use std::collections::VecDeque;

use aoc_2dmap::prelude::*;
use aoc_prelude::HashSet;

const ZERO: Pos = Pos::c_new(0, 0);

struct Buf {
    map: Map<char>,
    changes: Vec<(Pos, char)>,
    push_set: HashSet<Pos>,
    queue: VecDeque<Pos>,
}

fn solve() -> (i32, i32) {
    let (map, dirs) = include_str!("../../inputs/15.in").split_once("\n\n").unwrap();

    let map_size = Pos::from((
        map.chars().position(|x| x == '\n').unwrap(),
        map.chars().filter(|x| *x == '\n').count() + 1,
    ));

    let mut p1_map = Map::new(map_size, map.chars().filter(|&c| c != '\n'));
    let p1_bot = find_bot(&p1_map);

    let mut p2_map = Map::fill((2 * p1_map.size.x, p1_map.size.y), '.');
    for pos in p1_map.iter() {
        let (tl, tr) = match p1_map.get_unchecked(pos) {
            '#' => ('#', '#'),
            'O' => ('[', ']'),
            '.' => ('.', '.'),
            '@' => ('@', '.'),
            _ => continue,
        };
        p2_map.set(Pos::new(2 * pos.x, pos.y), tl);
        p2_map.set(Pos::new(2 * pos.x + 1, pos.y), tr);
    }
    let p2_bot = find_bot(&p2_map);

    p1_map.set(p1_bot, '.');
    p2_map.set(p2_bot, '.');

    let mut buf = Buf {
        map: p1_map,
        changes: Vec::with_capacity(512),
        push_set: HashSet::with_capacity(512),
        queue: VecDeque::with_capacity(10),
    };

    walk(dirs, p1_bot, &mut buf);
    let p1 = tally(&buf.map, 'O');

    buf.map = p2_map;
    walk(dirs, p2_bot, &mut buf);
    let p2 = tally(&buf.map, '[');

    (p1, p2)
}

fn ch_to_dir(c: char) -> Pos {
    match c {
        '^' => NORTH,
        '>' => EAST,
        'v' => SOUTH,
        '<' => WEST,
        _ => ZERO,
    }
}

fn walk(dirs: &str, start_pos: Pos, buf: &mut Buf) {
    let mut bot = start_pos;

    for ch in dirs.chars() {
        let dxy = ch_to_dir(ch);

        let dest = bot + dxy;
        let tile = buf.map.get_unchecked(dest);

        if tile == '.' {
            bot = dest;
            continue;
        } else if tile == '#' {
            continue;
        }

        push_set(dest, dxy, buf);
        if buf.push_set.iter().all(|&pos| buf.map.get_unchecked(pos + dxy) != '#') {
            buf.changes.clear();
            for &pos in &buf.push_set {
                buf.changes.push((pos + dxy, buf.map.get_unchecked(pos)));
                buf.map.set(pos, '.');
            }
            for &(new_pos, tile) in &buf.changes {
                buf.map.set(new_pos, tile);
            }
            bot = dest;
        }
    }
}

fn find_bot(map: &Map<char>) -> Pos {
    map.iter().find(|pos| map.get_unchecked(pos) == '@').unwrap()
}

fn push_set(start_pos: Pos, dy: Pos, buf: &mut Buf) {
    buf.push_set.clear();
    buf.queue.clear();

    buf.push_set.insert(start_pos);
    buf.queue.push_back(start_pos);

    while let Some(pos) = buf.queue.pop_front() {
        let tile = buf.map.get_unchecked(pos);
        if is_box(tile) {
            buf.push_set.insert(pos);
            buf.queue.push_back(pos + dy)
        }
        if tile == ']' && !buf.push_set.contains(&(pos + WEST)) {
            buf.queue.push_back(pos + WEST);
        }
        if tile == '[' && !buf.push_set.contains(&(pos + EAST)) {
            buf.queue.push_back(pos + EAST);
        }
    }
}

fn tally(map: &Map<char>, box_ch: char) -> i32 {
    map.iter().filter(|pos| map.get_unchecked(pos) == box_ch).map(|pos| pos.y * 100 + pos.x).sum()
}

fn is_box(tile: char) -> bool { tile == 'O' || tile == '[' || tile == ']' }

aoc_2024::main! {
    solve()
}
