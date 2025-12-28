use std::ops;

use crate::num_ext::IntegerExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub i: isize,
    pub j: isize,
}

impl Point {
    pub fn new(i: isize, j: isize) -> Self {
        Point { i, j }
    }

    /// 从 (0,0) 遍历到 (self.i, self.j)，不包含边界
    pub fn iter_to(self) -> super::iter::GridPointIter {
        super::iter::GridPointIter::new((self.i as usize, self.j as usize))
    }

    pub fn move_to(self, direction: Direction) -> Point {
        match direction {
            Direction::Up => Point {
                i: self.i - 1,
                ..self
            },
            Direction::Down => Point {
                i: self.i + 1,
                ..self
            },
            Direction::Left => Point {
                j: self.j - 1,
                ..self
            },
            Direction::Right => Point {
                j: self.j + 1,
                ..self
            },
        }
    }

    #[inline]
    pub fn move_left(self) -> Point {
        self.move_to(Direction::Left)
    }

    #[inline]
    pub fn move_right(self) -> Point {
        self.move_to(Direction::Right)
    }

    #[inline]
    pub fn move_up(self) -> Point {
        self.move_to(Direction::Up)
    }

    #[inline]
    pub fn move_down(self) -> Point {
        self.move_to(Direction::Down)
    }

    pub fn get<T>(self, grid: &Vec<Vec<T>>) -> Option<&T> {
        grid.get(self.i as usize)?.get(self.j as usize)
    }

    pub fn get_mut<T>(self, grid: &mut Vec<Vec<T>>) -> Option<&mut T> {
        grid.get_mut(self.i as usize)?.get_mut(self.j as usize)
    }

    /// 超出界限时什么也不做
    pub fn set<T>(self, grid: &mut Vec<Vec<T>>, value: T) {
        self.get_mut(grid).map(|v| *v = value);
    }

    /// 8方向邻接点，不检查边界
    pub fn adjacent(self) -> AdjacentIter<()> {
        AdjacentIter::new(self, ())
    }

    /// 8方向邻接点，检查边界
    pub fn adjacent_in(self, size: (usize, usize)) -> AdjacentIter<(usize, usize)> {
        AdjacentIter::new(self, size)
    }
}

/// 8方向邻接点迭代器，S 为 () 时不检查边界，为 (usize, usize) 时检查
pub struct AdjacentIter<S> {
    center: Point,
    idx: i8,
    bounds: S,
}

const ADJACENT_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl<S> AdjacentIter<S> {
    fn new(center: Point, bounds: S) -> Self {
        AdjacentIter {
            center,
            idx: 0,
            bounds,
        }
    }
}

impl Iterator for AdjacentIter<()> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 8 {
            return None;
        }
        let (di, dj) = ADJACENT_OFFSETS[self.idx as usize];
        self.idx += 1;
        Some(Point::new(self.center.i + di, self.center.j + dj))
    }
}

impl Iterator for AdjacentIter<(usize, usize)> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < 8 {
            let (di, dj) = ADJACENT_OFFSETS[self.idx as usize];
            self.idx += 1;
            let ni = self.center.i + di;
            let nj = self.center.j + dj;
            if ni >= 0 && nj >= 0 && (ni as usize) < self.bounds.0 && (nj as usize) < self.bounds.1
            {
                return Some(Point::new(ni, nj));
            }
        }
        None
    }
}

impl<T: IntegerExt> From<(T, T)> for Point {
    fn from((x, y): (T, T)) -> Self {
        Point {
            i: x.as_isize(),
            j: y.as_isize(),
        }
    }
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}

impl ops::Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self.move_to(rhs)
    }
}

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
        }
    }
}

impl ops::Sub<Direction> for Point {
    type Output = Self;

    fn sub(self, rhs: Direction) -> Self::Output {
        self.move_to(rhs.turn_around())
    }
}

impl ops::Mul for Point {
    type Output = isize;

    fn mul(self, rhs: Self) -> Self::Output {
        self.i * rhs.j + self.j * rhs.i
    }
}

impl<T: IntegerExt> ops::Mul<T> for Point {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point {
            i: self.i * rhs.as_isize(),
            j: self.j * rhs.as_isize(),
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        DIRECTIONS.to_vec()
    }

    pub fn as_pt(self) -> Point {
        match self {
            Direction::Up => Point::new(-1, 0),
            Direction::Down => Point::new(1, 0),
            Direction::Left => Point::new(0, -1),
            Direction::Right => Point::new(0, 1),
        }
    }

    pub fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn turn_around(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn get<T>(self, v: &Vec<T>) -> Option<&T> {
        match self {
            Direction::Up => v.get(0),
            Direction::Down => v.get(1),
            Direction::Left => v.get(2),
            Direction::Right => v.get(3),
        }
    }

    pub fn set<T>(self, v: &mut Vec<T>, val: T) {
        match self {
            Direction::Up => v[0] = val,
            Direction::Down => v[1] = val,
            Direction::Left => v[2] = val,
            Direction::Right => v[3] = val,
        };
    }
}

impl TryFrom<Point> for Direction {
    type Error = ();

    fn try_from(pt: Point) -> Result<Self, Self::Error> {
        match (pt.i, pt.j) {
            (i, 0) if i < 0 => Ok(Direction::Up),
            (i, 0) if i > 0 => Ok(Direction::Down),
            (0, j) if j < 0 => Ok(Direction::Left),
            (0, j) if j > 0 => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl ops::Mul<isize> for Direction {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        self.as_pt() * rhs
    }
}
