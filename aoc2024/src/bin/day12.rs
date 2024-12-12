use aoc::*;
use std::collections::BTreeSet;

type Parsed = Grid<u8>;

fn parse_input(input: &str) -> Parsed {
    gridify_ascii(input.lines())
}

fn part1(map: &Parsed) -> impl std::fmt::Display {
    let mut coords: BTreeSet<Vec2> = BTreeSet::from_iter(map.iter_pos());
    let mut price = 0;
    while let Some(coord) = coords.pop_first() {
        let cursor = map.cursor(coord);
        let mut visited = BTreeSet::new();
        let perimeter = explore_region(cursor, &mut visited);
        for c in &visited {
            coords.remove(c);
        }
        price += perimeter * visited.len() as u64;
    }
    price
}

fn part2(map: &Parsed) -> impl std::fmt::Display {
    let mut coords: BTreeSet<Vec2> = BTreeSet::from_iter(map.iter_pos());
    let mut price = 0;
    while let Some(coord) = coords.pop_first() {
        let cursor = map.cursor(coord);
        let mut visited = BTreeSet::new();
        let _ = explore_region(cursor, &mut visited);
        let sides = count_sides(&visited);
        for c in &visited {
            coords.remove(c);
        }
        price += sides * visited.len() as u64;
    }
    price
}

fn explore_region(cursor: Cursor<u8>, visited: &mut BTreeSet<Vec2>) -> u64 {
    if !visited.insert(cursor.pos()) {
        return 0;
    }
    let target = cursor.value();
    let mut perimeter = 4;
    for dir in Direction::iter() {
        if cursor.peek(dir).is_some_and(|val| val == target) {
            perimeter -= 1;
            let mut cursor = cursor.clone();
            cursor.step(dir);
            perimeter += explore_region(cursor, visited);
        }
    }
    perimeter
}

fn count_sides(region: &BTreeSet<Vec2>) -> u64 {
    let rows: BTreeSet<i64> = region.iter().map(|&Vec2 { y, .. }| y).collect();
    let cols: BTreeSet<i64> = region.iter().map(|&Vec2 { x, .. }| x).collect();

    // scan top -> bottom
    let mut sides = 0;
    for row in rows {
        let mut blocks = 0;

        // Select positions in the region which can count as an upper edge.
        let positions: Vec<_> = region
            .iter()
            .filter(|pos| pos.y == row && !region.contains(&pos.with_y(pos.y - 1)))
            .collect();

        // Iff there are any counting positions, then there must be a minimum of one
        // side.
        if !positions.is_empty() {
            blocks += 1;
        }

        // Find discontinuities in the top edge of this row.
        blocks += positions
            .windows(2)
            .map(|window| if window[0].x + 1 != window[1].x { 1 } else { 0 })
            .sum::<u64>();

        // Select positions in the region which can count as an upper edge.
        let positions: Vec<_> = region
            .iter()
            .filter(|pos| pos.y == row && !region.contains(&pos.with_y(pos.y + 1)))
            .collect();

        if !positions.is_empty() {
            blocks += 1;
        }

        // Find discontinuities in the bottom edge of this row.
        blocks += positions
            .windows(2)
            .map(|window| if window[0].x + 1 != window[1].x { 1 } else { 0 })
            .sum::<u64>();

        sides += blocks;
    }

    // scan left -> right
    for col in cols {
        let mut blocks = 0;
        let positions: Vec<_> = region
            .iter()
            .filter(|pos| pos.x == col && !region.contains(&pos.with_x(pos.x - 1)))
            .collect();

        if !positions.is_empty() {
            blocks += 1;
        }

        blocks += positions
            .windows(2)
            .map(|window| if window[0].y + 1 != window[1].y { 1 } else { 0 })
            .sum::<u64>();

        let positions: Vec<_> = region
            .iter()
            .filter(|pos| pos.x == col && !region.contains(&pos.with_x(pos.x + 1)))
            .collect();

        if !positions.is_empty() {
            blocks += 1;
        }

        blocks += positions
            .windows(2)
            .map(|window| if window[0].y + 1 != window[1].y { 1 } else { 0 })
            .sum::<u64>();

        sides += blocks;
    }

    sides
}

aoc::setup! {
    day12, parse_input;
    part1 == 1930,
    part2 == 1206
}
