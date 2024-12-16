use std::{fs, path::Path};

#[allow(dead_code)]
fn exec1(input: Vec<(usize, Vec<usize>)>) -> String {
    let mut res = 0;
    for (tgt, opds) in input {
        // dfs
        fn dfs(num: usize, idx: usize, tgt: usize, opds: &Vec<usize>) -> bool {
            if num == tgt {
                return true;
            }
            if idx >= opds.len() {
                return false;
            }
            if num >= tgt {
                return false;
            }
            dfs(num + opds[idx], idx + 1, tgt, opds) || dfs(num * opds[idx], idx + 1, tgt, opds)
        }

        let ok = dfs(opds[0], 1, tgt, &opds);
        if ok {
            res += tgt;
        }
    }

    res.to_string()
}

#[allow(dead_code)]
fn exec2(input: Vec<(usize, Vec<usize>)>) -> String {
    let mut res = 0;
    for (tgt, opds) in input {
        fn concat(a: usize, b: usize) -> usize {
            (a.to_string() + &b.to_string()).parse().unwrap()
        }

        // dfs
        fn dfs(num: usize, idx: usize, tgt: usize, opds: &Vec<usize>) -> bool {
            if idx == opds.len() {
                return num == tgt;
            }
            if num > tgt {
                return false;
            }
            dfs(num + opds[idx], idx + 1, tgt, opds)
                || dfs(num * opds[idx], idx + 1, tgt, opds)
                || dfs(concat(num, opds[idx]), idx + 1, tgt, opds)
        }

        if dfs(opds[0], 1, tgt, &opds) {
            res += tgt;
        }
    }

    res.to_string()
}

#[allow(unused_variables)]
fn main() {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);

    let input = r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
    .trim();
    let input = r"
727: 16 39 4 97 2
"
    .trim();
    let input = fs::read_to_string(file).unwrap();

    let input = input
        .lines()
        .map(|s| {
            let (p1, p2) = s.split_once(": ").unwrap();
            (
                p1.parse().unwrap(),
                p2.split(' ').map(|s| s.parse().unwrap()).collect(),
            )
        })
        .collect();

    println!("{:?}", exec2(input));
}
