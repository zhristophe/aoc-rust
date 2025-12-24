use utils::prelude::*;

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

fn go_left_right(cur: Point, tgt: Point) -> Vec<char> {
    let dis = tgt.j - cur.j;
    let step = if dis < 0 { '<' } else { '>' };
    let mut res = Vec::new();
    for _ in 0..dis.abs() {
        res.push(step);
    }
    res
}

fn go_up_down(cur: Point, tgt: Point) -> Vec<char> {
    let dis = tgt.i - cur.i;
    let step = if dis < 0 { '^' } else { 'v' };
    let mut res = Vec::new();
    for _ in 0..dis.abs() {
        res.push(step);
    }
    res
}

/// 注意优先级
fn seq_for_seq(tgt: Vec<char>, map: &Grid<char>) -> Vec<char> {
    let mut cur = map.find_point('A').unwrap();
    let mut res = Vec::new();
    for tgt in tgt {
        // <^ > ^<
        // ^> = <^
        // <v > v<
        // v> > >v
        let tgt = map.find_point(tgt).unwrap();
        if tgt.j < cur.j {
            let turning = Point::new(cur.i, tgt.j);
            if map.get(turning) == Some(&' ') {
                // 先上下
                res.extend(go_up_down(cur, tgt));
                res.extend(go_left_right(cur, tgt));
            } else {
                // 先左
                res.extend(go_left_right(cur, tgt));
                res.extend(go_up_down(cur, tgt));
            }
        } else {
            let turning = Point::new(tgt.i, cur.j);
            if map.get(turning) == Some(&' ') {
                // 先右
                res.extend(go_left_right(cur, tgt));
                res.extend(go_up_down(cur, tgt));
            } else {
                // 先上下
                res.extend(go_up_down(cur, tgt));
                res.extend(go_left_right(cur, tgt));
            }
        }
        res.push('A');
        cur = tgt;
    }

    res
}

/// 模拟
/// <^ > ^<
/// ^> = <^
/// <v > v<
/// v> > >v
pub fn part1(idx: usize) -> String {
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
        let ans_len = |mut input: Vec<char>| {
            input = seq_for_seq(input, &map1);
            input = seq_for_seq(input, &map2);
            input = seq_for_seq(input, &map2);
            input.len()
        };
        let num = input[..input.len() - 1]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        res += ans_len(input.clone()) * num;
    }

    res.to_string()
}

/// 模拟太慢
/// 记忆化搜索
pub fn part2(idx: usize) -> String {
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

    fn ss_with_turns(
        tgt: String,
        turns: usize,
        map: &Grid<char>,
        cache: &mut HashMap<(String, usize), usize>,
    ) -> usize {
        // 先划分，去掉末尾A防止多余空格
        let tgt = tgt[..tgt.len() - 1].split('A');
        let mut res = 0;
        for tgt in tgt {
            let tgt = tgt.to_string() + "A";
            if let Some(len) = cache.get(&(tgt.clone(), turns)) {
                res += len;
            } else {
                res += if turns == 1 {
                    let len = seq_for_seq(tgt.chars().collect(), map).len();
                    cache.insert((tgt.clone(), 1), len);
                    len
                } else {
                    let seq = seq_for_seq(tgt.chars().collect(), map);
                    let len = ss_with_turns(seq.iter().collect(), turns - 1, map, cache);
                    cache.insert((tgt.clone(), turns), len);
                    len
                }
            }
        }

        res
    }

    let mut cache = HashMap::new();

    for input in input {
        let mut ans_len = |input: Vec<char>| {
            let tgt = seq_for_seq(input, &map1);
            ss_with_turns(tgt.iter().collect(), 25, &map2, &mut cache)
        };
        let num = input[..input.len() - 1]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        res += ans_len(input.clone()) * num;
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
        // assert_eq!(part1(0), "128962");
        assert_eq!(part1(1), "126384");

        // assert_eq!(part2(0), "159684145150108");
    }
}
