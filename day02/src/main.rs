use std::{cmp::Ordering, fmt::Display};

fn main() {
    let input = std::io::read_to_string(std::io::stdin())
        .expect("puzzle input should be provided on standard input");
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

/// Determines whether an iterator produces monotonically increasing,
/// or monotonically decreasing values.
fn monotonic<I, T>(mut iterator: I) -> bool
where
    I: Iterator<Item = T>,
    T: Copy + Ord,
{
    let mut first = None;
    let mut has_increased = false;
    let mut has_decreased = false;

    while let Some(next) = iterator.next() {
        let Some(prev) = first.replace(next) else {
            continue;
        };

        match next.cmp(&prev) {
            Ordering::Equal => continue,
            Ordering::Less => {
                has_decreased = true;
                if has_increased {
                    return false;
                }
            }
            Ordering::Greater => {
                has_increased = true;
                if has_decreased {
                    return false;
                }
            }
        }
    }

    true
}

fn part1(input: &str) -> impl Display {
    input
        .lines()
        .filter(|line| {
            let samples: Vec<_> = line
                .split_whitespace()
                .flat_map(|v| v.parse::<isize>().ok())
                .collect();

            if samples.is_empty() || !monotonic(samples.iter()) {
                return false;
            }

            samples
                .windows(2)
                .map(|window| window[0].abs_diff(window[1]))
                .all(|delta| delta >= 1 && delta <= 3 && delta != 0)
        })
        .count()
}

fn part2(input: &str) -> impl Display {
    input
        .lines()
        .filter(|line| {
            let master_samples: Vec<_> = line
                .split_whitespace()
                .flat_map(|v| v.parse::<isize>().ok())
                .collect();

            if master_samples.is_empty() {
                return false;
            }

            (0..master_samples.len())
                .map(|index| {
                    let mut samples = master_samples.clone();
                    if index < samples.len() {
                        samples.remove(index);
                    }

                    if samples.is_empty() || !monotonic(samples.iter()) {
                        return false;
                    }

                    samples
                        .windows(2)
                        .map(|window| window[0].abs_diff(window[1]))
                        .all(|delta| delta >= 1 && delta <= 3 && delta != 0)
                })
                .any(|v| v)
        })
        .count()
}

#[cfg(test)]
mod example {
    use crate::*;

    const INPUT: &str = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "#;

    #[test]
    fn day02_part1() {
        assert_eq!(part1(INPUT).to_string(), "2");
    }

    #[test]
    fn day02_part2() {
        assert_eq!(part2(INPUT).to_string(), "4");
    }
}
