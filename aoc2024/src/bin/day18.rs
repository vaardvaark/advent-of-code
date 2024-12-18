use aoc::*;
use std::{cmp::Reverse, collections::BinaryHeap};

type Parsed = Vec<Vec2>;

fn parse_input(input: &str) -> Parsed {
    take_pairs!(input.lines(), ',')
        .map(|(x, y)| Vec2::new(x, y))
        .collect()
}

fn part1(coords: &Parsed, end: Vec2, len: usize) -> impl std::fmt::Display {
    let mut grid: Grid<bool> = Grid::new_empty(end.y as usize + 1, end.x as usize + 1);
    for pos in coords.iter().take(len) {
        grid.set(pos, true);
    }
    println!("{grid}");
    solve(Vec2::default(), end, &grid).expect("no route to exit")
}

fn part2(coords: &Parsed, end: Vec2) -> impl std::fmt::Display {
    let mut grid: Grid<bool> = Grid::new_empty(end.y as usize + 1, end.x as usize + 1);
    for i in 0..coords.len() {
        grid.set(&coords[i], true);
        if solve(Vec2::default(), end, &grid).is_none() {
            grid.set(&coords[i], false);
            println!("{grid}");
            return format!("{},{}", coords[i].x, coords[i].y);
        }
    }
    "no solution".to_string()
}

fn solve(start: Vec2, end: Vec2, grid: &Grid<bool>) -> Option<u64> {
    let mut queue = BinaryHeap::new();
    let mut costs = Grid::new_with(grid.cols(), grid.rows(), u64::MAX);
    queue.push(Reverse((0, start)));

    while let Some(Reverse((cost, pos))) = queue.pop() {
        if costs[pos] != u64::MAX {
            continue;
        }

        if pos == end {
            return Some(cost);
        }

        costs.set(&pos, cost);
        for direction in Direction::iter() {
            let next = pos.translate(direction);
            if grid.get(&next).is_some_and(|&v| v == false) && costs[next] > cost {
                queue.push(Reverse((cost + 1, next)));
            }
        }
    }

    None
}

fn main() {
    println!("day18");
    const INPUT: &str = include_str!("../input/day18.in");
    let (input, pre) = aoc::load(INPUT);
    let trimmed_input = input.trim();

    let mut args = std::env::args().skip(2);
    let x: i64 = args
        .next()
        .as_deref()
        .unwrap_or("70")
        .parse()
        .expect("Expected number of columns");

    let y: i64 = args
        .next()
        .as_deref()
        .unwrap_or("70")
        .parse()
        .expect("Expected number of rows");

    let len: usize = args
        .next()
        .as_deref()
        .unwrap_or("1024")
        .parse()
        .expect("Expected number of bytes to simulate");

    let (parsed, elapsed_parse) = aoc::time!(parse_input(trimmed_input));
    println!("{pre}parse ({elapsed_parse:?})");

    let dimensions: Vec2 = Vec2::new(x, y);
    let (part1, elapsed_part1) = aoc::time!(part1(&parsed, dimensions, len));
    println!("part1: {part1} ({elapsed_part1:?})");

    let (part2, elapsed_part2) = aoc::time!(part2(&parsed, dimensions));
    println!("part2: {part2} ({elapsed_part2:?})");
}

#[cfg(test)]
mod day18 {
    use aoc::Vec2;

    #[test]
    fn part1() {
        const INPUT: &str = include_str!("../examples/day18.in");
        let parsed = super::parse_input(INPUT);
        assert_eq!(
            super::part1(&parsed, Vec2::new(6, 6), 12).to_string(),
            (22).to_string()
        );
    }
    #[test]
    fn part2() {
        const INPUT: &str = include_str!("../examples/day18.in");
        let parsed = super::parse_input(INPUT);
        assert_eq!(super::part2(&parsed, Vec2::new(6, 6)).to_string(), "6,1");
    }
}
