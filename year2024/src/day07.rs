use utils::read_input;

fn read(idx: usize) -> Vec<(usize, Vec<usize>)> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
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
        .trim(),
    ][idx];

    input
        .lines()
        .map(|s| {
            let (p1, p2) = s.split_once(": ").unwrap();
            (
                p1.parse().unwrap(),
                p2.split(' ').map(|s| s.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn part1(idx: usize) -> String {
    let input = read(idx);
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

fn part2(idx: usize) -> String {
    let input = read(idx);
    let mut res = 0;
    for (tgt, opds) in input {
        fn concat(a: usize, b: usize) -> usize {
            (a.to_string() + &b.to_string()).parse().unwrap() // slow but easy
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

pub fn run() {
    println!("{:?}", part1(0));
    println!("{:?}", part2(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(1), "3749");
        // assert_eq!(part1(0), "129810356388");

        assert_eq!(part2(1), "11387");
        // assert_eq!(part2(0), "140575048428831");
    }
}
