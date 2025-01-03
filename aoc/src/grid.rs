use crate::Vec2;
use core::fmt;

/// Cardinal directions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cardinal {
    #[default]
    North,
    East,
    South,
    West,
}

impl Cardinal {
    pub fn next_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn prev_clockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub fn from_ascii(value: u8) -> Option<Self> {
        match value {
            b'N' | b'n' | b'^' => Some(Self::North),
            b'E' | b'e' | b'>' => Some(Self::East),
            b'S' | b's' | b'v' => Some(Self::South),
            b'W' | b'w' | b'<' => Some(Self::West),
            _ => None,
        }
    }

    /// Iterates over the cardinal directions once.
    pub fn iter() -> impl Iterator<Item = Self> {
        use Cardinal::*;
        [North, East, South, West].into_iter()
    }
}

#[derive(Clone, Debug)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn new_with(cols: usize, rows: usize, value: T) -> Self {
        Self {
            rows,
            cols,
            data: vec![value; cols * rows],
        }
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[inline]
    pub fn get(&self, pos: &Vec2) -> Option<&T> {
        match self.in_bounds(pos) {
            true => Some(&self.data[self.calc_index(pos)]),
            false => None,
        }
    }

    #[inline]
    pub fn set(&mut self, pos: impl Into<Vec2>, value: T) -> T {
        let pos = pos.into();
        assert!(self.in_bounds(&pos));
        let index = self.calc_index(&pos);
        std::mem::replace(&mut self.data[index], value)
    }

    /// Returns a column-first iterator of all the valid coordinates
    /// in the grid.
    #[inline]
    pub fn iter_pos(&self) -> impl Iterator<Item = Vec2> {
        let mut row = 0;
        let mut col = 0;
        let max_rows = self.rows() as i64;
        let max_cols = self.cols() as i64;
        std::iter::from_fn(move || {
            if col >= max_cols {
                col = 0;
                row += 1;
            }
            if row >= max_rows {
                return None;
            }
            let c = col;
            col += 1;
            Some(Vec2::new(c, row))
        })
    }

    /// Returns an iterator over the cells in a single row of the
    /// grid in column order.
    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = &T> {
        let mut column = 0;
        let max_column = self.cols();
        std::iter::from_fn(move || {
            if column < max_column {
                let value = Some(&self[(column, row)]);
                column += 1;
                value
            } else {
                None
            }
        })
    }

    /// Returns an iterator over the cells in a single column of the
    /// grid in row order.
    pub fn iter_column(&self, column: usize) -> impl Iterator<Item = &T> {
        let mut row = 0;
        let max_row = self.rows();
        std::iter::from_fn(move || {
            if row < max_row {
                let value = Some(&self[(column, row)]);
                row += 1;
                value
            } else {
                None
            }
        })
    }

    pub fn cursor(&self, pos: Vec2) -> Cursor<T> {
        Cursor { grid: self, pos }
    }

    #[inline]
    fn calc_index(&self, &Vec2 { x, y }: &Vec2) -> usize {
        ((y * self.cols() as i64) + x) as usize
    }

    pub fn in_bounds(&self, &Vec2 { x, y }: &Vec2) -> bool {
        (x >= 0 && x < self.cols() as i64) && (y >= 0 && y < self.rows() as i64)
    }
}

impl<T: Eq> Grid<T> {
    pub fn position<P>(&self, predicate: P) -> Option<Vec2>
    where
        P: Fn(&T) -> bool,
    {
        self.iter_pos().find(|&pos| predicate(&self[pos]))
    }

    pub fn position_all<P>(&self, predicate: P) -> Vec<Vec2>
    where
        P: Fn(&T) -> bool,
    {
        self.iter_pos()
            .filter(|&pos| predicate(&self[pos]))
            .collect()
    }
}

impl<T> std::ops::Index<Vec2> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, pos: Vec2) -> &Self::Output {
        assert!(
            self.in_bounds(&pos),
            "{pos} not in grid bounded by (0, 0) -> ({}, {})",
            self.cols(),
            self.rows()
        );
        &self.data[self.calc_index(&pos)]
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let pos = Vec2 {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        };
        &self[pos]
    }
}

impl<T> std::ops::IndexMut<Vec2> for Grid<T> {
    fn index_mut(&mut self, pos: Vec2) -> &mut Self::Output {
        assert!(
            self.in_bounds(&pos),
            "{pos} not in grid bounded by (0, 0) -> ({}, {})",
            self.cols(),
            self.rows()
        );
        let index = self.calc_index(&pos);
        &mut self.data[index]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let pos = Vec2 {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        };
        &mut self[pos]
    }
}

impl fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let pos = Vec2 {
                    x: col as i64,
                    y: row as i64,
                };
                write!(f, "{}", self[pos] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut group = [false, false];
        writeln!(f, " {0:▁>1$}", "", self.cols())?;
        for y in (0..self.rows() as i64).step_by(2) {
            if y as usize == self.rows() - 1 {
                write!(f, " ")?;
            } else {
                write!(f, "▕")?;
            }
            for x in 0..self.cols() as i64 {
                for offset_y in 0..2 {
                    let pos = Vec2 { x, y: offset_y + y };
                    group[offset_y as usize] = *self.get(&pos).unwrap_or(&false)
                }
                write!(
                    f,
                    "{}",
                    match group {
                        [false, false] if y as usize == self.rows() - 1 => "─",
                        [false, false] => " ",
                        [true, false] => "▀",
                        [false, true] => "▄",
                        [true, true] => "█",
                    }
                )?;
            }
            if y as usize == self.rows() - 1 {
                writeln!(f)?;
            } else {
                writeln!(f, "▏")?;
            }
        }
        if self.rows() % 2 == 0 {
            write!(f, " {0:▔>1$}", "", self.cols())?;
        }
        Ok(())
    }
}

/// A two-dimensional cursor associated with a [`Grid`].
#[derive(Clone, Debug)]
pub struct Cursor<'g, T> {
    grid: &'g Grid<T>,
    pos: Vec2,
}

impl<'g, T> Cursor<'g, T> {
    /// Creates a new `Cursor` for the specified grid, starting at `(0, 0)`.
    pub fn new(grid: &'g Grid<T>) -> Self {
        Self {
            grid,
            pos: Vec2::new(0, 0),
        }
    }

    /// Returns the current position of the `Cursor`.
    #[inline]
    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    /// Returns the value of the grid cell for the current cursor
    /// position.
    #[inline]
    pub fn value(&self) -> &T {
        &self.grid[self.pos()]
    }

    /// Moves the cursor in the specified direction.
    ///
    /// Returns false if the cursor could not be moved (eg. if moving
    /// the cursor places it outside the bounds of the grid).
    pub fn step(&mut self, direction: Cardinal) -> bool {
        use Cardinal::*;
        let v = match direction {
            North => Vec2::up(),
            East => Vec2::right(),
            South => Vec2::down(),
            West => Vec2::left(),
        };

        let pos = self.pos + v;
        if self.grid.in_bounds(&pos) {
            self.pos = pos;
            true
        } else {
            false
        }
    }

    /// Peeks at the cell in the specified direction.
    pub fn peek(&self, direction: Cardinal) -> Option<&T> {
        use Cardinal::*;
        let v = match direction {
            North => Vec2::up(),
            East => Vec2::right(),
            South => Vec2::down(),
            West => Vec2::left(),
        };

        let pos = self.pos + v;
        self.grid.get(&pos)
    }

    #[inline]
    pub fn right(&mut self) -> bool {
        self.step(Cardinal::East)
    }

    #[inline]
    pub fn peek_right(&mut self) -> Option<&T> {
        self.peek(Cardinal::East)
    }

    #[inline]
    pub fn down(&mut self) -> bool {
        self.step(Cardinal::South)
    }

    #[inline]
    pub fn peek_down(&mut self) -> Option<&T> {
        self.peek(Cardinal::South)
    }

    #[inline]
    pub fn up(&mut self) -> bool {
        self.step(Cardinal::North)
    }

    #[inline]
    pub fn peek_up(&mut self) -> Option<&T> {
        self.peek(Cardinal::North)
    }

    #[inline]
    pub fn left(&mut self) -> bool {
        self.step(Cardinal::West)
    }

    #[inline]
    pub fn peek_left(&mut self) -> Option<&T> {
        self.peek(Cardinal::West)
    }
}

pub fn gridify_ascii<'a>(lines: impl Iterator<Item = &'a str>) -> Grid<u8> {
    let mut cols = 0;
    let mut data = vec![];

    let rows = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let line = line.trim();
            if line.len() != cols && cols != 0 {
                eprintln!(
                    "WARNING: length of line '{line}' is {}, expected {cols}",
                    line.len()
                );
            } else {
                cols = line.len();
            }
            data.extend_from_slice(line.as_bytes());
        })
        .count();

    Grid { rows, cols, data }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_from_ascii() {
        let grid = gridify_ascii("ABCDE\nFGHIJ\nKLMNO".lines());
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 5);
        assert_eq!(grid.get(&Vec2::new(0, 0)), Some(&b'A'));
        assert_eq!(grid.get(&Vec2::new(4, 0)), Some(&b'E'));
        assert_eq!(grid.get(&Vec2::new(0, 1)), Some(&b'F'));
        assert_eq!(grid.get(&Vec2::new(4, 2)), Some(&b'O'));
        assert_eq!(grid.get(&Vec2::new(5, 0)), None);
        assert_eq!(grid.get(&Vec2::new(5, 5)), None);
    }
}
