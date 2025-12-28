use std::collections::HashMap;

use utils::{read_input, Grid, Point};

fn read(idx: usize) -> Vec<Vec<char>> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
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
        .trim(),
    ][idx];

    input.lines().map(|s| s.chars().collect()).collect()
}

pub fn part1(idx: usize) -> String {
    let input = read(idx);
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

pub fn part2(idx: usize) -> String {
    let input = read(idx);
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

    // is_node.display_with(|&c| if c { '#' } else { '.' });

    let mut res = 0;
    for pt in is_node.points() {
        if Some(&true) == is_node.get(pt) {
            res += 1;
        }
    }

    res.to_string()
}

pub fn run() {
    println!("{:?}", part1(0));
    println!("{:?}", part2(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(1), "14");
        // // assert_eq!(part1(0), "243");

        assert_eq!(part2(1), "34");
        // // assert_eq!(part2(0), "912");
    }
}
