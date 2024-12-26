//! # Code Chronicle
//!
//! Thunk about something fancy involving hashed differences, but the brute
//! runs in microseconds, so there we go.
//!
//! Happy solstice! 🎄

fn solve() -> (usize, &'static str) {
    let input = include_str!("../../inputs/25.in");

    let mut tumblers = Vec::with_capacity(300);
    let mut keys = Vec::with_capacity(300);

    input.split("\n\n").for_each(|pat| {
        let mut it = pat.bytes();
        let mut buf = (it.next().unwrap() & 1) as u64;
        let is_tumbler = buf == 1;

        it.for_each(|byte| {
            buf <<= (byte != b'\n') as u64;
            buf |= (byte & 1) as u64;
        });

        if is_tumbler {
            tumblers.push(buf)
        } else {
            keys.push(buf);
        }
    });

    let mut p1 = 0;
    for tumbler in &tumblers {
        for key in &keys {
            if tumbler & key == 0 {
                p1 += 1;
            }
        }
    }

    (p1, "💚")
}

aoc_2024::main! {
    solve()
}
