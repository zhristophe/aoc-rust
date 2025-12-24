use utils::prelude::*;

fn read(idx: usize) -> Vec<(i64, i64)> {
    let input = read_input(module_path!()).unwrap();

    let input = &vec![
        input,
        r"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
"
        .lines()
        .into_iter()
        .map(|s| s.trim())
        .collect::<String>(),
    ][idx];

    input
        .lines()
        .into_iter()
        .collect::<String>()
        .as_bytes()
        .split_by_byte(b',')
        .map(|r| {
            let (a, b) = r.splite_once(b'-').unwrap();
            let a = a.to_u64_unsafe() as i64;
            let b = b.to_u64_unsafe() as i64;

            (a, b)
        })
        .collect()
}

/// 先计算位数，然后枚举前一半
fn sum_invalid_ids(a: i64, b: i64) -> i64 {
    // 保证是偶数位数
    let dca = a.digit_count();
    if dca % 2 != 0 {
        return sum_invalid_ids(10_i64.pow(dca), b);
    }
    let dcb = b.digit_count();
    if dcb % 2 != 0 {
        return sum_invalid_ids(a, 10_i64.pow(dcb - 1) - 1);
    }

    let mut res = 0;
    // 如果位数不同，让它们相同
    let (mut a, mut dca) = (a, dca);
    while dca < dcb {
        res += sum_invalid_ids(a, 10_i64.pow(dca) - 1);
        a = 10_i64.pow(dca + 1);
        dca += 2;
    }
    // 位数相同，进行枚举
    let m = 10_i64.pow(dca / 2);
    let a_pre = a / m;
    let a_suf = a % m;
    let b_pre = b / m;
    let b_suf = b % m;
    let stt = if a_suf <= a_pre { a_pre } else { a_pre + 1 };
    let end = if b_suf >= b_pre { b_pre } else { b_pre - 1 };
    for i in stt..=end {
        res += i + i * m;
    }

    res
}

pub fn part1(idx: usize) -> i64 {
    let ranges = read(idx);

    ranges.iter().map(|(a, b)| sum_invalid_ids(*a, *b)).sum()
}

/// 升级版，允许重复k次而不是2次
fn get_k_rank_invalid_ids(a: i64, b: i64, k: u32) -> Vec<i64> {
    assert!(k > 1);
    if a > b {
        return vec![];
    }

    // 保证是k倍位数
    let dca = a.digit_count();
    if dca % k != 0 {
        // 修改后可能仍然不是k倍位数，不过可以递归。下同。
        return get_k_rank_invalid_ids(10_i64.pow(dca), b, k);
    }
    let dcb = b.digit_count();
    if dcb % k != 0 {
        return get_k_rank_invalid_ids(a, 10_i64.pow(dcb - 1) - 1, k);
    }

    let mut res = vec![];
    // 如果位数不同，让它们相同
    let (mut a, mut dca) = (a, dca);
    while dca < dcb {
        res.extend(get_k_rank_invalid_ids(a, 10_i64.pow(dca) - 1, k));
        a = 10_i64.pow(dca + k - 1);
        dca += k;
    }
    // 位数相同，进行枚举
    let m = 10_i64.pow(dca / k);

    fn slice_num(n: i64, m: i64) -> Vec<i64> {
        let mut res = vec![];
        let mut n = n;
        while n > 0 {
            res.push(n % m);
            n /= m;
        }
        res
    }

    let mut a_parts = slice_num(a, m);
    let mut b_parts = slice_num(b, m);

    let a_pre = a_parts.pop().unwrap();
    let b_pre = b_parts.pop().unwrap();

    let mut is_ge = true;
    for part in a_parts.iter().rev() {
        if a_pre > *part {
            is_ge = true;
            break;
        } else if a_pre < *part {
            is_ge = false;
            break;
        }
    }
    let stt = if is_ge { a_pre } else { a_pre + 1 };

    let mut is_le = true;
    for part in b_parts.iter().rev() {
        if b_pre < *part {
            is_le = true;
            break;
        } else if b_pre > *part {
            is_le = false;
            break;
        }
    }
    let end = if is_le { b_pre } else { b_pre - 1 };

    let mut times = 0;
    for i in 0..k {
        times += m.pow(i);
    }

    for i in stt..=end {
        res.push(i * times);
    }

    res
}

pub fn part2(idx: usize) -> i64 {
    let ranges = read(idx);

    let mut ans = 0;
    for range in ranges {
        let mut nums = HashSet::new();
        for i in 2..=range.1.digit_count() {
            nums.extend(get_k_rank_invalid_ids(range.0, range.1, i));
        }

        ans += nums.iter().sum::<i64>();
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
        assert_eq!(part1(0), 24043483400);
        assert_eq!(part1(1), 1227775554);

        assert_eq!(part2(0), 38262920235);
        assert_eq!(part2(1), 4174379265);
    }

    #[test]
    fn test_sum_invalid_ids() {
        assert_eq!(sum_invalid_ids(11, 22), 33);
        assert_eq!(sum_invalid_ids(95, 115), 99);
        assert_eq!(sum_invalid_ids(998, 1012), 1010);
        assert_eq!(sum_invalid_ids(1188511880, 1188511890), 1188511885);
        assert_eq!(sum_invalid_ids(222220, 222224), 222222);
        assert_eq!(sum_invalid_ids(1698522, 1698528), 0);
        assert_eq!(sum_invalid_ids(446443, 446449), 446446);
        assert_eq!(sum_invalid_ids(38593856, 38593862), 38593859);
    }

    #[test]
    fn test_k_rank_sum_invalid_ids() {
        assert_eq!(get_k_rank_invalid_ids(11, 22, 2), vec![11, 22]);
        assert_eq!(get_k_rank_invalid_ids(95, 115, 2), vec![99]);
        assert_eq!(get_k_rank_invalid_ids(95, 115, 3), vec![111]);
        assert_eq!(get_k_rank_invalid_ids(998, 1012, 2), vec![1010]);
        assert_eq!(get_k_rank_invalid_ids(998, 1012, 3), vec![999]);
        assert_eq!(
            get_k_rank_invalid_ids(38593856, 38593862, 2),
            vec![38593859]
        );
        assert_eq!(get_k_rank_invalid_ids(38593856, 38593862, 6), vec![]);
        assert_eq!(get_k_rank_invalid_ids(565653, 565659, 3), vec![565656]);
        assert_eq!(
            get_k_rank_invalid_ids(824824821, 824824827, 3),
            vec![824824824]
        );
        assert_eq!(
            get_k_rank_invalid_ids(2121212118, 2121212124, 5),
            vec![2121212121]
        );
    }
}
