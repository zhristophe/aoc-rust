use utils::prelude::*;

fn read(idx: usize) -> String {
    let input = read_input(module_path!()).unwrap();

    let input = vec![
        &input,
        r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
        .trim(),
    ][idx];

    input.to_string()
}

/// 略
pub fn part1(idx: usize) -> i64 {
    let input = read(idx);
    let lines = input.lines().collect::<Vec<_>>();
    let (nums, ops) = (
        lines
            .iter()
            .take(lines.len() - 1)
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
        lines
            .last()
            .unwrap()
            .split_whitespace()
            .map(|op| op.as_bytes().first().unwrap().clone())
            .collect::<Vec<_>>(),
    );

    let mut ans = 0;
    for i in 0..ops.len() {
        ans += match ops[i] {
            b'*' => {
                let mut acc = 1;
                for nums in &nums {
                    acc *= nums[i];
                }
                acc
            }
            b'+' => {
                let mut acc = 0;
                for nums in &nums {
                    acc += nums[i];
                }
                acc
            }
            _ => unreachable!(),
        }
    }

    ans
}

/// 简单模拟
pub fn part2(idx: usize) -> i64 {
    let input = read(idx);
    let mut lines = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    // 每一行加个空格
    for line in &mut lines {
        line.push(b' ');
    }

    // 逐列读取
    let mut ans = 0;
    let mut opds = vec![];
    let mut op = None;
    for j in 0..lines[0].len() {
        let mut opd = 0;
        for i in 0..lines.len() {
            if let Some(&b) = lines[i].get(j) {
                match b {
                    b'0'..=b'9' => opd = opd * 10 + (b - b'0') as i64,
                    b'*' | b'+' => op = Some(b),
                    b' ' => (),
                    _ => unreachable!(),
                }
            }
        }
        if opd == 0 {
            // 为0就是空行，进行计算
            ans += match op {
                Some(b'*') => {
                    let mut acc = 1;
                    for opd in &opds {
                        acc *= opd;
                    }
                    acc
                }
                Some(b'+') => {
                    let mut acc = 0;
                    for opd in &opds {
                        acc += opd;
                    }
                    acc
                }
                _ => unreachable!(),
            };
            opds = vec![];
            op = None;
        } else {
            opds.push(opd);
        }
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
        assert_eq!(part1(0), 5322004718681);
        assert_eq!(part1(1), 4277556);

        assert_eq!(part2(0), 9876636978528);
        assert_eq!(part2(1), 3263827);
    }
}
