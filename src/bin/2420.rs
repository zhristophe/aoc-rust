use aoc::prelude::*;

fn read(idx: usize) -> Vec<Vec<char>> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
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
",
    ][idx]
        .trim();

    {
        let tmp = input.lines().map(|s| s.chars().collect()).collect();
        tmp
    }
}

/// 先广播步数（bfs），再easy
fn part1(idx: usize) -> String {
    let map = read(idx);
    let map = Grid::from(map);

    let stt = map.find_point('S').unwrap();
    let end = map.find_point('E').unwrap();

    // 原本的代码虽然通过，但是疑似有bug
    // 新代码从end出发，遍历全图
    let mut steps = Grid::new(map.size(), usize::MAX);
    steps.get_mut(end).map(|v| *v = 0);
    map.bfs_iter(end)
        .skip_tiles(&'#')
        .on_discover(|old, new| {
            let &old_val = steps.get(old).unwrap();
            steps.get_mut(new).map(|v| *v = (*v).min(old_val + 1));
        })
        .run();

    // 然后从stt出发，遍历作弊方法
    let mut res = 0;
    map.bfs_iter(stt)
        .skip_tiles(&'#')
        .on_visit(|cur| {
            let &cur_step = steps.get(cur).unwrap();
            for d in DIRECTIONS {
                let cheat = cur + d * 2;
                if let Some(&cheat_step) = steps.get(cheat) {
                    if cheat_step == usize::MAX {
                        continue;
                    }
                    if cur_step > cheat_step + 2 {
                        let save_step = cur_step - cheat_step - 2;
                        if save_step >= 100 {
                            res += 1;
                        }
                    }
                }
            }
        })
        .run();

    res.to_string()
}

/// 基本一样，只是搜索20步
fn part2(idx: usize) -> String {
    let map = read(idx);
    let map = Grid::from(map);

    let stt = map.find_point('S').unwrap();
    let end = map.find_point('E').unwrap();

    // 类似第一问，从end出发，遍历全图
    let mut steps = Grid::new(map.size(), usize::MAX);
    steps.get_mut(end).map(|v| *v = 0);
    map.bfs_iter(end)
        .skip_tiles(&'#')
        .on_discover(|old, new| {
            let &old_val = steps.get(old).unwrap();
            steps.get_mut(new).map(|v| *v = (*v).min(old_val + 1));
        })
        .run();

    // 然后从stt出发，遍历作弊方法
    let mut res = 0;
    map.bfs_iter(stt)
        .skip_tiles(&'#')
        .on_visit(|cur| {
            let &cur_step = steps.get(cur).unwrap();
            for i in -20..=20isize {
                let max_j = 20 - i.abs();
                for j in -max_j..=max_j {
                    let cheat = cur + Point::new(i, j);
                    if let Some(&cheat_step) = steps.get(cheat) {
                        if cheat_step == usize::MAX {
                            continue;
                        }
                        let cheat_cnt = i.abs() + j.abs();
                        let cheat_cnt = cheat_cnt as usize;
                        if cur_step > cheat_step + cheat_cnt {
                            let save_step = cur_step - cheat_step - cheat_cnt;
                            if save_step >= 100 {
                                res += 1;
                            }
                        }
                    }
                }
            }
        })
        .run();

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
