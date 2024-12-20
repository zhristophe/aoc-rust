use aoc::prelude::*;

fn read(idx: usize) -> (HashMap<usize, (char, Vec<usize>)>, NamePool) {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"
        .trim(),
        r"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
        "
        .trim(),
    ][idx];

    {
        let mut map = HashMap::new();
        let mut pool = NamePool::new();
        input.lines().for_each(|line| {
            let tmp = line.split_once(" -> ").unwrap();
            let kind = tmp.0.chars().next().unwrap();
            let (name, kind) = match kind {
                '%' | '&' => (tmp.0.split_at(1).1, kind),
                _ => (tmp.0, ' '),
            };
            let id = pool.id(name);
            let rev = tmp.1.split(", ").map(|s| pool.id(s)).collect();

            map.insert(id, (kind, rev));
        });
        (map, pool)
    }
}

/// 模拟
fn part1(idx: usize) -> String {
    let (map, mut pool) = read(idx);

    let broadcaster = pool.id("broadcaster");
    // 队列实现。0表示低脉冲，1表示高脉冲
    let mut queue = VecDeque::new();
    queue.push_back((None::<usize>, broadcaster, 0));
    // Flip-flop
    let mut ff = HashMap::new();
    // Conjunction
    let mut cj = HashMap::new();
    for (&id, (kind, rev)) in map.iter() {
        for r in rev {
            match map.get(r) {
                Some(('&', _)) => {
                    let stt = cj.entry(*r).or_insert(HashMap::new());
                    stt.insert(id, 0);
                }
                _ => (),
            }
        }

        if *kind == '%' {
            let stt = cj.entry(id).or_insert(HashMap::new());
            for r in rev {
                stt.insert(*r, 0);
            }
        }
    }

    while let Some((from, to, input)) = queue.pop_front() {
        let (kind, rev) = map.get(&to).unwrap();
        // debug
        println!(
            "{} -{}-> {}",
            from.map_or("button".to_string(), |from| pool.get(from).unwrap().clone()),
            (if input == 0 { "low" } else { "high" }).to_string(),
            pool.get(to).unwrap()
        );

        let output = match kind {
            ' ' => input,
            '%' => {
                let stt = ff.entry(to).or_insert(false);
                if input == 0 {
                    *stt = !*stt;
                    if *stt {
                        1
                    } else {
                        0
                    }
                } else {
                    -1
                }
            }
            '&' => {
                let stt = cj.entry(to).or_insert(HashMap::new());
                stt.insert(from.unwrap(), input);
                if stt.iter().all(|(_, v)| *v == 1) {
                    0
                } else {
                    1
                }
            }
            _ => unreachable!(),
        };
        if output != -1 {
            for r in rev {
                queue.push_back((Some(to), *r, output));
            }
        }
    }

    0.to_string()
}

/// 基本一样，只是搜索20步
fn part2(idx: usize) -> String {
    0.to_string()
}

fn main() {
    println!("{:?}", part1(1));
    // println!("{:?}", part2(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(part1(0), "1363");

        // assert_eq!(part2(0), "1007186");
    }
}
