use crate::{u2, u4};

pub trait BitCount {
    const BIT_COUNT: usize;
}
pub trait ParseHex: Sized {
    fn parse_hex(str: &str) -> Option<Self>;
}
pub trait ParseBin: Sized {
    fn parse_bin(str: &str) -> Option<Self>;
}

impl<T: BitCount + TryFrom<usize>> ParseBin for T {
    fn parse_bin(str: &str) -> Option<Self> {
        let mut num: usize = 0;
        let mut i: usize = 0;
        for bit in str.chars() {
            match bit {
                '0' => {
                    i += 1;
                }
                '1' => {
                    num |= 0b1000_0000 >> i;
                    i += 1;
                }
                '_' => {}
                _ => {
                    return None;
                }
            }
        }
        if i >= T::BIT_COUNT {
            return None;
        }
        T::try_from(num).ok()
    }
}
impl<T: BitCount + TryFrom<usize>> ParseHex for T {
    fn parse_hex(str: &str) -> Option<Self> {}
}

impl BitCount for u8 {
    const BIT_COUNT: usize = 8;
}
impl BitCount for u4 {
    const BIT_COUNT: usize = 8;
}
impl BitCount for u2 {
    const BIT_COUNT: usize = 8;
}

impl UnsignedNum for u8 {
    const THEORETICAL_SIZE: usize = 8;
    const ZERO: Self = 0;

    fn parse_hex(str: &str) -> Option<Self> {
        if str.len() > 2 {
            return None;
        }

        u8::from_str_radix(str, 16).ok()
    }
}
impl UnsignedNum for u4 {
    const THEORETICAL_SIZE: usize = 4;
    const ZERO: Self = u4::ZERO;

    fn parse_hex(str: &str) -> Option<Self> {
        if str.len() != 1 {
            return None;
        }
        for char in str.chars() {
            return char.to_digit(16).map(|val| u4::from_u32(val));
        }
        return None;
    }
}
impl UnsignedNum for u2 {
    const THEORETICAL_SIZE: usize = 2;
    const ZERO: Self = Self::ZERO;

    fn parse_hex(str: &str) -> Option<Self> {
        if str.len() != 1 {
            return None;
        }
        match str {
            "0" => Some(u2::ZERO),
            "1" => Some(u2::ONE),
            "2" => Some(u2::TOW),
            "3" => Some(u2::THREE),
            _ => None,
        }
    }
}
