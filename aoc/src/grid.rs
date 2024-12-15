use crate::Vec2;
use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn next_clockwise(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn prev_clockwise(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    pub fn from_ascii(value: u8) -> Option<Self> {
        match value {
            b'^' => Some(Self::Up),
            b'>' => Some(Self::Right),
            b'v' => Some(Self::Down),
            b'<' => Some(Self::Left),
            _ => None,
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        use Direction::*;
        [Up, Right, Down, Left].into_iter()
    }
}

#[derive(Clone, Debug)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new_empty(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }
}

impl<T> Grid<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        Self { rows, cols, data }
    }

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
    pub fn set(&mut self, pos: &Vec2, value: T) -> T {
        assert!(self.in_bounds(pos));
        let index = self.calc_index(pos);
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
        assert!(self.in_bounds(&pos));
        &self.data[self.calc_index(&pos)]
    }
}

impl<T> std::ops::IndexMut<Vec2> for Grid<T> {
    fn index_mut(&mut self, pos: Vec2) -> &mut Self::Output {
        assert!(self.in_bounds(&pos));
        let index = self.calc_index(&pos);
        &mut self.data[index]
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
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let pos = Vec2 {
                    x: col as i64,
                    y: row as i64,
                };
                match self[pos] {
                    true => write!(f, "█")?,
                    false => write!(f, " ")?,
                }
            }
            writeln!(f)?;
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
    pub fn step(&mut self, direction: Direction) -> bool {
        use Direction::*;
        let v = match direction {
            Up => Vec2::up(),
            Right => Vec2::right(),
            Down => Vec2::down(),
            Left => Vec2::left(),
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
    pub fn peek(&self, direction: Direction) -> Option<&T> {
        use Direction::*;
        let v = match direction {
            Up => Vec2::up(),
            Right => Vec2::right(),
            Down => Vec2::down(),
            Left => Vec2::left(),
        };

        let pos = self.pos + v;
        self.grid.get(&pos)
    }

    #[inline]
    pub fn right(&mut self) -> bool {
        self.step(Direction::Right)
    }

    #[inline]
    pub fn peek_right(&mut self) -> Option<&T> {
        self.peek(Direction::Right)
    }

    #[inline]
    pub fn down(&mut self) -> bool {
        self.step(Direction::Down)
    }

    #[inline]
    pub fn peek_down(&mut self) -> Option<&T> {
        self.peek(Direction::Down)
    }

    #[inline]
    pub fn up(&mut self) -> bool {
        self.step(Direction::Up)
    }

    #[inline]
    pub fn peek_up(&mut self) -> Option<&T> {
        self.peek(Direction::Up)
    }

    #[inline]
    pub fn left(&mut self) -> bool {
        self.step(Direction::Left)
    }

    #[inline]
    pub fn peek_left(&mut self) -> Option<&T> {
        self.peek(Direction::Left)
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

    Grid::new(rows, cols, data)
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
