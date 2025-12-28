use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Graph {
    /// 邻接表：adj[u] 包含所有满足 u -> v 的 v
    adj: Vec<Vec<usize>>,
    /// 反向邻接表：rev[v] 包含所有满足 u -> v 的 u
    rev: Vec<Vec<usize>>,
    /// 节点数量
    n: usize,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            adj: vec![Vec::new(); n],
            rev: vec![Vec::new(); n],
            n,
        }
    }

    pub fn from_map(n: usize, map: std::collections::HashMap<usize, Vec<usize>>) -> Self {
        let mut graph = Graph::new(n);
        for (from, tos) in map {
            for to in tos {
                graph.add_edge(from, to);
            }
        }
        graph
    }

    /// 添加从 `from` 到 `to` 的有向边
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.adj[from].push(to);
        self.rev[to].push(from);
    }

    /// 添加 `u` 和 `v` 之间的无向边（两条有向边）
    pub fn add_bi_edge(&mut self, u: usize, v: usize) {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn neighbors(&self, u: usize) -> &[usize] {
        &self.adj[u]
    }

    pub fn bfs<'a>(&'a self, start: usize) -> BfsIter<'a> {
        BfsIter::new(self, start)
    }

    pub fn dfs<'a>(&'a self, start: usize) -> DfsIter<'a> {
        DfsIter::new(self, start)
    }

    /// 开始构建拓扑排序
    pub fn topo_sort<'a>(&'a self) -> TopoSort<'a> {
        TopoSort::new(self)
    }

    /// 辅助函数：对整个图进行标准拓扑排序。
    pub fn topo_sort_full(&self) -> Vec<usize> {
        self.topo_sort().into_iter().collect()
    }

    /// 辅助函数：对由 `mask` 定义的子图进行拓扑排序。
    pub fn topo_sort_with_mask(&self, mask: &[bool]) -> Vec<usize> {
        self.topo_sort().with_mask(mask).into_iter().collect()
    }
}

// --- Topological Sort Builder ---

pub struct TopoSort<'a> {
    graph: &'a Graph,
    mask: Option<&'a [bool]>,
}

impl<'a> TopoSort<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph, mask: None }
    }

    pub fn with_mask(mut self, mask: &'a [bool]) -> Self {
        self.mask = Some(mask);
        self
    }
}

impl<'a> IntoIterator for TopoSort<'a> {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        let n = self.graph.n;
        let mut in_degree = vec![0; n];
        let mut nodes = Vec::new();

        // Calculate in-degrees
        if let Some(mask) = self.mask {
            for u in 0..n {
                if !mask[u] {
                    continue;
                }
                nodes.push(u);
                for &v in &self.graph.rev[u] {
                    if mask[v] {
                        in_degree[u] += 1;
                    }
                }
            }
        } else {
            for u in 0..n {
                nodes.push(u);
                in_degree[u] = self.graph.rev[u].len();
            }
        }

        let mut queue = VecDeque::new();
        for &u in &nodes {
            if in_degree[u] == 0 {
                queue.push_back(u);
            }
        }

        let mut result = Vec::new();
        while let Some(u) = queue.pop_front() {
            result.push(u);

            for &v in &self.graph.adj[u] {
                // If mask is present, only consider edges to masked nodes
                if let Some(mask) = self.mask {
                    if !mask[v] {
                        continue;
                    }
                }

                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        result.into_iter()
    }
}

// --- BFS Iterator ---

pub struct BfsIter<'a> {
    graph: &'a Graph,
    queue: VecDeque<usize>,
    visited: Vec<bool>,

    // Optional filters/hooks
    visit_filter: Option<Box<dyn Fn(usize) -> bool + 'a>>,
    on_discover: Option<Box<dyn FnMut(usize, usize) + 'a>>, // (from, to)
    on_visit: Option<Box<dyn FnMut(usize) + 'a>>,
}

impl<'a> BfsIter<'a> {
    fn new(graph: &'a Graph, start: usize) -> Self {
        let mut visited = vec![false; graph.n];
        visited[start] = true;

        BfsIter {
            graph,
            queue: VecDeque::from([start]),
            visited,
            visit_filter: None,
            on_discover: None,
            on_visit: None,
        }
    }

    pub fn with_filter<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> bool + 'a,
    {
        self.visit_filter = Some(Box::new(f));
        self
    }

    pub fn on_discover<F>(mut self, f: F) -> Self
    where
        F: FnMut(usize, usize) + 'a,
    {
        self.on_discover = Some(Box::new(f));
        self
    }

    pub fn on_visit<F>(mut self, f: F) -> Self
    where
        F: FnMut(usize) + 'a,
    {
        self.on_visit = Some(Box::new(f));
        self
    }

    /// 消耗迭代器并返回所有可达节点的掩码
    pub fn reachable_mask(self) -> Vec<bool> {
        let mut mask = vec![false; self.graph.n];
        for u in self {
            mask[u] = true;
        }
        mask
    }
}

impl<'a> Iterator for BfsIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let u = self.queue.pop_front()?;

        if let Some(ref mut f) = self.on_visit {
            f(u);
        }

        for &v in &self.graph.adj[u] {
            if self.visited[v] {
                continue;
            }

            if let Some(ref filter) = self.visit_filter {
                if !filter(v) {
                    continue;
                }
            }

            self.visited[v] = true;
            if let Some(ref mut f) = self.on_discover {
                f(u, v);
            }
            self.queue.push_back(v);
        }

        Some(u)
    }
}

// --- DFS Iterator ---

pub struct DfsIter<'a> {
    graph: &'a Graph,
    stack: Vec<usize>,
    visited: Vec<bool>,

    visit_filter: Option<Box<dyn Fn(usize) -> bool + 'a>>,
    on_visit: Option<Box<dyn FnMut(usize) + 'a>>,
}

impl<'a> DfsIter<'a> {
    fn new(graph: &'a Graph, start: usize) -> Self {
        let mut visited = vec![false; graph.n];
        visited[start] = true;

        DfsIter {
            graph,
            stack: vec![start],
            visited,
            visit_filter: None,
            on_visit: None,
        }
    }

    pub fn with_filter<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> bool + 'a,
    {
        self.visit_filter = Some(Box::new(f));
        self
    }

    pub fn on_visit<F>(mut self, f: F) -> Self
    where
        F: FnMut(usize) + 'a,
    {
        self.on_visit = Some(Box::new(f));
        self
    }
}

impl<'a> Iterator for DfsIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let u = self.stack.pop()?;

        if let Some(ref mut f) = self.on_visit {
            f(u);
        }

        for &v in self.graph.adj[u].iter().rev() {
            if self.visited[v] {
                continue;
            }

            if let Some(ref filter) = self.visit_filter {
                if !filter(v) {
                    continue;
                }
            }

            self.visited[v] = true;
            self.stack.push(v);
        }

        Some(u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 4);

        let order: Vec<_> = g.bfs(0).collect();
        assert_eq!(order[0], 0);
        assert!(order.contains(&1));
        assert!(order.contains(&2));
        assert!(order.contains(&3));
        assert!(order.contains(&4));
    }

    #[test]
    fn test_dfs() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 4);

        let order: Vec<_> = g.dfs(0).collect();
        assert_eq!(order[0], 0);
    }

    #[test]
    fn test_topo_sort() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);

        let order: Vec<_> = g.topo_sort().into_iter().collect();
        assert_eq!(order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_topo_sort_subgraph() {
        let mut g = Graph::new(6);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 4);
        g.add_edge(4, 5);

        let mut mask = vec![false; 6];
        mask[3] = true;
        mask[4] = true;
        mask[5] = true;

        let order: Vec<_> = g.topo_sort().with_mask(&mask).into_iter().collect();
        assert_eq!(order, vec![3, 4, 5]);
    }

    #[test]
    fn test_bfs_reachable_mask() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        // 0 -> 1 -> 2
        // 3 -> 4 (disconnected)
        g.add_edge(3, 4);

        let mask = g.bfs(0).reachable_mask();
        assert!(mask[0]);
        assert!(mask[1]);
        assert!(mask[2]);
        assert!(!mask[3]);
        assert!(!mask[4]);
    }
}
