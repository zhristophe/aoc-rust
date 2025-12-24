use utils::prelude::*;

fn read(
    idx: usize,
) -> (
    NamePool,
    HashMap<usize, u8>,
    Vec<(usize, usize, String, usize)>,
) {
    let input = read_input(module_path!()).unwrap();

    let input = [
        input.as_str(),
        r"
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
",
        r"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
",
    ][idx]
        .trim();

    {
        let mut pool = NamePool::new();
        // 收集wire名称
        input.split_ascii_whitespace().for_each(|s| {
            match s {
                "1" | "0" | "->" | "AND" | "OR" | "XOR" => return,
                _ => (),
            };
            if s.ends_with(':') {
                pool.id(s.strip_suffix(":").unwrap());
            } else {
                pool.id(s);
            }
        });
        let tmp = input.split_once("\n\n").unwrap();
        let mut tmp1 = HashMap::new();
        tmp.0.lines().for_each(|s| {
            let tmp = s.split_once(": ").unwrap();
            tmp1.insert(pool.id(tmp.0), tmp.1.parse().unwrap());
        });
        let mut tmp2 = Vec::new();
        tmp.1.lines().for_each(|s| {
            let tmp = s.split(" ").collect::<Vec<_>>();
            tmp2.push((
                pool.id(tmp[0]),
                pool.id(tmp[2]),
                tmp[1].to_string(),
                pool.id(tmp[4]),
            ));
        });
        (pool, tmp1, tmp2)
    }
}

/// 模拟
fn part1(idx: usize) -> String {
    let (mut pool, mut wires_val, gates) = read(idx);

    let do_op = |in1, in2, op| match op {
        "AND" => in1 & in2,
        "OR" => in1 | in2,
        "XOR" => in1 ^ in2,
        _ => unreachable!(),
    };

    loop {
        let mut new_wires_val = HashMap::new();
        for (in1, in2, op, out) in gates.iter() {
            let in1 = wires_val.get(in1);
            let in2 = wires_val.get(in2);
            if in1.is_none() || in2.is_none() {
                continue;
            }
            let out_val = do_op(*in1.unwrap(), *in2.unwrap(), op);
            if wires_val.get(out).is_none() {
                new_wires_val.insert(*out, out_val);
            }
        }
        if new_wires_val.is_empty() {
            break;
        }
        wires_val.extend(new_wires_val);
    }

    let mut names = pool.names().map(|s| s.to_string()).collect::<Vec<_>>();
    names.sort();
    let mut res = 0usize;
    for name in names.iter().rev() {
        if !name.starts_with("z") {
            continue;
        }
        let id = pool.id(name);
        let val = wires_val.get(&id).unwrap();
        res <<= 1;
        res += *val as usize;
    }

    res.to_string()
}

/// 串行加法器
/// S    = (A XOR B) XOR Cin
/// Cout = (A AND B) OR (Cin AND (A XOR B))
/// 5 * 45 = 225个部件，少了第一位的3个部件，和输入行数一致
/// 在本题中，只需要检查加法器部件类型是否匹配即可
fn part2(idx: usize) -> String {
    let (pool, _, gates) = read(idx);

    // 计数
    let mut wire_cnt = HashMap::new();
    for gate in &gates {
        for gate in [gate.0, gate.1, gate.3] {
            wire_cnt.entry(gate).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    // 直接找出所有不匹配的部件，不管所处位数
    let mut error_vec = Vec::new();
    for gate in gates {
        if pool.name(gate.3).unwrap() == "z45" || pool.name(gate.0).unwrap().ends_with("00") {
            // 人工检查
            continue;
        }
        let &in1 = wire_cnt.get(&gate.0).unwrap();
        let &in2 = wire_cnt.get(&gate.1).unwrap();
        let &out = wire_cnt.get(&gate.3).unwrap();
        let ok = match gate.2.as_str() {
            "AND" => (in1 == 2 && in2 == 2 && out == 2) || (in1 == 3 && in2 == 3 && out == 2),
            "OR" => in1 == 2 && in2 == 2 && out == 3,
            "XOR" => (in1 == 2 && in2 == 2 && out == 3) || (in1 == 3 && in2 == 3 && out == 1),
            _ => unreachable!(),
        };
        if !ok {
            error_vec.push(pool.name(gate.3).unwrap());
        }
    }

    error_vec.sort();
    error_vec.join(",")
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
        // assert_eq!(part1(0), "57270694330992");
        assert_eq!(part1(1), "4");
        assert_eq!(part1(2), "2024");

        // assert_eq!(part2(0), "gwh,jct,rcb,wbw,wgb,z09,z21,z39");
    }
}
