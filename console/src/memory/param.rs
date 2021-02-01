use crate::ppu::PPU;
use crate::ppu::TRANSPARENT;

use super::Memory;

impl Memory {
    pub fn param_load8(&self, offset: usize) -> u8 {
        let value = self.param_load16(offset);
        value.to_le_bytes()[offset as usize & 1]
    }

    pub fn param_load16(&self, offset: usize) -> u16 {
        self.c().ppu.param_load16(offset)
    }

    pub fn param_load32(&self, offset: usize) -> u32 {
        let lo = self.param_load16(offset) as u32;
        let hi = self.param_load16(offset + 2) as u32;
        (hi << 16) | lo
    }

    pub fn param_store16(&self, offset: usize, value: u16) {
        self.c().ppu.param_store16(offset, value);
    }

    pub fn param_store32(&mut self, offset: usize, value: u32) {
        self.param_store16(offset, value as u16);
        self.param_store16(offset + 2, (value >> 16) as u16);
    }
}

impl PPU {
    /// Color 0 of palette 0 is the backdrop color
    #[inline]
    pub fn backdrop(&self) -> u16 {
        self.palette[0]
    }

    /// Take index to palette, return 0RGD u32 color.
    #[inline]
    pub fn bg_palette(&self, palette_n: u32, index: u32) -> u16 {
        if index == 0 {
            return TRANSPARENT;
        }

        self.palette[(palette_n << 4 | index) as usize]
    }

    #[inline]
    pub fn obj_palette(&self, palette_n: u32, index: u32) -> u16 {
        if index == 0 {
            return TRANSPARENT;
        }

        self.palette[(palette_n << 4 | index) as usize + 0x100]
    }

    #[inline]
    pub fn param_load16(&self, offset: usize) -> u16 {
        self.palette[offset / 2]
    }

    pub fn param_store16(&mut self, offset: usize, value: u16) {
        self.palette[offset / 2] = value;
    }
}
