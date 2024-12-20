use std::{fs, path::Path, usize};

use aoc::prelude::*;

fn read(idx: usize) -> Vec<Vec<char>> {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);
    let content = fs::read_to_string(file).unwrap();

    let inputs = [
        content.as_str(),
        r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"
        .trim(),
    ];

    let input = if idx >= inputs.len() {
        inputs.last().unwrap()
    } else {
        &inputs[idx]
    };

    {
        let tmp = input.lines().map(|s| s.chars().collect()).collect();
        tmp
    }
}

/// 只有一条路径，easy
fn part1(idx: usize) -> String {
    let map = read(idx);
    let map = Map::from(map);

    let start = map.find_point('S').unwrap();
    let end = map.find_point('E').unwrap();

    let mut steps = Map::new(map.size(), usize::MAX);
    steps.get_mut(start).map(|v| *v = 0);

    map.bfs_iter(start)
        .skip_tiles(&'#')
        .on_discover(|old, new| {
            let &old_val = steps.get(old).unwrap();
            steps.get_mut(new).map(|v| *v = (*v).min(old_val + 1));
        })
        .run_with_target(end);

    let mut res = 0;
    let mut cur = start;
    'main: loop {
        if cur == end {
            break 'main;
        }

        let step = steps.get(cur).unwrap();
        let mut next = cur;
        for d in DIRECTIONS {
            next = cur + d;
            if steps.get(next) == Some(&(step + 1)) {
                break;
            }
        }
        for d in DIRECTIONS {
            let cheat_next = cur + d * 2;
            if let Some(&cheat_step) = steps.get(cheat_next) {
                if cheat_step == usize::MAX {
                    continue;
                }
                if cheat_step > *step + 2 {
                    let rev_step = cheat_step - step - 2;
                    if rev_step >= 100 {
                        res += 1;
                    }
                }
            }
        }
        cur = next;
    }

    res.to_string()
}

/// 基本一样，只是搜索20步
fn part2(idx: usize) -> String {
    let map = read(idx);
    let map = Map::from(map);

    let start = map.find_point('S').unwrap();
    let end = map.find_point('E').unwrap();

    let mut steps = Map::new(map.size(), usize::MAX);
    steps.get_mut(start).map(|v| *v = 0);

    map.bfs_iter(start)
        .skip_tiles(&'#')
        .on_discover(|old, new| {
            let &old_val = steps.get(old).unwrap();
            steps.get_mut(new).map(|v| *v = (*v).min(old_val + 1));
        })
        .run_with_target(end);

    let mut res = 0;
    // 搜索20步以内小于100的
    let mut cur = start;
    'main: loop {
        if cur == end {
            break 'main;
        }

        let step = steps.get(cur).unwrap();
        let mut next = cur;
        for d in DIRECTIONS {
            next = cur + d;
            if steps.get(next) == Some(&(step + 1)) {
                break;
            }
        }
        for i in -20..=20 as isize {
            let max_j = 20 - i.abs();
            for j in -max_j..=max_j {
                let cheat_next = cur + Point::new(i, j);
                if let Some(&cheat_step) = steps.get(cheat_next) {
                    if cheat_step == usize::MAX {
                        continue;
                    }
                    let cheat_cnt = i.abs() + j.abs();
                    let cheat_cnt = cheat_cnt as usize;
                    if cheat_step > *step + cheat_cnt {
                        let rev_step = cheat_step - step - cheat_cnt;
                        if rev_step >= 100 {
                            res += 1;
                        }
                    }
                }
            }
        }

        cur = next;
    }

    res.to_string()
}

fn main() {
    println!("{:?}", part1(0));
    println!("{:?}", part2(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(0), "1363");

        assert_eq!(part2(0), "1007186");
    }
}
