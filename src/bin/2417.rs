use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread::available_parallelism,
    usize,
};

use rayon::prelude::*;

fn read(idx: usize) -> (Vec<usize>, Vec<u8>) {
    let name = module_path!().split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);
    let content = fs::read_to_string(file).unwrap();

    let inputs = [
        &content,
        r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
        .trim(),
        r"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
        "
        .trim(),
    ];

    let input = if idx > inputs.len() {
        inputs.last().unwrap()
    } else {
        inputs[idx]
    };

    {
        let tmp = input.split_once("\n\n").unwrap();
        (
            tmp.0
                .lines()
                .map(|s| s.split(": ").last().unwrap().parse().unwrap())
                .collect(),
            tmp.1
                .lines()
                .flat_map(|s| {
                    s.split(": ")
                        .last()
                        .unwrap()
                        .split(',')
                        .map(|s| s.parse().unwrap())
                })
                .collect(),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Res {
    IsDesired(bool),
    Output(String),
}

fn exec(mut registers: Vec<usize>, program: &Vec<u8>, desired: Option<&Vec<usize>>) -> Res {
    let mut pc = 0;
    let mut output = Vec::new();

    macro_rules! get_combo_opd {
        ($opd:expr) => {
            match $opd {
                0..=3 => $opd as usize,
                4..=6 => registers.clone()[$opd as usize - 4],
                _ => unreachable!(),
            }
        };
    }

    let is = 'main: loop {
        let (ins, opd) = (program[pc], program[pc + 1]);
        match ins {
            0 => registers[0] /= 2usize.pow(get_combo_opd!(opd) as u32),
            1 => registers[1] ^= opd as usize,
            2 => registers[1] = get_combo_opd!(opd).rem_euclid(8),
            3 => {
                if registers[0] != 0 {
                    pc = opd as usize;
                    continue 'main;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => {
                let tmp = get_combo_opd!(opd).rem_euclid(8);
                if let Some(desired) = &desired {
                    if Some(&tmp) != desired.get(output.len()) {
                        break 'main false;
                    }
                }
                output.push(tmp);
            }
            6 => registers[1] = registers[0] / 2usize.pow(get_combo_opd!(opd) as u32),
            7 => registers[2] = registers[0] / 2usize.pow(get_combo_opd!(opd) as u32),
            _ => unreachable!("invalid instruction {}", ins),
        }

        pc += 2;
        if pc >= program.len() {
            break true;
        }
    };

    if desired.is_some() {
        Res::IsDesired(is)
    } else {
        Res::Output(
            output
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
    }
}

fn part1(idx: usize) -> String {
    let (registers, program) = read(idx);
    match exec(registers, &program, None) {
        Res::Output(res) => res,
        _ => unreachable!(),
    }
}

/// 慢慢搜去吧 :)
fn part2_simple(idx: usize) -> String {
    let (registers, program) = read(idx);

    let tgt = program.iter().map(|&n| n as usize).collect::<Vec<_>>();
    let len = tgt.len() as u32;

    let num_threads = available_parallelism().unwrap().get().max(32);
    let found = AtomicBool::new(false);
    let res = AtomicUsize::new(usize::MAX);

    (0..num_threads).into_par_iter().for_each(|id| {
        let mut self_res = 8usize.pow(len - 1) + id;
        let mut registers = registers.clone();
        loop {
            if found.load(Ordering::Relaxed) && self_res > res.load(Ordering::Relaxed) {
                return;
            }

            registers[0] = self_res as usize;
            if exec(registers.clone(), &program, Some(&tgt)) == Res::IsDesired(true) {
                found.store(true, Ordering::Relaxed);
                res.fetch_min(self_res, Ordering::Relaxed);
                return;
            }
            self_res += num_threads;
        }
    });

    res.load(Ordering::Relaxed).to_string()
}

/// 注意到 :)
fn part2(idx: usize) -> String {
    let (mut registers, program) = read(idx);

    // 核心: A3 ^ 3 ^ (A >> A3 ^ 3) ^ 5 = X3
    // 倒推，最后一次A 最多三位 （A >> 3 == 0）
    fn dfs(reg_a: usize, idx: usize, program: &Vec<u8>) -> Option<usize> {
        if idx == program.len() {
            return Some(reg_a);
        }

        let tgt = program[program.len() - 1 - idx];
        // 枚举三位二进制数
        for a3 in 0b000..=0b111 {
            let new_a = (reg_a << 3) + a3;
            // dbg!(a3, new_a);
            if ((a3 ^ 3 ^ (new_a >> (a3 ^ 3)) ^ 5) % 8) as u8 == tgt {
                if let Some(res) = dfs(new_a, idx + 1, program) {
                    return Some(res);
                }
            }
        }
        None
    }

    let res = dfs(0, 0, &program).unwrap();
    // dbg!(res);

    registers[0] = res;
    // dbg!(exec(registers.clone(), &program, None));

    res.to_string()
}

fn main() {
    println!("{:?}", part1(0));
    println!("{:?}", part2_simple(2));
    println!("{:?}", part2(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(0), "2,0,1,3,4,0,2,1,7");
        assert_eq!(part1(1), "4,6,3,5,6,3,5,2,1,0");

        assert_eq!(part2(0), "236580836040301");
        assert_eq!(part2_simple(2), "117440");
    }
}
