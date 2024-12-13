use std::{fs, path::Path, vec};

#[allow(dead_code)]
fn exec1(input: &(Vec<Vec<i32>>, Vec<Vec<i32>>)) -> String {
    let len = 100;
    let mut map = vec![Vec::new(); len];
    for i in 0..input.0.len() {
        map[input.0[i][0] as usize].push(input.0[i][1]);
    }
    let mut res = 0;
    'main: for input in &input.1 {
        for i in (0..input.len()).rev() {
            for j in &map[input[i] as usize] {
                if input[..i].contains(j) {
                    println!("不正确");
                    continue 'main;
                }
            }
        }
        res += input[input.len() / 2];
    }

    res.to_string()
}

#[allow(dead_code)]
fn exec2(input: &(Vec<Vec<i32>>, Vec<Vec<i32>>)) -> String {
    // 陷阱：排序规则含有循环，离谱
    // 因此拓扑排序每次都要重排，放弃
    let len = 100;
    let mut map = vec![Vec::new(); len];
    for i in 0..input.0.len() {
        map[input.0[i][0] as usize].push(input.0[i][1]);
    }
    let mut res = 0;
    for input in &input.1 {
        let is_wrong = 'is_wrong: loop {
            for i in (0..input.len()).rev() {
                for j in &map[input[i] as usize] {
                    if input[..i].contains(j) {
                        break 'is_wrong true;
                    }
                }
            }
            break false;
        };
        if !is_wrong {
            continue;
        }

        let mut input = input.clone();
        input.sort_by(|a, b| {
            if map[*a as usize].contains(b) {
                std::cmp::Ordering::Less
            } else if map[*b as usize].contains(a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        res += input[input.len() / 2];
    }

    res.to_string()
}

#[allow(unused_variables)]
fn main() {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);

    let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let input = fs::read_to_string(file).unwrap();

    let input = input.split_once("\n\n").unwrap();
    let input0 = input
        .0
        .lines()
        .map(|s| s.split('|').map(|s| s.parse().unwrap()).collect())
        .collect();
    let input1 = input
        .1
        .lines()
        .map(|s| s.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();
    let input = (input0, input1);

    println!("{:?}", exec2(&input));
}
