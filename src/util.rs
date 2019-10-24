//! Helper functions for bitwise operations

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
}