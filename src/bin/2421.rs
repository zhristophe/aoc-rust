use aoc::prelude::*;

fn read(idx: usize) -> Vec<Vec<char>> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
place
holder
"
        .trim(),
    ][idx];

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
