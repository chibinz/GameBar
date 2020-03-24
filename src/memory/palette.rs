use crate::ppu::color::*;

use super::Memory;
use super::into16;

impl Memory
{
    /// Take index to palette, return 0RGD u32 color.
    #[inline]
    pub fn bg_palette(&self, palette_n: u32, index: u32) -> u32
    {
        // Color 0 of palette 0 is the backdrop color
        if index == 0 && palette_n != 0 {return TRANSPARENT}

        let a = (palette_n << 4 | index) as usize * 2;
        RGB(into16(&self.param[a..a+2]))
    }

    #[inline]
    pub fn obj_palette(&self, palette_n: u32, index: u32) -> u32
    {
        if index == 0 {return TRANSPARENT}

        let a = (palette_n << 4 | index) as usize * 2 + 0x200;
        RGB(into16(&self.param[a..a+2]))
    }
}