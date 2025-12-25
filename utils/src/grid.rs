use std::{
    collections::VecDeque,
    ops::{Add, Mul, Sub},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Grid<T> {
    inner: Vec<Vec<T>>,
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

    pub fn points(&self) -> MapIter<'_, T> {
        MapIter {
            row: 0,
            col: -1,
            map: self,
        }
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

    pub fn display_by_char<F>(&self, f: F)
    where
        F: Fn(&T) -> char,
    {
        for i in 0..self.inner.len() {
            for j in 0..self.inner[0].len() {
                print!("{}", f(&self.inner[i][j]));
            }
            println!();
        }
    }

    pub fn display_by_string<F>(&self, f: F)
    where
        F: Fn(&T) -> String,
    {
        for i in 0..self.inner.len() {
            for j in 0..self.inner[0].len() {
                print!("{}", f(&self.inner[i][j]));
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

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        Grid {
            inner: iter.into_iter().collect(),
        }
    }
}

pub struct BfsIter<'a, T> {
    queue: VecDeque<Point>,
    visited: Grid<bool>,
    discovered: Grid<bool>,
    map: &'a Grid<T>,

    visit_filter: Option<Box<dyn Fn(Point) -> bool + 'a>>,
    discovery_handler: Option<Box<dyn FnMut(Point, Point) + 'a>>,
    visit_handler: Option<Box<dyn FnMut(Point) + 'a>>,
}

impl<'a, T> BfsIter<'a, T> {
    fn new(map: &'a Grid<T>, start: Point) -> Self {
        BfsIter {
            queue: VecDeque::from([start]),
            visited: Grid::new(map.size(), false),
            discovered: Grid::new(map.size(), false),
            map,

            visit_filter: None,
            discovery_handler: None,
            visit_handler: None,
        }
    }

    /// 设置访问过滤，可访问点返回true
    pub fn with_visit_filter<F>(&mut self, filter: F) -> &mut Self
    where
        F: Fn(Point) -> bool + 'a,
    {
        self.visit_filter = Some(Box::new(filter));
        self
    }

    pub fn skip_tiles(&mut self, tile: &'a T) -> &mut Self
    where
        T: PartialEq<T> + Clone,
    {
        self.visit_filter = Some(Box::new(|pt| self.map.get(pt) != Some(tile)));
        self
    }

    pub fn only_tiles(&mut self, tile: &'a T) -> &mut Self
    where
        T: PartialEq<T> + Clone,
    {
        self.visit_filter = Some(Box::new(|pt| self.map.get(pt) == Some(tile)));
        self
    }

    /// 发现节点时执行函数
    pub fn on_discover<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(Point, Point) + 'a,
    {
        self.discovery_handler = Some(Box::new(f));
        self
    }

    /// 访问节点时执行函数
    pub fn on_visit<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(Point) + 'a,
    {
        self.visit_handler = Some(Box::new(f));
        self
    }

    /// 无目标搜索，直到没有点可以访问
    pub fn run(&mut self) {
        while self.next().is_some() {}
    }

    /// 有目标搜索，直到目标点被找到，或者没有点可以访问
    /// 返回是否找到目标点
    pub fn run_with_target(&mut self, target: Point) -> bool {
        while let Some(pt) = self.next() {
            if pt == target {
                return true;
            }
        }
        false
    }

    pub fn is_discovered(&self, pt: Point) -> bool {
        self.discovered.get(pt).copied().unwrap_or(false)
    }

    pub fn is_visited(&self, pt: Point) -> bool {
        self.visited.get(pt).copied().unwrap_or(false)
    }

    pub fn next_val(&mut self) -> Option<&T> {
        self.next().and_then(|pt| self.map.get(pt))
    }
}

impl<'a, T> Iterator for BfsIter<'a, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.queue.pop_front() {
            self.visited.set(cur, true);

            self.visit_handler.as_mut().map(|f| f(cur));

            for dir in DIRECTIONS {
                let next = cur + dir;

                if self.map.get(next).is_none() {
                    continue;
                }

                if self.is_discovered(next) || self.is_visited(next) {
                    continue;
                }

                if self.visit_filter.as_ref().map(|f| f(next)) == Some(false) {
                    continue;
                }

                self.discovered.set(next, true);

                self.discovery_handler.as_mut().map(|f| f(cur, next));

                self.queue.push_back(next);
            }

            return Some(cur);
        }
        None
    }
}

pub struct MapIter<'a, T> {
    row: isize,
    col: isize,
    map: &'a Grid<T>,
}

impl<'a, T> Iterator for MapIter<'a, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.col += 1;
        if self.col >= self.map.inner[0].len() as isize {
            self.col = 0;
            self.row += 1;
        }

        if self.row == self.map.inner.len() as isize {
            None
        } else {
            Some(Point::new(self.row, self.col))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub i: isize,
    pub j: isize,
}

impl Point {
    pub fn new(i: isize, j: isize) -> Self {
        Point { i, j }
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

    pub fn get<T>(self, map: &Vec<Vec<T>>) -> Option<&T> {
        map.get(self.i as usize)?.get(self.j as usize)
    }

    pub fn get_mut<T>(self, map: &mut Vec<Vec<T>>) -> Option<&mut T> {
        map.get_mut(self.i as usize)?.get_mut(self.j as usize)
    }

    /// 超出界限时什么也不做
    pub fn set<T>(self, map: &mut Vec<Vec<T>>, value: T) {
        self.get_mut(map).map(|v| *v = value);
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

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point {
            i: x as isize,
            j: y as isize,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self.move_to(rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
        }
    }
}

impl Sub<Direction> for Point {
    type Output = Self;

    fn sub(self, rhs: Direction) -> Self::Output {
        self.move_to(rhs.turn_around())
    }
}

impl Mul for Point {
    type Output = isize;

    fn mul(self, rhs: Self) -> Self::Output {
        self.i * rhs.j + self.j * rhs.i
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Point {
            i: self.i * rhs,
            j: self.j * rhs,
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

impl Mul<isize> for Direction {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        self.as_pt() * rhs
    }
}
