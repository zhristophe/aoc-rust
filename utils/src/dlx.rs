struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,

    row: usize,
    col: usize,
}

impl Node {
    fn new(row: usize, col: usize) -> Self {
        Self {
            left: 0,
            right: 0,
            up: 0,
            down: 0,
            row,
            col,
        }
    }
}

pub struct DlxSolver {
    nodes: Vec<Node>,
    col_sizes: Vec<usize>,
    solution: Vec<usize>,
    row_count: usize,

    // 资源约束
    row_groups: Vec<Option<usize>>, // 每行的资源组 id
    group_limits: Vec<usize>,       // 每个资源组的使用上限
    group_counts: Vec<usize>,       // 每个资源组当前使用量
}

impl DlxSolver {
    pub fn new(n_cols: usize) -> Self {
        let mut nodes = Vec::with_capacity(2000);

        // 根节点 (id 0)
        nodes.push(Node::new(0, 0));

        // 创建列头 (id 1 to n_cols)
        for i in 1..=n_cols {
            let mut node = Node::new(0, i);
            node.up = i;
            node.down = i;
            node.left = i - 1;
            node.right = if i == n_cols { 0 } else { i + 1 };

            nodes.push(node);
        }
        nodes[0].left = n_cols;
        nodes[0].right = 1;

        Self {
            nodes,
            col_sizes: vec![0; n_cols + 1],
            solution: Vec::new(),
            row_count: 0,
            row_groups: vec![None],
            group_limits: Vec::new(),
            group_counts: Vec::new(),
        }
    }

    /// 动态添加列
    pub fn add_columns(&mut self, n: usize) {
        let old_cols = self.col_sizes.len() - 1;
        let new_cols = old_cols + n;

        self.col_sizes.resize(new_cols + 1, 0);

        // 添加新列头
        for i in (old_cols + 1)..=new_cols {
            let mut node = Node::new(0, i);
            node.up = i;
            node.down = i;
            node.left = i - 1;
            node.right = 0; // 暂时设置为 0，稍后修复
            self.nodes.push(node);
        }

        // 旧的最后一个列（或根）应该指向第一个新列
        let old_last_idx = if old_cols == 0 { 0 } else { old_cols };
        self.nodes[old_last_idx].right = old_cols + 1;

        // 新列头互相连接
        for i in (old_cols + 1)..new_cols {
            self.nodes[i].right = i + 1;
        }

        // 最后一个新列应该指向根
        self.nodes[new_cols].right = 0;
        self.nodes[0].left = new_cols;
    }

    /// 设置资源组的使用上限
    pub fn set_group_limit(&mut self, group_id: usize, limit: usize) {
        if group_id >= self.group_limits.len() {
            self.group_limits.resize(group_id + 1, usize::MAX);
            self.group_counts.resize(group_id + 1, 0);
        }
        self.group_limits[group_id] = limit;
    }

    /// 添加一行，`cols` 是该行包含 1 的列号（1-indexed）
    pub fn add_row(&mut self, cols: &[usize], group: Option<usize>) {
        self.row_count += 1;
        let row_id = self.row_count;
        self.row_groups.push(group);

        let mut first_node = 0;

        for &col in cols {
            let cur_node = self.nodes.len();
            let mut node = Node::new(row_id, col);

            // 插入到列的底部
            let header_node = col;
            let bottom_node = self.nodes[header_node].up;
            self.nodes[bottom_node].down = cur_node;
            node.up = bottom_node;
            self.nodes[header_node].up = cur_node;
            node.down = header_node;

            self.col_sizes[col] += 1;

            // 横向插入
            if first_node == 0 {
                first_node = cur_node;
                node.left = cur_node;
                node.right = cur_node;
            } else {
                let last_node = self.nodes[first_node].left;
                self.nodes[last_node].right = cur_node;
                node.left = last_node;
                self.nodes[first_node].left = cur_node;
                node.right = first_node;
            }

            self.nodes.push(node);
        }
    }

    fn cover(&mut self, col: usize) {
        let right_col = self.nodes[col].right;
        let left_col = self.nodes[col].left;
        self.nodes[left_col].right = right_col;
        self.nodes[right_col].left = left_col;

        let mut i = self.nodes[col].down;
        while i != col {
            let mut j = self.nodes[i].right;
            while j != i {
                let up_node = self.nodes[j].up;
                let down_node = self.nodes[j].down;
                self.nodes[up_node].down = down_node;
                self.nodes[down_node].up = up_node;
                self.col_sizes[self.nodes[j].col] -= 1;
                j = self.nodes[j].right;
            }
            i = self.nodes[i].down;
        }
    }

    fn uncover(&mut self, col: usize) {
        let mut i = self.nodes[col].up;
        while i != col {
            let mut j = self.nodes[i].left;
            while j != i {
                let up_node = self.nodes[j].up;
                let down_node = self.nodes[j].down;
                self.nodes[up_node].down = j;
                self.nodes[down_node].up = j;
                self.col_sizes[self.nodes[j].col] += 1;
                j = self.nodes[j].left;
            }
            i = self.nodes[i].up;
        }

        let right_col = self.nodes[col].right;
        let left_col = self.nodes[col].left;
        self.nodes[left_col].right = col;
        self.nodes[right_col].left = col;
    }

    fn select_column(&self) -> usize {
        let mut min_len = usize::MAX;
        let mut min_col = 0;

        let mut col = self.nodes[0].right;
        while col != 0 {
            if self.col_sizes[col] < min_len {
                min_len = self.col_sizes[col];
                min_col = col;
                if min_len <= 1 {
                    break;
                }
            }
            col = self.nodes[col].right;
        }

        min_col
    }

    fn search(&mut self) -> bool {
        if self.nodes[0].right == 0 {
            return true;
        }

        let col = self.select_column();
        if self.col_sizes[col] == 0 {
            return false;
        }

        self.cover(col);

        let mut i = self.nodes[col].down;
        while i != col {
            let row_id = self.nodes[i].row;

            // Resource Check
            let mut group_ok = true;
            if let Some(group_id) = self.row_groups[row_id] {
                if self.group_counts[group_id] >= self.group_limits[group_id] {
                    group_ok = false;
                } else {
                    self.group_counts[group_id] += 1;
                }
            }

            if group_ok {
                self.solution.push(row_id);

                let mut j = self.nodes[i].right;
                while j != i {
                    self.cover(self.nodes[j].col);
                    j = self.nodes[j].right;
                }

                if self.search() {
                    return true;
                }

                self.solution.pop();
                let mut j = self.nodes[i].left;
                while j != i {
                    self.uncover(self.nodes[j].col);
                    j = self.nodes[j].left;
                }

                // Resource Backtrack
                if let Some(group_id) = self.row_groups[row_id] {
                    self.group_counts[group_id] -= 1;
                }
            }

            i = self.nodes[i].down;
        }

        self.uncover(col);

        false
    }

    /// 求解，返回解中包含的行号（1-indexed）
    pub fn solve(&mut self) -> Option<Vec<usize>> {
        println!("rows: {}", self.row_count);
        println!("cols: {}", self.col_sizes.len());

        self.solution.clear();
        self.group_counts.fill(0); // Reset resource usage
        if self.search() {
            Some(self.solution.clone())
        } else {
            None
        }
    }
}

pub struct DlxBuilder {
    solver: DlxSolver,
    next_col: usize,
    next_group: usize,
}

impl DlxBuilder {
    pub fn new() -> Self {
        Self {
            solver: DlxSolver::new(0),
            next_col: 1,
            next_group: 0,
        }
    }

    /// 添加 `n` 个精确覆盖约束。返回新添加列的索引。
    pub fn add_constraints(&mut self, n: usize) -> Vec<usize> {
        self.solver.add_columns(n);
        let start = self.next_col;
        self.next_col += n;
        (start..self.next_col).collect()
    }

    /// 添加一个资源约束。返回资源组 ID。
    pub fn add_resource(&mut self, limit: usize) -> usize {
        let id = self.next_group;
        self.solver.set_group_limit(id, limit);
        self.next_group += 1;
        id
    }

    pub fn add_choice(&mut self, cols: &[usize], resource: Option<usize>) {
        self.solver.add_row(cols, resource);
    }

    pub fn solve(&mut self) -> Option<Vec<usize>> {
        self.solver.solve()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut builder = DlxBuilder::new();
        let cols = builder.add_constraints(7);
        assert_eq!(cols, vec![1, 2, 3, 4, 5, 6, 7]);

        builder.add_choice(&[1, 4, 7], None);
        builder.add_choice(&[1, 4], None);
        builder.add_choice(&[4, 5, 7], None);
        builder.add_choice(&[3, 5, 6], None);
        builder.add_choice(&[2, 3, 6, 7], None);
        builder.add_choice(&[2, 7], None);

        // Expected solution: rows 1, 4, 5 (indices 1, 4, 5) or 2, 4, 6
        // Row 1: 1,4,7
        // Row 4: 3,5,6
        // Row 5: 2,3,6,7 -> Conflict with Row 4 on 3,6
        // Wait, manual check:
        // Sol 1: Row 2 (1,4) + Row 6 (2,7) + Row 4 (3,5,6) -> Covers 1,2,3,4,5,6,7. YES.

        let sol = builder.solve().expect("Should have solution");
        assert_eq!(sol.len(), 3);
    }

    #[test]
    fn test_sudoku() {
        let puzzle = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        let mut builder = DlxBuilder::new();
        let col_cell = builder.add_constraints(81);
        let col_row = builder.add_constraints(81);
        let col_col = builder.add_constraints(81);
        let col_box = builder.add_constraints(81);

        let mut row_info = Vec::new();

        for r in 0..9 {
            for c in 0..9 {
                let b = (r / 3) * 3 + c / 3;
                let digits = if puzzle[r][c] != 0 {
                    vec![puzzle[r][c]]
                } else {
                    (1..=9).collect()
                };

                for d in digits {
                    // d is 1-9, adjust to 0-8 for indexing
                    let d_idx = d as usize - 1;

                    builder.add_choice(
                        &[
                            col_cell[r * 9 + c],
                            col_row[r * 9 + d_idx],
                            col_col[c * 9 + d_idx],
                            col_box[b * 9 + d_idx],
                        ],
                        None,
                    );

                    row_info.push((r, c, d));
                }
            }
        }

        let sol = builder.solve().expect("Sudoku should have solution");

        let mut result = [[0; 9]; 9];
        for &row_id in &sol {
            let (r, c, d) = row_info[row_id - 1];
            result[r][c] = d;
        }

        let expected = [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_resources() {
        // Test resource constraints
        // Constraints: 1, 2 (must cover exactly once)
        // Resources: A (limit 1)
        // Choices:
        // 1. Cover 1, use A
        // 2. Cover 2, use A
        // 3. Cover 1, no resource
        // 4. Cover 2, no resource

        // Scenario 1: Only A available (limit 1). Can we cover 1 and 2?
        // Rows:
        // 1. [1], use A
        // 2. [2], use A
        // Should FAIL since we need 2 A's but only have 1.

        let mut b = DlxBuilder::new();
        let cols = b.add_constraints(2);
        let res_a = b.add_resource(1);

        b.add_choice(&[cols[0]], Some(res_a));
        b.add_choice(&[cols[1]], Some(res_a));

        assert!(b.solve().is_none());

        // Scenario 2: Increase limit to 2
        let mut b = DlxBuilder::new();
        let cols = b.add_constraints(2);
        let res_a = b.add_resource(2);
        b.add_choice(&[cols[0]], Some(res_a));
        b.add_choice(&[cols[1]], Some(res_a));
        assert!(b.solve().is_some());
    }
}
