use std::collections::HashMap;

fn part1(input: &str) -> impl std::fmt::Display {
    let (mut left, mut right): (Vec<_>, Vec<_>) = take_pairs!(input.lines(), isize).unzip();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum::<usize>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let mut left = vec![];
    let mut right: HashMap<usize, usize> = Default::default();
    for (a, b) in take_pairs!(input.lines()) {
        left.push(a);
        right.entry(b).and_modify(|count| *count += 1).or_insert(1);
    }
    left.into_iter()
        .map(|a| a * right.get(&a).cloned().unwrap_or_default())
        .sum::<usize>()
}

aoc::aoc!(day01, "11", "31");
