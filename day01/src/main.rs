use std::{collections::HashMap, fmt::Display};

fn main() {
    let input = std::io::read_to_string(std::io::stdin())
        .expect("puzzle input should be provided on standard input");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> impl Display {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut elements = line.split_whitespace();
        match (elements.next(), elements.next()) {
            (Some(a), Some(b)) => {
                let a: isize = a.parse().unwrap();
                let b: isize = b.parse().unwrap();
                left.push(a);
                right.push(b);
            }
            _ => continue,
        }
    }

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum::<usize>()
}

fn part2(input: &str) -> impl Display {
    let mut left = vec![];
    let mut right: HashMap<usize, usize> = Default::default();
    for line in input.lines() {
        let mut elements = line.split_whitespace();
        match (elements.next(), elements.next()) {
            (Some(a), Some(b)) => {
                let a: usize = a.parse().unwrap();
                let b: usize = b.parse().unwrap();
                left.push(a);
                right.entry(b).and_modify(|count| *count += 1).or_insert(1);
            }
            _ => continue,
        }
    }

    left.into_iter()
        .map(|a| a * right.get(&a).cloned().unwrap_or_default())
        .sum::<usize>()
}

#[cfg(test)]
mod example {
    use crate::*;

    const INPUT: &str = r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "#;

    #[test]
    fn day01_part1() {
        assert_eq!(part1(INPUT).to_string(), "11");
    }

    #[test]
    fn day01_part2() {
        assert_eq!(part2(INPUT).to_string(), "31");
    }
}
