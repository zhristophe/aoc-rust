use std::{collections::HashMap, fs, path::Path};

use aoc::{Grid, Point};

#[allow(dead_code)]
fn exec1(input: Vec<Vec<char>>) -> String {
    let map = Grid::from(input);
    let mut ant_map: HashMap<char, Vec<Point>> = HashMap::new();

    for pt in map.points() {
        let ch = *map.get(pt).unwrap();
        if ch == '.' {
            continue;
        }
        ant_map
            .entry(ch)
            .and_modify(|v| v.push(pt))
            .or_insert(vec![pt]);
    }

    let mut is_node = Grid::new(map.size(), false);
    for (_, ants) in &ant_map {
        for i in 0..ants.len() {
            for j in i + 1..ants.len() {
                for n in [ants[i] * 2 - ants[j], ants[j] * 2 - ants[i]] {
                    is_node.set(n, true);
                }
            }
        }
    }

    let mut res = 0;
    for pt in is_node.points() {
        // println!("{:?}", pt);
        if Some(&true) == is_node.get(pt) {
            res += 1;
        }
    }

    res.to_string()
}

#[allow(dead_code)]
fn exec2(input: Vec<Vec<char>>) -> String {
    let map = Grid::from(input);
    let mut ant_map: HashMap<char, Vec<Point>> = HashMap::new();

    for pt in map.points() {
        let ch = *map.get(pt).unwrap();
        if ch == '.' {
            continue;
        }
        ant_map
            .entry(ch)
            .and_modify(|v| v.push(pt))
            .or_insert(vec![pt]);
    }

    let mut is_node = Grid::new(map.size(), false);
    for (_, ants) in &ant_map {
        for i in 0..ants.len() {
            for j in i + 1..ants.len() {
                for (o, d) in [(ants[i], ants[i] - ants[j]), (ants[j], ants[j] - ants[i])] {
                    for k in 0.. {
                        let p = o + d * k;
                        if let Some(is) = is_node.get_mut(p) {
                            *is = true;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    is_node.display_by_char(|&c| if c { '#' } else { '.' });

    let mut res = 0;
    for pt in is_node.points() {
        if Some(&true) == is_node.get(pt) {
            res += 1;
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
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
    .trim();
    //     let input = r"
    // T.........
    // ...T......
    // .T........
    // ..........
    // ..........
    // ..........
    // ..........
    // ..........
    // ..........
    // ..........
    // "
    //     .trim();
    let input = fs::read_to_string(file).unwrap();

    let input = input.lines().map(|s| s.chars().collect()).collect();

    println!("{:?}", exec2(input));
}
