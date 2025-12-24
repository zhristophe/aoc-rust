use utils::prelude::*;

fn read(idx: usize) -> String {
    let input = read_input(module_path!()).unwrap();

    let input = [
        &input,
        r"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"
        .trim(),
        r"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"
        .trim(),
    ][idx];

    input.into()
}

/// 正则即可，这里简单写个语法解析器
fn part1(idx: usize) -> String {
    let input = read(idx);

    enum State {
        Mul(usize),
        LP,
        RP,
        Lhs(usize),
        Rhs(usize),
        Com,
        Empty,
    }

    let mut state = State::Empty;
    let mut lhs = 0;
    let mut rhs = 0;
    let mut res = 0;
    for ch in input.chars() {
        state = match state {
            State::Mul(n) => match ch {
                'u' if n == 1 => State::Mul(2),
                'l' if n == 2 => State::Mul(3),
                '(' if n == 3 => State::LP,
                _ => State::Empty,
            },
            State::LP => match ch {
                '0'..='9' => State::Lhs(ch as usize - '0' as usize),
                _ => State::Empty,
            },
            State::Lhs(n) => match ch {
                '0'..='9' if n < 1000 => State::Lhs(n * 10 + ch as usize - '0' as usize),
                ',' => State::Com,
                _ => State::Empty,
            },
            State::Com => match ch {
                '0'..='9' => State::Rhs(ch as usize - '0' as usize),
                _ => State::Empty,
            },
            State::Rhs(n) => match ch {
                '0'..='9' if n < 1000 => State::Rhs(n * 10 + ch as usize - '0' as usize),
                ')' => State::RP,
                _ => State::Empty,
            },
            State::Empty | State::RP => match ch {
                'm' => State::Mul(1),
                _ => State::Empty,
            },
        };
        match state {
            State::Lhs(n) => lhs = n,
            State::Rhs(n) => rhs = n,
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

/// 和1差不多，稍微改改
fn part2(idx: usize) -> String {
    let input = read(idx);

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

pub fn run() {
    println!("{:?}", part1(0));
    println!("{:?}", part2(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(part1(0), "190604937");
        assert_eq!(part1(1), "161");

        // assert_eq!(part2(0), "82857512");
        assert_eq!(part2(2), "48");
    }
}
