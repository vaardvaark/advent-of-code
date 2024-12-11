use aoc::*;
use std::collections::HashMap;

type Parsed = HashMap<u64, u64>;

fn parse_input(input: &str) -> Parsed {
    let mut stones = HashMap::new();
    for stone in input.split_whitespace().map(parse) {
        *stones.entry(stone).or_default() += 1;
    }
    stones
}

fn part1(stones: &Parsed) -> impl std::fmt::Display {
    solve(stones, 25)
}

fn part2(stones: &Parsed) -> impl std::fmt::Display {
    solve(stones, 75)
}

fn solve(stones: &Parsed, iterations: u64) -> u64 {
    let mut stones = stones.clone();

    for _ in 0..iterations {
        let mut new_stones = HashMap::with_capacity(stones.len());
        for (key, count) in stones {
            match key {
                0 => *new_stones.entry(1).or_default() += count,
                _ => {
                    let digit_count = key.ilog10() + 1;
                    if digit_count % 2 == 0 {
                        *new_stones
                            .entry(key % 10u64.pow(digit_count / 2))
                            .or_default() += count;
                        *new_stones
                            .entry(key / 10u64.pow(digit_count / 2))
                            .or_default() += count;
                    } else {
                        *new_stones.entry(key * 2024).or_default() += count;
                    }
                }
            }
        }
        stones = new_stones;
    }

    stones.into_values().sum()
}

aoc::setup! {
    day11, parse_input;
    part1 == 55312,
    part2 == 65601038650482_u64
}
