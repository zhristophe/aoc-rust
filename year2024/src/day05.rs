use utils::read_input;

fn read(idx: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
47|53
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
97,13,75,29,47
"
        .trim(),
    ][idx];

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
    (input0, input1)
}

pub fn part1(idx: usize) -> String {
    let input = read(idx);
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
                    // println!("不正确");
                    continue 'main;
                }
            }
        }
        res += input[input.len() / 2];
    }

    res.to_string()
}

pub fn part2(idx: usize) -> String {
    let input = read(idx);
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

pub fn run() {
    println!("{:?}", part1(0));
    println!("{:?}", part2(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(1), "143");
        // assert_eq!(part1(0), "5091");

        assert_eq!(part2(1), "123");
        // assert_eq!(part2(0), "4681");
    }
}
