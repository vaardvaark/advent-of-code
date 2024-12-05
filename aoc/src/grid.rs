pub type Pos = (usize, usize);

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
    pub fn get(&self, col: usize, row: usize) -> Option<&T> {
        match col < self.cols() && row < self.rows() {
            true => Some(&self.data[self.calc_index(col, row)]),
            false => None,
        }
    }

    #[inline]
    pub fn set(&mut self, col: usize, row: usize, value: T) -> T {
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

    #[inline]
    fn calc_index(&self, col: usize, row: usize) -> usize {
        (row * self.cols()) + col
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

    pub fn right(&mut self) -> bool {
        if (self.col + 1) < self.grid.cols() {
            self.col += 1;
            true
        } else {
            false
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

    pub fn up(&mut self) -> bool {
        if self.row > 0 {
            self.row -= 1;
            true
        } else {
            false
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
        assert_eq!(grid.get(0, 0), Some(&b'A'));
        assert_eq!(grid.get(4, 0), Some(&b'E'));
        assert_eq!(grid.get(0, 1), Some(&b'F'));
        assert_eq!(grid.get(4, 2), Some(&b'O'));
        assert_eq!(grid.get(5, 0), None);
        assert_eq!(grid.get(5, 5), None);
    }
}
