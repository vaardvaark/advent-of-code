use aoc::*;
use std::collections::VecDeque;

type Parsed = Vec<Vec2>;

fn parse_input(input: &str) -> Parsed {
    take_pairs!(input.lines(), ',')
        .map(|(x, y)| Vec2::new(x, y))
        .collect()
}

fn part1(coords: &Parsed, end: Vec2, len: usize) -> impl std::fmt::Display {
    let mut grid = Grid::new_with(1 + end.x as usize, 1 + end.y as usize, false);
    for pos in coords.iter().take(len) {
        grid.set(pos, true);
    }
    let mut min_distance = Grid::new_with(grid.cols(), grid.rows(), u64::MAX);
    compute_distance(0, Vec2::default(), end, &grid, &mut min_distance)
}

fn part2(coords: &Parsed, end: Vec2) -> impl std::fmt::Display {
    let mut grid = Grid::new_with(1 + end.x as usize, 1 + end.y as usize, false);
    for coord in coords {
        grid.set(coord, true);
    }

    let mut min_distance = Grid::new_with(grid.cols(), grid.rows(), u64::MAX);
    min_distance[Vec2::default()] = 0;
    compute_distance(0, Vec2::default(), end, &grid, &mut min_distance);

    for coord in coords.iter().rev() {
        grid.set(coord, false);
        let min = Direction::iter()
            .filter_map(|direction| min_distance.get(&coord.translate(direction)))
            .min()
            .unwrap_or(&u64::MAX);

        if min == &u64::MAX {
            continue;
        }

        if compute_distance(min + 1, *coord, end, &grid, &mut min_distance) != u64::MAX {
            return format!("{},{}", coord.x, coord.y);
        }
    }

    "no solution".to_string()
}

fn compute_distance(
    dist: u64,
    start: Vec2,
    end: Vec2,
    grid: &Grid<bool>,
    min_distance: &mut Grid<u64>,
) -> u64 {
    let mut queue = VecDeque::new();
    queue.push_back((dist, start));

    while let Some((distance, pos)) = queue.pop_front() {
        min_distance[pos] = std::cmp::min(min_distance[pos], distance);
        if pos == end {
            continue;
        }
        for next in Direction::iter().map(|d| pos.translate(d)) {
            if grid.get(&next).is_some_and(|&wall| !wall)
                && min_distance[next] > min_distance[pos] + 1
            {
                min_distance[next] = min_distance[pos] + 1;
                queue.push_back((min_distance[next], next));
            }
        }
    }

    min_distance[end]
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
