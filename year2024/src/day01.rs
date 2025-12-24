use utils::prelude::*;

fn read(idx: usize) -> Vec<Vec<isize>> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
3   4
4   3
2   5
1   3
3   9
3   3
"
        .trim(),
    ][idx];

    {
        input
            .lines()
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect()
    }
}

/// easy
fn part1(idx: usize) -> String {
    let input = read(idx);

    let len: usize = input.len();
    let input = {
        let mut tmp = vec![vec![0; len]; 2];
        for i in 0..2 {
            for j in 0..len {
                tmp[i][j] = input[j][i];
            }
            tmp[i].sort();
        }
        tmp
    };

    let mut ret = 0;
    for i in 0..len {
        ret += (input[0][i] - input[1][i]).abs();
    }

    ret.to_string()
}

/// easy
fn part2(idx: usize) -> String {
    let input = read(idx);

    let len = input.len();
    let mut res = 0;
    for i in 0..len {
        let left = input[i][0];
        let mut cnt = 0;
        for j in 0..len {
            if left == input[j][1] {
                cnt += 1;
            }
        }
        res += left * cnt;
    }

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
        assert_eq!(part1(1), "11");
        // assert_eq!(part1(0), "936063");

        assert_eq!(part2(1), "31");
        // assert_eq!(part2(0), "23150395");
    }
}
