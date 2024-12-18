use std::{
    collections::VecDeque,
    ops::{Add, Mul, Sub},
};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};

pub mod prelude;

struct Guard<F>(F)
where
    F: FnOnce() + Copy;

impl<F> Drop for Guard<F>
where
    F: FnOnce() + Copy,
{
    fn drop(&mut self) {
        self.0()
    }
}

pub fn wait_key() -> Option<KeyCode> {
    let _guard = Guard(|| terminal::disable_raw_mode().unwrap());

    terminal::enable_raw_mode().unwrap();
    if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
        Some(code)
    } else {
        None
    }
}

pub fn clear_screen() {
    execute!(std::io::stdout(), terminal::Clear(ClearType::All)).unwrap();
    execute!(std::io::stdout(), cursor::MoveTo(0, 0)).unwrap();
}

#[derive(Clone, Debug)]
pub struct Map<T> {
    inner: Vec<Vec<T>>,
}

impl<T> Map<T> {
    pub fn new(size: (usize, usize), default: T) -> Self
    where
        T: Clone,
    {
        Map {
            inner: vec![vec![default; size.1]; size.0],
        }
    }

    pub fn from(inner: Vec<Vec<T>>) -> Self {
        Map { inner }
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

    pub fn set(&mut self, index: Point, value: T) {
        self.get_mut(index).map(|v| *v = value);
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter().flatten()
    }

    pub fn points(&self) -> MapIter<T> {
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

    pub fn bfs_iter(&self, start: Point) -> BfsIter<T> {
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

pub struct BfsIter<'a, T> {
    queue: VecDeque<Point>,
    visited: Map<bool>,
    map: &'a Map<T>,

    visit_filter: Option<Box<dyn Fn(Point) -> bool + 'a>>,
    end_condition: Option<Box<dyn Fn(Point) -> bool + 'a>>,
    update_rule: Option<Box<dyn FnMut(Point, Point) + 'a>>,
}

impl<'a, T> BfsIter<'a, T> {
    fn new(map: &'a Map<T>, start: Point) -> Self {
        BfsIter {
            queue: VecDeque::from([start]),
            visited: Map::new(map.size(), false),
            map,

            visit_filter: None,
            end_condition: None,
            update_rule: None,
        }
    }

    pub fn with_visit_filter<F>(&mut self, filter: F) -> &mut Self
    where
        F: Fn(Point) -> bool + 'a,
    {
        self.visit_filter = Some(Box::new(filter));
        self
    }

    pub fn with_update_rule<F>(&mut self, rule: F) -> &mut Self
    where
        F: FnMut(Point, Point) + 'a,
    {
        self.update_rule = Some(Box::new(rule));
        self
    }

    pub fn run(&mut self) {
        while self.next().is_some() {}
    }

    /// 闭包函数判断点是否是目标
    pub fn run_with_target<F>(&mut self, target: F) -> bool
    where
        F: Fn(Point) -> bool + 'a,
    {
        self.end_condition = Some(Box::new(target));
        while let Some(pt) = self.next() {
            if let Some(cond) = &self.end_condition {
                if cond(pt) {
                    return true;
                }
            }
        }
        false
    }

    pub fn next_val(&mut self) -> Option<&T> {
        if let Some(pt) = self.next() {
            self.map.get(pt)
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for BfsIter<'a, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(pt) = self.queue.pop_front() {
            self.visited.set(pt, true);

            for dir in DIRECTIONS {
                let next_pt = pt + dir;
                if self.visited.get(next_pt) != Some(&false) {
                    continue;
                }
                if let Some(filter) = &self.visit_filter {
                    if !filter(next_pt) {
                        continue;
                    }
                }
                if let Some(update) = &mut self.update_rule {
                    update(pt, next_pt);
                }
                if let Some(cond) = &self.end_condition {
                    if cond(next_pt) {
                        self.queue.clear();
                        return Some(next_pt);
                    }
                }
                if !self.queue.contains(&next_pt) {
                    self.queue.push_back(next_pt);
                }
            }
            return Some(pt);
        }
        None
    }
}

pub struct MapIter<'a, T> {
    row: isize,
    col: isize,
    map: &'a Map<T>,
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

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

impl Mul<isize> for Direction {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        self.as_pt() * rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let p = Point::from((1, 2));
        assert_eq!(p.move_to(Direction::Up), Point::from((0, 2)));
        assert_eq!(p.move_to(Direction::Down), Point::from((2, 2)));
        assert_eq!(p.move_to(Direction::Left), Point::from((1, 1)));
        assert_eq!(p.move_to(Direction::Right), Point::from((1, 3)));
    }
}
