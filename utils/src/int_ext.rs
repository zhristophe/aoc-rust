pub trait IntegerExt {
    fn digit_count(&self) -> u32;
}

trait UnsignedAbs {
    fn unsigned_abs(&self) -> Self;
}

macro_rules! impl_unsigned_abs {
    ($($t:ty),*) => {
        $(
            impl UnsignedAbs for $t {
                fn unsigned_abs(&self) -> Self {
                    *self
                }
            }
        )*
    }
}

impl_unsigned_abs!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_int_ext {
    ($($t:ty),*) => {
        $(
            impl IntegerExt for $t {
                fn digit_count(&self) -> u32 {
                    if *self == 0 {
                        1
                    } else {
                        self.unsigned_abs().ilog10() + 1
                    }
                }
            }
        )*
    }
}

impl_int_ext!(
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(0_i32.digit_count(), 1);
        assert_eq!(5_i32.digit_count(), 1);
        assert_eq!(10_usize.digit_count(), 2);
        assert_eq!(99_u8.digit_count(), 2);
        assert_eq!((-100_i32).digit_count(), 3);

        assert_eq!(i32::MIN.digit_count(), 10);
    }
}
