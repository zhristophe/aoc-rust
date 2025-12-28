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

/// 带计数的拓扑排序，
/// 拓扑排序前应该确定可达性以确定入度
pub fn part1(idx: usize) -> i64 {
    let (names, graph) = read(idx);
    let n = names.len();
    let graph = Graph::from_map(n, graph);

    let you = names.get_id("you").unwrap();
    let out = names.get_id("out").unwrap();

    let mut cnts = vec![0; n];
    cnts[you] = 1;

    let mask = graph.bfs(you).reachable_mask();
    for u in graph.topo_sort().with_mask(&mask) {
        for &v in graph.neighbors(u) {
            cnts[v] += cnts[u];
        }
    }

    cnts[out]
}

/// 分别计算几个节点之间的路线数量即可
pub fn part2(idx: usize) -> i64 {
    let (names, graph) = read(idx);
    let n = names.len();
    let graph = Graph::from_map(n, graph);

    fn calc(graph: &Graph, stt: usize, end: usize) -> i64 {
        let mut cnts = vec![0; graph.len()];
        cnts[stt] = 1;

        let mask = graph.bfs(stt).reachable_mask();
        for u in graph.topo_sort().with_mask(&mask) {
            for &v in graph.neighbors(u) {
                cnts[v] += cnts[u];
            }
        }

        cnts[end]
    }

    let svr = names.get_id("svr").unwrap();
    let fft = names.get_id("fft").unwrap();
    let dac = names.get_id("dac").unwrap();
    let out = names.get_id("out").unwrap();

    let svr2fft = calc(&graph, svr, fft);
    let fft2dac = calc(&graph, fft, dac);
    let dac2out = calc(&graph, dac, out);
    let svr2dac = calc(&graph, svr, dac);
    let dac2fft = calc(&graph, dac, fft);
    let fft2out = calc(&graph, fft, out);

    svr2fft * fft2dac * dac2out + svr2dac * dac2fft * fft2out
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

        assert_eq!(part2(0), 458948453421420);
        assert_eq!(part2(2), 2);
    }
}
