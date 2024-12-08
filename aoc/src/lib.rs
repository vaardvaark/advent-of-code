mod grid;
mod vec2;

pub use grid::{gridify_ascii, Cursor, Direction, Grid};
pub use rayon;
pub use vec2::Vec2;

pub trait AllPairs<T> {
    fn all_pairs<'a>(&'a self) -> impl Iterator<Item = (&'a T, &'a T)>
    where
        T: 'a;
}

impl<T> AllPairs<T> for Vec<T> {
    #[inline]
    fn all_pairs<'a>(&'a self) -> impl Iterator<Item = (&'a T, &'a T)>
    where
        T: 'a,
    {
        let len = self.len();
        let mut i = 0;
        let mut j = 0;
        std::iter::from_fn(move || {
            while i < len {
                if i == j {
                    j += 1;
                    continue;
                }
                if j >= len {
                    j = 0;
                    i += 1;
                }
                if i >= len {
                    break;
                }
                let k = j;
                j += 1;
                return Some((&self[i], &self[k]));
            }
            None
        })
    }
}

#[macro_export]
macro_rules! time {
    ($e:expr) => {{
        let start = std::time::Instant::now();
        let res = { $e };
        let elapsed = start.elapsed();
        (res, elapsed)
    }};
}

#[macro_export]
macro_rules! test {
    ($m:ident, $parser:ident, $func:ident, $expect:expr) => {
        #[test]
        fn $func() {
            const INPUT: &str = include_str!(concat!("../examples/", stringify!($m), ".in"));
            let parsed = super::$parser(INPUT);
            assert_eq!(super::$func(&parsed).to_string(), ($expect).to_string());
        }
    };
    ($m:ident, $parser:ident, $f:literal, $func:ident, $expect:expr) => {
        #[test]
        fn $func() {
            const INPUT: &str = include_str!(concat!("../examples/", $f));
            let parsed = super::$parser(INPUT);
            assert_eq!(super::$func(&parsed).to_string(), ($expect).to_string());
        }
    };
    ($m:ident, $func:ident, $expect:expr) => {
        #[test]
        fn $func() {
            const INPUT: &str = include_str!(concat!("../examples/", stringify!($m), ".in"));
            assert_eq!(super::$func(INPUT).to_string(), ($expect).to_string());
        }
    };
    ($m:ident, $f:literal, $func:ident, $expect:expr) => {
        #[test]
        fn $func() {
            const INPUT: &str = include_str!(concat!("../examples/", $f));
            assert_eq!(super::$func(INPUT).to_string(), ($expect).to_string());
        }
    };
}

#[macro_export]
macro_rules! setup {
    ($m:ident, $parser:ident; $($f1:literal:)? $part1:ident == $e1:expr, $($f2:literal:)? $part2:ident == $e2:expr) => {
        fn main() {
            eprintln!("{}", stringify!($m));
            const INPUT: &str = include_str!(concat!("../input/", stringify!($m), ".in"));

            let (parsed, elapsed_parse) = aoc::time!($parser(INPUT));
            eprintln!("parse ({elapsed_parse:?})");

            let (part1, elapsed_part1) = aoc::time!($part1(&parsed));
            eprintln!("part1: {part1} ({elapsed_part1:?})");

            let (part2, elapsed_part2) = aoc::time!($part2(&parsed));
            eprintln!("part2: {part2} ({elapsed_part2:?})");
        }

        #[cfg(test)]
        mod $m {
            $crate::test!($m, $parser, $($f1,)? $part1, $e1);
            $crate::test!($m, $parser, $($f2,)? $part2, $e2);
        }
    };
    ($m:ident; $($f1:literal:)? $part1:ident == $e1:expr, $($f2:literal:)? $part2:ident == $e2:expr) => {
        fn main() {
            eprintln!("{}", stringify!($m));
            const INPUT: &str = include_str!(concat!("../input/", stringify!($m), ".in"));

            let (part1, elapsed_part1) = aoc::time!($part1(INPUT));
            eprintln!("part1: {part1} ({elapsed_part1:?})");

            let (part2, elapsed_part2) = aoc::time!($part2(INPUT));
            eprintln!("part2: {part2} ({elapsed_part2:?})");
        }

        #[cfg(test)]
        mod $m {
            $crate::test!($m, $($f1,)? $part1, $e1);
            $crate::test!($m, $($f2,)? $part2, $e2);
        }
    };
}

pub fn parse<T>(s: impl AsRef<str>) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    s.as_ref().parse().unwrap()
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
