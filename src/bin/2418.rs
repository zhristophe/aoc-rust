use std::{collections::VecDeque, fs, path::Path, usize};

use aoc::{Direction, Map, Point};

fn read(idx: usize) -> Vec<Vec<usize>> {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);
    let content = fs::read_to_string(file).unwrap();

    let inputs = [&content];

    let input = if idx > inputs.len() {
        inputs.last().unwrap()
    } else {
        inputs[idx]
    };

    {
        let tmp = input
            .lines()
            .map(|s| s.split(',').map(|s| s.parse().unwrap()).collect())
            .collect();

        tmp
    }
}

/// 简简单单BFS
fn part1(idx: usize) -> String {
    let bytes = read(idx);

    let (x, y) = (70 + 1, 70 + 1);
    let mut map = Map::new(x, y, 0);
    for i in 0..1024 {
        map.get_mut(Point::from((bytes[i][1], bytes[i][0])))
            .map(|v| *v += 1);
    }
    let stt = Point::from((0, 0));
    let end = Point::from((x - 1, y - 1));

    let mut steps = Map::new(x, y, usize::MAX);
    let mut queue = VecDeque::new();
    queue.push_back(stt);
    steps.get_mut(stt).map(|v| *v = 0);
    while let Some(p) = queue.pop_front() {
        let step = steps.get(p).unwrap().clone() + 1;
        for d in Direction::all() {
            let p = p.move_to(d);
            if map.get(p) != Some(&0) {
                continue;
            }
            let raw_step = steps.get(p).unwrap().clone();
            if step < raw_step {
                steps.get_mut(p).map(|v| *v = step);
                if !queue.contains(&p) && p != end {
                    queue.push_back(p);
                }
            }
        }
    }
    let res = steps.get(end).unwrap().clone();

    res.to_string()
}

/// BFS + 二分查找
fn part2(idx: usize) -> String {
    let bytes = read(idx);

    let (x, y) = (70 + 1, 70 + 1);
    let stt = Point::from((0, 0));
    let end = Point::from((x - 1, y - 1));

    let mut maxn = bytes.len();
    let mut minn = 0;
    let res = loop {
        if minn == maxn - 1 {
            break maxn;
        }

        let mid = (maxn + minn) / 2;
        let mut map = Map::new(x, y, 0);
        for i in 0..mid {
            map.get_mut(Point::from((bytes[i][1], bytes[i][0])))
                .map(|v| *v += 1);
        }
        let able = 'bfs: loop {
            let mut visited = Map::new(x, y, false);
            let mut queue = VecDeque::new();
            queue.push_back(stt);
            while let Some(p) = queue.pop_front() {
                visited.get_mut(p).map(|v| *v = true);
                for d in Direction::all() {
                    let p = p.move_to(d);
                    if map.get(p) != Some(&0) {
                        continue;
                    }
                    if visited.get(p) == Some(&true) {
                        continue;
                    }
                    if p == end {
                        break 'bfs true;
                    }
                    if !queue.contains(&p) {
                        queue.push_back(p);
                    }
                }
            }

            break 'bfs false;
        };
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
        assert_eq!(part1(0), "454");

        assert_eq!(part2(0), "8,51");
    }
}
