use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

use aoc::*;
use regex::Regex;

struct Robot {
    pos: Vec2,
    vel: Vec2,
}

type Parsed = Vec<Robot>;

fn parse_input(input: &str) -> Parsed {
    let re =
        Regex::new(r#"p=(?<px>[0-9]+),(?<py>[0-9]+) v=(?<vx>\-?[0-9]+),(?<vy>\-?[0-9]+)"#).unwrap();
    let extract = |line: &str| -> Option<Robot> {
        re.captures(line).map(|caps| {
            let (_, [px, py, vx, vy]) = caps.extract();
            Robot {
                pos: Vec2::new(parse(px), parse(py)),
                vel: Vec2::new(parse(vx), parse(vy)),
            }
        })
    };

    input.lines().map(|line| extract(line).unwrap()).collect()
}

fn part1(input: &Parsed, max: Vec2) -> impl std::fmt::Display {
    const SECONDS: i64 = 100;

    let mut quads = [0, 0, 0, 0];
    for robot in input {
        let x = (robot.pos.x + robot.vel.x * SECONDS).rem_euclid(max.x);
        let y = (robot.pos.y + robot.vel.y * SECONDS).rem_euclid(max.y);
        let quad = match (x.cmp(&(max.x / 2)), y.cmp(&(max.y / 2))) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => continue,
            (Ordering::Less, Ordering::Less) => 0,
            (Ordering::Greater, Ordering::Less) => 1,
            (Ordering::Less, Ordering::Greater) => 2,
            (Ordering::Greater, Ordering::Greater) => 3,
        };

        quads[quad] += 1;
    }
    quads.into_iter().product::<i64>()
}

fn part2(input: &Parsed, max: Vec2) {
    fn simulate(bots: &Parsed, seconds: i64, dims: Vec2) -> Vec<Vec2> {
        let mut pos = vec![];
        for robot in bots {
            let x = (robot.pos.x + robot.vel.x * seconds).rem_euclid(dims.x);
            let y = (robot.pos.y + robot.vel.y * seconds).rem_euclid(dims.y);
            pos.push(Vec2::new(x, y));
        }
        pos
    }

    let mut candidates: BTreeMap<usize, i64> = Default::default();
    for i in 0..(max.x * max.y) {
        let positions = simulate(input, i, max);
        let mut bot_adjacency_x = BTreeSet::new();
        let mut bot_adjacency_y = BTreeSet::new();
        for y in 0..max.y {
            let mut xs: Vec<_> = positions
                .iter()
                .filter_map(|&v| if v.y == y { Some(v.x) } else { None })
                .collect();

            xs.sort();
            let bot_rows = xs.windows(2).filter(|w| w[0] + 1 == w[1]).count();
            bot_adjacency_x.insert(bot_rows);
        }
        for x in 0..max.x {
            let mut ys: Vec<_> = positions
                .iter()
                .filter_map(|&v| if v.x == x { Some(v.y) } else { None })
                .collect();

            ys.sort();
            let bot_cols = ys.windows(2).filter(|w| w[0] + 1 == w[1]).count();
            bot_adjacency_y.insert(bot_cols);
        }
        candidates.insert(bot_adjacency_x.len() * bot_adjacency_y.len(), i);
    }

    if let Some((factor, seconds)) = candidates.pop_last() {
        println!("candidate: {seconds}, factor = {factor:?}");
        let mut grid: Grid<bool> = Grid::new(max.x as usize, max.y as usize);
        for pos in simulate(input, seconds, max) {
            grid[pos] |= true;
        }
        println!("{grid}\n");
    }
}

fn main() {
    println!("day14");
    const INPUT: &str = include_str!("../input/day14.in");
    let (input, pre) = aoc::load(INPUT);
    let trimmed_input = input.trim();

    let (parsed, elapsed_parse) = aoc::time!(parse_input(trimmed_input));
    println!("{pre}parse ({elapsed_parse:?})");

    const DIMENSIONS: (usize, usize) = (101, 103);
    let (part1, elapsed_part1) = aoc::time!(part1(&parsed, DIMENSIONS.into()));
    println!("part1: {part1} ({elapsed_part1:?})");

    let (_, elapsed_part2) = aoc::time!(part2(&parsed, DIMENSIONS.into()));
    println!("part2: () ({elapsed_part2:?})");
}

#[cfg(test)]
mod day14 {
    #[test]
    fn part1() {
        const INPUT: &str = include_str!("../examples/day14.in");
        let parsed = super::parse_input(INPUT);
        assert_eq!(
            super::part1(&parsed, (11, 7).into()).to_string(),
            (12).to_string()
        );
    }
}
