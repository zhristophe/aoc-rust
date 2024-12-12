use std::{collections::HashMap, fs, path::Path};

#[allow(dead_code)]
fn exec1(input: &[usize], times: usize) -> String {
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
        res += blink(*stone, times, &mut cache)
    }

    res.to_string()
}

fn main() {
    let input = vec![125, 17];

    // let file = Path::new("data/2412/input");
    // let input = fs::read_to_string(file).unwrap();
    // let input = input
    //     .split(" ")
    //     .into_iter()
    //     .map(|s| s.parse().unwrap())
    //     .collect::<Vec<_>>();

    let times = 75;
    println!("{:?}", exec1(&input, times))
}
