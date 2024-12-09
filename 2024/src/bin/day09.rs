use std::cmp::max;

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

fn solve_struct() -> (usize, usize) {
    let mut exts = Vec::new();
    let mut start = 0;
    include_str!("../../inputs/09.in")
        .trim()
        .chars()
        .enumerate()
        .for_each(|(idx, c)| {
            let size = ((c as u8) - b'0') as usize;
            if is_file(idx) {
                exts.push(Ext {
                    files: vec![Fext {
                        file_no: idx / 2,
                        size,
                    }],
                    start,
                    free: 0,
                });
                start += size;
            } else {
                exts.push(Ext {
                    files: Vec::new(),
                    start,
                    free: size,
                });
                start += size;
            }
        });

    let mut i = 0;
    let mut j = exts.len() - 1;

    let p2_exts = exts.clone();

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
    let p1: usize = exts
        .iter()
        .filter(|ext| !ext.files.is_empty())
        .map(|e| e.checksum())
        .sum();

    // p2 - reset
    exts = p2_exts;
    j = exts.len() - 1;

    let mut max_move = usize::MAX;

    while j > 1 {
        let tail_file = exts[j].files[0];
        let mut could_move = false;
        // optimization: if we couldn't move size X, ignore all sizes >X
        if tail_file.size < max_move {
            for i in 0..j / 2 {
                let idx = 2 * i + 1;
                if exts[idx].free >= exts[j].files[0].size {
                    // the whole file fits => move it
                    exts[idx].files.push(tail_file);
                    exts[idx].free -= tail_file.size;
                    exts[j].files = Vec::new();
                    could_move = true;
                    break;
                }
            }
            if !could_move {
                max_move = tail_file.size;
            }
        }
        j -= 2;
    }

    let p2: usize = exts
        .iter()
        .filter(|ext| !ext.files.is_empty())
        .map(|e| e.checksum())
        .sum();

    (p1, p2)
}

fn block_sum(start_block: usize, end_block: usize) -> usize {
    ((max(end_block, 1) - 1) * end_block - (max(start_block, 1) - 1) * start_block) / 2
}

fn is_file(cursor: usize) -> bool {
    cursor & 0b1 == 0
}

aoc_2024::main! {
    solve_struct()
}