use utils::prelude::*;

fn read(idx: usize) -> Vec<Vec<u8>> {
    let input = read_input(module_path!()).unwrap();

    let input = vec![
        &input,
        r"
987654321111111
811111111111119
234234234234278
818181911112111
"
        .trim(),
    ][idx];

    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect())
        .collect()
}

/// 贪心法
pub fn part1(idx: usize) -> i64 {
    let banks = read(idx);

    let mut ans = 0;
    for bank in banks {
        let cal = || {
            for n in (1..=9).rev() {
                // 不能是最后一个数
                for i in 0..bank.len() - 1 {
                    if bank[i] == n {
                        return n * 10 + bank[i + 1..].iter().max().unwrap();
                    }
                }
            }
            unreachable!()
        };

        ans += cal() as i64;
    }

    ans
}

/// 依旧贪心法
pub fn part2(idx: usize) -> i64 {
    let banks = read(idx);

    fn cal(bank: &[u8], k: u32) -> i64 {
        if k == 1 {
            return bank.iter().max().map_or(i64::MIN, |n| *n as i64);
        }
        for n in (1..=9).rev() {
            // 保留至少 k - 1 个数
            for i in 0..bank.len() - k as usize + 1 {
                if bank[i] == n {
                    return n as i64 * 10_i64.pow(k - 1) + cal(&bank[i + 1..], k - 1);
                }
            }
        }

        unreachable!()
    }

    banks.iter().map(|bank| cal(bank, 12)).sum()
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
        assert_eq!(part1(0), 17034);
        assert_eq!(part1(1), 357);

        assert_eq!(part2(0), 168798209663590);
        assert_eq!(part2(1), 3121910778619);
    }
}
