use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

fn explore(grid: &Grid<u8>, pos: Pos) -> Option<HashSet<Pos>> {
    let mut visited = HashSet::new();
    let mut visited2 = HashSet::new();
    let mut cursor = grid.cursor(pos);
    let mut direction = Direction::Up;
    loop {
        if !visited2.insert((cursor.pos(), direction)) {
            return None;
        }
        visited.insert(cursor.pos());
        match cursor.peek(direction) {
            Some(b'#') => {
                direction = direction.next_clockwise();
                continue;
            }
            Some(_) => {
                if !cursor.step(direction) {
                    break;
                }
            }
            None => break,
        }
    }
    Some(visited)
}

fn part1(input: &str) -> impl std::fmt::Display {
    let grid = gridify_ascii(input.lines());
    let start = grid.position(|&v| v == b'^').unwrap();
    explore(&grid, start).unwrap().len()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let grid = gridify_ascii(input.lines());
    let start = grid.position(|&v| v == b'^').unwrap();
    let positions: Vec<_> = explore(&grid, start).unwrap().into_iter().collect();
    let count = positions
        .into_par_iter()
        .map(|pos| {
            let mut grid = grid.clone();
            grid.set(pos, b'#');
            grid
        })
        .filter(|grid| explore(grid, start).is_none())
        .count();

    count
}

aoc::aoc!(day06, "41", "6");
