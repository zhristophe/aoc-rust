use std::{fs, path::Path};

#[allow(dead_code)]
fn exec1(input: &Vec<Vec<char>>) -> String {
    let directions = {
        let mut tmp = Vec::new();
        for i in -1..=1 as isize {
            for j in -1..=1 as isize {
                if i != 0 || j != 0 {
                    tmp.push((i, j));
                }
            }
        }
        tmp
    };

    let tgt_str = "XMAS";
    let tgt_len = tgt_str.len() as isize;

    macro_rules! is_out_of_bound {
        ($i:expr) => {{
            let (i, j) = $i;
            if i < 0 || i >= input.len() as isize || j < 0 || j >= input[0].len() as isize {
                true
            } else {
                false
            }
        }};
    }

    // 两个方法，一个是做子串匹配，一个是遍历起点
    // 懒得写子串匹配 :)
    let mut res = 0;
    for i in 0..input.len() as isize {
        for j in 0..input[0].len() as isize {
            'd: for d in &directions {
                let end = (i + (tgt_len - 1) * d.0, j + (tgt_len - 1) * d.1);
                if is_out_of_bound!(end) {
                    continue 'd;
                }
                let mut cur = (i, j);
                let mut chars = tgt_str.chars();
                while let Some(ch) = chars.next() {
                    if input[cur.0 as usize][cur.1 as usize] != ch {
                        continue 'd;
                    }
                    cur = (cur.0 + d.0, cur.1 + d.1);
                }
                res += 1;
            }
        }
    }

    res.to_string()
}

fn exec2(input: &Vec<Vec<char>>) -> String {
    macro_rules! get_ch {
        ($p:expr) => {{
            let (i, j) = $p;
            if 0 < i && i <= input.len() && 0 < j && j <= input[0].len() {
                Some(input[i - 1][j - 1])
            } else {
                None
            }
        }};
    }

    let mut res = 0;
    for i in 1 + 1..=input.len() - 1 {
        for j in 1 + 1..=input[0].len() - 1 {
            if let Some('A') = get_ch!((i, j)) {
                let points = [
                    (i - 1, j - 1),
                    (i - 1, j + 1),
                    (i + 1, j - 1),
                    (i + 1, j + 1),
                ];
                let chars = points
                    .iter()
                    .map(|p| get_ch!(*p).unwrap())
                    .collect::<Vec<_>>();
                let is_xmas = if chars[0] == chars[1] {
                    chars[2] == chars[3]
                        && ((chars[0] == 'M' && chars[2] == 'S')
                            || (chars[0] == 'S' && chars[2] == 'M'))
                } else {
                    chars[0] == chars[2]
                        && chars[1] == chars[3]
                        && ((chars[0] == 'M' && chars[1] == 'S')
                            || (chars[0] == 'S' && chars[1] == 'M'))
                };
                if is_xmas {
                    res += 1;
                }
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

    let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let input = fs::read_to_string(file).unwrap();

    let input = input.lines().map(|s| s.chars().collect()).collect();

    println!("{:?}", exec2(&input));
}
