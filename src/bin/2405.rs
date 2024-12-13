use std::{fs, path::Path};

#[allow(dead_code)]
fn exec1(input: &(Vec<Vec<i32>>, Vec<Vec<i32>>)) -> String {
    // 原汁原味的拓扑排序啊
    // 所有输入都是两位数，不需要哈希
    let mut f = vec![-1; 100];
    for i in 0..input.0.len() {
        for j in [0, 1] {
            if f[input.0[i][j] as usize] == -1 {
                f[input.0[i][j] as usize] = 0;
            }
        }
        f[input.0[i][1] as usize] += 1;
    }
    let mut top = Vec::new();
    top.reserve(100);
    loop {
        let mut new_f = f.clone();
        for i in 0..100 {
            if f[i] == 0 {
                top.push(i as i32);
                // for (i, j, _) in input.1 {}
            } else {
                new_f[i] -= 1;
            }
        }
        f = new_f;
    }

    0.to_string()
}

fn exec2(input: &(Vec<Vec<i32>>, Vec<Vec<i32>>)) -> String {
    0.to_string()
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
    // let input = fs::read_to_string(file).unwrap();

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

    println!("{:?}", exec1(&input));
}
