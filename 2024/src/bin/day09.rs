//! # Disk Fragmenter
//!
//! Part 1: break the the tail file into chunks if it can't fit into the current
//! empty extent.
//!
//! Part 2: Since free space blocks can only have size 1..9, keep a list of nine
//! `BinaryHeap`s containing the extent indexes (in reversed order). To place a file
//! iterate through all binary heaps that can fit it (`file_size..=0`) and select the
//! extent with the minimum index.
//!
//! Gotcha was to stop moving files if the destination index is higher than the file
//! index.
use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone)]
struct Fext {
    file_no: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct Ext {
    start: usize,
    files: Vec<Fext>,
    free: usize,
}

impl Ext {
    fn checksum(&self) -> usize {
        if self.files.is_empty() {
            return 0;
        }
        let mut res = 0;
        let mut block_id = self.start;
        self.files.iter().for_each(|f| {
            res += block_sum(block_id, block_id + f.size) * f.file_no;
            block_id += f.size;
        });
        res
    }
}

fn solve() -> (usize, usize) {
    let mut p1_exts = Vec::with_capacity(10000);
    let mut start = 0;
    let mut spaces = vec![BinaryHeap::new(); 10];

    include_str!("../../inputs/09.in")
        .trim()
        .chars()
        .enumerate()
        .for_each(|(idx, c)| {
            let size = ((c as u8) - b'0') as usize;
            if is_file(idx) {
                p1_exts.push(Ext {
                    files: vec![Fext {
                        file_no: idx / 2,
                        size,
                    }],
                    start,
                    free: 0,
                });
                start += size;
            } else {
                spaces[size].push(Reverse(idx));
                p1_exts.push(Ext {
                    files: Vec::new(),
                    start,
                    free: size,
                });
                start += size;
            }
        });
    let mut p2_exts = p1_exts.clone();

    part1(&mut p1_exts);
    part2(&mut p2_exts, &mut spaces);

    (checksum(&p1_exts), checksum(&p2_exts))
}

fn part1(exts: &mut [Ext]) {
    let mut i = 0;
    let mut j = exts.len() - 1;

    'outer: while i <= j {
        // it's a file
        if exts[i].free != 0 {
            while exts[i].free > 0 {
                let tail_file = exts[j].files[0];

                if tail_file.size >= exts[i].free {
                    // last file larger than free space => move part of it
                    let head_free = exts[i].free;

                    exts[i].files.push(Fext {
                        file_no: tail_file.file_no,
                        size: head_free,
                    });

                    exts[i].free = 0;
                    exts[j].files[0].size -= head_free;

                    break;
                } else {
                    // last file smaller than free space => move all of it
                    exts[i].files.push(tail_file);
                    exts[i].free -= tail_file.size;
                    exts[j].files = Vec::new();
                    j -= 2;
                    if j <= i {
                        break 'outer;
                    }
                }
            }
        }
        i += 1;
    }
}

fn part2(exts: &mut [Ext], spaces: &mut [BinaryHeap<Reverse<usize>>]) {
    let exts_len = exts.len();

    let mut max_move = usize::MAX;
    (0..exts_len / 2).for_each(|cnt| {
        let j = exts_len - cnt * 2 - 1;

        let tail_file = exts[j].files[0];

        // optimization: if we couldn't move size X, ignore all sizes >X
        if tail_file.size >= max_move {
            return;
        }

        if let Some(Reverse(idx)) = (tail_file.size..10)
            .filter_map(|bucket| {
                spaces[bucket]
                    .peek()
                    .take_if(|x| x.0 <= j)
                    .map(|x| (bucket, x.0))
            })
            .min_by(|(_, idx1), (_, idx2)| idx1.cmp(idx2))
            .and_then(|(bucket, _)| spaces[bucket].pop())
        {
            exts[idx].files.push(tail_file);
            exts[idx].free -= tail_file.size;
            if exts[idx].free > 0 {
                spaces[exts[idx].free].push(Reverse(idx));
            }
            exts[j].files.clear();
        } else {
            max_move = tail_file.size;
        }
    });
}

fn checksum(exts: &[Ext]) -> usize {
    exts.iter()
        .filter(|ext| !ext.files.is_empty())
        .map(|e| e.checksum())
        .sum()
}

fn block_sum(start_block: usize, end_block: usize) -> usize {
    ((max(end_block, 1) - 1) * end_block - (max(start_block, 1) - 1) * start_block) / 2
}

fn is_file(cursor: usize) -> bool {
    cursor & 0b1 == 0
}

aoc_2024::main! {
    solve()
}
