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

fn seq_for_seq(tgt: Vec<char>, map: &Grid<char>) -> Vec<char> {
    let mut cur = map.find_point('A').unwrap();
    let mut res = Vec::new();
    for tgt in tgt {
        let tgt = map.find_point(tgt).unwrap();
        let mut seq = VecDeque::new();
        let mut steps = Grid::new(map.size(), Direction::Up);
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
                steps.set(new, dir);
            })
            .run_with_target(tgt);

        let mut tmp = tgt;
        while tmp != cur {
            let &step = steps.get(tmp).unwrap();
            seq.push_front(match step {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            });
            tmp = tmp + step.turn_around();
        }

        res.extend(seq);

        cur = tgt;
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
        let tgt = input;
        let seq1 = seq_for_seq(tgt, &map2);
        let seq2 = seq_for_seq(seq1, &map1);
        let seq3 = seq_for_seq(seq2, &map1);

        dbg!(seq3);
    }
    0.to_string()
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
