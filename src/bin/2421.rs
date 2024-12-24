use aoc::prelude::*;

fn read(idx: usize) -> Vec<Vec<char>> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
029A
980A
179A
456A
379A
",
    ][idx]
        .trim();

    {
        let tmp = input.lines().map(|s| s.chars().collect()).collect();
        tmp
    }
}

fn dir2char(dir: Direction) -> char {
    match dir {
        Direction::Up => '^',
        Direction::Down => 'v',
        Direction::Left => '<',
        Direction::Right => '>',
    }
}

/// 返回从起点到终点的路径的所有可能性
fn seq_for_seq(tgt: Vec<char>, map: &Grid<char>) -> Vec<Vec<Vec<char>>> {
    let mut cur = map.find_point('A').unwrap();
    let mut res = Vec::new();
    for tgt in tgt {
        let tgt = map.find_point(tgt).unwrap();
        let mut cache = Grid::new(map.size(), (usize::MAX, HashSet::new()));
        map.bfs_iter(cur)
            .skip_tiles(&' ')
            .on_discover(|old, new| {
                let dis = new - old;
                let Point { i, j } = dis;
                let dir = match (i, j) {
                    (-1, 0) => Direction::Up,
                    (1, 0) => Direction::Down,
                    (0, -1) => Direction::Left,
                    (0, 1) => Direction::Right,
                    _ => unreachable!(),
                };
                let new_step = cache.get(old).unwrap().0 + 1;
                cache.get_mut(new).map(|(old_step, dirs)| {
                    if new_step < *old_step {
                        *old_step = new_step;
                        dirs.clear();
                        dirs.insert(dir);
                    } else if new_step == *old_step {
                        dirs.insert(dir);
                    }
                });
            })
            .run_with_target(tgt);

        let mut res = Vec::new();

        fn get_paths_to(
            from: Point,
            to: Point,
            cache: Grid<(usize, HashSet<Direction>)>,
        ) -> Vec<Vec<char>> {
            let mut res = Vec::new();
            if from == to {
                return vec![vec![]];
            }

            for &dir in &cache.get(from).unwrap().1 {
                let paths = get_paths_to(from + dir.turn_around(), to, cache.clone());
                for mut path in paths {
                    path.push(dir2char(dir));
                    res.push(path);
                }
            }

            res
        }

        res.push(get_paths_to(tgt, cur, cache));
    }

    res
}

/// 模拟
fn part1(idx: usize) -> String {
    let input = read(idx);
    let mut res = 0;
    let map1 = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ];
    let map2 = vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']];
    let (map1, map2) = (Grid::from(map1), Grid::from(map2));
    for input in input {
        // let get_best_paths = ||

        // let ans_len = |mut input: Vec<char>| {
        //     input = seq_for_seq(input, &map1);
        //     input = seq_for_seq(input, &map2);
        //     input = seq_for_seq(input, &map2);
        //     input.len()
        // };
        // let num = input[..input.len() - 1]
        //     .iter()
        //     .collect::<String>()
        //     .parse::<usize>()
        //     .unwrap();
        // res += dbg!(ans_len(input.clone())) * dbg!(num);
        // break;
    }
    res.to_string()
}

/// 基本一样，只是搜索20步
fn part2(idx: usize) -> String {
    0.to_string()
}

fn main() {
    println!("{:?}", part1(1));
    // println!("{:?}", part2(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(part1(0), "1363");

        // assert_eq!(part2(0), "1007186");
    }
}
