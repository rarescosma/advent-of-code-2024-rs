//! # Hoof It
//!
//! Both parts solvable through DFS: set of unique positions for the end goals (9) for part 1,
//! and number of times we reach the end goal for part 2.
use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{HashSet, Itertools};
use std::collections::VecDeque;

struct Buf {
    seen: Vec<bool>,
    ends: HashSet<Pos>,
    queue: VecDeque<Pos>,
}

impl Buf {
    fn sized<const N: usize>() -> Self {
        Self {
            seen: vec![false; N],
            ends: HashSet::with_capacity(16),
            queue: VecDeque::with_capacity(64),
        }
    }

    fn clear(&mut self) {
        self.seen.fill(false);
        self.ends.clear();
        self.queue.clear();
    }
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/10.in").lines().collect_vec();

    let map = Map::new(
        (input[0].len(), input.len()),
        input.join("").as_bytes().iter().map(|b| b - b'0'),
    );

    let mut buf = Buf::sized::<{ 59 * 59 }>();

    map.iter()
        .filter(|p| map.get_unchecked(p) == 0)
        .map(|p| dfs(p, &map, &mut buf))
        .fold((0, 0), |acc, el| (acc.0 + el.0, acc.1 + el.1))
}

fn dfs(start: Pos, map: &Map<u8>, buf: &mut Buf) -> (usize, usize) {
    buf.clear();
    let mut res = 0;

    buf.queue.push_back(start);

    while !buf.queue.is_empty() {
        let cur = buf.queue.pop_front().unwrap();
        let idx = (cur.y * map.size.x + cur.x) as usize;
        buf.seen[idx] = true;

        let cur_val = map.get_unchecked(cur);
        if cur_val == 9 {
            res += 1;
            buf.ends.insert(cur);
            continue;
        }

        buf.queue.extend(cur.neighbors_simple().filter_map(|n| {
            map.get(n).and_then(|val| {
                let n_idx = (n.y * map.size.x + n.x) as usize;
                if val == cur_val + 1 && !buf.seen[n_idx] {
                    Some(n)
                } else {
                    None
                }
            })
        }));
    }

    (buf.ends.len(), res)
}

aoc_2024::main! {
    solve()
}
