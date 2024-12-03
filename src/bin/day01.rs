use std::collections::HashMap;

fn main() {
    let input = std::io::read_to_string(std::io::stdin())
        .expect("puzzle input should be provided on standard input");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> impl std::fmt::Display {
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

fn part2(input: &str) -> impl std::fmt::Display {
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
mod day01 {

    const EXAMPLE: &str = r#"
            3   4
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
