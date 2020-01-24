use super::Memory;

use crate::util::*;

impl Memory
{
    /// Take index to palette, return 0RGD u32 color.
    /// This function contain unsafe code because is frequently called in
    /// rendering loops. However, it will NOT function properly on a big
    /// endian system.
    #[inline]
    pub fn palette(&self, index: u32) -> u32
    {
        // Object palette starts at 0x05000200
        // let offset = (index + if obj {0x100} else {0}) as usize;

        unsafe
        {
            let ptr = self.param.as_ptr() as *const u16;

            RGB(*ptr.add(index as usize))
        }
    }
}

#[inline]
fn RGB(a: u16) -> u32
{
    let r = a.bits(4, 0) << 19;
    let g = a.bits(9, 5) << 11;
    let b = a.bits(14, 10) << 3;

    r | g | b
}