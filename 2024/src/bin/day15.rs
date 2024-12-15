//! # Warehouse Woes
//!
//! Part 2: Look at a window of 4 tiles above or below the starting position
//! and check whether we need to add any boxes to the push set.

use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::HashSet;

const UP: Pos = Pos::c_new(0, -1);
const DOWN: Pos = Pos::c_new(0, 1);
const LEFT: Pos = Pos::c_new(-1, 0);
const RIGHT: Pos = Pos::c_new(1, 0);
const ZERO: Pos = Pos::c_new(0, 0);

struct Buf {
    map: Map<char>,
    changes: Vec<(Pos, char)>,
    push_set: HashSet<Pos>,
}

fn solve() -> (i32, i32) {
    let (map, dirs) = include_str!("../../inputs/15.in")
        .split_once("\n\n")
        .unwrap();

    let map_size = Pos::from((
        map.chars().position(|x| x == '\n').unwrap(),
        map.chars().filter(|x| *x == '\n').count() + 1,
    ));

    let mut map = Map::new(map_size, map.chars().filter(|&c| c != '\n'));
    let p1_bot = find_bot(&map);

    let mut ext_map = Map::fill((2 * map.size.x, map.size.y), '.');
    for pos in map.iter() {
        let (tl, tr) = match map.get_unchecked(pos) {
            '#' => ('#', '#'),
            'O' => ('[', ']'),
            '.' => ('.', '.'),
            '@' => ('@', '.'),
            _ => continue,
        };
        ext_map.set(Pos::new(2 * pos.x, pos.y), tl);
        ext_map.set(Pos::new(2 * pos.x + 1, pos.y), tr);
    }
    let p2_bot = find_bot(&ext_map);

    map.set(p1_bot, '.');
    ext_map.set(p2_bot, '.');

    let mut buf = Buf {
        map,
        changes: Vec::with_capacity(512),
        push_set: HashSet::with_capacity(512),
    };

    let p1 = walk(dirs, p1_bot, &mut buf, false);

    buf.map = ext_map;
    let p2 = walk(dirs, p2_bot, &mut buf, true);

    (p1, p2)
}

fn ch_to_dir(c: char) -> Pos {
    match c {
        '^' => UP,
        '>' => RIGHT,
        '<' => LEFT,
        'v' => DOWN,
        _ => ZERO,
    }
}

fn walk(dirs: &str, start_pos: Pos, buf: &mut Buf, p2: bool) -> i32 {
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

        if !p2 || (dxy == LEFT || dxy == RIGHT) {
            if step(dest, dxy, &mut buf.map) {
                bot = dest;
            }
            continue;
        }

        buf.push_set.clear();
        push_set(dest, dxy, buf);
        if buf
            .push_set
            .iter()
            .all(|&pos| buf.map.get_unchecked(pos + dxy) != '#')
        {
            buf.changes.clear();
            for pos in &buf.push_set {
                buf.changes.push((*pos + dxy, buf.map.get_unchecked(pos)));
                buf.map.set(pos, '.');
            }
            for (new_pos, tile) in &buf.changes {
                buf.map.set(new_pos, *tile);
            }
            bot = dest;
        }
    }

    let box_tile = if p2 { '[' } else { 'O' };
    buf.map
        .iter()
        .filter(|pos| buf.map.get_unchecked(pos) == box_tile)
        .map(|pos| pos.y * 100 + pos.x)
        .sum()
}

fn push_set(start_pos: Pos, dy: Pos, buf: &mut Buf) {
    if buf.map.get_unchecked(start_pos) == '.' {
        return;
    }

    let mut start_pos = start_pos;
    if buf.map.get_unchecked(start_pos) == ']' {
        start_pos += LEFT;
    }

    buf.push_set.insert(start_pos);
    buf.push_set.insert(start_pos + RIGHT);
    [LEFT, ZERO, RIGHT]
        .iter()
        .map(|&dx| start_pos + dx + dy)
        .for_each(|check| {
            if buf.map.get(check) == Some('[') {
                push_set(check, dy, buf);
            }
        });
}

fn step(dest: Pos, dxy: Pos, map: &mut Map<char>) -> bool {
    let mut tile = map.get_unchecked(dest);
    let mut run = dest;
    while is_box(tile) {
        run += dxy;
        tile = map.get_unchecked(run);
    }
    if tile == '.' {
        while run != dest {
            map.swap(run, run - dxy);
            run = run - dxy;
        }
        return true;
    }
    false
}

fn find_bot(map: &Map<char>) -> Pos {
    map.iter()
        .find(|pos| map.get_unchecked(pos) == '@')
        .unwrap()
}

fn is_box(tile: char) -> bool {
    tile == 'O' || tile == '[' || tile == ']'
}

aoc_2024::main! {
    solve()
}
