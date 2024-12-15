use aoc::*;
use std::collections::{HashMap, HashSet};

type Parsed = (Vec2, HashSet<Vec2>, HashSet<Vec2>, Vec<Direction>);

fn parse_input(input: &str) -> Parsed {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut position = Default::default();
    let mut walls = HashSet::default();
    let mut boxes = HashSet::default();
    for (row, line) in map.lines().enumerate() {
        for (column, &value) in line.as_bytes().iter().enumerate() {
            let pos = Vec2::new(column as i64, row as i64);
            match value {
                b'#' => _ = walls.insert(pos),
                b'O' => _ = boxes.insert(pos),
                b'@' => position = pos,
                _ => {}
            }
        }
    }

    let moves = moves
        .as_bytes()
        .iter()
        .filter_map(|&x| Direction::from_ascii(x))
        .collect();

    (position, walls, boxes, moves)
}

fn part1((mut position, walls, boxes, moves): &Parsed) -> impl std::fmt::Display {
    let mut boxes = boxes.clone();
    for &direction in moves {
        position = try_move(position, direction, walls, &mut boxes).unwrap_or(position);
    }
    boxes
        .into_iter()
        .map(|Vec2 { x, y }| x + y * 100)
        .sum::<i64>()
}

fn try_move(
    position: Vec2,
    direction: Direction,
    walls: &HashSet<Vec2>,
    boxes: &mut HashSet<Vec2>,
) -> Option<Vec2> {
    let next_position = position.translate(direction);
    if walls.contains(&next_position) {
        None
    } else if boxes.contains(&next_position) {
        let available = try_move(next_position, direction, walls, boxes)?;
        boxes.insert(available);
        boxes.remove(&next_position);
        Some(next_position)
    } else {
        Some(next_position)
    }
}

fn part2((start, old_walls, old_boxes, moves): &Parsed) -> impl std::fmt::Display {
    let mut walls = HashSet::new();
    for &Vec2 { x, y } in old_walls {
        walls.insert(Vec2 { x: x * 2, y });
        walls.insert(Vec2 { x: x * 2 + 1, y });
    }

    let mut boxes = HashMap::new();
    for (id, &Vec2 { x, y }) in old_boxes.iter().enumerate() {
        boxes.insert(Vec2 { x: x * 2, y }, id);
        boxes.insert(Vec2 { x: x * 2 + 1, y }, id);
    }

    let mut position = Vec2::new(start.x * 2, start.y);
    for &direction in moves {
        position = match direction {
            Direction::Left | Direction::Right => {
                try_move_lr(position, direction, &walls, &mut boxes)
            }
            Direction::Up | Direction::Down => try_move_ud(position, direction, &walls, &mut boxes),
        }
        .unwrap_or(position);
    }

    let mut filtered = vec![];
    let mut mapping: HashMap<usize, Vec<Vec2>> = HashMap::new();
    for (pos, id) in boxes {
        let positions = mapping.entry(id).or_default();
        positions.push(pos);
        if positions.len() == 2 {
            positions.sort();
            positions.pop();
            filtered.push(positions.pop().unwrap());
        }
    }

    filtered
        .into_iter()
        .map(|Vec2 { x, y }| x + y * 100)
        .sum::<i64>()
}

fn try_move_lr(
    position: Vec2,
    direction: Direction,
    walls: &HashSet<Vec2>,
    boxes: &mut HashMap<Vec2, usize>,
) -> Option<Vec2> {
    let next_position = position.translate(direction);
    if walls.contains(&next_position) {
        None
    } else if let Some(&id) = boxes.get(&next_position) {
        let available = try_move_lr(next_position, direction, walls, boxes)?;
        boxes.insert(available, id);
        boxes.remove(&next_position);
        Some(next_position)
    } else {
        Some(next_position)
    }
}

fn try_move_ud(
    position: Vec2,
    direction: Direction,
    walls: &HashSet<Vec2>,
    boxes: &mut HashMap<Vec2, usize>,
) -> Option<Vec2> {
    fn step(
        id: usize,
        pos1: Vec2,
        pos2: Vec2,
        direction: Direction,
        walls: &HashSet<Vec2>,
        boxes: &mut HashMap<Vec2, usize>,
        test: bool,
    ) -> Option<Vec2> {
        if boxes.get(&pos2).is_some_and(|&i| i == id) {
            let (Some(new1), Some(new2)) = (
                inner(pos1, direction, walls, boxes, test),
                inner(pos2, direction, walls, boxes, test),
            ) else {
                return None;
            };

            if !test {
                boxes.remove(&pos1);
                boxes.remove(&pos2);
                boxes.insert(new1, id);
                boxes.insert(new2, id);
            }

            Some(pos1)
        } else {
            None
        }
    }

    fn inner(
        position: Vec2,
        direction: Direction,
        walls: &HashSet<Vec2>,
        boxes: &mut HashMap<Vec2, usize>,
        test: bool,
    ) -> Option<Vec2> {
        let next_position = position.translate(direction);
        if walls.contains(&next_position) {
            None
        } else if let Some(&id) = boxes.get(&next_position) {
            for d in [Direction::Right, Direction::Left] {
                if let Some(pos) = step(
                    id,
                    next_position,
                    next_position.translate(d),
                    direction,
                    walls,
                    boxes,
                    test,
                ) {
                    return Some(pos);
                }
            }
            None
        } else {
            Some(next_position)
        }
    }

    inner(position, direction, walls, boxes, true)
        .and_then(|_| inner(position, direction, walls, boxes, false))
}

aoc::setup! {
    day15, parse_input;
    part1 == 10092,
    part2 == 9021
}
