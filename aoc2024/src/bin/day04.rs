fn part1(input: &str) -> impl std::fmt::Display {
    let grid = gridify_ascii(input.lines());
    let mut count = 0;
    for Vec2 { x, y } in grid.iter_pos() {
        let mut search = vec![
            [
                Vec2::new(x, y),
                Vec2::new(x + 1, y),
                Vec2::new(x + 2, y),
                Vec2::new(x + 3, y),
            ],
            [
                Vec2::new(x, y),
                Vec2::new(x, y + 1),
                Vec2::new(x, y + 2),
                Vec2::new(x, y + 3),
            ],
            [
                Vec2::new(x, y),
                Vec2::new(x + 1, y + 1),
                Vec2::new(x + 2, y + 2),
                Vec2::new(x + 3, y + 3),
            ],
        ];
        if y >= 3 {
            search.push([
                Vec2::new(x, y),
                Vec2::new(x + 1, y - 1),
                Vec2::new(x + 2, y - 2),
                Vec2::new(x + 3, y - 3),
            ]);
        }

        for search in search {
            let Some(found) = map(&grid, &search) else {
                continue;
            };
            if ["XMAS", "SAMX"].contains(&found.as_str()) {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &str) -> impl std::fmt::Display {
    let grid = gridify_ascii(input.lines());
    let mut count = 0;
    for Vec2 { x, y } in grid.iter_pos() {
        let pattern = [
            Vec2::new(x, y),         // top-left
            Vec2::new(x + 2, y),     // top-right
            Vec2::new(x, y + 2),     // bottom-left
            Vec2::new(x + 2, y + 2), // bottom-right
            Vec2::new(x + 1, y + 1), // middle
        ];
        let Some(found) = map(&grid, &pattern) else {
            continue;
        };
        if ["MMSSA", "MSMSA", "SSMMA", "SMSMA"].contains(&found.as_str()) {
            count += 1;
        }
    }

    count
}
fn map(grid: &Grid<u8>, positions: &[Vec2]) -> Option<String> {
    let bytes: Vec<_> = positions
        .into_iter()
        .filter_map(|pos| grid.get(pos).copied())
        .collect();

    if bytes.len() == positions.len() {
        Some(String::from_utf8(bytes).unwrap())
    } else {
        None
    }
}

aoc::aoc!(day04, "18", "9");
