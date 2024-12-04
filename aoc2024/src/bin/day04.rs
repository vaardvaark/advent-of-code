aoc2024::aoc!();

fn part1(input: &str) -> impl std::fmt::Display {
    let grid = Grid::new(input);
    let mut count = 0;
    for (col, row) in grid.iter_pos() {
        let mut patterns = vec![
            [(col, row), (col + 1, row), (col + 2, row), (col + 3, row)],
            [(col, row), (col, row + 1), (col, row + 2), (col, row + 3)],
            [
                (col, row),
                (col + 1, row + 1),
                (col + 2, row + 2),
                (col + 3, row + 3),
            ],
        ];
        if row >= 3 {
            patterns.push([
                (col, row),
                (col + 1, row - 1),
                (col + 2, row - 2),
                (col + 3, row - 3),
            ]);
        }
        for pattern in patterns {
            let Some(found) = grid.map(&pattern) else {
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
    let grid = Grid::new(input);
    let mut count = 0;
    for (col, row) in grid.iter_pos() {
        let pattern = [
            (col, row),         // top-left
            (col + 2, row),     // top-right
            (col, row + 2),     // bottom-left
            (col + 2, row + 2), // bottom-right
            (col + 1, row + 1), // middle
        ];
        let Some(found) = grid.map(&pattern) else {
            continue;
        };
        if ["MMSSA", "MSMSA", "SSMMA", "SMSMA"].contains(&found.as_str()) {
            count += 1;
        }
    }

    count
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    text: Vec<u8>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut cols = 0;
        let mut text = vec![];
        let rows = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let line = line.trim();
                cols = line.len();
                text.extend_from_slice(line.as_bytes());
            })
            .count();
        Self { rows, cols, text }
    }

    fn get(&self, col: usize, row: usize) -> Option<u8> {
        if row < self.rows && col < self.cols {
            return Some(self.text[(row * self.cols) + col]);
        }
        None
    }

    fn map(&self, positions: &[(usize, usize)]) -> Option<String> {
        let bytes: Vec<_> = positions
            .into_iter()
            .flat_map(|&(col, row)| self.get(col, row))
            .collect();

        if bytes.len() == positions.len() {
            Some(String::from_utf8(bytes).unwrap())
        } else {
            None
        }
    }

    fn iter_pos(&self) -> impl Iterator<Item = (usize, usize)> {
        aoc::iter_pos(self.rows, self.cols)
    }
}

#[cfg(test)]
mod day04 {

    #[test]
    fn part1() {
        const EXAMPLE: &str = r#"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "#;
        assert_eq!(super::part1(EXAMPLE).to_string(), "18");
    }

    #[test]
    fn part2() {
        const EXAMPLE: &str = r#"
            .M.S......
            ..A..MSMS.
            .M.S.MAA..
            ..A.ASMSM.
            .M.S.M....
            ..........
            S.S.S.S.S.
            .A.A.A.A..
            M.M.M.M.M.
            ..........
        "#;
        assert_eq!(super::part2(EXAMPLE).to_string(), "9");
    }
}
