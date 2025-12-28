use core::{fmt, ops};

pub mod iter;
pub mod point;
pub mod transform;

pub use iter::*;
pub use point::*;
pub use transform::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub(crate) inner: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(size: (usize, usize), default: T) -> Self
    where
        T: Clone,
    {
        Grid {
            inner: vec![vec![default; size.1]; size.0],
        }
    }

    pub fn new_with(size: (usize, usize), f: impl Fn(Point) -> T) -> Self {
        Grid {
            inner: (0..size.0)
                .map(|i| (0..size.1).map(|j| f(Point::from((i, j)))).collect())
                .collect(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.inner.len(), self.inner[0].len())
    }

    pub fn n_rows(&self) -> usize {
        self.inner.len()
    }

    pub fn n_cols(&self) -> usize {
        self.inner[0].len()
    }

    pub fn get(&self, index: Point) -> Option<&T> {
        self.inner.get(index.i as usize)?.get(index.j as usize)
    }

    pub fn get_mut(&mut self, index: Point) -> Option<&mut T> {
        self.inner
            .get_mut(index.i as usize)?
            .get_mut(index.j as usize)
    }

    pub fn set(&mut self, pt: Point, value: T) {
        if let Some(v) = self.get_mut(pt) {
            *v = value;
        }
    }

    pub fn set_with(&mut self, pt: Point, f: impl FnOnce(&mut T)) {
        if let Some(v) = self.get_mut(pt) {
            f(v);
        }
    }

    pub fn contains(&self, pt: Point) -> bool {
        pt.i >= 0
            && pt.i < self.inner.len() as isize
            && pt.j >= 0
            && pt.j < self.inner[0].len() as isize
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter().flatten()
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.inner.iter()
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<T>> {
        self.inner.iter_mut()
    }

    pub fn points(&self) -> GridPointIter {
        GridPointIter::new(self.size())
    }

    pub fn find_point(&self, c: T) -> Option<Point>
    where
        T: PartialEq<T>,
    {
        let data = &self.inner;
        for i in 0..data.len() {
            for j in 0..data[0].len() {
                if data[i][j] == c {
                    return Some(Point::from((i, j)));
                }
            }
        }

        None
    }

    pub fn find_all_points(&self, c: T) -> Vec<Point>
    where
        T: PartialEq<T>,
    {
        let mut res = Vec::new();
        let data = &self.inner;
        for i in 0..data.len() {
            for j in 0..data[0].len() {
                if data[i][j] == c {
                    res.push(Point::from((i, j)));
                }
            }
        }

        res
    }

    pub fn bfs_iter(&self, start: Point) -> BfsIter<'_, T> {
        BfsIter::new(self, start)
    }

    pub fn display_with<F, D>(&self, f: F)
    where
        F: Fn(&T) -> D,
        D: fmt::Display,
    {
        for i in 0..self.inner.len() {
            for j in 0..self.inner[0].len() {
                print!("{}", f(&self.inner[i][j]));
            }
            println!();
        }
    }

    pub fn display_aligned<F>(&self, f: F)
    where
        F: Fn(&T) -> String,
    {
        let (rows, cols) = self.size();
        let mut s_grid = vec![vec![String::new(); cols]; rows];
        let mut max_width = 0;

        for i in 0..rows {
            for j in 0..cols {
                let s = f(&self.inner[i][j]);
                if s.len() > max_width {
                    max_width = s.len();
                }
                s_grid[i][j] = s;
            }
        }

        for i in 0..rows {
            for j in 0..cols {
                print!("{:width$} ", s_grid[i][j], width = max_width);
            }
            println!();
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(inner: Vec<Vec<T>>) -> Self {
        Grid { inner }
    }
}

impl From<&str> for Grid<u8> {
    fn from(s: &str) -> Self {
        Grid::from(
            s.lines()
                .map(|line| line.as_bytes().to_vec())
                .collect::<Vec<_>>(),
        )
    }
}

impl fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.inner.len() {
            for j in 0..self.inner[0].len() {
                write!(f, "{}", self.inner[i][j] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid<u8> {
    pub fn display(&self) {
        println!("{self}");
    }
}

impl fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.inner.len() {
            for j in 0..self.inner[0].len() {
                write!(f, "{}", self.inner[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid<char> {
    pub fn display(&self) {
        println!("{self}");
    }
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        Grid {
            inner: iter.into_iter().collect(),
        }
    }
}

impl<T, I: Into<Point>> ops::Index<I> for Grid<T> {
    type Output = T;
    fn index(&self, pt: I) -> &Self::Output {
        let pt = pt.into();
        &self.inner[pt.i as usize][pt.j as usize]
    }
}
impl<T, I: Into<Point>> ops::IndexMut<I> for Grid<T> {
    fn index_mut(&mut self, pt: I) -> &mut Self::Output {
        let pt = pt.into();
        &mut self.inner[pt.i as usize][pt.j as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_basic() {
        let grid: Grid<i32> = Grid::new((3, 4), 0);
        assert_eq!(grid.size(), (3, 4));
        assert_eq!(grid.n_rows(), 3);
        assert_eq!(grid.n_cols(), 4);
    }

    #[test]
    fn test_grid_index() {
        let mut grid: Grid<i32> = Grid::new((2, 2), 0);
        grid[Point::from((0, 1))] = 5;
        assert_eq!(grid[Point::from((0, 1))], 5);
        assert_eq!(grid[Point::from((0, 0))], 0);
    }

    #[test]
    fn test_grid_from_str() {
        let grid: Grid<u8> = "ab\ncd".into();
        assert_eq!(grid.size(), (2, 2));
        assert_eq!(grid[Point::from((0, 0))], b'a');
        assert_eq!(grid[Point::from((1, 1))], b'd');
    }

    #[test]
    fn test_grid_transform() {
        let grid: Grid<u8> = "ab\ncd".into();
        // a b
        // c d

        // Rotate 90
        // c a
        // d b
        let g90 = grid.rotate_cw();
        assert_eq!(g90[Point::new(0, 0)], b'c');
        assert_eq!(g90[Point::new(0, 1)], b'a');
        assert_eq!(g90[Point::new(1, 0)], b'd');
        assert_eq!(g90[Point::new(1, 1)], b'b');

        // Flip H
        // b a
        // d c
        let gfh = grid.flip_h();
        assert_eq!(gfh[Point::new(0, 0)], b'b');
        assert_eq!(gfh[Point::new(0, 1)], b'a');
        assert_eq!(gfh[Point::new(1, 0)], b'd');
        assert_eq!(gfh[Point::new(1, 1)], b'c');
    }
}
