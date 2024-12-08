use aoc::*;
use std::collections::{HashMap, HashSet};

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

fn part1((grid, antennas): &Parsed) -> impl std::fmt::Display {
    let mut nodes = HashSet::new();
    for locs in antennas.values() {
        for (a, b) in locs.all_pairs() {
            let node = *a + (a - b);
            if grid.in_bounds(&node) {
                nodes.insert(node);
            }
        }
    }
    nodes.len()
}

fn part2((grid, antennas): &Parsed) -> impl std::fmt::Display {
    let mut nodes = HashSet::new();
    for locs in antennas.values() {
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

aoc::setup! {
    day08, parse_input;
    part1 == 14,
    part2 == 34
}
