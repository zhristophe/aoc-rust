pub trait ParseExt {
    fn to_i32_unsafe(&self) -> i32;
}

impl ParseExt for [u8] {
    fn to_i32_unsafe(&self) -> i32 {
        let mut res = 0;
        for &c in self {
            res = res * 10 + (c - b'0') as i32;
        }

        res
    }
}
