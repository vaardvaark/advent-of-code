use aoc::*;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

type Parsed = (Grid<u8>, Vec2, Vec2);

fn parse_input(input: &str) -> Parsed {
    let grid = gridify_ascii(input.lines());
    let start = grid.position(|&v| v == b'S').unwrap();
    let end = grid.position(|&v| v == b'E').unwrap();
    (grid, start, end)
}

fn part1(parsed: &Parsed) -> impl std::fmt::Display {
    let (score, _) = solve(parsed).unwrap();
    score
}

fn part2(parsed: &Parsed) -> impl std::fmt::Display {
    let end = parsed.2;
    let (score, scores) = solve(parsed).unwrap();

    let mut queue = VecDeque::new();
    for dir in Direction::iter() {
        if scores.get(&(end, dir)).is_some_and(|&c| c == score) {
            queue.push_back((score, end, dir));
        }
    }

    let mut visited = HashSet::new();
    while let Some((score, pos, dir)) = queue.pop_front() {
        visited.insert(pos);

        let prev_pos = pos.translate(dir.reverse());
        if score >= 1
            && scores
                .get(&(prev_pos, dir))
                .is_some_and(|&c| c == score - 1)
        {
            queue.push_back((score - 1, prev_pos, dir));
        }

        if score >= 1000 {
            for turn in [dir.next_clockwise(), dir.prev_clockwise()] {
                if scores.get(&(pos, turn)).is_some_and(|&c| c == score - 1000) {
                    queue.push_back((score - 1000, pos, turn));
                }
            }
        }
    }

    visited.len()
}

type ScoreMap = HashMap<(Vec2, Direction), u64>;

fn solve((grid, start, end): &Parsed) -> Option<(u64, ScoreMap)> {
    let mut scores: ScoreMap = ScoreMap::with_capacity(grid.cols() * grid.rows());
    let mut queue: BinaryHeap<Reverse<(u64, Vec2, Direction)>> = BinaryHeap::new();
    queue.push(Reverse((1000, *start, Default::default())));

    while let Some(Reverse((score, pos, dir))) = queue.pop() {
        if scores.contains_key(&(pos, dir)) {
            continue;
        }

        scores.insert((pos, dir), score);
        if pos == *end {
            return Some((score, scores));
        }

        let next_pos = pos.translate(dir);
        if grid[next_pos] != b'#' && scores.get(&(next_pos, dir)).is_none_or(|&c| c > score) {
            queue.push(Reverse((score + 1, next_pos, dir)));
        }

        for turn in [dir.next_clockwise(), dir.prev_clockwise()] {
            if scores.get(&(pos, turn)).is_none_or(|&c| c >= score + 1000) {
                queue.push(Reverse((score + 1000, pos, turn)));
            }
        }
    }

    None
}

aoc::setup! {
    day16, parse_input;
    part1 == 7036,
    part2 == 45
}
