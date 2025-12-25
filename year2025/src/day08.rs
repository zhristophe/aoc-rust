use utils::prelude::*;

fn read(idx: usize) -> Vec<(i64, i64, i64)> {
    let input = read_input(module_path!()).unwrap();

    let input = vec![
        &input,
        r"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
        .trim(),
    ][idx];

    input
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();
            let z = it.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect()
}

/// 获取距离的平方
fn get_dis(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

/// 简单模拟 + 并查集
pub fn part1(idx: usize) -> i64 {
    let boxes = read(idx);

    let n = boxes.len();
    let mut dis = Vec::with_capacity(n * (n - 1));

    for i in 0..n {
        for j in i + 1..n {
            dis.push((get_dis(boxes[i], boxes[j]), i, j));
        }
    }

    dis.sort_by_key(|(d, _, _)| *d);

    let mut uf = UnionFind::new(n);

    for (_, i, j) in dis.iter().take(if idx == 0 { 1000 } else { 10 }) {
        uf.union(*i, *j);
    }

    let mut cnt = vec![0; n];
    for i in 0..n {
        cnt[uf.root(i)] += 1;
    }

    cnt.sort();

    cnt.into_iter().rev().take(3).fold(1, |acc, v| acc * v)
}

/// 和part1类似
pub fn part2(idx: usize) -> i64 {
    let boxes = read(idx);

    let n = boxes.len();
    let mut dis = Vec::with_capacity(n * (n - 1));

    for i in 0..n {
        for j in i + 1..n {
            dis.push((get_dis(boxes[i], boxes[j]), i, j));
        }
    }

    dis.sort_by_key(|(d, _, _)| *d);

    let mut uf = UnionFind::new(n);
    let mut n = n;

    for &(_, i, j) in dis.iter() {
        if uf.union(i, j) {
            n -= 1;
        }
        if n == 1 {
            return boxes[i].0 * boxes[j].0;
        }
    }

    unreachable!()
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
        assert_eq!(part1(0), 68112);
        assert_eq!(part1(1), 40);

        assert_eq!(part2(0), 44543856);
        assert_eq!(part2(1), 25272);
    }
}
