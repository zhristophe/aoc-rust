use std::ops::{Add, Mul, Sub};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};

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
    pub fn new(width: usize, height: usize, default: T) -> Self
    where
        T: Clone,
    {
        Map {
            inner: vec![vec![default; height]; width],
        }
    }

    pub fn row_len(&self) -> usize {
        self.inner.len()
    }

    pub fn col_len(&self) -> usize {
        self.inner[0].len()
    }

    pub fn from(inner: Vec<Vec<T>>) -> Self {
        Map { inner }
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

    pub fn display_by<F>(&self, f: F)
    where
        F: Fn(&T) -> String,
    {
        // print!("\x1B[2J\x1B[1;1H");
        for i in 0..self.inner.len() {
            for j in 0..self.inner[0].len() {
                print!("{}", f(&self.inner[i][j]));
            }
            println!();
        }
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

    pub fn at<T>(self, map: &Vec<Vec<T>>) -> Option<&T> {
        self.get(map)
    }

    pub fn get<T>(self, map: &Vec<Vec<T>>) -> Option<&T> {
        map.get(self.i as usize)?.get(self.j as usize)
    }

    pub fn get_mut<T>(self, map: &mut Vec<Vec<T>>) -> Option<&mut T> {
        map.get_mut(self.i as usize)?.get_mut(self.j as usize)
    }

    pub fn set<T>(self, map: &mut Vec<Vec<T>>, value: T) {
        if 0 <= self.i
            && self.i < map.len() as isize
            && 0 <= self.j
            && self.j < map[0].len() as isize
        {
            map[self.i as usize][self.j as usize] = value;
        }
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
        }
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
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

    pub fn at<T>(self, v: &Vec<T>) -> Option<&T> {
        self.get(v)
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
