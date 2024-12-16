//! # Guard Gallivant
//!
//! Brute-force, unfortunately. Could be sped up by pre-computing a "jump table" so we teleport
//! the guard when hitting an obstacle instead of incrementing its position 1 by 1.
//!
//! Part 2: the big win is only trying to put obstacles in the positions walked by the guard
//! during part 1.
//!
//! Gotcha for part 2: when using a Vec as boolean array, you gotta `.fill(false)` instead of
//! `.clear()` it.
use aoc_2dmap::prelude::*;
use aoc_prelude::{num_integer::Integer, Itertools};

const DXY: [Pos; 4] = [Pos::c_new(0, -1), Pos::c_new(1, 0), Pos::c_new(0, 1), Pos::c_new(-1, 0)];

fn turn_right(dir: usize) -> usize { (dir + 1) % 4 }

fn turn_back(dir: usize) -> usize { (dir + 2) % 4 }

struct Buffers {
    states: Vec<bool>,
    visited_pos: Vec<bool>,
}

impl Buffers {
    fn allocate(size: MapSize) -> Self {
        let states = vec![false; (size.x * size.y) as usize * 4];
        let visited_pos = vec![false; (size.x * size.y) as usize];
        Self { states, visited_pos }
    }

    fn clear(&mut self) {
        self.states.fill(false);
        self.visited_pos.fill(false);
    }
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/06.in").lines().collect_vec();

    let mut map = Map::new((input[0].len(), input.len()), input.join("").chars());

    let mut start = Pos::default();
    for pos in map.iter() {
        if map.get_unchecked(pos) == '^' {
            start = pos;
            break;
        }
    }

    let mut buffers = Buffers::allocate(map.size);
    has_cycle(&map, start, &mut buffers);
    let p1 = buffers.visited_pos.iter().filter(|&x| *x).count();

    let mut p2 = 0;
    for hash in buffers.visited_pos.clone().iter().enumerate().filter(|x| *x.1).map(|x| x.0) {
        let (x, y) = hash.div_rem(&(map.size.y as usize));

        let cand = Pos::from((x, y));
        if map.get(cand) == Some('.') {
            map.set(cand, '#');
            if has_cycle(&map, start, &mut buffers) {
                p2 += 1;
            }
            map.set(cand, 'b');
        }
    }

    (p1, p2)
}

fn has_cycle(map: &Map<char>, start: Pos, buffers: &mut Buffers) -> bool {
    let mut cur = start;
    let mut dir = 0;
    buffers.clear();
    buffers.visited_pos[(start.x * map.size.y + start.y) as usize] = true;

    loop {
        cur += DXY[dir];
        if cur.x < 0 || cur.y < 0 || cur.x == map.size.x || cur.y == map.size.y {
            return false;
        }
        let c = map.get_unchecked(cur);
        let hash = ((cur.x * map.size.y + cur.y) * 4) as usize + dir;
        if c == '#' {
            // backtrack + turn right
            let opposite = DXY[turn_back(dir)];
            cur += opposite;
            dir = turn_right(dir);
        } else {
            // cycle detected
            if buffers.states[hash] {
                return true;
            }
        }
        buffers.states[hash] = true;
        buffers.visited_pos[(cur.x * map.size.y + cur.y) as usize] = true;
    }
}

aoc_2024::main! {
    solve()
}
