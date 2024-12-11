use num_bigint::BigInt;

#[allow(dead_code)]
fn exec(input: &mut [u8]) -> String {
    // 硬算！
    let len = input.len();
    let len = if len % 2 == 0 { len - 1 } else { len };

    let mut res = BigInt::from(0);
    let mut i = 1usize;
    let mut j = len - 1;
    let mut pos = (*input.get(0).unwrap()) as usize;

    // 第一个块的校验和
    // res += pos * (pos - 1) / 2 * 0;

    loop {
        while input[i] == 0u8 {
            i += 2;
            let cnt = input[i - 1] as usize;
            res += (pos + pos + cnt - 1) * cnt * (i / 2) / 2;
            pos += cnt;
        }
        while input[j] == 0u8 {
            j -= 2;
        }

        if i >= j {
            break;
        }

        let id = j / 2;

        let space = input[i];
        let file = input[j];
        let moved = if space < file { space } else { file };
        input[i] -= moved;
        input[j] -= moved;

        res += (pos + pos + moved as usize - 1) * moved as usize * id / 2;
        pos += moved as usize;
    }

    res.to_string()
}

#[allow(dead_code)]
fn debug_files_pos(files_pos: &[usize], input: &[u8]) -> String {
    // 已排序
    let mut idx_pos_vec = vec![(0usize, 0usize); files_pos.len()];
    for i in 0..files_pos.len() {
        idx_pos_vec[i] = (i, files_pos[i]);
    }
    idx_pos_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let mut res = String::new();
    let mut last_pos = 0usize;
    for i in 0..idx_pos_vec.len() {
        let (file_id, file_pos) = idx_pos_vec[i];
        let file_size = input[file_id * 2] as usize;
        res += &".".repeat(file_pos - last_pos);
        res += &file_id.to_string().repeat(file_size);
        last_pos = file_pos + file_size;
    }
    res
}

fn exec2(input: &[u8]) -> String {
    // rust的链表，谁用谁知道 :)
    let len = input.len();
    let n_file = len / 2 + 1;
    let mut files_pos = vec![0usize; n_file];

    let mut pos = 0usize;
    for (i, v) in input.iter().enumerate() {
        if i % 2 == 0 {
            files_pos[i / 2] = pos;
        }
        pos += *v as usize;
    }

    let mut last_id = n_file;
    loop {
        // 把文件按照位置排序
        let mut idx_pos_vec = vec![(0usize, 0usize); n_file];
        for i in 0..n_file {
            idx_pos_vec[i] = (i, files_pos[i]);
        }
        idx_pos_vec.sort_by(|a, b| a.1.cmp(&b.1));

        // dbg!(&idx_pos_vec);

        // 计算space
        let mut last_pos = 0;
        // let mut n_space = 0;
        let mut space_pos_len = vec![(0usize, 0usize); 0];
        for i in 0..n_file {
            let space_size = idx_pos_vec[i].1 - last_pos;
            if space_size != 0 {
                space_pos_len.push((last_pos, space_size));
            }
            let file_size = input[idx_pos_vec[i].0 * 2] as usize;
            last_pos = idx_pos_vec[i].1 + file_size;
        }

        // dbg!(&space_pos_len);

        let mut find_file_and_space = || {
            for i in (0..last_id).rev() {
                // let (file_id, file_pos) = idx_pos_vec[i];
                let file_id = i;
                let file_pos = files_pos[file_id];
                let file_size = input[file_id * 2] as usize;
                // dbg!(file_id, file_pos, file_size);

                let res = 'outer: loop {
                    for j in 0..space_pos_len.len() {
                        let (space_pos, space_size) = space_pos_len[j];
                        // dbg!(j, space_pos, space_size);
                        if space_pos > file_pos {
                            break 'outer None;
                        }
                        if space_size >= file_size {
                            break 'outer Some(space_pos);
                        }
                    }
                    break None;
                };

                if let Some(space_pos) = res {
                    last_id = file_id;
                    files_pos[file_id] = space_pos;
                    return true;
                }
            }
            false
        };

        if !find_file_and_space() {
            break;
        }
        // dbg!(debug_files_pos(&files_pos, input));
    }
    // dbg!(debug_files_pos(&files_pos, input));

    let mut res = BigInt::from(0);

    // for i in 0..n_file {

    // }
    // dbg!(&files_pos);
    // dbg!(debug_files_pos(&files_pos, input));

    for i in 0..n_file {
        let file_size = input[i * 2] as usize;
        let file_pos = files_pos[i];
        // dbg!(i, file_pos, file_size);
        res += (file_pos + file_pos + file_size - 1) * file_size * i / 2;
    }

    res.to_string()
}

fn main() {
    let input = "2333133121414131402";
    // let file = Path::new("data/2409/input");
    // let input = fs::read_to_string(file).unwrap();

    let mut input = input
        .chars()
        .map(|c| c as u8 - '0' as u8)
        .collect::<Vec<u8>>();
    println!("{}", exec2(&mut input))
}
