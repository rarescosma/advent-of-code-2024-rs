fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/02.in")
        .lines()
        .map(|line| {
            line.split_whitespace().map(|el| el.parse().unwrap()).collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let p1 = input.clone().into_iter().filter(is_valid).count();

    let p2 = input.into_iter().filter(|row| {
        is_valid(row) || permute(row).any(|del_row| is_valid(&del_row))
    }).count();

    (p1, p2)
}

fn permute(row: &[i32]) -> impl Iterator<Item=Vec<i32>> + '_ {
    (0..row.len()).map(|idx| {
        let mut cp = row.to_owned();
        cp.remove(idx);
        cp
    })
}

fn is_valid(row: &Vec<i32>) -> bool {

    let rev = row.clone().into_iter().rev().collect::<Vec<_>>();

    let mut sorted = row.clone();
    sorted.sort();

    sorted
        .iter()
        .zip(sorted.iter().skip(1))
        .all(|(left, right)| (1..=3).contains(&(right - left)))
        && (row == &sorted || rev == sorted)
}

aoc_2024::main! {
    solve()
}
