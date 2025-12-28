pub trait U8SliceExt {
    fn as_str_unchecked(&self) -> &str;
    fn parse_u32_unsafe(&self) -> u32;
    fn parse_u64_unsafe(&self) -> u64;
    fn split_by_byte(&self, b: u8) -> impl Iterator<Item = &[u8]>;
    fn split_once_by_byte(&self, b: u8) -> Option<(&[u8], &[u8])>;
}

impl U8SliceExt for [u8] {
    fn as_str_unchecked(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self) }
    }

    fn parse_u32_unsafe(&self) -> u32 {
        let mut res = 0;
        for &c in self {
            res = res * 10 + (c - b'0') as u32;
        }

        res
    }

    fn parse_u64_unsafe(&self) -> u64 {
        let mut res = 0;
        for &c in self {
            res = res * 10 + (c - b'0') as u64;
        }

        res
    }

    fn split_by_byte(&self, b: u8) -> impl Iterator<Item = &[u8]> {
        self.split(move |c| c == &b)
    }

    fn split_once_by_byte(&self, b: u8) -> Option<(&[u8], &[u8])> {
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
    fn test_as_str_unchecked() {
        assert_eq!(b"hello".as_str_unchecked(), "hello");
        assert_eq!(b"".as_str_unchecked(), "");
    }

    #[test]
    fn test_parse_u32() {
        assert_eq!(b"123".parse_u32_unsafe(), 123);
        assert_eq!(b"0".parse_u32_unsafe(), 0);
        assert_eq!(b"999999".parse_u32_unsafe(), 999999);
    }

    #[test]
    fn test_parse_u64() {
        assert_eq!(b"12345678901234".parse_u64_unsafe(), 12345678901234);
    }

    #[test]
    fn test_split_by_byte() {
        let s = b"a,b,c";
        let v: Vec<_> = s.split_by_byte(b',').collect();
        assert_eq!(v, vec![b"a".as_slice(), b"b".as_slice(), b"c".as_slice()]);
    }

    #[test]
    fn test_split_once_byte() {
        assert_eq!(
            b"a-b".split_once_by_byte(b'-'),
            Some((b"a".as_slice(), b"b".as_slice()))
        );
        assert_eq!(b"abc".split_once_by_byte(b'-'), None);
    }
}
