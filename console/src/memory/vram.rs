use crate::ppu::PPU;

use super::Memory;

impl Memory {
    pub fn vram_load8(&self, offset: usize) -> u8 {
        self.c().ppu.vram_load8(offset)
    }

    pub fn vram_load16(&self, offset: usize) -> u16 {
        let lo = self.vram_load8(offset) as u16;
        let hi = self.vram_load8(offset + 1) as u16;
        (hi << 8) | lo
    }

    pub fn vram_load32(&self, offset: usize) -> u32 {
        let lo = self.vram_load16(offset) as u32;
        let hi = self.vram_load16(offset + 2) as u32;
        (hi << 16) | lo
    }

    pub fn vram_store8(&mut self, offset: usize, value: u8) {
        self.c().ppu.vram_store8(offset, value);
    }

    pub fn vram_store16(&mut self, offset: usize, value: u16) {
        let halfword = value.to_le_bytes();
        self.vram_store8(offset, halfword[0]);
        self.vram_store8(offset + 1, halfword[1]);
    }

    pub fn vram_store32(&mut self, offset: usize, value: u32) {
        let word = value.to_le_bytes();
        self.vram_store8(offset, word[0]);
        self.vram_store8(offset + 1, word[1]);
        self.vram_store8(offset + 2, word[2]);
        self.vram_store8(offset + 3, word[3]);
    }
}

impl PPU {
    pub fn vram_load8(&mut self, offset: usize) -> u8 {
        self.vram[offset]
    }

    pub fn vram_store8(&mut self, offset: usize, value: u8) {
        self.vram[offset] = value;
    }

    /// Return a byte from vram, offset is in bytes
    #[inline]
    pub fn vram8(&self, offset: u32) -> u8 {
        self.vram[offset as usize]
    }

    /// Return a halfword from vram, offset is in bytes
    #[inline]
    pub fn vram16(&self, offset: u32) -> u16 {
        let a = offset as usize;
        Memory::into16(&self.vram[a..a + 2])
    }

    #[inline]
    pub fn tile_data(
        &self,
        palette_f: bool,
        tile_b: u32,
        tile_n: u32,
        pixel_x: u32,
        pixel_y: u32,
    ) -> u32 {
        if palette_f {
            self.tile_data8(tile_b, tile_n, pixel_x, pixel_y)
        } else {
            self.tile_data4(tile_b, tile_n, pixel_x, pixel_y)
        }
    }

    #[inline]
    pub fn tile_data4(&self, tile_b: u32, tile_n: u32, pixel_x: u32, pixel_y: u32) -> u32 {
        let b = self.vram8(tile_b * 0x4000 + tile_n * 32 + pixel_y * 4 + pixel_x / 2);

        // Take upper nibble if pixel_x is odd else lower nibble
        if pixel_x & 1 == 1 {
            b as u32 >> 4
        } else {
            b as u32 & 0x0f
        }
    }

    #[inline]
    pub fn tile_data8(&self, tile_b: u32, tile_n: u32, pixel_x: u32, pixel_y: u32) -> u32 {
        let b = self.vram8(tile_b * 0x4000 + tile_n * 64 + pixel_y * 8 + pixel_x);

        b as u32
    }

    /// Return tile map entry
    #[inline]
    pub fn text_tile_map(&self, index: u32, size_r: u32, tile_x: u32, tile_y: u32) -> u16 {
        let offset = map_entry(size_r, tile_y, tile_x) * 2;

        self.vram16(index * 0x800 + offset)
    }

    //     #[inline]
    //     pub fn affine_tile_map(&self, index: u32, size_r: u32, tile_x: u32, tile_y: u32) -> u8
    //     {
    //         let offset = tile_y * (16 << size_r) + tile_x;

    //         self.vram8(index * 0x800 + offset)
    //     }
}

// Referenced from TONC GBA
#[inline]
pub fn map_entry(size_r: u32, tile_y: u32, tile_x: u32) -> u32 {
    let mut n = tile_y * 32 + tile_x;

    if tile_x >= 32 {
        n += 0x03e0;
    }

    if tile_y >= 32 && size_r == 0b11
    // 64x64
    {
        n += 0x0400;
    }

    n
}
