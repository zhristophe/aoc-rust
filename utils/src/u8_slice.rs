pub trait U8SliceExt {
    fn to_string_unsafe(&self) -> String;
    fn to_u32_unsafe(&self) -> u32;
    fn to_u64_unsafe(&self) -> u64;
    fn split_by_byte(&self, b: u8) -> impl Iterator<Item = &[u8]>;
    fn splite_once(&self, b: u8) -> Option<(&[u8], &[u8])>;
}

impl U8SliceExt for [u8] {
    fn to_string_unsafe(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.to_vec()) }
    }

    fn to_u32_unsafe(&self) -> u32 {
        let mut res = 0;
        for &c in self {
            res = res * 10 + (c - b'0') as u32;
        }

        res
    }

    fn to_u64_unsafe(&self) -> u64 {
        let mut res = 0;
        for &c in self {
            res = res * 10 + (c - b'0') as u64;
        }

        res
    }

    fn split_by_byte(&self, b: u8) -> impl Iterator<Item = &[u8]> {
        self.split(move |c| c == &b)
    }

    fn splite_once(&self, b: u8) -> Option<(&[u8], &[u8])> {
        for i in 0..self.len() {
            if self[i] == b {
                return Some((&self[..i], &self[i + 1..]));
            }
        }

        None
    }
}
