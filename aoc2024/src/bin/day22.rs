use aoc::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

type Parsed = Vec<i64>;

const CYCLES: usize = 2000;

fn parse_input(input: &str) -> Parsed {
    input.lines().map(parse).collect()
}

fn part1(secrets: &Parsed) -> impl std::fmt::Display {
    let secrets = secrets.clone();
    secrets
        .into_par_iter()
        .map(|mut secret| {
            for _ in 0..CYCLES {
                secret = next_secret(secret);
            }
            secret
        })
        .sum::<i64>()
}

fn part2(secrets: &Parsed) -> impl std::fmt::Display {
    let mut map: HashMap<[i16; 4], Vec<i16>> = Default::default();

    let vendors = secrets.len();
    let secrets = secrets.clone();
    for (monke, mut secret) in secrets.into_iter().enumerate() {
        let mut prices = Vec::with_capacity(CYCLES);
        prices.push(secret.rem_euclid(10) as i16);

        for cycle in 1..CYCLES {
            secret = next_secret(secret);

            let price = secret.rem_euclid(10) as i16;
            prices.push(price);

            if cycle > 3 {
                let window = [
                    prices[cycle - 3] - prices[cycle - 4],
                    prices[cycle - 2] - prices[cycle - 3],
                    prices[cycle - 1] - prices[cycle - 2],
                    prices[cycle] - prices[cycle - 1],
                ];

                let monkes = map.entry(window).or_insert_with(|| vec![0; vendors]);
                if monkes[monke] == 0 {
                    monkes[monke] = price;
                }
            }
        }
    }

    map.into_values()
        .map(|v| v.iter().sum::<i16>())
        .max()
        .unwrap()
}

fn next_secret(value: i64) -> i64 {
    fn mix(sec: i64, val: i64) -> i64 {
        (sec ^ val).rem_euclid(16777216)
    }
    let value = mix(value, value * 64);
    let value = mix(value, value / 32);
    mix(value, value * 2048)
}

aoc::setup! {
    day22, parse_input;
    part1 == 37327623,
    "day22-alt.in": part2 == 23
}
