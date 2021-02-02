//! Helper functions for bitwise operations and extracting bitfields

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
