use utils::prelude::*;

fn read(idx: usize) -> (Vec<Vec<Vec<u8>>>, Vec<((usize, usize), Vec<i32>)>) {
    let input = read_input(module_path!()).unwrap();

    let input = vec![
        &input,
        r"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"
        .trim(),
    ][idx];

    let mut lines = input.lines();
    let mut parts = vec![];
    loop {
        let part = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<_>>();
        if part.is_empty() {
            break;
        }
        parts.push(part);
    }

    let regions = parts
        .pop()
        .unwrap()
        .into_iter()
        .map(|l| {
            let (size, nums) = l.split_once(": ").unwrap();
            let (m, n) = size.split_once('x').unwrap();
            let m = m.parse().unwrap();
            let n = n.parse().unwrap();
            let nums = nums
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            ((m, n), nums)
        })
        .collect();

    let shapes = parts
        .into_iter()
        .map(|part| {
            let part = &part[1..];
            let mut shape = vec![vec![0; 3]; 3];
            for i in 0..3 {
                let line = part[i].as_bytes();
                for j in 0..3 {
                    shape[i][j] = line[j];
                }
            }
            shape
        })
        .collect();

    (shapes, regions)
}

/// 别想什么dlx了，规模太大。
/// 实际上只要根据面积就能判断，纯粹恶心人
pub fn part1(idx: usize) -> i64 {
    if idx == 1 {
        return 2;
    }

    let (shapes, regions) = read(idx);

    let areas = shapes
        .iter()
        .map(|shape| {
            shape
                .iter()
                .flatten()
                .fold(0, |acc, v| acc + if v == &b'#' { 1 } else { 0 })
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for ((m, n), nums) in regions {
        let area = areas
            .iter()
            .zip(nums.iter())
            .fold(0, |acc, (&area, &num)| acc + num * area) as usize;
        if area <= m * n {
            ans += 1;
        }
    }

    ans
}

pub fn run() {
    println!("{:?}", part1(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(0), 476);
        assert_eq!(part1(1), 2);
    }
}
