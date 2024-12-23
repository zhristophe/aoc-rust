use std::{fs, path::Path};

use aoc::{Direction, Grid};
// use crossterm::{
//     cursor,
//     event::{read, Event, KeyCode, KeyEvent},
//     execute,
//     terminal::{self, Clear, ClearType},
//     Result,
// };

#[allow(dead_code)]
fn exec1(input: (Vec<Vec<char>>, Vec<char>)) -> String {
    let (mut map, seq) = (Grid::from(input.0), input.1);

    let mut rbt = map.find_point('@').unwrap();
    map.set(rbt, '.');

    for dir in seq {
        // println!("{:?}", dir);
        // map.display_by(|c| c.to_string());
        // println!();
        let dir = match dir {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => continue,
        };
        let next = rbt.move_to(dir);
        match map.get(next) {
            Some('#') => continue,
            Some('.') => {
                rbt = next;
                continue;
            }
            Some('O') => {
                let able = {
                    let mut next = next;
                    loop {
                        next = next.move_to(dir);
                        match map.get(next) {
                            Some('.') => break true,
                            Some('#') => break false,
                            Some('O') => continue,
                            _ => unreachable!(),
                        }
                    }
                };

                if able {
                    rbt = next;
                    map.set(next, '.');
                    let mut next = next.move_to(dir);
                    while let Some('O') = map.get(next) {
                        next = next.move_to(dir);
                    }
                    map.set(next, 'O');
                }
            }
            _ => unreachable!(),
        };
    }

    let mut res = 0;
    for pt in map.points() {
        if map.get(pt) == Some(&'O') {
            res += 100 * pt.i + pt.j;
        }
    }

    res.to_string()
}

#[allow(dead_code)]
fn exec2(input: (Vec<Vec<char>>, Vec<char>)) -> String {
    let (mut map, seq) = (
        {
            let tmp = input
                .0
                .iter()
                .map(|v| {
                    v.iter()
                        .flat_map(|&c| {
                            match c {
                                '#' => "##",
                                'O' => "[]",
                                '.' => "..",
                                '@' => "@.",
                                _ => unreachable!(),
                            }
                            .chars()
                        })
                        .collect()
                })
                .collect();
            Grid::from(tmp)
        },
        input.1,
    );

    // map.display_by(|c| c.to_string());

    let mut rbt = map.find_point('@').unwrap();

    // let mut dir = Direction::Down;
    for dir in seq {
        // 交互式运行
        // loop {
        // terminal::disable_raw_mode().unwrap();

        // execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
        // execute!(std::io::stdout(), cursor::MoveTo(0, 0)).unwrap();
        // println!("dir: {:?}", dir);
        // execute!(std::io::stdout(), cursor::MoveTo(1, 0)).unwrap();
        // map.display_by(|c| c.to_string());
        // println!();

        // terminal::enable_raw_mode().unwrap();
        // if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
        //     // execute!(std::io::stdout(), cursor::MoveTo(0, 0)).unwrap();
        //     match code {
        //         KeyCode::Char('w') => dir = Direction::Up,
        //         KeyCode::Char('s') => dir = Direction::Down,
        //         KeyCode::Char('a') => dir = Direction::Left,
        //         KeyCode::Char('d') => dir = Direction::Right,
        //         _ => continue,
        //     }
        // }

        // println!("{:?}", dir);
        let dir = match dir {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => continue,
        };
        let next = rbt.move_to(dir);
        match map.get(next) {
            Some('#') => continue,
            Some('.') => (),
            Some('[') | Some(']') => {
                // 传导移动，有一个不能移动就停止
                let mut box_layers = Vec::new();
                let mut boxes = [next].to_vec();
                let able = 'able: loop {
                    if boxes.is_empty() {
                        break 'able true;
                    }
                    // 如果是竖直方向，考虑箱内传导
                    boxes = if dir == Direction::Up || dir == Direction::Down {
                        let mut tmp = boxes.clone();
                        for box_ in boxes {
                            match map.get(box_) {
                                Some('[') => {
                                    let other = box_.move_to(Direction::Right);
                                    if !tmp.contains(&other) {
                                        tmp.push(other);
                                    }
                                }
                                Some(']') => {
                                    let other = box_.move_to(Direction::Left);
                                    if !tmp.contains(&other) {
                                        tmp.push(other);
                                    }
                                }
                                _ => (),
                            }
                        }
                        tmp
                    } else {
                        boxes
                    };
                    // 加入箱子层
                    box_layers.push(boxes.clone());
                    // 相邻传导
                    let mut next_boxes = Vec::new();
                    for box_ in boxes {
                        let next = box_.move_to(dir);
                        match map.get(next) {
                            Some('[') | Some(']') => {
                                next_boxes.push(next);
                            }
                            Some('.') => continue,
                            _ => break 'able false,
                        }
                    }
                    boxes = next_boxes;
                };

                if able {
                    for box_layer in box_layers.iter().rev() {
                        for &box_ in box_layer {
                            let next = box_.move_to(dir);
                            map.set(next, *map.get(box_).unwrap());
                            map.set(box_, '.');
                        }
                    }
                } else {
                    continue;
                }
            }
            _ => unreachable!(),
        };

        map.set(rbt, '.');
        rbt = next;
        map.set(rbt, '@');
    }

    // map.display_by(|c| c.to_string());

    let mut res = 0;
    for pt in map.points() {
        if map.get(pt) == Some(&'[') {
            res += 100 * pt.i + pt.j;
        }
    }

    res.to_string()
}

#[allow(unused_variables)]
fn main() {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);

    let input = r"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"
    .trim();
    let input = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "
    .trim();
    let input = fs::read_to_string(file).unwrap();

    let input = {
        let (map, seq) = input.split_once("\n\n").unwrap();
        let map = map.lines().map(|s| s.chars().collect()).collect();
        let seq = seq.chars().collect();
        (map, seq)
    };

    println!("{:?}", exec2(input));
}
