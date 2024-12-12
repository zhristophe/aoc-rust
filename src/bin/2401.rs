use std::{fs, path::Path, vec};

#[allow(dead_code)]
fn exec1(input: &Vec<Vec<i32>>) -> String {
    let len = input.len();
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

    // dbg!(&input);

    let mut ret = 0;
    for i in 0..len {
        ret += (input[0][i] - input[1][i]).abs();
    }

    ret.to_string()
}

fn exec2(input: &Vec<Vec<i32>>) -> String {
    let len = input.len();
    // 数据量太小，懒得优化了
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

#[allow(unused_variables)]
fn main() {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);

    let input = r"3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
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
