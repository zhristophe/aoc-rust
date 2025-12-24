use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::available_parallelism,
};

use rayon::prelude::*;
use utils::prelude::*;

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
                    for s in [&cur, a, b] {
                        if s.starts_with('t') {
                            res += 1;
                            break;
                        }
                    }
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
/// 并行后只需要0.09s
/// 但是提升效果不是来自并行而是来自初始值的选择
fn part2(idx: usize) -> String {
    let map = read(idx);
    let len = map.len();
    let mut pool = NamePool::new();
    let (adj, adj_cnt) = {
        let mut adj = vec![vec![false; len]; len];
        let mut adj_cnt = vec![0; len];
        for (a, b) in map {
            let (a, b) = (pool.id(a), pool.id(b));
            adj_cnt[a] += 1;
            adj_cnt[b] += 1;
            adj[a][b] = true;
            adj[b][a] = true;
        }
        (adj, adj_cnt)
    };

    fn find_max_clique(cur: usize, state: &mut LocalState, global: &GlobalState) {
        if global.end.load(Ordering::Relaxed)
            || state.max_clique.len() >= state.len_limit
            || cur >= state.max_idx
        {
            return;
        }
        if state.max_clique.len() < state.clique.len() {
            state.max_clique = state.clique.clone();
        }
        let can_add = {
            let LocalState {
                adj,
                adj_cnt,
                clique,
                ..
            } = state;
            adj_cnt[cur] >= clique.len() && clique.iter().all(|&node| adj[node][cur])
        };
        // 选择
        if can_add {
            state.clique.push(cur);
            find_max_clique(cur + 1, state, global);
            state.clique.pop();
        }
        // 不选择
        find_max_clique(cur + 1, state, global);
    }

    // 数据中所有节点的度一致，基于度的启发式搜索无效
    let &len_limit = adj_cnt.iter().max().unwrap();
    // 下面进行并行搜索
    // 并行状态
    struct GlobalState {
        end: Arc<AtomicBool>,
        max_clique: Arc<Mutex<Vec<usize>>>,
    }
    struct LocalState<'a> {
        clique: Vec<usize>,
        adj: &'a Vec<Vec<bool>>,
        adj_cnt: &'a Vec<usize>,
        max_idx: usize,
        max_clique: Vec<usize>,
        len_limit: usize,
    }
    let global_state = GlobalState {
        end: Arc::new(AtomicBool::new(false)),
        max_clique: Arc::new(Mutex::new(Vec::new())),
    };
    // 碰运气，如果正好以正确答案开始就会搜得很快
    let num_threads = available_parallelism().unwrap().get().max(32);
    (0..num_threads).into_par_iter().for_each(|id| {
        let mut local_state = LocalState {
            clique: Vec::new(),
            adj: &adj,
            adj_cnt: &adj_cnt,
            max_idx: adj_cnt.len(),
            max_clique: Vec::new(),
            len_limit,
        };
        // 每个线程搜一个子空间，起始位置为id乘以2，增加撞答案概率
        find_max_clique(id * 2, &mut local_state, &global_state);

        let mut max_clique = global_state.max_clique.lock().unwrap();
        if local_state.max_clique.len() > max_clique.len() {
            *max_clique = local_state.max_clique.clone();
        }
        if local_state.max_clique.len() >= len_limit {
            global_state.end.store(true, Ordering::Relaxed);
        }
    });
    let mut res = {
        let mut tmp = Vec::new();
        let max_clique = global_state.max_clique.lock().unwrap();
        for &i in max_clique.iter() {
            tmp.push(pool.name(i).unwrap());
        }
        tmp
    };
    res.sort();

    res.join(",")
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
        // assert_eq!(part1(0), "1238");
        assert_eq!(part1(1), "7");

        // assert_eq!(part2(0), "bg,bl,ch,fn,fv,gd,jn,kk,lk,pv,rr,tb,vw");
        assert_eq!(part2(1), "co,de,ka,ta");
    }
}
