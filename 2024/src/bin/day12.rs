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
use aoc_prelude::{HashMap, HashSet, Itertools};
use rayon::prelude::*;
use std::collections::VecDeque;

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

struct Explorer<'a> {
    pos: Pos,
    dir: Dir,
    map: &'a Map<Tile>,
    initial: (Pos, Dir),
}

impl<'a> Explorer<'a> {
    fn new(map: &'a Map<Tile>, positions: impl Iterator<Item = &'a Pos>) -> Self {
        let pos = leftmost(map, positions);
        Self {
            pos,
            dir: DOWN,
            map,
            initial: (pos, DOWN),
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

    let mut regions = Vec::new();

    let num_regions = assign_regions(&mut map, |region_id, pos| {
        if regions.len() <= region_id {
            regions.push(HashSet::new());
        }
        regions[region_id].insert(pos);
    });

    let (p1, p2) = (0..num_regions)
        .into_par_iter()
        .map(|region_id| {
            let positions = &regions[region_id];
            let (area, perimeter) = area_and_perimeter(&map, positions.iter());
            let sides = Explorer::new(&map, positions.iter()).sides();
            let inner_sides = inner_sides(&map, positions);
            (perimeter * area, (sides + inner_sides) * area)
        })
        .reduce(|| (0, 0), |acc, val| (acc.0 + val.0, acc.1 + val.1));

    (p1, p2)
}

fn assign_regions<F>(map: &mut Map<Tile>, mut cache_builder: F) -> usize
where
    F: FnMut(usize, Pos),
{
    let map_copy = map.clone();

    let to_visit: Vec<_> = map.iter().collect();
    let mut to_set = vec![true; to_visit.len()];
    let mut q = VecDeque::new();
    let mut num_regions = 0;

    while let Some((idx, _)) = to_set.iter().enumerate().find(|(_, b)| **b) {
        q.push_back(to_visit[idx]);

        while let Some(cur) = q.pop_front() {
            let idx_ = (cur.x + cur.y * map.size.x) as usize;
            if !to_set[idx_] {
                continue;
            }
            to_set[idx_] = false;

            cache_builder(num_regions, cur);

            let tile_ref = map.get_unchecked_mut_ref(cur);

            // R,D,L,U
            for (dir, neigh) in cur.neighbors_rdlu().enumerate() {
                if !map_copy.within(neigh) || tile_ref.ch != map_copy.get_unchecked(neigh).ch {
                    tile_ref.fences |= 1 << dir;
                } else {
                    q.push_back(neigh);
                }
            }
        }

        num_regions += 1;
    }

    num_regions
}

fn inner_sides(map: &Map<Tile>, positions: &HashSet<Pos>) -> usize {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for pos in positions {
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
    }

    let our_tile = Tile::from(
        map.get_unchecked(positions.iter().next().expect("empty region"))
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
                if tile.ch == our_tile.ch && positions.contains(&pos) {
                    our_tile
                } else {
                    other_tile
                }
            }),
    );

    let mut region_id_to_pos = HashMap::new();
    let num_regions = assign_regions(&mut mini_map, |region_id, pos| {
        region_id_to_pos
            .entry(region_id)
            .or_insert(Vec::new())
            .push(pos);
    });

    (0..num_regions)
        .filter_map(|region_id| region_id_to_pos.get(&region_id))
        .filter(|positions| !is_outside_region(&mini_map, positions[0]))
        .map(|positions| Explorer::new(&mini_map, positions.iter()).sides())
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
            if map.get(neigh).is_some_and(|t| t.ch == '.') {
                q.push_back(neigh)
            } else if map.get(neigh).is_none() {
                return true;
            }
        }
    }
    false
}

fn area_and_perimeter<'a>(
    map: &'a Map<Tile>,
    positions: impl Iterator<Item = &'a Pos>,
) -> (usize, usize) {
    let mut area = 0;
    let perimeter = positions
        .map(|pos| map.get_unchecked(pos))
        .map(|tile| {
            area += 1;
            tile.fences.count_ones() as usize
        })
        .sum::<usize>();
    (area, perimeter)
}

fn leftmost<'a>(map: &'a Map<Tile>, positions: impl Iterator<Item = &'a Pos>) -> Pos {
    *positions
        .min_by_key(|p| p.x * map.size.y + p.y)
        .expect("empty region")
}

aoc_2024::main! {
    solve()
}
