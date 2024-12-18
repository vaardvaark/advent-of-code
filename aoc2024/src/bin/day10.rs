use aoc::*;
use std::collections::HashSet;

type Parsed = Grid<u8>;

fn parse_input(input: &str) -> Parsed {
    gridify_ascii(input.lines())
}

fn part1(grid: &Parsed) -> impl std::fmt::Display {
    grid.position_all(|&v| v == b'0')
        .into_iter()
        .map(|head| {
            let peaks: HashSet<_> = walk(grid.cursor(head), vec![])
                .into_iter()
                .map(|path| path[9])
                .collect();
            peaks.len()
        })
        .sum::<usize>()
}

fn part2(grid: &Parsed) -> impl std::fmt::Display {
    grid.position_all(|&v| v == b'0')
        .into_iter()
        .map(|head| walk(grid.cursor(head), vec![]).len())
        .sum::<usize>()
}

fn walk(cursor: Cursor<u8>, mut path: Vec<Vec2>) -> HashSet<Vec<Vec2>> {
    let &value = cursor.value();
    path.push(cursor.pos());
    if value == b'9' {
        assert_eq!(path.len(), 10);
        return HashSet::from_iter([path]);
    }
    let mut peaks = HashSet::new();
    for direction in [
        Cardinal::North,
        Cardinal::East,
        Cardinal::South,
        Cardinal::West,
    ] {
        if cursor.peek(direction).is_some_and(|&v| v == value + 1) {
            let mut cursor = cursor.clone();
            cursor.step(direction);
            peaks.extend(walk(cursor, path.clone()));
        }
    }
    peaks
}

aoc::setup! {
    day10, parse_input;
    part1 == 36,
    part2 == 81
}
