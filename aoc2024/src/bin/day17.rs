use aoc::*;
use std::collections::VecDeque;

type Parsed = ([u64; 3], Vec<u8>);

fn parse_input(input: &str) -> Parsed {
    let mut lines = input.lines();
    let a = parse(lines.next().unwrap().split_at(12).1);
    let b = parse(lines.next().unwrap().split_at(12).1);
    let c = parse(lines.next().unwrap().split_at(12).1);
    lines.next();
    let program = parse_list!(lines.next().unwrap().split_at(9).1, ',');
    ([a, b, c], program)
}

fn part1((registers, program): &Parsed) -> impl std::fmt::Display {
    run(*registers, program)
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2((_, program): &Parsed) -> impl std::fmt::Display {
    let mut queue = VecDeque::new();
    queue.push_back((0, program.len()));

    while let Some((a, position)) = queue.pop_front() {
        for i in 0..8 {
            let n = (a << 3) + i;
            let output = run([n, 0, 0], program);
            if output == program[position - 1..] {
                queue.push_back((n, position - 1));
                if output.len() == program.len() {
                    return n.to_string();
                }
            }
        }
    }

    "no solution".to_string()
}

fn run([mut a, mut b, mut c]: [u64; 3], program: &[u8]) -> Vec<u8> {
    let mut pc = 0;
    let mut output = Vec::with_capacity(program.len());
    while pc < program.len() - 1 {
        let opcode = program[pc];
        let literal = program[pc + 1];
        let combo = match literal {
            0..=3 => literal as u64,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!(),
        };

        match opcode {
            0 => a /= 2u64.pow(combo as u32),
            1 => b ^= literal as u64,
            2 => b = combo & 7,
            3 => {
                if a != 0 {
                    pc = literal as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => output.push((combo & 7) as u8),
            6 => b = a / 2u64.pow(combo as u32),
            7 => c = a / 2u64.pow(combo as u32),
            _ => unreachable!(),
        }
        pc += 2;
    }
    output
}

aoc::setup! {
    day17, parse_input;
    part1 == "4,6,3,5,6,3,5,2,1,0",
    "day17-part2.in": part2 == 117440
}
