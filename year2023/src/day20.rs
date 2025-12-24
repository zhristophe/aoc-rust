use utils::prelude::*;

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
",
        r"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
",
    ][idx]
        .trim();

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

fn get_init_states(
    map: &HashMap<usize, (char, Vec<usize>)>,
) -> (HashMap<usize, bool>, HashMap<usize, HashMap<usize, bool>>) {
    // 返回ff状态，cj状态
    let mut ff = HashMap::new();
    let mut cj = HashMap::new();
    for (&id, (kind, rev)) in map.iter() {
        // ff初始化为关
        if *kind == '%' {
            ff.insert(id, false);
        }
        // cj每个输入初始化为低
        for r in rev {
            let stt = cj.entry(*r).or_insert(HashMap::new());
            stt.insert(id, false);
        }
    }

    (ff, cj)
}

fn push_button(
    map: &HashMap<usize, (char, Vec<usize>)>,
    pool: &mut NamePool,
    ff: &mut HashMap<usize, bool>,
    cj: &mut HashMap<usize, HashMap<usize, bool>>,
    watch_rx: bool,
) -> (usize, usize) {
    let broadcaster = pool.id("broadcaster");
    let rx = pool.id("rx");
    // 队列实现。0表示低脉冲，1表示高脉冲
    let mut queue = VecDeque::new();
    queue.push_back((None::<usize>, broadcaster, false));

    let mut low_cnt = 0;
    let mut high_cnt = 0;
    while let Some((from, to, input)) = queue.pop_front() {
        if !map.contains_key(&to) {
            continue;
        }
        let (kind, rev) = map.get(&to).unwrap();
        // dbg!(watch_rx, input, to, rx);
        if watch_rx && !input && to == rx {
            return (0, 0);
        }
        // debug
        // println!(
        //     "{} -{}-> {}",
        //     from.map_or("button".to_string(), |from| pool.get(from).unwrap().clone()),
        //     (if input { "high" } else { "low" }).to_string(),
        //     pool.get(to).unwrap()
        // );

        let output = match kind {
            ' ' => Some(input),
            '%' => {
                let stt = ff.entry(to).or_insert(false);
                if input {
                    None // 高脉冲无事发生
                } else {
                    *stt = !*stt;
                    if *stt {
                        Some(true)
                    } else {
                        Some(false)
                    }
                }
            }
            '&' => {
                let stt = cj.get_mut(&to).unwrap();
                stt.insert(from.unwrap(), input);
                if stt.iter().all(|(_, v)| *v) {
                    Some(false) // 全高发低脉冲
                } else {
                    Some(true) // 否则发高脉冲
                }
            }
            _ => unreachable!(),
        };
        if let Some(output) = output {
            for r in rev {
                queue.push_back((Some(to), *r, output));
                if output {
                    high_cnt += 1;
                } else {
                    low_cnt += 1;
                }
            }
        }
    }

    (low_cnt + 1, high_cnt)
}

/// 模拟
pub fn part1(idx: usize) -> String {
    let (map, mut pool) = read(idx);

    let (mut ff, mut cj) = get_init_states(&map);

    // 记录出现过的状态，记录前若干轮结果
    let mut record: Vec<(usize, usize)> = Vec::new();
    let mut cache = HashMap::new();

    let max_times = 1000;
    let mut time = 0;
    let (low_cnt, high_cnt) = loop {
        let mut state = 0usize;
        let len = pool.len();
        // dbg!(&pool, len, &map);
        for i in 0..len {
            map.get(&i).map(|(kind, _)| match kind {
                '%' => state = (state << 1) + if *ff.get(&i).unwrap() { 1 } else { 0 },
                '&' => {
                    let stt = cj.get(&i).unwrap();
                    for j in 0..len {
                        if let Some(b) = stt.get(&j) {
                            state = (state << 1) + if *b { 1 } else { 0 };
                        }
                    }
                }
                _ => (),
            });
        }
        let last_time = if time >= 1000 {
            Some(&0)
        } else {
            cache.get(&state)
        };
        if let Some(&last_time) = last_time {
            let (mut low_cnt, mut high_cnt) = (0, 0);
            // 循环节前的结果
            for time in 0..last_time {
                low_cnt += record[time].0;
                high_cnt += record[time].1;
            }
            // 循环节的结果
            let cycle_len = time - last_time;
            for time in last_time + 1..=max_times {
                let time = last_time + (time - last_time) % cycle_len;
                low_cnt += record[time].0;
                high_cnt += record[time].1;
            }
            break (low_cnt, high_cnt);
        }
        cache.insert(state, time);
        record.push(push_button(&map, &mut pool, &mut ff, &mut cj, false));

        time += 1;
        // dbg!(time);
    };

    (low_cnt * high_cnt).to_string()
}

/// 模拟不出来

#[allow(dead_code)]
fn part2_simple(idx: usize) -> String {
    let (map, mut pool) = read(idx);

    let (mut ff, mut cj) = get_init_states(&map);

    // 记录出现过的状态，记录前若干轮结果
    let mut time = 0;
    let time = loop {
        let mut state = 0usize;
        let len = pool.len();
        // dbg!(&pool, len, &map);
        for i in 0..len {
            map.get(&i).map(|(kind, _)| match kind {
                '%' => state = (state << 1) + if *ff.get(&i).unwrap() { 1 } else { 0 },
                '&' => {
                    let stt = cj.get(&i).unwrap();
                    for j in 0..len {
                        if let Some(b) = stt.get(&j) {
                            state = (state << 1) + if *b { 1 } else { 0 };
                        }
                    }
                }
                _ => (),
            });
        }
        if (0, 0) == push_button(&map, &mut pool, &mut ff, &mut cj, true) {
            break time + 1;
        }
        time += 1;
    };

    time.to_string()
}

/// 利用周期性质，推导周期
/// %：数0就行
/// &：最小公倍数
/// 广播周期
pub fn part2(idx: usize) -> String {
    // let (map, mut pool) = read(idx);

    // let (mut ff, mut cj) = get_init_states(&map);

    let _ = idx;
    // let _ = ff;
    // let _ = cj;

    0.to_string()
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
        // assert_eq!(part1(0), "806332748");
        assert_eq!(part1(1), "32000000");
        assert_eq!(part1(2), "11687500");

        // // assert_eq!(part2(0), "1007186");
    }
}
