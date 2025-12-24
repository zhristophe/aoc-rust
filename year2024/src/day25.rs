use utils::prelude::*;

fn read(idx: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
",
    ][idx]
        .trim();

    {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        input.split("\n\n").for_each(|s| {
            let tmp = s
                .lines()
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let mut res = vec![0; tmp[0].len()];
            for j in 0..tmp[0].len() {
                for i in 0..tmp.len() {
                    if tmp[i][j] == '#' {
                        res[j] += 1;
                    }
                }
                res[j] -= 1;
            }
            if tmp[0][0] == '#' {
                locks.push(res);
            } else {
                keys.push(res);
            }
        });

        (locks, keys)
    }
}

/// easy
pub fn part1(idx: usize) -> String {
    let (locks, keys) = read(idx);

    let mut res = 0;
    for lock in &locks {
        for key in &keys {
            if lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5) {
                res += 1;
            }
        }
    }

    res.to_string()
}

pub fn run() {
    println!("{:?}", part1(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(part1(0), "3307");
        assert_eq!(part1(1), "3");
    }
}
