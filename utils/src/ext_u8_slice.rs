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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_unsafe() {
        assert_eq!(b"hello".to_string_unsafe(), "hello");
        assert_eq!(b"".to_string_unsafe(), "");
    }

    #[test]
    fn test_to_u32_unsafe() {
        assert_eq!(b"123".to_u32_unsafe(), 123);
        assert_eq!(b"0".to_u32_unsafe(), 0);
        assert_eq!(b"999999".to_u32_unsafe(), 999999);
    }

    #[test]
    fn test_to_u64_unsafe() {
        assert_eq!(b"12345678901234".to_u64_unsafe(), 12345678901234);
    }

    #[test]
    fn test_split_by_byte() {
        let s = b"a,b,c";
        let v: Vec<_> = s.split_by_byte(b',').collect();
        assert_eq!(v, vec![b"a".as_slice(), b"b".as_slice(), b"c".as_slice()]);
    }

    #[test]
    fn test_splite_once() {
        assert_eq!(
            b"a-b".splite_once(b'-'),
            Some((b"a".as_slice(), b"b".as_slice()))
        );
        assert_eq!(b"abc".splite_once(b'-'), None);
    }
}
