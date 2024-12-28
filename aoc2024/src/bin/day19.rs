use aoc::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use std::collections::HashMap;

type Parsed<'a> = (Vec<&'a str>, Vec<&'a str>);

fn parse_input(input: &str) -> Parsed {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let mut towels: Vec<_> = towels.split(',').map(str::trim).collect();
    towels.sort();
    let designs = designs.lines().collect();
    (towels, designs)
}

fn part1((towels, designs): &Parsed) -> impl std::fmt::Display {
    let re = Regex::new(&format!("^({})+$", towels.join("|"))).unwrap();
    designs.iter().filter(|design| re.is_match(design)).count()
}

fn part2((towels, designs): &Parsed) -> impl std::fmt::Display {
    designs
        .par_iter()
        .map(|design| count_solutions(towels, design))
        .sum::<usize>()
}

fn count_solutions(towels: &[&str], design: &str) -> usize {
    fn inner<'a>(towels: &[&str], design: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(&c) = cache.get(design) {
            return c;
        }

        let count = towels
            .iter()
            .filter(|&towel| design.starts_with(towel))
            .map(|towel| inner(towels, &design[towel.len()..], cache))
            .sum();

        cache.insert(design, count);
        count
    }

    inner(towels, design, &mut HashMap::new())
}

aoc::setup! {
    day19, parse_input;
    part1 == 6,
    part2 == 16
}
