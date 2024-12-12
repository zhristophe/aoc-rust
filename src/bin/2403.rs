use std::{fs, path::Path};

#[allow(dead_code)]
fn exec1(input: &str) -> String {
    // 语法解析器，直接调库就行，这里简单写一个
    enum State {
        Mul(usize),
        LP,
        RP,
        LHS(usize),
        RHS(usize),
        Com,
        Empty,
    }
    let mut iter = input.chars();
    let mut state = State::Empty;
    let mut lhs = 0;
    let mut rhs = 0;
    let mut res = 0;
    loop {
        let ch = if let Some(ch) = iter.next().take() {
            ch
        } else {
            break;
        };
        state = match state {
            State::Mul(n) => match ch {
                'u' if n == 1 => State::Mul(2),
                'l' if n == 2 => State::Mul(3),
                '(' if n == 3 => State::LP,
                _ => State::Empty,
            },
            State::LP => match ch {
                '0'..='9' => State::LHS(ch as usize - '0' as usize),
                _ => State::Empty,
            },
            State::LHS(n) => match ch {
                '0'..='9' if n < 1000 => State::LHS(n * 10 + ch as usize - '0' as usize),
                ',' => State::Com,
                _ => State::Empty,
            },
            State::Com => match ch {
                '0'..='9' => State::RHS(ch as usize - '0' as usize),
                _ => State::Empty,
            },
            State::RHS(n) => match ch {
                '0'..='9' if n < 1000 => State::RHS(n * 10 + ch as usize - '0' as usize),
                ')' => State::RP,
                _ => State::Empty,
            },
            State::Empty | State::RP => match ch {
                'm' => State::Mul(1),
                _ => State::Empty,
            },
        };
        match state {
            State::LHS(n) => lhs = n,
            State::RHS(n) => rhs = n,
            State::RP => {
                res += lhs * rhs;
                lhs = 0;
                rhs = 0;
            }
            _ => (),
        }
    }

    res.to_string()
}

fn exec2(input: &str) -> String {
    // 和1差不多，稍微改改
    // don't和mul没有重复字母
    enum State {
        Mul(usize),
        LP,
        RP,
        LHS(usize),
        RHS(usize),
        Com,
        Do(usize),
        Not(usize),
        Empty,
    }
    let mut iter = input.chars();
    let mut state = State::Empty;
    let mut lhs = 0;
    let mut rhs = 0;
    let mut res = 0;
    let mut enable = true;
    loop {
        let ch = if let Some(ch) = iter.next().take() {
            ch
        } else {
            break;
        };
        state = match state {
            State::Mul(n) => match ch {
                ch if Some(ch) == "mul".chars().nth(n) => State::Mul(n + 1),
                '(' if n == 3 => State::LP,
                _ => State::Empty,
            },
            State::LP => match ch {
                '0'..='9' => State::LHS(ch as usize - '0' as usize),
                _ => State::Empty,
            },
            State::LHS(n) => match ch {
                '0'..='9' if n < 100 => State::LHS(n * 10 + ch as usize - '0' as usize),
                ',' => State::Com,
                _ => State::Empty,
            },
            State::Com => match ch {
                '0'..='9' => State::RHS(ch as usize - '0' as usize),
                _ => State::Empty,
            },
            State::RHS(n) => match ch {
                '0'..='9' if n < 100 => State::RHS(n * 10 + ch as usize - '0' as usize),
                ')' => State::RP,
                _ => State::Empty,
            },
            State::Do(n) => match ch {
                ch if Some(ch) == "do()".chars().nth(n) => State::Do(n + 1),
                'n' if n == 2 => State::Not(1),
                _ => State::Empty,
            },
            State::Not(n) => match ch {
                ch if Some(ch) == "n't()".chars().nth(n) => State::Not(n + 1),
                _ => State::Empty,
            },
            State::RP | State::Empty => State::Empty, // 下面处理
        };
        match state {
            State::Empty => {
                state = match ch {
                    'm' => State::Mul(1),
                    'd' => State::Do(1),
                    _ => State::Empty,
                }
            }
            _ => (),
        }
        match state {
            State::LHS(n) => lhs = n,
            State::RHS(n) => rhs = n,
            State::RP => {
                if enable {
                    res += lhs * rhs;
                }
                lhs = 0;
                rhs = 0;
            }
            State::Do(n) if n == 4 => {
                enable = true;
            }
            State::Not(n) if n == 5 => {
                enable = false;
            }
            _ => (),
        }
    }

    res.to_string()
}
#[allow(unused_variables)]
fn main() {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);

    // let input = r"don't()mul(114,100)";
    let input = fs::read_to_string(file).unwrap();

    println!("{:?}", exec2(&input));
}
