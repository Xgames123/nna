use core::fmt;
use std::fmt::LowerHex;

#[macro_export]
macro_rules! u2 {
    ($val:literal) => {
        u2::from_low($val)
    };
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct u2(u8);
impl u2 {
    pub const ZERO: u2 = u2::from_low(0b00000000);
    pub const ONE: u2 = u2::from_low(0b00000001);
    pub const TOW: u2 = u2::from_low(0b00000010);
    pub const THREE: u2 = u2::from_low(0b00000011);

    #[inline]
    pub const fn from_low(val: u8) -> Self {
        Self(val & 0b00000011)
    }
    #[inline]
    pub const fn into_low(self) -> u8 {
        self.0
    }
    #[inline]
    pub const fn into_high(self) -> u8 {
        self.0 << 6
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct u4(u8);

impl u4 {
    pub const ZERO: u4 = u4::from_low(0x00);
    pub const ONE: u4 = u4::from_low(0x01);

    #[inline]
    pub const fn from_u32(val: u32) -> Self {
        Self::from_low(val as u8)
    }

    #[inline]
    pub const fn from_low(val: u8) -> Self {
        Self(val & 0x0F)
    }

    #[inline]
    pub const fn from_high(val: u8) -> Self {
        Self::from_low(val >> 4)
    }

    #[inline]
    pub const fn into_high(self) -> u8 {
        self.0 << 4
    }
    #[inline]
    pub const fn into_low(self) -> u8 {
        self.0
    }
    #[inline]
    pub const fn into_u32(self) -> u32 {
        self.0 as u32
    }
    #[inline]
    pub const fn into_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn overflowing_add(self, other: u4) -> u4 {
        Self::from_low(self.0.overflowing_add(other.0).0)
    }
    #[inline]
    pub fn overflowing_sub(self, other: u4) -> u4 {
        Self::from_low(self.0.overflowing_sub(other.0).0)
    }
    #[inline]
    pub fn overflowing_mul(self, other: u4) -> u4 {
        Self::from_low(self.0.overflowing_mul(other.0).0)
    }
}
impl LowerHex for u4 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}
