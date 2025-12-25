use utils::prelude::*;

fn read(idx: usize) -> Grid<u8> {
    let input = read_input(module_path!()).unwrap();

    let input = vec![
        &input,
        r"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
        .trim(),
    ][idx];

    input.into()
}

/// 简单模拟
pub fn part1(idx: usize) -> i64 {
    let mut grid = read(idx);

    let stt = grid.find_point(b'S').unwrap();
    let mut cur = vec![stt];

    let mut ans = 0;
    loop {
        let mut next = vec![];
        for pt in cur {
            let pt2 = pt.move_down();
            match grid.get(pt2) {
                Some(b'.') => {
                    next.push(pt2);
                    grid.set(pt2, b'|');
                }
                Some(b'^') => {
                    ans += 1;
                    for pt2 in [pt2.move_left(), pt2.move_right()] {
                        if grid.get(pt2) == Some(&b'.') {
                            next.push(pt2);
                            grid.set(pt2, b'|');
                        }
                    }
                }
                _ => (),
            }
        }

        if next.is_empty() {
            break;
        }

        cur = next;
    }

    ans
}

/// part1的基础上进行计数
pub fn part2(idx: usize) -> i64 {
    let grid = read(idx);

    let stt = grid.find_point(b'S').unwrap();
    let mut cur = vec![(stt, 1)];

    loop {
        let mut next = std::collections::HashMap::new();
        for (pt, cnt) in cur.clone() {
            let pt2 = pt.move_down();
            match grid.get(pt2) {
                Some(b'.') => {
                    next.entry(pt2).and_modify(|v| *v += cnt).or_insert(cnt);
                }
                Some(b'^') => {
                    for pt2 in [pt2.move_left(), pt2.move_right()] {
                        if grid.get(pt2) == Some(&b'.') {
                            next.entry(pt2).and_modify(|v| *v += cnt).or_insert(cnt);
                        }
                    }
                }
                _ => (),
            }
        }

        if next.is_empty() {
            return cur.into_iter().map(|(_, cnt)| cnt).sum();
        }

        cur = next.into_iter().collect();
    }
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
        assert_eq!(part1(0), 1613);
        assert_eq!(part1(1), 21);

        assert_eq!(part2(0), 48021610271997);
        assert_eq!(part2(1), 40);
    }
}
