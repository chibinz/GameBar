use crate::util::*;

use super::Memory;
use super::into16;

impl Memory
{
    /// Take index to palette, return 0RGD u32 color.
    #[inline]
    pub fn palette(&self, index: u32) -> u32
    {
        let a = index as usize * 2;
        RGB(into16(&self.param[a..a+2]))
    }
}

#[inline]
pub fn RGB(a: u16) -> u32
{
    let r = a.bits(4, 0) << 19;
    let g = a.bits(9, 5) << 11;
    let b = a.bits(14, 10) << 3;

    r | g | b
}