use std::collections::HashMap;

use utils::read_input;

fn read(idx: usize) -> Vec<usize> {
    let input = read_input(module_path!()).unwrap();

    let input = [input.as_str(), "125 17"][idx];

    input
        .trim()
        .split(" ")
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}

fn solve(idx: usize, times: usize) -> String {
    let input = read(idx);
    fn count_digits(mut stone: usize) -> usize {
        if stone == 0 {
            return 1;
        }
        let mut res = 0;
        while stone > 0 {
            res += 1;
            stone /= 10;
        }
        res
    }

    fn blink(stone: usize, times: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
        if times == 0 {
            return 1;
        }
        if let Some(v) = cache.get(&(stone, times)) {
            return *v;
        }
        let mut res = 0usize;
        if stone == 0 {
            res += blink(1, times - 1, cache);
        } else {
            let n_digits = count_digits(stone);
            if n_digits % 2 == 0 {
                let n = 10usize.pow((n_digits / 2) as u32);
                res += blink(stone / n, times - 1, cache);
                res += blink(stone % n, times - 1, cache);
            } else {
                res += blink(stone * 2024, times - 1, cache);
            }
        }

        cache.insert((stone, times), res);

        res
    }

    // 缓存一个数在k轮后的数量
    let mut cache = HashMap::new();
    let mut res = 0;
    for stone in input {
        res += blink(stone, times, &mut cache)
    }

    res.to_string()
}

pub fn part1(idx: usize) -> String {
    solve(idx, 25)
}

pub fn part2(idx: usize) -> String {
    solve(idx, 75)
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
        assert_eq!(part1(1), "55312");
        // assert_eq!(part1(0), "183688");

        assert_eq!(part2(1), "65601038650482");
        // assert_eq!(part2(0), "217443384398698");
    }
}
