use std::{fs, path::Path, usize};

use aoc::prelude::*;

fn read(idx: usize) -> (Vec<Vec<usize>>, usize, usize) {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);
    let content = fs::read_to_string(file).unwrap();

    let inputs = [
        (content.as_str(), 70),
        (
            r"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"
            .trim(),
            6,
        ),
    ];

    let input = if idx > inputs.len() {
        inputs.last().unwrap()
    } else {
        &inputs[idx]
    };

    {
        let tmp = (
            input
                .0
                .lines()
                .map(|s| s.split(',').map(|s| s.parse().unwrap()).collect())
                .collect(),
            input.1 + 1,
            input.1 + 1,
        );

        tmp
    }
}

/// 简简单单BFS
fn part1(idx: usize) -> String {
    let (bytes, x, y) = read(idx);
    let stt = Point::from((0, 0));
    let end = Point::from((x - 1, y - 1));

    let mut map = Map::new((x, y), 0);
    let n = if x == 71 { 1024 } else { 12 };
    for i in 0..n {
        map.get_mut(Point::from((bytes[i][1], bytes[i][0])))
            .map(|v| *v += 1);
    }

    let mut steps = Map::new((x, y), usize::MAX);
    steps.get_mut(stt).map(|v| *v = 0);
    map.bfs_iter(stt)
        .with_visit_filter(|p| map.get(p) == Some(&0))
        .with_update_rule(|old, new| {
            let &old_val = steps.get(old).unwrap();
            steps.get_mut(new).map(|v| *v = (*v).min(old_val + 1));
        })
        .run_with_target(|p| p == end);
    let &res = steps.get(end).unwrap();

    res.to_string()
}

/// BFS + 二分查找
fn part2(idx: usize) -> String {
    let (bytes, x, y) = read(idx);
    let stt = Point::from((0, 0));
    let end = Point::from((x - 1, y - 1));

    let mut maxn = bytes.len();
    let mut minn = 0;
    let res = loop {
        if minn == maxn - 1 {
            break maxn;
        }
        let mid = (maxn + minn) / 2;
        let mut map = Map::new((x, y), 0);
        for i in 0..mid {
            map.get_mut(Point::from((bytes[i][1], bytes[i][0])))
                .map(|v| *v += 1);
        }
        let able = map
            .bfs_iter(stt)
            .with_visit_filter(|p| map.get(p) == Some(&0))
            .run_with_target(|p| p == end);
        if able {
            minn = mid;
        } else {
            maxn = mid;
        }
    };
    let res = &bytes[res - 1];
    let res = format!("{},{}", res[0], res[1]);

    res
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
        assert_eq!(part1(1), "22");
        assert_eq!(part1(0), "454");

        assert_eq!(part2(1), "6,1");
        assert_eq!(part2(0), "8,51");
    }
}
