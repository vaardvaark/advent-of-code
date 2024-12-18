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
        let mut visited = BTreeSet::new();
        let perimeter = explore_region(map.cursor(coord), &mut visited);
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
        let mut visited = BTreeSet::new();
        let _ = explore_region(map.cursor(coord), &mut visited);
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
    for dir in Cardinal::iter() {
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

    let scan = |acc: &dyn Fn(&Vec2) -> i64, f: &dyn Fn(&Vec2) -> bool| {
        let mut prev: Option<&Vec2> = None;
        let mut sides = 0;
        for pos in region.iter().filter(|v| f(v)) {
            match prev {
                None => sides += 1,
                Some(prev) => {
                    if acc(prev) + 1 != acc(pos) {
                        sides += 1
                    }
                }
            }
            prev.replace(pos);
        }
        sides
    };

    // scan top -> bottom
    let mut sides = 0;
    for row in rows {
        sides += scan(&|pos| pos.x, &|pos| {
            pos.y == row && !region.contains(&pos.with_y(pos.y - 1))
        });
        sides += scan(&|pos| pos.x, &|pos| {
            pos.y == row && !region.contains(&pos.with_y(pos.y + 1))
        });
    }

    // scan left -> right
    for col in cols {
        sides += scan(&|pos| pos.y, &|pos| {
            pos.x == col && !region.contains(&pos.with_x(pos.x - 1))
        });
        sides += scan(&|pos| pos.y, &|pos| {
            pos.x == col && !region.contains(&pos.with_x(pos.x + 1))
        });
    }

    sides
}

aoc::setup! {
    day12, parse_input;
    part1 == 1930,
    part2 == 1206
}
