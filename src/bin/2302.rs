use aoc::prelude::*;

fn read(idx: usize) -> Vec<Vec<Vec<Cube>>> {
    let input = read_input(module_path!()).unwrap();

    let input = [
        &input,
        r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
        .trim(),
    ][idx];

    {
        let tmp = input
            .lines()
            .map(|line| {
                line.split_once(": ")
                    .unwrap()
                    .1
                    .split("; ")
                    .map(|s| {
                        s.split(", ")
                            .map(|s| {
                                let (num, color) = s.split_once(' ').unwrap();
                                let num = num.parse().unwrap();
                                match color {
                                    "red" => Cube::Red(num),
                                    "green" => Cube::Green(num),
                                    "blue" => Cube::Blue(num),
                                    _ => unreachable!(),
                                }
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        tmp
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

fn part1(idx: usize) -> String {
    let inputs = read(idx);

    let mut res = 0;
    for (idx, input) in inputs.iter().enumerate() {
        let possible = 'possible: loop {
            for round in input {
                for cube in round {
                    match *cube {
                        Cube::Red(red) => {
                            if red > 12 {
                                break 'possible false;
                            }
                        }
                        Cube::Green(green) => {
                            if green > 13 {
                                break 'possible false;
                            }
                        }
                        Cube::Blue(blue) => {
                            if blue > 14 {
                                break 'possible false;
                            }
                        }
                    }
                }
            }
            break 'possible true;
        };

        if possible {
            res += idx + 1;
        }
    }

    res.to_string()
}

fn part2(idx: usize) -> String {
    let inputs = read(idx);

    let mut res = 0;
    for (_, input) in inputs.iter().enumerate() {
        let mut minc = (0, 0, 0);
        for round in input {
            for cube in round {
                match *cube {
                    Cube::Red(red) => {
                        minc.0 = minc.0.max(red);
                    }
                    Cube::Green(green) => {
                        minc.1 = minc.1.max(green);
                    }
                    Cube::Blue(blue) => {
                        minc.2 = minc.2.max(blue);
                    }
                }
            }
        }

        res += minc.0 * minc.1 * minc.2;
    }

    res.to_string()
}

fn main() {
    println!("{:?}", part1(0));
    println!("{:?}", part2(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(0), "2528");
        assert_eq!(part1(1), "8");

        assert_eq!(part2(0), "67363");
        assert_eq!(part2(1), "2286");
    }
}
