use aoc::*;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../../input/aoc2024-day08.txt");

type Parsed = (Grid<u8>, HashMap<u8, Vec<Vec2>>);

fn parse_input(input: &str) -> Parsed {
    let grid = gridify_ascii(input.lines());
    let mut antennas: HashMap<u8, Vec<Vec2>> = HashMap::new();
    for pos in grid.position_all(|v| ![b'.', b'#'].contains(v)) {
        antennas
            .entry(grid[pos])
            .and_modify(|locs| locs.push(pos))
            .or_insert(vec![pos]);
    }
    (grid, antennas)
}

fn part1((grid, antennas): &Parsed) -> usize {
    let mut nodes = HashSet::new();
    for (_, locs) in antennas {
        for (a, b) in locs.all_pairs() {
            let node = *a + (a - b);
            if grid.in_bounds(&node) {
                nodes.insert(node);
            }
        }
    }
    nodes.len()
}

fn part2((grid, antennas): &Parsed) -> usize {
    let mut nodes = HashSet::new();
    for (_, locs) in antennas {
        for (a, b) in locs.all_pairs() {
            let mut antenna = *a;
            let vector = antenna - *b;
            while grid.in_bounds(&antenna) {
                nodes.insert(antenna);
                antenna += vector;
            }
        }
    }
    nodes.len()
}

fn main() {
    let (parsed, elapsed_parse) = aoc::time!(parse_input(INPUT));
    let (part1, elapsed_part1) = aoc::time!(part1(&parsed));
    let (part2, elapsed_part2) = aoc::time!(part2(&parsed));

    eprintln!("parse ({elapsed_parse:?})");
    eprintln!("part1: {part1} ({elapsed_part1:?})");
    eprintln!("part2: {part2} ({elapsed_part2:?})");
}

#[cfg(test)]
mod day08 {
    #[test]
    fn part1() {
        let input = super::parse_input(include_str!("../../examples/example08.txt"));
        assert_eq!(super::part1(&input), 14);
    }

    #[test]
    fn part2() {
        let input = super::parse_input(include_str!("../../examples/example08.txt"));
        assert_eq!(super::part2(&input), 34);
    }

    #[test]
    fn part2_alt() {
        let input = super::parse_input(include_str!("../../examples/example08-alt.txt"));
        assert_eq!(super::part2(&input), 9);
    }
}
