//! # Garden Groups
//!
//! Part 1: summing the number of fences for each block of every region and
//! keeping track of its area is enough. We store the fences in an u8 and use
//! bit shifting tricks.
//!
//! Part 2: Ouuuffff, this was quite tedious:
//! - implement an explorer that "hugs" the right wall of any region
//! - the number of turns it takes before getting back to the initial state is
//!   equal to the number of sides
//! - run Explorers until the set of fenced tiles is empty
//! - choose start direction so that we have a fence on our right

use std::collections::VecDeque;

use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::HashSet;

type Region = HashSet<Pos>;

type Dir = u8;

const UP: Dir = 0b1000;
const LEFT: Dir = 0b0100;
const DOWN: Dir = 0b0010;
const RIGHT: Dir = 0b0001;

trait DirHelper {
    fn turn_right(self) -> Self;
    fn turn_left(self) -> Self;
    fn to_pos(self) -> Pos;
}

impl DirHelper for Dir {
    fn turn_right(self) -> Self {
        match self {
            UP => RIGHT,
            x => x << 1,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            RIGHT => UP,
            x => x >> 1,
        }
    }

    fn to_pos(self) -> Pos {
        (match self {
            UP => (0, -1),
            DOWN => (0, 1),
            LEFT => (-1, 0),
            RIGHT => (1, 0),
            _ => unreachable!(),
        })
        .into()
    }
}

#[derive(Copy, Clone)]
struct Tile {
    ch: char,
    fences: u8, // UDLR
    start_dir: u8,
}

impl From<char> for Tile {
    fn from(value: char) -> Self { Self { ch: value, fences: 0, start_dir: 0 } }
}

impl Tile {
    fn has_fence(&self, dir: Dir) -> bool { self.fences & dir == dir }
}

struct Explorer<'a> {
    pos: Pos,
    dir: Dir,
    map: &'a Map<Tile>,
    initial: (Pos, Dir),
}

impl<'a> Explorer<'a> {
    fn new(map: &'a Map<Tile>, start_pos: Pos, dir: Dir) -> Self {
        Self { pos: start_pos, dir, map, initial: (start_pos, dir) }
    }

    fn sides<F: FnMut(Pos)>(&mut self, mut visit: F) -> usize {
        let mut turns = 0;
        loop {
            visit(self.pos);

            if turns > 0 && (self.pos, self.dir) == self.initial {
                break;
            }

            let cur_tile = self.map.get_unchecked(self.pos);

            // try to turn right and advance
            let right = self.dir.turn_right();
            if !cur_tile.has_fence(right) {
                turns += 1;
                self.dir = right;
                self.pos += self.dir.to_pos();
                continue;
            }

            // try to advance
            if !cur_tile.has_fence(self.dir) {
                self.pos += self.dir.to_pos();
                continue;
            }

            // try to turn left
            self.dir = self.dir.turn_left();
            turns += 1;
        }
        turns
    }
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/12.in");
    let map_size = Pos::from((
        input.chars().position(|x| x == '\n').unwrap(),
        input.chars().filter(|x| *x == '\n').count(),
    ));
    let mut map = Map::new(map_size, input.chars().filter(|&c| c != '\n').map(Tile::from));

    let mut seen = vec![false; (map.size.x * map.size.y) as usize];
    let mut fenced = Region::with_capacity(1024);
    let mut q = VecDeque::with_capacity(512);

    let index_of = |p: Pos| (p.x + p.y * map_size.x) as usize;

    let mut p1 = 0;
    let mut p2 = 0;

    for pos in map_size.iter() {
        if seen[index_of(pos)] {
            continue;
        }

        let mut area = 0;
        let mut perimeter = 0;
        fenced.clear();

        seen[index_of(pos)] = true;
        q.clear();
        q.push_back(pos);

        while let Some(cur) = q.pop_front() {
            // R,D,L,U
            let crop = map.get_unchecked(cur).ch;

            for (dir, neigh) in cur.neighbors_rdlu().enumerate() {
                if !map.within(neigh) || map.get_unchecked(neigh).ch != crop {
                    let tile = map.get_unchecked_mut_ref(cur);
                    let fence = 1 << dir;
                    tile.fences |= fence;
                    tile.start_dir = fence.turn_left();
                    fenced.insert(cur);
                    perimeter += 1;
                } else if !seen[index_of(neigh)] {
                    seen[index_of(neigh)] = true;
                    q.push_back(neigh);
                }
            }
            area += 1;
        }
        p1 += area * perimeter;
        p2 += area * count_sides(&map, &mut fenced);
    }
    (p1, p2)
}

fn count_sides(map: &Map<Tile>, fenced: &mut Region) -> usize {
    let mut sides = 0;
    while !fenced.is_empty() {
        let start_pos = *fenced.iter().next().unwrap();
        let start_dir = map.get_unchecked(start_pos).start_dir;
        sides += Explorer::new(map, start_pos, start_dir).sides(|pos| {
            fenced.remove(&pos);
        });
    }
    sides
}

aoc_2024::main! {
    solve()
}
