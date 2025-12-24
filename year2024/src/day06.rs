use utils::{read_input, Direction, Point};

fn read(idx: usize) -> Vec<Vec<char>> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
        .trim(),
    ][idx];

    input.lines().map(|s| s.chars().collect()).collect()
}

fn part1(idx: usize) -> String {
    let input = read(idx);
    let mut cur = 'start: loop {
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j] == '^' {
                    break 'start (i, j);
                }
            }
        }
        unreachable!();
    };
    cur = (cur.0 + 1, cur.1 + 1);

    macro_rules! step {
        ($cur:expr, $cur_dir:expr) => {
            (
                ($cur.0 as isize + $cur_dir.0) as usize,
                ($cur.1 as isize + $cur_dir.1) as usize,
            )
        };
    }

    let dir = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    // let mut stop = vec![vec![false; input[0].len()]; input.len()];
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut cur_dir = 0;
    'outer: loop {
        loop {
            let mut next = step!(cur, dir[cur_dir]);
            if next.0 <= 0 || next.0 > input.len() || next.1 <= 0 || next.1 > input[0].len() {
                visited[cur.0 - 1][cur.1 - 1] = true;
                break 'outer;
            }
            if input[next.0 - 1][next.1 - 1] == '#' {
                if visited[cur.0 - 1][cur.1 - 1] {
                    break 'outer;
                } else {
                    next = cur;
                    cur_dir = (cur_dir + 1) % 4;
                }
            }
            visited[cur.0 - 1][cur.1 - 1] = true;
            cur = next;
        }
    }

    let mut res = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if visited[i][j] {
                // print!("X");
                res += 1;
            } else {
                // print!(".");
            }
        }
        // println!();
    }

    res.to_string()
}

fn part2(idx: usize) -> String {
    let input = read(idx);
    let start = 'start: loop {
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j] == '^' {
                    break 'start (i, j);
                }
            }
        }
        unreachable!();
    };
    let start = Point::from(start);

    let mut res = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if (i, j) == (start.i as usize, start.j as usize) {
                continue;
            }
            if input[i][j] == '#' {
                continue;
            }

            let input = {
                let mut input = input.clone();
                input[i][j] = '#';
                input
            };
            let mut cur = start;
            let mut dir = Direction::Up;
            // let mut visited = vec![vec![false; input[0].len()]; input.len()];
            let mut stop = vec![vec![false; input[0].len()]; input.len()];

            'search: loop {
                let mut next = cur.move_to(dir);
                match next.get(&input) {
                    Some('#') => {
                        if Some(&true) == cur.get(&stop) {
                            res += 1;
                            // dbg!(i, j);
                            break 'search;
                        }
                        cur.get_mut(&mut stop).map(|v| *v = true);
                        loop {
                            dir = dir.turn_right();
                            next = cur.move_to(dir);
                            match next.get(&input) {
                                Some('#') => {
                                    continue;
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                    }
                    Some(_) => {}
                    None => {
                        // dbg!(i, j);
                        break 'search;
                    }
                }
                // visited[cur.i as usize][cur.j as usize] = true;
                cur = next;
            }
        }
    }

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
        assert_eq!(part1(1), "41");
        // assert_eq!(part1(0), "5177");

        assert_eq!(part2(1), "6");
        // assert_eq!(part2(0), "1917");
    }
}
