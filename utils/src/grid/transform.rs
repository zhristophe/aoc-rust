use super::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridTransform {
    Rotate90,  // 顺时针 90 度
    Rotate180, // 180 度
    Rotate270, // 270 度 (逆时针 90 度)
    FlipH,     // 水平翻转 (左右)
    FlipV,     // 垂直翻转 (上下)
    Transpose, // 转置 (行列互换)
}

impl GridTransform {
    pub fn all() -> &'static [GridTransform] {
        &[
            GridTransform::Rotate90,
            GridTransform::Rotate180,
            GridTransform::Rotate270,
            GridTransform::FlipH,
            GridTransform::FlipV,
            GridTransform::Transpose,
        ]
    }

    pub fn rotations() -> &'static [GridTransform] {
        &[
            GridTransform::Rotate90,
            GridTransform::Rotate180,
            GridTransform::Rotate270,
        ]
    }

    pub fn flips() -> &'static [GridTransform] {
        &[GridTransform::FlipH, GridTransform::FlipV]
    }
}

impl<T: Clone> Grid<T> {
    pub fn transform(&self, t: GridTransform) -> Grid<T> {
        let (rows, cols) = self.size();
        let mut new_grid = match t {
            GridTransform::Rotate90 | GridTransform::Rotate270 | GridTransform::Transpose => {
                Grid::new((cols, rows), self.inner[0][0].clone())
            }
            _ => Grid::new((rows, cols), self.inner[0][0].clone()),
        };

        for i in 0..rows {
            for j in 0..cols {
                let val = self.inner[i][j].clone();
                let (ni, nj) = match t {
                    GridTransform::Rotate90 => (j, rows - 1 - i),
                    GridTransform::Rotate180 => (rows - 1 - i, cols - 1 - j),
                    GridTransform::Rotate270 => (cols - 1 - j, i),
                    GridTransform::FlipH => (i, cols - 1 - j),
                    GridTransform::FlipV => (rows - 1 - i, j),
                    GridTransform::Transpose => (j, i),
                };
                new_grid.inner[ni][nj] = val;
            }
        }
        new_grid
    }

    pub fn rotate_cw(&self) -> Grid<T> {
        self.transform(GridTransform::Rotate90)
    }

    pub fn rotate_ccw(&self) -> Grid<T> {
        self.transform(GridTransform::Rotate270)
    }

    pub fn flip_h(&self) -> Grid<T> {
        self.transform(GridTransform::FlipH)
    }

    pub fn flip_v(&self) -> Grid<T> {
        self.transform(GridTransform::FlipV)
    }

    /// 生成所有 8 种同构变换（4 个旋转 * 2 个翻转）
    pub fn all_orientations(&self) -> Vec<Grid<T>> {
        let mut res = Vec::with_capacity(8);
        let mut curr = self.clone();
        for _ in 0..4 {
            res.push(curr.clone());
            res.push(curr.flip_h());
            curr = curr.rotate_cw();
        }
        res
    }
}
