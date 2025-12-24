use utils::prelude::*;

fn read(idx: usize) -> Vec<Vec<i32>> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        &input,
        r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"
        .trim(),
    ][idx];

    input
        .lines()
        .into_iter()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(idx: usize) -> String {
    let input = read(idx);

    let len = input.len();
    let mut res = len;
    for i in 0..len {
        let input = &input[i];
        let mut sign = input[1] - input[0];
        for j in 0..input.len() - 1 {
            let new_sign = input[j + 1] - input[j];
            if new_sign.abs() > 3 || new_sign.abs() < 1 || new_sign * sign < 0 {
                res -= 1;
                break;
            }
            sign = new_sign;
        }
    }

    res.to_string()
}

pub fn part2(idx: usize) -> String {
    let input = read(idx);

    let len = input.len();
    let mut res = len;

    fn is_safe(a: isize, b: isize, c: isize, input: &[i32]) -> bool {
        if a < 0 || c as usize >= input.len() {
            return true;
        }
        let (a, b, c) = (input[a as usize], input[b as usize], input[c as usize]);

        for d in [a - b, b - c] {
            if d.abs() > 3 || d.abs() < 1 {
                return false;
            }
        }
        if (a - b) * (b - c) < 0 {
            return false;
        }

        true
    }

    for i in 0..len {
        let input = &input[i];
        let mut first = true;
        let mut j = 0isize;
        loop {
            if j as usize >= input.len() - 2 {
                break;
            }
            if is_safe(j, j + 1, j + 2, input) {
                j += 1;
                continue;
            }
            if !first {
                res -= 1;
                break;
            }
            first = false;
            // 只有三个删除选择，j,j+1,j+2
            if is_safe(j - 1, j + 1, j + 2, input) && is_safe(j + 1, j + 2, j + 3, input) {
                j += 1;
                continue;
            }
            if is_safe(j - 1, j, j + 2, input) && is_safe(j, j + 2, j + 3, input) {
                j += 2;
                continue;
            }
            if is_safe(j - 1, j, j + 1, input)
                && is_safe(j, j + 1, j + 3, input)
                && is_safe(j + 1, j + 3, j + 4, input)
            {
                j += 3;
                continue;
            }
        }
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
        // assert_eq!(part1(0), "369");
        assert_eq!(part1(1), "2");

        // assert_eq!(part2(0), "428");
        assert_eq!(part2(1), "4");
    }
}
