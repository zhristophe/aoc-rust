use utils::prelude::*;

fn read(idx: usize) -> (Vec<(i64, i64)>, Vec<i64>) {
    let input = read_input(module_path!()).unwrap();

    let input = vec![
        &input,
        r"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
        .trim(),
    ][idx];

    let (ranges, ids) = input.split_once("\n\n").unwrap();

    (
        ranges
            .lines()
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect(),
        ids.lines().map(|line| line.parse().unwrap()).collect(),
    )
}

/// BTree缓存范围
pub fn part1(idx: usize) -> i64 {
    let (ranges, ids) = read(idx);

    let mut bt = std::collections::BTreeMap::<i64, i64>::new();
    for range in ranges {
        bt.entry(range.0)
            .and_modify(|v| *v = (*v).max(range.1))
            .or_insert(range.1);
    }

    let mut ans = 0;
    for id in ids {
        for (_, &hi) in bt.range(..=id) {
            if hi >= id {
                ans += 1;
                break;
            }
        }
    }

    ans
}

/// 和part1类似，BTree缓存后处理重叠
pub fn part2(idx: usize) -> i64 {
    let (ranges, _) = read(idx);

    let mut bt = std::collections::BTreeMap::<i64, i64>::new();
    for range in ranges {
        bt.entry(range.0)
            .and_modify(|v| *v = (*v).max(range.1))
            .or_insert(range.1);
    }

    let mut ans = 0;
    let mut pos = 0;
    while let Some((&lo, &hi)) = bt.range(pos..).next() {
        let mut hi = hi;
        loop {
            if let Some((&lo2, &hi2)) = bt.range(lo..=hi).next() {
                bt.remove(&lo2);
                hi = hi.max(hi2);
            } else {
                break;
            }
        }

        ans += hi - lo + 1;
        pos = hi + 1;
    }

    ans
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
        assert_eq!(part1(0), 868);
        assert_eq!(part1(1), 3);

        assert_eq!(part2(0), 354143734113772);
        assert_eq!(part2(1), 14);
    }
}
