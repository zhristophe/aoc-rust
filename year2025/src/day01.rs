use utils::prelude::*;

fn read(idx: usize) -> Vec<i32> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        &input,
        r"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
        .trim(),
    ][idx];

    input
        .lines()
        .map(|s| {
            let s = s.as_bytes();
            let d = s[1..].to_u32_unsafe() as i32;
            if s[0] == b'L' {
                -d
            } else {
                d
            }
        })
        .collect()
}

pub fn part1(idx: usize) -> i32 {
    let rotations = read(idx);

    let mut res = 0;
    let mut pos = 50;
    for rotation in rotations {
        pos += rotation;
        pos %= 100;
        if pos == 0 {
            res += 1
        }
    }

    res
}

pub fn part2(idx: usize) -> i32 {
    let rotations = read(idx);

    let mut res = 0;
    let mut pos = 50;
    for rotation in rotations {
        if rotation < 0 {
            if pos == 0 {
                res -= 1;
            }
            pos += rotation;
            while pos < 0 {
                pos += 100;
                res += 1;
            }
        } else {
            pos += rotation;
            while pos > 100 {
                pos -= 100;
                res += 1;
            }
        }
        pos %= 100;
        if pos == 0 {
            res += 1;
        }
    }

    res
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
        assert_eq!(part1(0), 1150);
        assert_eq!(part1(1), 3);

        assert_eq!(part2(0), 6738);
        assert_eq!(part2(1), 6);
    }
}
