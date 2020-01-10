//! Helper functions for bitwise operations and extracting bitfields

pub trait BitField
{
    fn bit(self, b: u32) -> bool;
    fn bits(self, hi: u32, lo: u32) -> u32;
}

impl BitField for u32
{
    /// Return certain bits of a word as unsigned integer
    /// ```
    /// assert_eq!(0xaabbccdd.bits(15, 8), 0xcc);
    /// ```
    #[inline]
    fn bits(self, hi: u32, lo: u32) -> u32
    {
        debug_assert!(hi < 32);
        debug_assert!(lo < hi);
    
        ((self >> lo) & ((1 << (hi - lo + 1)) - 1)) as u32
    }
    
    /// Test certains bit of a word, return true if set
    /// ```
    /// assert_eq!(0b10.bit(1), true)
    /// ```
    #[inline]
    fn bit(self, b: u32) -> bool
    {
        debug_assert!(b < 32);
    
        self & (1 << b) == (1 << b)
    }
}

impl BitField for u16
{
    
    /// Return certain bits of a word as unsigned integer
    /// ```
    /// assert_eq!(0xaabbccdd.bits(15, 8), 0xcc);
    /// ```
    #[inline]
    fn bits(self, hi: u32, lo: u32) -> u32
    {
        debug_assert!(hi < 16);
        debug_assert!(lo < hi);
    
        ((self >> lo) & ((1 << (hi - lo + 1)) - 1)) as u32
    }
    
    /// Test certains bit of a word, return true if set
    /// ```
    /// assert_eq!(0b10.bit(1), true)
    /// ```
    #[inline]
    fn bit(self, b: u32) -> bool
    {
        debug_assert!(b < 16);
    
        self & (1 << b) == (1 << b)
    }
}

/// Return certain bits of a word as unsigned integer
/// ```
/// assert_eq!(bits(0xaabbccdd, 15, 8), 0xcc);
/// ```
#[inline]
pub fn bits(a: u32, hi: u32, lo: u32) -> u32
{
    debug_assert!(hi < 32);
    debug_assert!(lo < hi);

    (a >> lo) & ((1 << (hi - lo + 1)) - 1)
}

/// Test certains bit of a word, return true if set
/// ```
/// assert_eq!(bit(0b10, 1), true)
/// ```
#[inline]
pub fn bit(a: u32, b: u32) -> bool
{
    debug_assert!(b < 32);

    a & (1 << b) == (1 << b)
}

/// Sign extend a word.
/// `s` is the place of the most significant bit.
#[inline]
pub fn sign_extend(a: u32, s: u32) -> i32
{
    debug_assert!(s < 32);

    if bit(a, s)
    {
        let extension = !((1 << s) - 1);
        (a | extension) as i32
    }
    else
    {
        a as i32
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_bits()
    {
        assert_eq!(bits(0b11110, 4, 1), 0b1111);
    }

    #[test]
    fn test_bit()
    {
        assert_eq!(bit(0x80000000, 31), true);
    }

    #[test]
    fn test_sign_extend()
    {
        assert_eq!(sign_extend(0b10, 1), -2);
    }
}