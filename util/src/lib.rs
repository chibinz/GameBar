//! Helper functions for bitwise operations and extracting bitfields
use std::convert::TryInto;

pub use log::*;

pub trait BitField {
    fn bit(self, b: u32) -> bool;
    fn bits(self, hi: u32, lo: u32) -> u32;
}

impl<T: std::convert::Into<u32>> BitField for T {
    /// Return certain bits of a integer as u32
    /// ```
    /// use util::BitField;
    /// assert_eq!(0xaabbccddu32.bits(15, 8), 0xcc);
    /// ```
    #[inline]
    fn bits(self, hi: u32, lo: u32) -> u32 {
        ((self.into() >> lo) & ((1 << (hi - lo + 1)) - 1)) as u32
    }

    /// Test certains bit of a integer, return true if set
    /// ```
    /// use util::BitField;
    /// assert_eq!(0b10u32.bit(1), true)
    /// ```
    #[inline]
    fn bit(self, b: u32) -> bool {
        self.into() & (1 << b) == (1 << b)
    }
}

pub trait Color {
    fn to_rgb24(self) -> u32;
}

impl Color for u16 {
    /// RGB15 -> RGB24 conversion
    fn to_rgb24(self) -> u32 {
        let r = self.bits(4, 0) << 19;
        let g = self.bits(9, 5) << 11;
        let b = self.bits(14, 10) << 3;

        r | g | b
    }
}

/// Sign extend a word.
/// `s` is the place of the most significant bit.
#[inline]
pub fn sign_extend(a: u32, s: u32) -> i32 {
    debug_assert!(s < 32);

    if a.bit(s) {
        let extension = !((1 << s) - 1);
        (a | extension) as i32
    } else {
        a as i32
    }
}

pub trait Bus
 {
    #[inline]
    #[allow(unused_variables)]
    fn load8(&self, address: u32) -> u8 {
        unimplemented!()
    }
    #[inline]
    fn load16(&self, address: u32) -> u16 {
        let lo = self.load8(address);
        let hi = self.load8(address + 1);
        u16::from_le_bytes([lo, hi])
    }
    #[inline]
    fn load32(&self, address: u32) -> u32 {
        let lo = self.load16(address);
        let hi = self.load16(address + 2);
        (hi as u32) << 16 | (lo as u32)
    }
    #[inline]
    #[allow(unused_variables)]
    fn store8(&mut self, address: u32, value: u8) {
        unimplemented!()
    }
    #[inline]
    fn store16(&mut self, address: u32, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.store8(address, lo);
        self.store8(address + 1, hi);
    }
    #[inline]
    fn store32(&mut self, address: u32, value: u32) {
        let lo = value as u16;
        let hi = (value >> 16) as u16;
        self.store16(address, lo);
        self.store16(address + 2, hi);
    }
}

impl Bus for [u8] {
    fn load8(&self, address: u32) -> u8 {
        self[address as usize]
    }
    fn load16(&self, address: u32) -> u16 {
        let a = address as usize;
        u16::from_le_bytes(self[a..a + 2].try_into().unwrap())
    }
    fn load32(&self, address: u32) -> u32 {
        let a = address as usize;
        u32::from_le_bytes(self[a..a + 4].try_into().unwrap())
    }
    fn store8(&mut self, address: u32, value: u8) {
        self[address as usize] = value
    }
    fn store16(&mut self, address: u32, value: u16) {
        let a = address as usize;
        self[a..a + 2].copy_from_slice(&value.to_le_bytes());
    }
    fn store32(&mut self, address: u32, value: u32) {
        let a = address as usize;
        self[a..a + 4].copy_from_slice(&value.to_le_bytes());
    }
}

#[inline]
pub fn into16(a: &[u8]) -> u16 {
    u16::from_le_bytes(a[0..2].try_into().unwrap())
}

#[inline]
pub fn into32(a: &[u8]) -> u32 {
    u32::from_le_bytes(a[0..4].try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits() {
        assert_eq!(0b11110u32.bits(4, 1), 0b1111);
    }

    #[test]
    fn test_bit() {
        assert_eq!(0x80000000u32.bit(31), true);
    }

    #[test]
    fn test_sign_extend() {
        assert_eq!(sign_extend(0b10, 1), -2);
    }
}
