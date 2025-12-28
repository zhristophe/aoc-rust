use std::collections::VecDeque;

use super::{
    Grid,
    point::{DIRECTIONS, Point},
};

pub struct BfsIter<'a, T> {
    queue: VecDeque<Point>,
    visited: Grid<bool>,
    discovered: Grid<bool>,
    grid: &'a Grid<T>,

    visit_filter: Option<Box<dyn Fn(Point) -> bool + 'a>>,
    discovery_handler: Option<Box<dyn FnMut(Point, Point) + 'a>>,
    visit_handler: Option<Box<dyn FnMut(Point) + 'a>>,
}

impl<'a, T> BfsIter<'a, T> {
    pub(crate) fn new(grid: &'a Grid<T>, start: Point) -> Self {
        BfsIter {
            queue: VecDeque::from([start]),
            visited: Grid::new(grid.size(), false),
            discovered: Grid::new(grid.size(), false),
            grid,

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
        self.visit_filter = Some(Box::new(|pt| self.grid.get(pt) != Some(tile)));
        self
    }

    pub fn only_tiles(&mut self, tile: &'a T) -> &mut Self
    where
        T: PartialEq<T> + Clone,
    {
        self.visit_filter = Some(Box::new(|pt| self.grid.get(pt) == Some(tile)));
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
        self.next().and_then(|pt| self.grid.get(pt))
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

                if self.grid.get(next).is_none() {
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

/// 遍历所有点的迭代器，不借用 Grid
pub struct GridPointIter {
    row: isize,
    col: isize,
    size: (usize, usize),
}

impl GridPointIter {
    pub fn new(size: (usize, usize)) -> Self {
        GridPointIter {
            row: 0,
            col: -1,
            size,
        }
    }
}

impl Iterator for GridPointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.col += 1;
        if self.col >= self.size.1 as isize {
            self.col = 0;
            self.row += 1;
        }

        if self.row >= self.size.0 as isize {
            None
        } else {
            Some(Point::new(self.row, self.col))
        }
    }
}
