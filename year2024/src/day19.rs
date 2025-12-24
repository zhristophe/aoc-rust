use utils::prelude::*;

fn read(idx: usize) -> (Vec<String>, Vec<String>) {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"
        .trim(),
    ][idx];

    {
        let tmp = input.split_once("\n\n").unwrap();
        let towels = tmp.0.split(", ").map(|s| s.to_string()).collect();
        let desired = tmp.1.lines().map(|s| s.to_string()).collect();

        (towels, desired)
    }
}

/// 简简单单记忆化搜索
pub fn part1(idx: usize) -> String {
    let (towels, desired) = read(idx);

    let mut cache = HashSet::new();
    for towel in &towels {
        cache.insert(towel.clone());
    }

    let mut res = 0;
    fn is_possible(desired: String, towels: &Vec<String>, cache: &mut HashSet<String>) -> bool {
        if cache.contains(&desired) {
            return true;
        }

        for towel in towels {
            if let Some(last) = desired.strip_prefix(towel) {
                if is_possible(last.to_string(), towels, cache) {
                    cache.insert(desired.clone());
                    return true;
                }
            }
        }
        false
    }

    for desired in &desired {
        if is_possible(desired.clone(), &towels, &mut cache) {
            res += 1
        }
    }

    res.to_string()
}

/// 和part1类似，只是计数
pub fn part2(idx: usize) -> String {
    let (towels, desired) = read(idx);

    let mut cache: HashMap<String, usize> = HashMap::new();

    let mut res = 0;
    fn count(desired: String, towels: &Vec<String>, cache: &mut HashMap<String, usize>) -> usize {
        if desired.is_empty() {
            return 1;
        }
        if let Some(cnt) = cache.get(&desired) {
            return *cnt;
        }

        let mut cnt = 0;
        for towel in towels {
            if let Some(last) = desired.strip_prefix(towel) {
                cnt += count(last.to_string(), towels, cache);
            }
        }

        cache.insert(desired.clone(), cnt);
        cnt
    }

    for desired in &desired {
        res += count(desired.clone(), &towels, &mut cache);
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
        assert_eq!(part1(1), "6");
        // assert_eq!(part1(0), "242");

        assert_eq!(part2(1), "16");
        // assert_eq!(part2(0), "595975512785325");
    }
}
