use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

#[allow(dead_code)]
fn exec1(input: &Vec<Vec<u8>>) -> String {
    let height = input.len();
    let width = input[0].len();

    let mut score = 0usize;

    for i in 0..height {
        for j in 0..width {
            let cur = input[i][j];
            if cur != 0 {
                continue;
            }

            // let mut f = input.clone();
            // let mut points = vec![(i, j); 0];
            let mut points = HashSet::new();
            points.insert((i, j));
            for depth in 0..9 {
                let mut new_points = HashSet::new();
                let next_depth = depth + 1;
                for (w, h) in points.into_iter() {
                    if w > 0 && input[w - 1][h] == next_depth {
                        new_points.insert((w - 1, h));
                    }
                    if w < width - 1 && input[w + 1][h] == next_depth {
                        new_points.insert((w + 1, h));
                    }
                    if h > 0 && input[w][h - 1] == next_depth {
                        new_points.insert((w, h - 1));
                    }
                    if h < height - 1 && input[w][h + 1] == next_depth {
                        new_points.insert((w, h + 1));
                    }
                }
                points = new_points;
            }
            score += points.len();
        }
    }

    score.to_string()
}

fn exec2(input: &Vec<Vec<u8>>) -> String {
    let height = input.len();
    let width = input[0].len();

    let mut score = 0usize;

    for i in 0..height {
        for j in 0..width {
            let cur = input[i][j];
            if cur != 0 {
                continue;
            }

            let mut points_score = HashMap::new();
            points_score.insert((i, j), 1);
            for depth in 0..9 {
                let mut new_points_score = HashMap::new();
                let next_depth = depth + 1;
                for ((w, h), s) in points_score.into_iter() {
                    if w > 0 && input[w - 1][h] == next_depth {
                        new_points_score
                            .entry((w - 1, h))
                            .and_modify(|s_old| *s_old += s)
                            .or_insert(s);
                    }
                    if w < width - 1 && input[w + 1][h] == next_depth {
                        new_points_score
                            .entry((w + 1, h))
                            .and_modify(|s_old| *s_old += s)
                            .or_insert(s);
                    }
                    if h > 0 && input[w][h - 1] == next_depth {
                        new_points_score
                            .entry((w, h - 1))
                            .and_modify(|s_old| *s_old += s)
                            .or_insert(s);
                    }
                    if h < height - 1 && input[w][h + 1] == next_depth {
                        new_points_score
                            .entry((w, h + 1))
                            .and_modify(|s_old| *s_old += s)
                            .or_insert(s);
                    }
                }
                points_score = new_points_score;
            }
            for (_, s) in points_score.into_iter() {
                score += s;
            }
        }
    }

    score.to_string()
}

fn main() {
    // let input = vec![
    //     "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
    //     "10456732",
    // ];
    // let input = vec!["012345", "123456", "234567", "345678", "416789", "567891"];

    let file = Path::new("data/2410/input");
    let input = fs::read_to_string(file).unwrap();
    let input = input.lines().collect::<Vec<_>>();

    let input = input
        .iter()
        .map(|s| s.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    // println!("{:?}", &input);
    println!("{}", exec2(&input));

    // println!("1");
}
