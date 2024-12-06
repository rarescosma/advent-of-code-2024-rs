use aoc_2dmap::prelude::*;
use aoc_prelude::num_integer::Integer;
use aoc_prelude::Itertools;

const DXY: [Pos; 4] = [
    Pos::c_new(0, -1),
    Pos::c_new(1, 0),
    Pos::c_new(0, 1),
    Pos::c_new(-1, 0),
];

fn turn_right(dir: usize) -> usize {
    (dir + 1) % 4
}

fn turn_back(dir: usize) -> usize {
    (dir + 2) % 4
}

struct Buffers {
    states: Vec<bool>,
    visited_pos: Vec<bool>,
}

impl Buffers {
    fn allocate(size: MapSize) -> Self {
        let states = vec![false; (size.x * size.y) as usize * 4];
        let visited_pos = vec![false; (size.x * size.y) as usize];
        Self {
            states,
            visited_pos,
        }
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
    let (p1, _) = sim_map(&map, start, 0, &mut buffers);

    let mut p2 = 0;
    for hash in buffers
        .states
        .clone()
        .iter()
        .enumerate()
        .filter(|x| *x.1)
        .map(|x| x.0)
    {
        let (rest, dir) = hash.div_rem(&4);
        let (x, y) = rest.div_rem(&(map.size.y as usize));

        let blockage = Pos::from((x, y));
        for cand in [blockage, blockage + DXY[dir]] {
            if map.get(cand) == Some('.') {
                map.set(cand, '#');
                let (_, has_cycle) = sim_map(&map, start, 0, &mut buffers);
                map.set(cand, 'b');
                if has_cycle {
                    p2 += 1;
                }
            }
        }
    }

    (p1, p2)
}

fn sim_map(map: &Map<char>, start: Pos, start_dir: usize, buffers: &mut Buffers) -> (usize, bool) {
    let mut cur = start;
    let mut dxy_idx = start_dir;
    buffers.clear();
    buffers.visited_pos[(start.x * map.size.y + start.y) as usize] = true;

    loop {
        cur += DXY[dxy_idx];
        match map.get(cur) {
            None => return (buffers.visited_pos.iter().filter(|&x| *x).count(), false),
            Some(c) => {
                let hash = ((cur.x * map.size.y + cur.y) * 4) as usize + dxy_idx;
                if c == '#' {
                    // backtrack + turn right
                    let opposite = DXY[turn_back(dxy_idx)];
                    cur += opposite;
                    dxy_idx = turn_right(dxy_idx);
                } else {
                    // cycle detected
                    if buffers.states[hash] {
                        return (0, true);
                    }
                }
                buffers.states[hash] = true;
                buffers.visited_pos[(cur.x * map.size.y + cur.y) as usize] = true;
            }
        }
    }
}

aoc_2024::main! {
    solve()
}
