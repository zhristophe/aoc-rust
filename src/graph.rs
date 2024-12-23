// use std::collections::HashMap;

// #[derive(Debug)]
// pub struct Graph<T> {
//     nodes: HashMap<NodeId, T>,
//     edges: HashMap<NodeId, Vec<Edge>>,
// }

// impl<T> Graph<T> {
//     // 创建
//     pub fn new() -> Self {
//         Graph {
//             nodes: HashMap::new(),
//             edges: HashMap::new(),
//         }
//     }

//     pub fn with_capacity(capacity: usize) -> Self {
//         Graph {
//             nodes: HashMap::with_capacity(capacity),
//             edges: HashMap::with_capacity(capacity),
//         }
//     }

//     // 节点操作
//     pub fn add_node(&mut self, id: NodeId, value: T) -> Option<T> {
//         self.nodes.insert(id, value)
//     }

//     pub fn remove_node(&mut self, id: &NodeId) -> Option<T> {
//         // 需要同时删除相关的边
//         self.edges.remove(id);
//         // 从其他节点的边列表中删除到此节点的边
//         for edges in self.edges.values_mut() {
//             edges.retain(|edge| edge.to != *id);
//         }
//         self.nodes.remove(id)
//     }

//     pub fn get_node(&self, id: &NodeId) -> Option<&T> {
//         self.nodes.get(id)
//     }

//     pub fn get_node_mut(&mut self, id: &NodeId) -> Option<&mut T> {
//         self.nodes.get_mut(id)
//     }

//     // 边操作
//     pub fn add_edge(&mut self, from: NodeId, to: NodeId, weight: isize) -> bool {
//         if !self.nodes.contains_key(&from) || !self.nodes.contains_key(&to) {
//             return false;
//         }
//         self.edges
//             .entry(from)
//             .or_insert_with(Vec::new)
//             .push(Edge { to, weight });
//         true
//     }

//     pub fn remove_edge(&mut self, from: &NodeId, to: &NodeId) -> bool {
//         if let Some(edges) = self.edges.get_mut(from) {
//             let len = edges.len();
//             edges.retain(|edge| edge.to != *to);
//             edges.len() != len
//         } else {
//             false
//         }
//     }

//     // 遍历
//     pub fn neighbors(&self, id: &NodeId) -> impl Iterator<Item = &Edge> {
//         self.edges.get(id).into_iter().flatten()
//     }

//     pub fn nodes(&self) -> impl Iterator<Item = (&NodeId, &T)> {
//         self.nodes.iter()
//     }

//     // 图的属性
//     pub fn node_count(&self) -> usize {
//         self.nodes.len()
//     }

//     pub fn edge_count(&self) -> usize {
//         self.edges.values().map(|v| v.len()).sum()
//     }

//     // 搜索相关
//     pub fn bfs_iter(&self, start: NodeId) -> BfsIter<T> {
//         // 类似Grid的BFS实现
//         todo!()
//     }

//     pub fn dfs_iter(&self, start: NodeId) -> DfsIter<T> {
//         todo!()
//     }

//     // 可选：路径查找
//     pub fn shortest_path(&self, from: NodeId, to: NodeId) -> Option<Vec<NodeId>> {
//         todo!()
//     }
// }
