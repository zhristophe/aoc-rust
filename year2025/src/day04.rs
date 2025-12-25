use utils::prelude::*;

fn read(idx: usize) -> Grid<u8> {
    let input = read_input(module_path!()).unwrap();

    let input = vec![
        &input,
        r"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
        .trim(),
    ][idx];

    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

/// 简单遍历
pub fn part1(idx: usize) -> i64 {
    let grid = read(idx);

    let mut ans = 0;
    for pt in grid.points() {
        if grid[pt] == b'@' {
            let mut cnt = 0;
            for pt2 in pt.adjacent() {
                if grid.get(pt2) == Some(&b'@') {
                    cnt += 1;
                }
            }
            if cnt < 4 {
                ans += 1;
            }
        }
    }

    ans
}

/// 维护邻接状态
pub fn part2(idx: usize) -> i64 {
    let grid = read(idx);
    let mut adjv = Grid::new(grid.size(), 0);

    for pt in grid.points() {
        if grid[pt] == b'@' {
            for pt2 in pt.adjacent() {
                if grid.get(pt2) == Some(&b'@') {
                    adjv[pt] += 1;
                }
            }
        }
    }

    let mut q = VecDeque::new();
    for pt in grid.points() {
        if grid[pt] == b'@' && adjv[pt] < 4 {
            q.push_back(pt);
        }
    }

    let mut ans = 0;
    while let Some(pt) = q.pop_front() {
        ans += 1;
        for pt2 in pt.adjacent() {
            if grid.get(pt2) == Some(&b'@') && adjv[pt2] >= 4 {
                adjv[pt2] -= 1;
                if adjv[pt2] < 4 {
                    q.push_back(pt2);
                }
            }
        }
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
        assert_eq!(part1(0), 1464);
        assert_eq!(part1(1), 13);

        assert_eq!(part2(0), 8409);
        assert_eq!(part2(1), 43);
    }
}
