use std::{
    sync::atomic::{AtomicIsize, Ordering},
    thread::available_parallelism,
};

use rayon::prelude::*;
use utils::prelude::*;

fn read(idx: usize) -> Vec<usize> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
1
10
100
2024
",
        r"
1
2
3
2024
",
    ][idx]
        .trim();

    {
        input.lines().map(|s| s.parse().unwrap()).collect()
    }
}

#[inline]
fn mix_and_prune(val: usize, old: usize) -> usize {
    (val ^ old) % 16777216
}

/// 64 = 2 ^ 6, 32 = 2 ^ 5, 2048 = 2 ^ 11
fn next_val(mut val: usize) -> usize {
    val = mix_and_prune(val << 6, val);
    val = mix_and_prune(val >> 5, val);
    mix_and_prune(val << 11, val)
}

/// 简单模拟
fn part1(idx: usize) -> String {
    let input = read(idx);

    let mut res = 0;
    for mut input in input {
        for _ in 0..2000 {
            input = next_val(input);
        }
        res += input;
    }

    res.to_string()
}

/// 暴力求解
fn part2(idx: usize) -> String {
    let input = read(idx);

    fn change_id(change: &[isize]) -> isize {
        let mut res = 0;
        for i in 0..4 {
            res *= 20;
            res += change[i] + 9;
        }
        res
    }

    let len = 20usize.pow(4);
    let mut change_val_map = Vec::with_capacity(len);
    for _ in 0..len {
        change_val_map.push(AtomicIsize::new(0));
    }
    let num_threads = available_parallelism().unwrap().get().max(32);
    (0..num_threads).into_par_iter().for_each(|id| {
        let len = input.len();
        let stt = id * len / num_threads;
        let end = (id + 1) * len / num_threads;
        let end = if end > len { len } else { end };
        for i in stt..end {
            let mut first_occur = HashSet::new();
            let mut input = input[i] as isize;
            let mut seq = Vec::new();
            for _ in 0..2000 {
                let new_input = next_val(input as usize) as isize;
                seq.push(new_input % 10 - input % 10);
                if seq.len() > 4 {
                    seq.remove(0);
                }
                if seq.len() == 4 {
                    if first_occur.insert(seq.clone()) {
                        change_val_map[change_id(&seq) as usize]
                            .fetch_add(new_input % 10, Ordering::Relaxed);
                    }
                }
                input = new_input;
            }
        }
    });

    let res = change_val_map
        .iter()
        .map(|v| v.load(Ordering::Relaxed))
        .max()
        .unwrap();

    res.to_string()
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
        // assert_eq!(part1(0), "19927218456");
        assert_eq!(part1(1), "37327623");

        // assert_eq!(part2(0), "2189");
        assert_eq!(part2(2), "23");
    }
}
