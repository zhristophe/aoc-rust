use utils::prelude::*;

fn read(idx: usize) -> (NamePool, HashMap<usize, Vec<usize>>) {
    let input = read_input(module_path!()).unwrap();

    let input = vec![
        &input,
        r"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"
        .trim(),
        r"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"
        .trim(),
    ][idx];

    let mut pool = NamePool::new();
    let mut graph = HashMap::new();

    pool.id("out");
    pool.id("you");

    for input in input.lines() {
        let mut parts = input.split(' ');
        let from = parts.next().unwrap();
        let from = pool.id(&from[..from.len() - 1]);
        for to in parts {
            let to = pool.id(to);
            graph.entry(from).or_insert_with(Vec::new).push(to);
        }
    }

    (pool, graph)
}

/// 无环路，因此宽搜 + 递推
pub fn part1(idx: usize) -> i64 {
    let (_, graph) = read(idx);

    let mut q = VecDeque::new();
    let mut visited = HashMap::new();
    q.push_back(1); // you
    visited.insert(1, 1); // you只有一种路线

    while let Some(device) = q.pop_front() {
        let cnt = visited.entry(device).or_insert(0).clone();
        if let Some(nexts) = graph.get(&device) {
            for next in nexts {
                use std::collections::hash_map::Entry::*;
                match visited.entry(*next) {
                    Occupied(mut entry) => {
                        *entry.get_mut() += cnt;
                    }
                    Vacant(entry) => {
                        entry.insert(cnt);
                        q.push_back(*next);
                    }
                }
            }
        }
    }

    visited.get(&0).unwrap().clone()
}

/// 状态空间太大，无法像part1那样记忆化宽搜。
///
/// 我们将问题建模为不定方程组 + 最小化，
/// 是一个整数线性规划问题。
/// 我们甚至可以调用一个LP或者OMT库来解决。
pub fn part2(idx: usize) -> i64 {
    todo!()
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
        assert_eq!(part1(0), 649);
        assert_eq!(part1(1), 5);

        // assert_eq!(part2(0), 15017);
        assert_eq!(part2(2), 2);
    }
}
