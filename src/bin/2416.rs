use std::{collections::VecDeque, fs, path::Path, vec};

use aoc::{Direction, Point};

#[allow(dead_code)]
fn exec1(input: &Vec<Vec<char>>) -> String {
    fn find_point(input: &Vec<Vec<char>>, c: char) -> Point {
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j] == c {
                    return Point::from((i, j));
                }
            }
        }
        unreachable!();
    }

    let stt = find_point(input, 'S');
    let end = find_point(input, 'E');

    // bfs
    let mut score = vec![vec![(usize::MAX, Direction::Up); input[0].len()]; input.len()];
    let mut queue = VecDeque::new();
    stt.set(&mut score, (0, Direction::Right));
    queue.push_back(stt);
    while let Some(p) = queue.pop_front() {
        let (s_p, d_p) = p.get(&score).unwrap().clone();
        for (d, s) in [(d_p, 0), (d_p.turn_left(), 1000), (d_p.turn_right(), 1000)] {
            let next = p.move_to(d);
            match next.get(input) {
                Some('#') | None => continue,
                _ => (),
            }
            let s_next = s_p + s + 1;
            if s_next >= end.get(&score).unwrap().0 {
                continue;
            }
            if s_next > next.get(&score).unwrap().0 {
                continue;
            }
            next.set(&mut score, (s_next, d));
            if next != end && !queue.contains(&next) {
                queue.push_back(next);
            }
        }
    }

    end.get(&score).unwrap().0.to_string()
}

#[allow(dead_code)]
fn exec2(input: &Vec<Vec<char>>) -> String {
    fn find_point(input: &Vec<Vec<char>>, c: char) -> Point {
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j] == c {
                    return Point::from((i, j));
                }
            }
        }
        unreachable!();
    }

    let stt = find_point(input, 'S');
    let end = find_point(input, 'E');

    // bfs
    let mut score = vec![vec![vec![usize::MAX; 4]; input[0].len()]; input.len()];
    let mut queue = VecDeque::new();
    stt.set(&mut score, {
        let mut tmp = vec![usize::MAX; 4];
        let dir = Direction::Right;
        dir.set(&mut tmp, 0);
        tmp
    });
    queue.push_back(stt);
    while let Some(p) = queue.pop_front() {
        let score_p = p.get(&score).unwrap().clone();
        for d in Direction::all() {
            let score_p_d = d.get(&score_p).unwrap().clone();
            if score_p_d == usize::MAX {
                continue;
            }
            for (d, s) in [(d, 0), (d.turn_left(), 1000), (d.turn_right(), 1000)] {
                let next = p.move_to(d);
                match next.get(input) {
                    Some('#') | None => continue,
                    _ => (),
                }
                let s_next = score_p_d + s + 1;
                let min_end = end.get(&score).unwrap().iter().min().unwrap();
                if s_next >= *min_end {
                    continue;
                }
                let mut score_next = next.get(&score).unwrap().clone();
                if s_next >= *d.get(&score_next).unwrap() {
                    continue;
                }
                d.set(&mut score_next, s_next);
                next.set(&mut score, score_next);
                if next != end && !queue.contains(&next) {
                    queue.push_back(next);
                }
            }
        }
    }

    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut queue = VecDeque::new();
    let mins = *end.get(&score).unwrap().iter().min().unwrap();
    for d in Direction::all() {
        if *d.get(end.get(&score).unwrap()).unwrap() == mins {
            queue.push_back((end, d));
        }
    }
    let mut res = 0;
    while let Some((p, d)) = queue.pop_front() {
        let s_p_d = *d.get(p.get(&score).unwrap()).unwrap();
        if p.get(&visited) == Some(&false) {
            res += 1;
        }
        *p.get_mut(&mut visited).unwrap() = true;
        let prev = p.move_to(d.turn_around());
        for (d, s) in [(d, 0), (d.turn_right(), 1000), (d.turn_left(), 1000)] {
            let s_prev_d = *d.get(prev.get(&score).unwrap()).unwrap();
            if s_p_d > s && s_prev_d == s_p_d - s - 1 {
                queue.push_back((prev, d));
            }
        }
    }

    res.to_string()
}

#[allow(unused_variables)]
fn main() {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);

    let input = r"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"
    .trim();
    let input = fs::read_to_string(file).unwrap();

    let input = input.lines().map(|s| s.chars().collect()).collect();

    println!("{:?}", exec2(&input));
}
