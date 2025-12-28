/// 并查集 (Disjoint Set Union)
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    #[inline]
    pub fn root(&mut self, x: usize) -> usize {
        self.find(x)
    }

    /// 查找根节点，带路径压缩
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// 合并两个集合，返回是否成功合并（false 表示已在同一集合）
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return false;
        }
        // 按秩合并
        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => self.parent[px] = py,
            std::cmp::Ordering::Greater => self.parent[py] = px,
            std::cmp::Ordering::Equal => {
                self.parent[py] = px;
                self.rank[px] += 1;
            }
        }
        true
    }

    /// 判断两个元素是否在同一集合
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(5);
        assert!(!uf.connected(0, 4));
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(1, 3);
        assert!(uf.connected(0, 2));
        assert!(!uf.connected(0, 4));
    }
}
