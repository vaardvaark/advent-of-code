mod grid;
use std::path::PathBuf;

pub use grid::{gridify_ascii, Cursor, Grid, Pos};

#[macro_export]
macro_rules! main {
    () => {
        use aoc::*;

        const CAL: &str = env!("CARGO_PKG_NAME");
        const DAY: &str = env!("CARGO_BIN_NAME");

        fn main() {
            let input = if let Some(path) = std::env::args().skip(1).next() {
                std::fs::read_to_string(&path)
                    .expect(&format!("failed to read input from {path:?}"))
            } else {
                puzzle_input(CAL, DAY)
            };
            println!("{DAY}");
            println!("part1: {}", part1(&input));
            println!("part2: {}", part2(&input));
        }
    };
}

#[macro_export]
macro_rules! tests {
    ($day:ident, $value01:literal) => {
        #[cfg(test)]
        mod $day {
            #[test]
            fn part1() {
                let base = env!("CARGO_MANIFEST_DIR");
                let input = aoc::test_input(base, super::DAY, aoc::Part::Part1);
                assert_eq!(super::part1(&input).to_string(), $value01);
            }
        }
    };
    ($day:ident, $value01:literal, $value02:literal) => {
        #[cfg(test)]
        mod $day {
            #[test]
            fn part1() {
                let base = env!("CARGO_MANIFEST_DIR");
                let input = aoc::test_input(base, super::DAY, aoc::Part::Part1);
                assert_eq!(super::part1(&input).to_string(), $value01);
            }
            #[test]
            fn part2() {
                let base = env!("CARGO_MANIFEST_DIR");
                let input = aoc::test_input(base, super::DAY, aoc::Part::Part2);
                assert_eq!(super::part2(&input).to_string(), $value02);
            }
        }
    };
}

#[macro_export]
macro_rules! aoc {
    ($day:ident) => {
        aoc::main!();
    };
    ($day:ident, $value01:literal) => {
        aoc::main!();
        aoc::tests!($day, $value01);
    };
    ($day:ident, $value01:literal, $value02:literal) => {
        aoc::main!();
        aoc::tests!($day, $value01, $value02);
    };
}

#[derive(Debug)]
pub enum Part {
    Part1,
    Part2,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Part1 => write!(f, "part1"),
            Self::Part2 => write!(f, "part2"),
        }
    }
}

pub fn test_input(base: &str, day: &str, part: Part) -> String {
    let mut base = PathBuf::from(base);
    base.push("examples");

    let mut part_input = base.clone();
    part_input.push(format!("{}-{part}.txt", day.replace("day", "example")));

    if let Ok(input) = std::fs::read_to_string(&part_input) {
        return input;
    }

    // try more general example
    part_input.pop();
    part_input.push(format!("{}.txt", day.replace("day", "example")));
    if let Ok(input) = std::fs::read_to_string(&part_input) {
        return input;
    }

    panic!("could not load example input");
}

pub fn puzzle_input(year: &str, day: &str) -> String {
    let mut path = std::env::current_dir().unwrap();
    path.push(format!("input"));
    path.push(format!("{year}-{day}.txt"));

    let input = match std::fs::read_to_string(&path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("failed to read puzzle input from {path:?}: {error}");
            eprintln!("place puzzle input in {path:?}, or provide filename on command line.");
            std::process::exit(1);
        }
    };

    if input.is_empty() {
        eprintln!("WARNING: puzzle input empty");
    }

    input
}

pub fn parse<T>(s: impl AsRef<str>) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    s.as_ref().parse().unwrap()
}

/// Iterates from `(0, 0)`, `(1, 0)`, (2, 0)`, ... to `(cols, rows)`.
pub fn iter_pos(rows: usize, cols: usize) -> impl Iterator<Item = Pos> {
    let mut row = 0;
    let mut col = 0;
    std::iter::from_fn(move || {
        if col >= cols {
            col = 0;
            row += 1;
        }
        if row >= rows {
            return None;
        }
        let c = col;
        col += 1;
        Some((c, row))
    })
}

#[macro_export]
macro_rules! parse_list {
    ($line:expr) => {
        $line.split_whitespace().map(parse).collect::<Vec<_>>()
    };
    ($line:expr, $ty:ty) => {
        $line.split_whitespace().map(parse).collect::<Vec<$ty>>()
    };
    ($line:expr, $pat:literal) => {
        $line.split($pat).map(parse).collect::<Vec<_>>()
    };
    ($line:expr, $pat:literal, $ty:ty) => {
        $line.split($pat).map(parse).collect::<Vec<$ty>>()
    };
}

#[macro_export]
macro_rules! take_lists {
    ($iter:expr, $pat:literal) => {
        $iter.map_while(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let parsed: Vec<_> = line.split($pat).map(parse).collect();
            Some(parsed)
        })
    };
    ($iter:expr) => {
        $iter.map_while(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let parsed: Vec<_> = line.split_whitespace().map(parse).collect();
            Some(parsed)
        })
    };
}

#[macro_export]
macro_rules! take_pairs {
    ($iter:expr, $pat:literal) => {
        $iter.map_while(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let mut parsed = line.split($pat).map(parse);
            let a = parsed.next().unwrap();
            let b = parsed.next().unwrap();
            assert!(
                parsed.next().is_none(),
                "expected a pair found an extra element in {line}"
            );
            Some((a, b))
        })
    };
    ($iter:expr, $ty:ty) => {
        $iter.map_while(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let mut parsed = line.split_whitespace().map(|x| parse::<$ty>(x));
            let a = parsed.next().unwrap();
            let b = parsed.next().unwrap();
            assert!(
                parsed.next().is_none(),
                "expected a pair found an extra element in {line}"
            );
            Some((a, b))
        })
    };
    ($iter:expr) => {
        $iter.map_while(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let mut parsed = line.split_whitespace().map(parse);
            let a = parsed.next().unwrap();
            let b = parsed.next().unwrap();
            assert!(
                parsed.next().is_none(),
                "expected a pair found an extra element in {line}"
            );
            Some((a, b))
        })
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_pairs() {
        let pairs = "1 2\n3 4\n4 5";
        let mut pairs = take_pairs!(pairs.lines());
        assert_eq!(pairs.next(), Some((1, 2)));
        assert_eq!(pairs.next(), Some((3, 4)));
        assert_eq!(pairs.next(), Some((4, 5)));
        assert_eq!(pairs.next(), None);
    }

    #[test]
    fn custom_pairs() {
        let pairs = "1|2\n3|4\n4|5";
        let mut pairs = take_pairs!(pairs.lines(), '|');
        assert_eq!(pairs.next(), Some((1, 2)));
        assert_eq!(pairs.next(), Some((3, 4)));
        assert_eq!(pairs.next(), Some((4, 5)));
        assert_eq!(pairs.next(), None);
    }
}
