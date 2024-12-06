pub type Pos = (usize, usize);

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

    pub fn from_ascii(value: u8) -> Option<Self> {
        match value {
            b'^' => Some(Self::Up),
            b'>' => Some(Self::Right),
            b'v' => Some(Self::Down),
            b'<' => Some(Self::Left),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
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
    pub fn get(&self, (col, row): Pos) -> Option<&T> {
        match col < self.cols() && row < self.rows() {
            true => Some(&self.data[self.calc_index(col, row)]),
            false => None,
        }
    }

    #[inline]
    pub fn set(&mut self, (col, row): Pos, value: T) -> T {
        assert!(col < self.cols() && row < self.rows());
        let index = self.calc_index(col, row);
        std::mem::replace(&mut self.data[index], value)
    }

    pub fn left_from(&self, col: usize, row: usize) -> Option<Pos> {
        if col > 0 && col < self.cols() && row < self.rows() {
            Some((col - 1, row))
        } else {
            None
        }
    }

    pub fn right_from(&self, col: usize, row: usize) -> Option<Pos> {
        if (col + 1) < self.cols() && row < self.rows() {
            Some((col + 1, row))
        } else {
            None
        }
    }

    pub fn up_from(&self, col: usize, row: usize) -> Option<Pos> {
        if row > 0 && col < self.cols() && row < self.rows() {
            Some((col, row - 1))
        } else {
            None
        }
    }

    pub fn down_from(&self, col: usize, row: usize) -> Option<Pos> {
        if col < self.cols() && (row + 1) < self.rows() {
            Some((col, row + 1))
        } else {
            None
        }
    }

    #[inline]
    pub fn iter_pos(&self) -> impl Iterator<Item = (usize, usize)> {
        super::iter_pos(self.rows(), self.cols())
    }

    pub fn cursor(self, (col, row): Pos) -> Cursor<T> {
        Cursor {
            grid: self,
            col,
            row,
        }
    }

    #[inline]
    fn calc_index(&self, col: usize, row: usize) -> usize {
        (row * self.cols()) + col
    }
}

impl<T: Eq> Grid<T> {
    pub fn position<P>(&self, predicate: P) -> Option<Pos>
    where
        P: Fn(&T) -> bool,
    {
        for pos in self.iter_pos() {
            if predicate(&self[pos]) {
                return Some(pos);
            }
        }
        None
    }
}

impl<T> std::ops::Index<Pos> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, (col, row): Pos) -> &Self::Output {
        assert!(col < self.cols() && row < self.rows());
        &self.data[self.calc_index(col, row)]
    }
}

impl<T> std::ops::IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, (col, row): Pos) -> &mut Self::Output {
        assert!(col < self.cols() && row < self.rows());
        let index = self.calc_index(col, row);
        &mut self.data[index]
    }
}

/// A two-dimensional cursor associated with a [`Grid`].
pub struct Cursor<T> {
    grid: Grid<T>,
    col: usize,
    row: usize,
}

impl<T> Cursor<T> {
    /// Creates a new `Cursor` for the specified grid, start at `(0, 0)`.
    pub fn new(grid: Grid<T>) -> Self {
        Self {
            grid,
            col: 0,
            row: 0,
        }
    }

    pub fn take(self) -> Grid<T> {
        self.grid
    }

    /// Returns the current position of the `Cursor`.
    #[inline]
    pub fn pos(&self) -> Pos {
        (self.col, self.row)
    }

    /// Returns the value of the grid cell for the current cursor
    /// position.
    #[inline]
    pub fn value(&self) -> &T {
        &self.grid[self.pos()]
    }

    pub fn set(&mut self, value: T) -> T {
        self.grid.set((self.col, self.row), value)
    }

    pub fn step(&mut self, dir: Direction) -> bool {
        use Direction::*;
        match dir {
            Up => self.up(),
            Right => self.right(),
            Down => self.down(),
            Left => self.left(),
        }
    }

    pub fn peek(&mut self, dir: Direction) -> Option<&T> {
        use Direction::*;
        match dir {
            Up => self.peek_up(),
            Right => self.peek_right(),
            Down => self.peek_down(),
            Left => self.peek_left(),
        }
    }

    pub fn right(&mut self) -> bool {
        if (self.col + 1) < self.grid.cols() {
            self.col += 1;
            true
        } else {
            false
        }
    }

    pub fn peek_right(&mut self) -> Option<&T> {
        if (self.col + 1) < self.grid.cols() {
            self.grid.get((self.col + 1, self.row))
        } else {
            None
        }
    }

    pub fn down(&mut self) -> bool {
        if (self.row + 1) < self.grid.rows() {
            self.row += 1;
            true
        } else {
            false
        }
    }

    pub fn peek_down(&mut self) -> Option<&T> {
        if (self.row + 1) < self.grid.rows() {
            self.grid.get((self.col, self.row + 1))
        } else {
            None
        }
    }

    pub fn up(&mut self) -> bool {
        if self.row > 0 {
            self.row -= 1;
            true
        } else {
            false
        }
    }

    pub fn peek_up(&mut self) -> Option<&T> {
        if self.row > 0 {
            self.grid.get((self.col, self.row - 1))
        } else {
            None
        }
    }

    pub fn left(&mut self) -> bool {
        if self.col > 0 {
            self.col -= 1;
            true
        } else {
            false
        }
    }

    pub fn peek_left(&mut self) -> Option<&T> {
        if self.col > 0 {
            self.grid.get((self.col - 1, self.row))
        } else {
            None
        }
    }

    pub fn right_down(&mut self) -> bool {
        if (self.col + 1) < self.grid.cols() && (self.row + 1) < self.grid.rows() {
            self.col += 1;
            self.row += 1;
            true
        } else {
            false
        }
    }

    pub fn right_up(&mut self) -> bool {
        if (self.col + 1) < self.grid.cols() && self.row > 0 {
            self.col += 1;
            self.row -= 1;
            true
        } else {
            false
        }
    }

    pub fn left_up(&mut self) -> bool {
        if self.col > 0 && self.row > 0 {
            self.col -= 1;
            self.row -= 1;
            true
        } else {
            false
        }
    }

    pub fn left_down(&mut self) -> bool {
        if self.col > 0 && (self.row + 1) < self.grid.rows() {
            self.col -= 1;
            self.row += 1;
            true
        } else {
            false
        }
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
        assert_eq!(grid.get((0, 0)), Some(&b'A'));
        assert_eq!(grid.get((4, 0)), Some(&b'E'));
        assert_eq!(grid.get((0, 1)), Some(&b'F'));
        assert_eq!(grid.get((4, 2)), Some(&b'O'));
        assert_eq!(grid.get((5, 0)), None);
        assert_eq!(grid.get((5, 5)), None);
    }
}
