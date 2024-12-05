use aoc_prelude::*;
use std::cmp::Ordering;

fn solve() -> (usize, usize) {
    let (ord, rep) = include_str!("../../inputs/05.in")
        .split_once("\n\n")
        .unwrap();

    let mut before_set = HashMap::new();

    ord.lines().for_each(|line| {
        let (left, right) = line.split_once('|').unwrap();
        let r_num = right.parse::<usize>().unwrap();
        let l_num = left.parse::<usize>().unwrap();
        before_set
            .entry(l_num)
            .or_insert(HashSet::new())
            .insert(r_num);
    });

    let reports = rep
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let p1 = reports
        .iter()
        .filter_map(|rep| {
            if report_ok(rep, &before_set) {
                Some(rep[rep.len() / 2])
            } else {
                None
            }
        })
        .sum();

    let p2 = reports
        .into_iter()
        .filter(|rep| !report_ok(rep, &before_set))
        .map(|mut rep| {
            rep.sort_by(|left, right| {
                if ord_ok((left, right), &before_set) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            rep[rep.len() / 2]
        })
        .sum();

    (p1, p2)
}

fn ord_ok((left, right): (&usize, &usize), before_set: &HashMap<usize, HashSet<usize>>) -> bool {
    if let Some(val) = before_set.get(right) {
        !val.contains(left)
    } else {
        false
    }
}

fn report_ok(report: &[usize], before_set: &HashMap<usize, HashSet<usize>>) -> bool {
    let _ord_ok = |(left, right)| ord_ok((left, right), before_set);
    report.iter().zip(report.iter().skip(1)).all(_ord_ok)
}

aoc_2024::main! {
    solve()
}
