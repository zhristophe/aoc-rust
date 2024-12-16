use std::{fs, path::Path};

use aoc::Point;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Input {
    p: Point,
    v: Point,
}

#[allow(dead_code)]
fn exec1(input: &Vec<Input>) -> String {
    let max_y = 101;
    let max_x = 103;
    let steps = 100;

    // dbg!(input.len());

    fn mod_isize(x: isize, y: usize) -> usize {
        let res = x % y as isize;
        if res < 0 {
            (res + y as isize) as usize
        } else {
            res as usize
        }
    }

    let middle_x = max_x / 2;
    let middle_y = max_y / 2;
    let mut cnt = vec![0; 4];
    for input in input {
        let mut p = input.p;
        let v = input.v;
        p.i += v.i * steps;
        p.j += v.j * steps;
        let res = (mod_isize(p.i, max_y), mod_isize(p.j, max_x));
        let a = if res.0 < middle_y {
            0
        } else if res.0 > middle_y {
            1
        } else {
            continue;
        };
        let b = if res.1 < middle_x {
            0
        } else if res.1 > middle_x {
            2
        } else {
            continue;
        };
        cnt[a + b] += 1;
        // map[mod_isize(p.i, max_y)][mod_isize(p.j, max_x)] += 1;
    }

    // dbg!(&cnt);
    let mut res = 1;
    for i in &cnt {
        res *= i;
    }

    res.to_string()
}

#[allow(dead_code)]
fn exec2(input: &Vec<Input>) -> String {
    // What is a Christmas tree in hell???
    let max_y = 101;
    let max_x = 103;

    // 8 8 12 35 30

    0.to_string()
}

#[allow(unused_variables)]
fn main() {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);

    let input = r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
    .trim();
    let input = fs::read_to_string(file).unwrap();

    let input = {
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let mut res = Vec::new();

        for input in input.lines() {
            let input = if let Some(captures) = re.captures(input) {
                Some(Input {
                    p: Point {
                        i: captures[1].parse().unwrap(),
                        j: captures[2].parse().unwrap(),
                    },
                    v: Point {
                        i: captures[3].parse().unwrap(),
                        j: captures[4].parse().unwrap(),
                    },
                })
            } else {
                None
            };
            res.push(input.unwrap());
        }

        res
    };

    println!("{:?}", exec2(&input));
}
