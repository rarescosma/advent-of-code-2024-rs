use std::cmp::Ordering;

type MapType = [[bool; 100]; 100];

fn make_map<const N: usize>() -> [[bool; N]; N] {
    [[false; N]; N]
}

fn solve() -> (usize, usize) {
    let (ord, rep) = include_str!("../../inputs/05.in")
        .split_once("\n\n")
        .unwrap();

    let mut before_set = make_map::<100>();

    ord.lines().for_each(|line| {
        let (left, right) = line.split_once('|').unwrap();
        let r_num = right.parse::<usize>().unwrap();
        let l_num = left.parse::<usize>().unwrap();
        before_set[l_num][r_num] = true
    });

    let mut p1 = 0;
    let mut p2 = 0;

    let mut rep_buf = Vec::with_capacity(100);

    rep.lines().for_each(|line| {
        rep_buf.clear();
        rep_buf.extend(line.split(',').filter_map(|n| n.parse::<usize>().ok()));

        if report_ok(&rep_buf, &before_set) {
            p1 += rep_buf[rep_buf.len() / 2]
        } else {
            rep_buf.sort_by(|&left, &right| {
                if before_set[left][right] {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            p2 += rep_buf[rep_buf.len() / 2]
        }
    });

    (p1, p2)
}

fn report_ok(report: &[usize], before_set: &MapType) -> bool {
    report
        .iter()
        .zip(report.iter().skip(1))
        .all(|(&left, &right)| before_set[left][right])
}

aoc_2024::main! {
    solve()
}
