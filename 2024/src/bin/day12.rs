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
//! - for each region we consider a smaller mini-map that contains it, and
//!   replace all other tiles with '.' (so we get an implicit merge of contained sub-regions)
//! - we then look at all regions again to figure out whether we can reach the
//!   mini-map edges. If not => we got ourselves a sub-region
//! - run Explorers on the sub-regions and add the resulting perimeter to the
//!   parent region

use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{HashSet, Itertools};
use rayon::prelude::*;
use std::collections::{BTreeSet, VecDeque};
use std::fmt::{Display, Formatter, Write};

type Region = BTreeSet<Pos>;

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
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Self {
            ch: value,
            fences: 0,
        }
    }
}

impl Tile {
    fn has_fence(&self, dir: Dir) -> bool {
        self.fences & dir == dir
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.ch)
    }
}

struct Explorer<'a> {
    pos: Pos,
    dir: Dir,
    map: &'a Map<Tile>,
    initial: (Pos, Dir),
}

impl<'a> Explorer<'a> {
    fn new(map: &'a Map<Tile>, start_pos: Pos) -> Self {
        Self {
            pos: start_pos,
            dir: DOWN,
            map,
            initial: (start_pos, DOWN),
        }
    }

    fn sides(&mut self) -> usize {
        let mut turns = 0;
        loop {
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

            if (self.pos, self.dir) == self.initial {
                break;
            }
        }
        turns
    }
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/12.in").lines().collect_vec();

    let mut map = Map::new(
        (input[0].len(), input.len()),
        input.join("").chars().map(Tile::from),
    );

    let regions = assign_regions(&mut map);

    let (p1, p2) = regions
        .into_par_iter()
        .map(|region| {
            let (area, perimeter) = area_and_perimeter(&map, &region);
            let sides = Explorer::new(&map, *region.iter().next().expect("empty region")).sides();
            let inner_sides = inner_sides(&map, &region);
            (perimeter * area, (sides + inner_sides) * area)
        })
        .reduce(|| (0, 0), |acc, val| (acc.0 + val.0, acc.1 + val.1));

    (p1, p2)
}

fn assign_regions(map: &mut Map<Tile>) -> Vec<Region> {
    let mut seen = vec![false; (map.size.x * map.size.y) as usize];
    let mut q = VecDeque::new();
    let mut regions = Vec::new();

    let row_size = map.size.x;
    let index_of = |p: Pos| (p.x + p.y * row_size) as usize;

    for pos in map.iter().collect_vec() {
        if seen[index_of(pos)] {
            continue;
        }
        seen[index_of(pos)] = true;
        let mut region = Region::new();
        region.insert(pos);
        q.clear();
        q.push_back(pos);

        while let Some(cur) = q.pop_front() {
            // R,D,L,U
            for (dir, neigh) in cur.neighbors_rdlu().enumerate() {
                if !map.within(neigh) || map.get_unchecked(cur).ch != map.get_unchecked(neigh).ch {
                    map.get_unchecked_mut_ref(cur).fences |= 1 << dir;
                } else if !region.contains(&neigh) {
                    seen[index_of(neigh)] = true;
                    region.insert(neigh);
                    q.push_back(neigh);
                }
            }
        }
        regions.push(region);
    }
    regions
}

fn inner_sides(map: &Map<Tile>, region: &Region) -> usize {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for pos in region {
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
    }

    let our_tile = Tile::from(
        map.get_unchecked(region.iter().next().expect("empty region"))
            .ch,
    );
    let other_tile = Tile::from('.');

    let mut mini_map = Map::new(
        (max_x - min_x + 1, max_y - min_y + 1),
        (min_y..=max_y)
            .cartesian_product(min_x..=max_x)
            .map(|(y, x)| Pos::from((x, y)))
            .map(|pos| {
                let tile = map.get_unchecked(pos);
                if tile.ch == our_tile.ch && region.contains(&pos) {
                    our_tile
                } else {
                    other_tile
                }
            }),
    );

    let regions = assign_regions(&mut mini_map);

    regions
        .into_iter()
        .filter_map(|region| region.into_iter().next())
        .filter(|&pos| mini_map.get_unchecked(pos).ch == '.' && !is_outside_region(&mini_map, pos))
        .map(|pos| Explorer::new(&mini_map, pos).sides())
        .sum()
}

fn is_outside_region(map: &Map<Tile>, start_pos: Pos) -> bool {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(start_pos);

    while let Some(cur) = q.pop_back() {
        if seen.contains(&cur) {
            continue;
        }
        seen.insert(cur);
        for neigh in cur.neighbors_diag() {
            match map.get(neigh) {
                None => return true,
                Some(tile) if tile.ch == '.' => q.push_back(neigh),
                _ => {}
            }
        }
    }
    false
}

fn area_and_perimeter(map: &Map<Tile>, region: &Region) -> (usize, usize) {
    let mut area = 0;
    let perimeter = region
        .iter()
        .map(|pos| map.get_unchecked(pos))
        .map(|tile| {
            area += 1;
            tile.fences.count_ones() as usize
        })
        .sum::<usize>();
    (area, perimeter)
}

aoc_2024::main! {
    solve()
}
