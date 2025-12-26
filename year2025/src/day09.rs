use utils::prelude::*;

fn read(idx: usize) -> Vec<(i64, i64)> {
    let input = read_input(module_path!()).unwrap();

    let input = vec![
        &input,
        r"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
        .trim(),
    ][idx];

    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            (x, y)
        })
        .collect()
}

/// 简单枚举
pub fn part1(idx: usize) -> i64 {
    let tiles = read(idx);

    let n = tiles.len();
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            ans = ans.max(((tiles[i].0 - tiles[j].0 + 1) * (tiles[i].1 - tiles[j].1 + 1)).abs());
        }
    }

    ans
}

/// 我们把问题推广到一般情况，有如下命题：
///
/// 给定连续封闭曲线围成的图形A和凸多边形B，则：
/// B在A内部 <=> B的边界在A内部。
///
/// 证明: 必要性显然，充分性反证法易证。
pub fn part2(idx: usize) -> i64 {
    /// 按x向边过滤矩形
    fn filter_rects(tiles: &[(i64, i64)], rects: &mut HashSet<(usize, usize)>) {
        // 先获取x向边下方是不是内侧
        // 最高点所在横边的下方一定是内部
        let n = tiles.len();
        let (mut stt, mut h) = (0, 0);
        for i in 0..n {
            if tiles[i].1 > h {
                stt = i;
                h = tiles[i].1;
            }
        }
        if tiles[(stt + 1) % n].1 != h {
            stt = (stt + n - 1) % n;
        }

        // 从stt+1开始顺序遍历
        let mut cur = (stt + 1) % n;
        let mut cur_up = tiles[stt].0 < tiles[cur].0; // stt边的趋势
        let mut cur_in = true; // stt边的下方是内部
        let mut x_edges = vec![(
            tiles[stt].1,
            tiles[stt].0.min(tiles[cur].0),
            tiles[stt].0.max(tiles[cur].0),
            cur_in,
        )];
        loop {
            if cur == stt {
                break;
            }
            let nxt = (cur + 1) % n;
            let (x0, y0) = tiles[cur];
            let (x1, y1) = tiles[nxt];
            if y0 == y1 {
                // x边，计算cur_in，并记录下来
                if (x1 > x0) == cur_up {
                    cur_in = !cur_in;
                }
                cur_up = x1 > x0;

                x_edges.push((
                    y0,         // 高度
                    x0.min(x1), // 左端点
                    x0.max(x1), // 右端点
                    cur_in,     // 下方是否是内部
                ));
            } else {
                // y边，计算cur_in，代表左侧是否是内部
                if (y1 > y0) == cur_up {
                    cur_in = !cur_in;
                }
                cur_up = y1 > y0;
            }
            cur = nxt;
        }
        // 边按高度排序
        x_edges.sort_by_key(|(y, ..)| *y);

        // 缓存y向边，按x坐标分类
        let mut y_edges = HashMap::new();
        let mut i = 0;
        loop {
            let j = (i + 1) % n;
            let (x0, y0) = tiles[i];
            let (x1, y1) = tiles[j];
            if x0 == x1 {
                y_edges
                    .entry(x0)
                    .or_insert(vec![])
                    .push((y0.min(y1), y0.max(y1)));
            }
            i = j;
            if i == 0 {
                break;
            }
        }
        // y向边按高度排序
        y_edges
            .values_mut()
            .for_each(|edges| edges.sort_by_key(|(y0, ..)| *y0));

        // 判断一个y向边是否全在x边的内侧
        let is_in = |x0: i64, y0: i64, y1: i64| -> bool {
            let (mut y0, y1) = if y0 < y1 { (y0, y1) } else { (y1, y0) };

            // 分段判断该y向线段是否和y边重叠，
            // 或者是否在x向边内侧
            let mut x_edges = x_edges.iter();
            let mut y_edges = y_edges
                .get(&x0)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .peekable();

            'outer: loop {
                if y1 < y0 {
                    // 终止条件
                    return true;
                }

                // 先判断是否和y边重叠
                if let Some(&(y2, y3)) = y_edges.peek() {
                    if y0 >= y2 {
                        // 重叠：更新y0，进行下一层
                        y0 = y3 + 1;
                        y_edges.next();
                        continue 'outer;
                    }
                }

                // 没有重叠，考察x向边
                while let Some(&(y2, x1, x2, in_)) = x_edges.next() {
                    if y2 < y0 || !(x1 <= x0 && x0 <= x2) {
                        // 不是正上方的边，或者不相交的边
                        continue;
                    }
                    if y2 != y0 && !in_ {
                        // 这意味着[y0, y2)这一段不是内部
                        return false;
                    }
                    // 否则[y0, y2]这一段是内部，我们继续考察(y2, y1]这一段
                    y0 = y2 + 1;
                    continue 'outer;
                }

                // 没有找到包含y0的x边
                return false;
            }
        };

        let mut filtered = HashSet::new();
        for &(i0, i1) in rects.iter() {
            let (x0, y0) = tiles[i0];
            let (x1, y1) = tiles[i1];
            let (x0, x1) = (x0.min(x1), x0.max(x1));
            let (y0, y1) = (y0.min(y1), y0.max(y1));

            if (if x0 == x1 { vec![x0] } else { vec![x0, x1] })
                .iter()
                .all(|&x0| is_in(x0, y0, y1))
            {
                filtered.insert((i0, i1));
            }
        }

        *rects = filtered;
    }

    let tiles = read(idx);
    let n = tiles.len();

    let mut rects = HashSet::new();
    for i in 0..n {
        for j in i + 1..n {
            rects.insert((i, j));
        }
    }

    // 按x向边过滤
    filter_rects(&tiles, &mut rects);
    // 按y向边过滤
    let mut tiles = tiles;
    for tile in &mut tiles {
        *tile = (tile.1, tile.0);
    }
    filter_rects(&tiles, &mut rects);

    let mut ans = 0;
    for rect in rects {
        let (x0, y0) = tiles[rect.0];
        let (x1, y1) = tiles[rect.1];

        ans = ans.max(((x1 - x0).abs() + 1) * ((y1 - y0).abs() + 1));
    }

    ans
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
        assert_eq!(part1(0), 4758598740);
        assert_eq!(part1(1), 50);

        assert_eq!(part2(0), 1474699155);
        assert_eq!(part2(1), 24);
    }
}
