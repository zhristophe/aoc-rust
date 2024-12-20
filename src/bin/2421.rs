use std::{fs, path::Path, usize};

use aoc::prelude::*;

fn read(idx: usize) -> Vec<Vec<char>> {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);
    let content = fs::read_to_string(file).unwrap();

    let inputs = [
        content.as_str(),
        r"
place
holder
"
        .trim(),
    ];

    let input = if idx >= inputs.len() {
        inputs.last().unwrap()
    } else {
        &inputs[idx]
    };

    {
        let tmp = input.lines().map(|s| s.chars().collect()).collect();
        tmp
    }
}

///
fn part1(idx: usize) -> String {
    let map = read(idx);
    0.to_string()
}

/// 基本一样，只是搜索20步
fn part2(idx: usize) -> String {
    0.to_string()
}

fn main() {
    println!("{:?}", part1(1));
    // println!("{:?}", part2(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(part1(0), "1363");

        // assert_eq!(part2(0), "1007186");
    }
}
