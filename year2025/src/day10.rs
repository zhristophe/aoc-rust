use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel};

use utils::prelude::*;

fn read(idx: usize) -> Vec<(Vec<i32>, Vec<Vec<i32>>, Vec<i32>)> {
    let input = read_input(module_path!()).unwrap();

    let input = vec![
        &input,
        r"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
        .trim(),
    ][idx];

    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ').collect::<Vec<_>>();

            let init = parts[0];
            let init = init[1..init.len() - 1]
                .as_bytes()
                .iter()
                .map(|b| if *b == b'.' { 0 } else { 1 })
                .collect();

            let levels = parts.pop().unwrap();
            let levels = levels[1..levels.len() - 1]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            let buttons = parts[1..]
                .iter()
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(',')
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect();

            (init, buttons, levels)
        })
        .collect()
}

/// 宽搜
pub fn part1(idx: usize) -> i64 {
    let machines = read(idx);

    let mut ans = 0;
    for (init, buttons, _) in machines {
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back((init.clone(), 0));
        visited.insert(init);

        while let Some((state, step)) = q.pop_front() {
            if state.iter().all(|&x| x == 0) {
                ans += step;
                break;
            }

            for button in &buttons {
                let mut next = state.clone();
                for &pos in button {
                    next[pos as usize] ^= 1;
                }

                if !visited.contains(&next) {
                    q.push_back((next.clone(), step + 1));
                    visited.insert(next);
                }
            }
        }
    }

    ans
}

/// 状态空间太大，无法像part1那样记忆化宽搜。
///
/// 我们将问题建模为不定方程组 + 最小化，
/// 是一个整数线性规划问题。
/// 我们甚至可以调用一个LP或者OMT库来解决。
pub fn part2(idx: usize) -> i64 {
    let machines = read(idx);

    let mut ans = 0;
    for (_, buttons, levels) in machines {
        let mut exps = levels
            .iter()
            .map(|_| Expression::default())
            .collect::<Vec<_>>();

        let mut vars = variables!();
        let mut all_times = Vec::with_capacity(buttons.len());
        for button in buttons {
            let times = vars.add(variable().min(0).integer());
            for pos in button {
                exps[pos as usize] += times;
            }
            all_times.push(times);
        }

        let mut sum = Expression::default();
        for times in &all_times {
            sum += times;
        }

        let mut solution = vars.minimise(sum).using(default_solver);
        for (exp, level) in exps.into_iter().zip(levels.into_iter()) {
            solution = solution.with(exp.eq(level));
        }
        let solution = solution.solve().unwrap();

        let mut tmp = 0;
        for times in all_times {
            let times = solution.value(times);
            tmp += times.round() as i64;
        }
        ans += tmp;
    }

    ans
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
        assert_eq!(part1(0), 401);
        assert_eq!(part1(1), 7);

        assert_eq!(part2(0), 15017);
        assert_eq!(part2(1), 33);
    }
}
