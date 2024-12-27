use aoc::*;
use std::collections::HashSet;

type Parsed = (Grid<i64>, Vec<Vec2>);

fn parse_input(input: &str) -> Parsed {
    let grid = gridify_ascii(input.lines());
    let start = grid
        .position(|&v| v == b'S')
        .expect("starting location should be marked with an S");
    let end = grid
        .position(|&v| v == b'E')
        .expect("end location should be marked with an E");

    let mut dist: Grid<i64> = Grid::new_with(grid.cols(), grid.rows(), i64::MAX);
    let track = map_track(&grid, &start, &end);
    for &(pos, len) in &track {
        dist[pos] = len;
    }

    (dist, track.into_iter().map(|(pos, _)| pos).collect())
}

fn part1((grid, track): &Parsed) -> impl std::fmt::Display {
    solve(grid, track, 2)
}

fn part2((grid, track): &Parsed) -> impl std::fmt::Display {
    solve(grid, track, 20)
}

fn solve(grid: &Grid<i64>, track: &[Vec2], max_distance: i64) -> usize {
    let mut cheats: HashSet<(Vec2, Vec2)> = Default::default();
    for &from in track {
        for distance in 1..=max_distance {
            for x in 0..=distance {
                let y = distance - x;
                for v in [
                    Vec2::new(x, y),
                    Vec2::new(-x, y),
                    Vec2::new(x, -y),
                    Vec2::new(-x, -y),
                ] {
                    let to = from.translate(v);
                    let start = grid[from];
                    if let Some(&stop) = grid.get(&to) {
                        if stop != i64::MAX
                            && stop > start
                            && (stop - start).abs() - distance >= 100
                        {
                            cheats.insert((from, to));
                        }
                    }
                }
            }
        }
    }

    cheats.len()
}

fn map_track(grid: &Grid<u8>, start: &Vec2, &end: &Vec2) -> Vec<(Vec2, i64)> {
    fn is_track(v: &u8) -> bool {
        [b'.', b'E'].contains(v)
    }

    let mut track: Vec<(Vec2, i64)> = Default::default();
    let mut distance = 1;

    let mut visited = HashSet::new();
    let mut cursor = grid.cursor(*start);
    'outer: while cursor.value() != &b'E' {
        visited.insert(cursor.pos());
        track.push((cursor.pos(), distance));
        for direction in Cardinal::iter() {
            if visited.contains(&cursor.pos().translate(direction)) {
                continue;
            }

            if cursor.peek(direction).is_some_and(is_track) {
                cursor.step(direction);
                distance += 1;
                continue 'outer;
            }
        }
        break;
    }

    track.push((end, distance));
    assert!(visited.contains(start));

    track
}

aoc::setup! {
    day20, parse_input;
    part1 == 36,
    part2 == 81
}
