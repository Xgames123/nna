use crate::{u2, u4};

pub trait MaxValue {
    const MAX_VALUE: u64;

    const BIT_COUNT: u8 = Self::MAX_VALUE.count_ones() as u8;
}
pub trait ParseHex: Sized {
    fn parse_hex(str: &str) -> Option<Self>;
}
pub trait ParseBin: Sized {
    fn parse_bin(str: &str) -> Option<Self>;
}

pub trait BitOps: Sized {
    fn set_bit(self, n: usize) -> Self;
}

impl<T: MaxValue + TryFrom<u64>> ParseBin for T {
    fn parse_bin(str: &str) -> Option<Self> {
        let str = str.strip_prefix("0b").unwrap_or(str);
        let mut num: u64 = 0;
        let mut i: u64 = 0;
        for bit in str.chars().rev() {
            match bit {
                '0' => {
                    i += 1;
                }
                '1' => {
                    num |= 0b1 << i;
                    i += 1;
                }
                '_' => {}
                _ => {
                    return None;
                }
            }
        }
        if num > T::MAX_VALUE {
            return None;
        }
        T::try_from(num).ok()
    }
}
impl<T: MaxValue + TryFrom<u64>> ParseHex for T {
    fn parse_hex(str: &str) -> Option<Self> {
        let str = str.strip_prefix("0x").unwrap_or(str);
        let mut num: u64 = 0;
        let mut i: u64 = 0;
        for char in str.chars().rev() {
            if char == '_' {
                continue;
            }
            num |= (char.to_digit(16)? as u64) << i;
            i += 4;
        }
        if num > T::MAX_VALUE {
            return None;
        }
        T::try_from(num).ok()
    }
}

impl MaxValue for u64 {
    const MAX_VALUE: u64 = u64::MAX;
}
impl MaxValue for u32 {
    const MAX_VALUE: u64 = u32::MAX as u64;
}
impl MaxValue for u16 {
    const MAX_VALUE: u64 = u16::MAX as u64;
}
impl MaxValue for u8 {
    const MAX_VALUE: u64 = u8::MAX as u64;
}
impl MaxValue for u4 {
    const MAX_VALUE: u64 = u4::MAX.into_low() as u64;
}
impl MaxValue for u2 {
    const MAX_VALUE: u64 = u2::MAX.into_low() as u64;
}

#[cfg(test)]
mod test {
    use crate::u4;

    use super::{ParseBin, ParseHex};

    #[test]
    fn parse_bin() {
        assert_eq!(
            u64::parse_bin(
                "1000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010_0000_0000"
            ),
            Some(0b1000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010_0000_0000)
        );

        assert_eq!(
            u32::parse_bin("1000_1000_0000_0100_0000_0000_0000_1000"),
            Some(0b1000_1000_0000_0100_0000_0000_0000_1000)
        );

        assert_eq!(
            u16::parse_bin("1000_1000_0000_0100",),
            Some(0b1000_1000_0000_0100)
        );

        assert_eq!(u8::parse_bin("1000_0000",), Some(0b1000_0000));
        assert_eq!(u4::parse_bin("1100"), Some(u4::from_low(0b1100)));

        assert_eq!(u32::parse_bin("01"), Some(0b01));

        assert_ne!(u4::parse_bin("1100_00"), Some(u4::from_low(0b1100)));

        assert_eq!(u4::parse_bin("00_1100"), Some(u4::from_low(0b1100)));
        assert_eq!(u4::parse_bin("0001"), Some(u4::from_low(0b1)));
        assert_eq!(u4::parse_bin("1"), Some(u4::from_low(0b0001)));

        assert_eq!(u4::parse_bin("111111111111111"), None);

        assert_eq!(
            u16::parse_bin("0b1000_1000_0000_0100",),
            Some(0b1000_1000_0000_0100)
        );
    }

    #[test]
    fn parse_hex() {
        assert_eq!(
            u64::parse_hex("10AB_20F0_1090_FFD1"),
            Some(0x10AB_20F0_1090_FFD1)
        );

        assert_eq!(u32::parse_hex("10AB_20F0"), Some(0x10AB_20F0));

        assert_eq!(u16::parse_hex("10AB"), Some(0x10AB));

        assert_eq!(u8::parse_hex("AB"), Some(0xAB));
        assert_eq!(u8::parse_hex("69"), Some(0x69));

        assert_eq!(u8::parse_hex("0x69"), Some(0x69));

        assert_eq!(u32::parse_hex("0x6"), Some(0x6));
        assert_eq!(u4::parse_hex("0x6"), Some(u4::from_low(0x6)));

        assert_eq!(u4::parse_hex("0x61"), None);
    }
}
