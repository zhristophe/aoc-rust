use std::collections::VecDeque;
use utils::read_input;

fn read(idx: usize) -> Vec<Vec<u8>> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"
        .trim(),
    ][idx];

    input
        .lines()
        .into_iter()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<_>>()
}

pub fn part1(idx: usize) -> String {
    let input = read(idx);
    let row = input.len();
    let col = input[0].len();
    let mut visited = vec![vec![false; col]; row];

    macro_rules! visit {
        ($x:expr, $y:expr) => {
            if $x < 0 || $x >= row as i32 || $y < 0 || $y >= col as i32 {
                true
            } else {
                let ret = visited[$x as usize][$y as usize];
                visited[$x as usize][$y as usize] = true;
                ret
            }
        };
    }

    macro_rules! is_same {
        ($x:expr, $y:expr, $v:expr) => {
            if $x < 0 || $x >= row as i32 || $y < 0 || $y >= col as i32 {
                false
            } else {
                input[$x as usize][$y as usize] == $v
            }
        };
    }

    let mut res = 0usize;
    for i in 0..row {
        for j in 0..col {
            if visited[i][j] {
                continue;
            }
            let mut area = 0usize;
            let mut perimeter = 0isize;
            let this = input[i][j];
            let (i, j) = (i as i32, j as i32);
            let mut q = VecDeque::new();
            q.push_back((i, j));
            while let Some((i, j)) = q.pop_front() {
                if visit!(i, j) {
                    continue;
                }
                // dbg!((i, j));
                let mut adj = 0;
                for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let i = i + di;
                    let j = j + dj;
                    if is_same!(i, j, this) {
                        if visited[i as usize][j as usize] {
                            adj += 1;
                        } else {
                            q.push_back((i, j));
                        }
                    }
                }
                area += 1;
                perimeter += 4 - 2 * adj as isize;
            }
            // dbg!(this, area, perimeter);
            res += area * perimeter as usize;
        }
    }

    res.to_string()
}

pub fn part2(idx: usize) -> String {
    let input = read(idx);
    let row = input.len();
    let col = input[0].len();
    let mut color = vec![vec![0; col]; row];

    // 先为区域染色，然后遍历所有行列找到所有边，乘以对应染色的面积
    let mut idx_color = 1usize;
    let mut areas = vec![0];
    for i in 0..row {
        for j in 0..col {
            if color[i][j] != 0 {
                continue;
            }
            let mut area = 0usize;
            let this = input[i as usize][j as usize];
            let (i, j) = (i as i32, j as i32);
            let mut q = VecDeque::new();
            q.push_back((i, j));
            while let Some((i, j)) = q.pop_back() {
                if i < 0 || i >= row as i32 || j < 0 || j >= col as i32 {
                    continue;
                }
                if color[i as usize][j as usize] != 0 {
                    continue;
                }
                if input[i as usize][j as usize] != this {
                    continue;
                }
                area += 1;
                color[i as usize][j as usize] = idx_color;
                for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    q.push_back((i + di, j + dj));
                }
            }
            areas.push(area);
            idx_color += 1;
        }
    }

    macro_rules! color_of {
        ($i:expr, $j:expr) => {
            if $i < 0 || $i >= row as i32 || $j < 0 || $j >= col as i32 {
                0
            } else {
                color[$i as usize][$j as usize]
            }
        };
    }

    // 染色完毕，遍历所有边
    let mut res = 0usize;
    #[derive(Debug, Clone, Copy)]
    struct Edge {
        color: usize,
        len: usize,
    }
    // let mut edge_cnt = areas.clone();
    // for cnt in &mut edge_cnt {
    //     *cnt = 0;
    // }
    for i in 0..row as i32 + 1 {
        // 这里的len仅用于标记是否是边
        let mut edges = vec![Edge { color: 0, len: 0 }; 2];
        for d in [0, 1] {
            edges[d] = Edge {
                color: color_of!(i - d as i32, 0),
                len: 1,
            }
        }
        if edges[0].color == edges[1].color {
            (edges[0].len, edges[1].len) = (0, 0);
        }

        for j in 1..col as i32 + 1 {
            let mut next_edges = edges.clone();
            for d in [0, 1] {
                next_edges[d].color = color_of!(i - d as i32, j);
            }
            for d in [0, 1] {
                if next_edges[0].color == next_edges[1].color
                    || next_edges[d].color != edges[d].color
                {
                    // 边结束
                    // res += areas[edges[d].color] * edges[d].len;
                    if edges[d].len > 0 {
                        res += areas[edges[d].color];
                        // edge_cnt[edges[d].color] += 1;
                    }
                    next_edges[d].len = if next_edges[0].color == next_edges[1].color {
                        0
                    } else {
                        1
                    };
                } else {
                    // 边继续
                    next_edges[d].len += 1;
                }
            }
            edges = next_edges;
        }
    }
    // 另一个方向，懒得写函数了，复制一遍
    for j in 0..col as i32 + 1 {
        // 这里的len仅用于标记是否是边
        let mut edges = vec![Edge { color: 0, len: 0 }; 2];
        for d in [0, 1] {
            edges[d] = Edge {
                color: color_of!(0, j - d as i32),
                len: 1,
            }
        }
        if edges[0].color == edges[1].color {
            (edges[0].len, edges[1].len) = (0, 0);
        }

        for i in 1..row as i32 + 1 {
            let mut next_edges = edges.clone();
            for d in [0, 1] {
                next_edges[d].color = color_of!(i, j - d as i32);
            }
            for d in [0, 1] {
                if next_edges[0].color == next_edges[1].color
                    || next_edges[d].color != edges[d].color
                {
                    // 边结束
                    // res += areas[edges[d].color] * edges[d].len;
                    if edges[d].len > 0 {
                        res += areas[edges[d].color];
                        // edge_cnt[edges[d].color] += 1;
                    }
                    next_edges[d].len = if next_edges[0].color == next_edges[1].color {
                        0
                    } else {
                        1
                    };
                } else {
                    // 边继续
                    next_edges[d].len += 1;
                }
            }
            edges = next_edges;
        }
    }
    // dbg!(edge_cnt);
    // dbg!(areas);

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
        assert_eq!(part1(1), "1930");
        // assert_eq!(part1(0), "1409398");

        assert_eq!(part2(1), "1206");
        // assert_eq!(part2(0), "899642");
    }
}
