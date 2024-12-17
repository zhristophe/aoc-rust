use std::{fs, path::Path};

use regex::Regex;

fn read(idx: usize) -> Vec<Vec<char>> {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);
    let content = fs::read_to_string(file).unwrap();

    let inputs = [
        &content,
        r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"
        .trim(),
        r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"
        .trim(),
    ];

    inputs[idx].lines().map(|s| s.chars().collect()).collect()
}

fn part1(idx: usize) -> String {
    let lines = read(idx);
    let mut res = 0;
    for line in lines {
        let digit1 = line.iter().find(|c| c.is_ascii_digit()).unwrap();
        let digit2 = line.iter().rev().find(|c| c.is_ascii_digit()).unwrap();
        // dbg!(digit1, digit2);
        res += digit1.to_digit(10).unwrap() * 10 + digit2.to_digit(10).unwrap();
    }

    res.to_string()
}

fn part2(idx: usize) -> String {
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let nums2 = (1..=9).map(|n| n.to_string()).collect::<Vec<_>>();

    let lines = read(idx);
    let mut res = 0;
    for line in lines {
        let mut first = (line.len(), 0);
        let mut last = (0, 0);
        let line = line.iter().collect::<String>();

        let mut update_first_and_last = |num, num_str| {
            let regex = Regex::new(num_str).unwrap();
            let locs = regex.captures_iter(&line).collect::<Vec<_>>();
            if let Some(loc) = locs.first() {
                let first_pos = loc.get(0).unwrap().start();
                if first_pos < first.0 {
                    first = (first_pos, num);
                }
            }

            if let Some(loc) = locs.last() {
                let last_pos = loc.get(0).unwrap().end();
                if last_pos >= last.0 {
                    last = (last_pos, num);
                }
            }
        };

        for (num, num_str) in nums.iter().enumerate() {
            update_first_and_last(num + 1, num_str);
        }

        for (num, num_str) in nums2.iter().enumerate() {
            update_first_and_last(num + 1, &num_str);
        }

        res += first.1 * 10 + last.1;
    }

    res.to_string()
}

#[allow(unused_variables)]
fn main() {
    println!("{:?}", part1(0));
    println!("{:?}", part2(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(0), "53334");
        assert_eq!(part1(1), "142");

        assert_eq!(part2(0), "52834");
        assert_eq!(part2(2), "281");
    }
}
