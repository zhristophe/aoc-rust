pub fn display_map<T, F>(map: &Vec<Vec<T>>, f: F)
where
    F: Fn(&T) -> char,
{
    // print!("\x1B[2J\x1B[1;1H");
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{}", f(&map[i][j]));
        }
        println!();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub i: isize,
    pub j: isize,
}

impl Point {
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
        map.get(self.i as usize)?.get(self.j as usize)
    }

    pub fn assign_at<T>(self, map: &mut Vec<Vec<T>>) -> Option<&mut T> {
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

    pub fn get<T>(self, map: &Vec<Vec<T>>) -> Option<&T> {
        map.get(self.i as usize)?.get(self.j as usize)
    }

    pub fn get_mut<T>(self, map: &mut Vec<Vec<T>>) -> Option<&mut T> {
        map.get_mut(self.i as usize)?.get_mut(self.j as usize)
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

// impl PartialEq for Point {
//     fn eq(&self, other: &Self) -> bool {
//         self.i == other.i && self.j == other.j
//     }
// }

#[derive(Copy, Clone, Debug)]
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

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn turn_around(&self) -> Direction {
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

    pub fn at<T>(self, v: &Vec<T>) -> Option<&T> {
        match self {
            Direction::Up => v.get(0),
            Direction::Down => v.get(1),
            Direction::Left => v.get(2),
            Direction::Right => v.get(3),
        }
    }

    pub fn at_mut<T>(self, v: &mut Vec<T>) -> Option<&mut T> {
        match self {
            Direction::Up => v.get_mut(0),
            Direction::Down => v.get_mut(1),
            Direction::Left => v.get_mut(2),
            Direction::Right => v.get_mut(3),
        }
    }

    pub fn at_opt<T>(self, v: Option<&Vec<T>>) -> Option<&T> {
        self.at(v?)
    }
}
