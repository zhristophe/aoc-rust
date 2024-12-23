use aoc::prelude::*;

fn read(idx: usize) -> Vec<(String, String)> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
",
    ][idx]
        .trim();

    {
        input
            .lines()
            .map(|s| {
                let tmp = s.split_once('-').unwrap();
                (tmp.0.to_string(), tmp.1.to_string())
            })
            .collect()
    }
}

/// 简单遍历
fn part1(idx: usize) -> String {
    let map = read(idx);
    // let mut set = HashSet::new();
    let map = {
        let mut tmp = HashMap::new();
        for (a, b) in map {
            tmp.entry(a.clone())
                .or_insert_with(Vec::new)
                .push(b.clone());
            tmp.entry(b).or_insert_with(Vec::new).push(a);
        }
        tmp
    };

    // 目前的库里没有邻接表的遍历
    let mut visited = HashSet::new();
    let mut discovered = HashSet::new();
    let stt = map.iter().next().unwrap().0.clone();
    let mut queue = VecDeque::new();
    queue.push_back(stt);
    let mut res = 0;
    while let Some(cur) = queue.pop_front() {
        visited.insert(cur.clone());

        let neighbors = map.get(&cur).unwrap();
        let len = neighbors.len();
        for i in 0..len {
            let a = &neighbors[i];
            if visited.contains(a) {
                continue;
            }
            for j in i + 1..len {
                let b = &neighbors[j];
                if visited.contains(b) {
                    continue;
                }
                if map.get(a).unwrap().contains(b) {
                    // dbg!((&cur, a, b));
                    for s in [&cur, a, b] {
                        if s.starts_with('t') {
                            res += 1;
                            break;
                        }
                    }
                    // res += 1;
                }
            }
            if discovered.contains(a) || visited.contains(a) {
                continue;
            }
            discovered.insert(a.clone());
            queue.push_back(a.clone());
        }
    }

    res.to_string()
}

/// 最大团？？？
/// 最简单的回溯，大约要算40s
fn part2(idx: usize) -> String {
    let map = read(idx);
    // let mut set = HashSet::new();
    let map = {
        let mut tmp = HashMap::new();
        for (a, b) in map {
            tmp.entry(a.clone())
                .or_insert_with(Vec::new)
                .push(b.clone());
            tmp.entry(b).or_insert_with(Vec::new).push(a);
        }
        tmp
    };

    fn find_max_clique(
        map: &HashMap<String, Vec<String>>,
        nodes: &Vec<&String>,
        idx: usize,
        clique: Vec<String>,
    ) -> Vec<String> {
        if idx >= nodes.len() {
            return clique;
        }

        let cur = nodes[idx];
        let is_clique = 'is_clique: loop {
            for node in &clique {
                if !map.get(node).unwrap().contains(cur) {
                    break 'is_clique false;
                }
            }
            break 'is_clique true;
        };

        let mut res = Vec::new();

        if is_clique {
            let mut new_clique = clique.clone();
            new_clique.push(cur.clone());
            res.push(find_max_clique(map, nodes, idx + 1, new_clique));
        }

        res.push(find_max_clique(map, nodes, idx + 1, clique));

        if res.len() == 1 {
            res[0].clone()
        } else {
            if res[0].len() > res[1].len() {
                res[0].clone()
            } else {
                res[1].clone()
            }
        }
    }

    let nodes = map.keys().collect::<Vec<_>>();
    let mut res = find_max_clique(&map, &nodes, 0, vec![]);
    res.sort();

    res.join(",")
}

fn main() {
    println!("{:?}", part1(0));
    println!("{:?}", part2(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(0), "1238");
        assert_eq!(part1(1), "7");

        assert_eq!(part2(0), "bg,bl,ch,fn,fv,gd,jn,kk,lk,pv,rr,tb,vw");
        assert_eq!(part2(1), "co,de,ka,ta");
    }
}
