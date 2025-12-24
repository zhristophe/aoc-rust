use regex::Regex;
use utils::read_input;

#[derive(Debug, Clone, Copy)]
struct Input {
    a: (isize, isize),
    b: (isize, isize),
    p: (isize, isize),
}

fn read(idx: usize) -> Vec<Input> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    ][idx];

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let mut res = Vec::new();

    for input in input.split("\n\n") {
        let input = if let Some(captures) = re.captures(input) {
            Some(Input {
                a: (
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                ),
                b: (
                    captures.get(3).unwrap().as_str().parse().unwrap(),
                    captures.get(4).unwrap().as_str().parse().unwrap(),
                ),
                p: (
                    captures.get(5).unwrap().as_str().parse().unwrap(),
                    captures.get(6).unwrap().as_str().parse().unwrap(),
                ),
            })
        } else {
            None
        };
        if let Some(i) = input {
            res.push(i);
        }
    }

    res
}

pub fn part1(idx: usize) -> String {
    let input = read(idx);
    // 枚举！
    let mut res = 0;
    for input in input {
        let mut minv = 114514;
        for a in 0..101 {
            for b in 0..101 {
                let p = (a * input.a.0 + b * input.b.0, a * input.a.1 + b * input.b.1);
                if p.0 == input.p.0 && p.1 == input.p.1 {
                    minv = minv.min(3 * a + b);
                }
            }
        }
        // dbg!(minv);
        if minv != 114514 {
            res += minv;
        }
    }

    res.to_string()
}

pub fn part2(idx: usize) -> String {
    let input = read(idx);
    let offset = 10000000000000;
    let mut res = 0;
    for input in input {
        let (a, b, p) = (input.a, input.b, (input.p.0 + offset, input.p.1 + offset));

        let b_x = a.1 * b.0 - a.0 * b.1;
        let p_x = a.1 * p.0 - a.0 * p.1;
        // dbg!(b_x, p_x);
        if p_x % b_x != 0 {
            // println!("无解");
            continue;
        }
        let a_y = b.1 * a.0 - b.0 * a.1;
        let p_y = b.1 * p.0 - b.0 * p.1;
        // dbg!(a_y, p_y);
        if p_y % a_y != 0 {
            // println!("无解");
            continue;
        }

        let (a, b) = (p_y / a_y, p_x / b_x);
        res += 3 * a + b;
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
        assert_eq!(part1(1), "480");
        // assert_eq!(part1(0), "36250");

        assert_eq!(part2(1), "875318608908");
        // assert_eq!(part2(0), "83232379451012");
    }
}
