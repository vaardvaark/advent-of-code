use std::collections::HashMap;

aoc::main!();

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

#[cfg(test)]
mod day01 {

    const EXAMPLE: &str = r#"3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(EXAMPLE).to_string(), "11");
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(EXAMPLE).to_string(), "31");
    }
}
