use aoc::*;
use regex::Regex;
use std::str::Lines;

type Ty = (i64, i64);
type Parsed = Vec<(Ty, Ty, Ty)>;

fn parse_input(input: &str) -> Parsed {
    let re = Regex::new(r#"X.?(?<x>[0-9]+), Y.?(?<y>[0-9]+)"#).unwrap();
    let extract = |line: &str| -> Option<Ty> {
        re.captures(line).map(|caps| {
            let (_, [x, y]) = caps.extract();
            (parse(x), parse(y))
        })
    };

    let parse = |lines: &mut Lines<'_>| -> Option<(Ty, Ty, Ty)> {
        let a = lines.next().and_then(extract)?;
        let b = lines.next().and_then(extract)?;
        let p = lines.next().and_then(extract)?;
        lines.next();
        Some((a, b, p))
    };

    let mut output = vec![];
    let mut lines = input.lines();
    while let Some(x) = parse(&mut lines) {
        output.push(x);
    }
    output
}

fn part1(input: &Parsed) -> impl std::fmt::Display {
    let mut total = 0;
    for &(a, b, c) in input {
        // https://en.wikipedia.org/wiki/Cramer%27s_rule#Explicit_formulas_for_small_systems
        assert_ne!(a.0 * b.1 - a.1 * b.0, 0);
        let x = (c.0 * b.1 - b.0 * c.1) / (a.0 * b.1 - b.0 * a.1);
        let y = (a.0 * c.1 - c.0 * a.1) / (a.0 * b.1 - b.0 * a.1);
        // Verify the integer solution is valid.
        if a.0 * x + b.0 * y == c.0 && a.1 * x + b.1 * y == c.1 {
            let price = 3 * x + y;
            total += price;
        }
    }
    total
}

fn part2(input: &Parsed) -> impl std::fmt::Display {
    let mut input = input.clone();
    for input in input.iter_mut() {
        input.2 .0 += 10000000000000;
        input.2 .1 += 10000000000000;
    }
    part1(&input)
}

aoc::setup! {
    day13, parse_input;
    part1 == 480,
    part2 == 875318608908_i64
}
