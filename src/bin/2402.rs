use std::{fs, path::Path};

#[allow(dead_code)]
fn exec1(input: &Vec<Vec<i32>>) -> String {
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

fn exec2(input: &Vec<Vec<i32>>) -> String {
    let len = input.len();
    let mut res = len;

    fn is_safe(a: isize, b: isize, c: isize, input: &Vec<i32>) -> bool {
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
                j = j + 1;
                continue;
            }
            if is_safe(j - 1, j, j + 2, input) && is_safe(j, j + 2, j + 3, input) {
                j = j + 2;
                continue;
            }
            if is_safe(j - 1, j, j + 1, input)
                && is_safe(j, j + 1, j + 3, input)
                && is_safe(j + 1, j + 3, j + 4, input)
            {
                j = j + 3;
                continue;
            }
        }
    }

    res.to_string()
}

#[allow(unused_variables)]
fn main() {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);

    let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let input = fs::read_to_string(file).unwrap();

    let input = input
        .lines()
        .into_iter()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", exec2(&input));
}
