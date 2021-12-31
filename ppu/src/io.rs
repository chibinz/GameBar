use crate::Ppu;
use crate::TRANSPARENT;
use util::*;

/// Video ram access
impl Ppu {
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
        util::into16(&self.vram[a..a + 2])
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

/// Palette memory access
impl Ppu {
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

impl Ppu {
    pub fn get_dispcnt(&self) -> u16 {
        self.dispcnt
    }

    pub fn set_dispcnt(&mut self, value: u16) {
        self.dispcnt = value;
        self.mode = value.bits(2, 0);
        self.flip = value.bit(4);
        self.sequential = value.bit(6);
        self.fblank = value.bit(7);
    }

    pub fn get_dispstat(&self) -> u16 {
        self.dispstat
    }

    pub fn set_dispstat(&mut self, value: u16) {
        self.dispstat = value;
    }

    pub fn get_vcount(&self) -> u16 {
        self.vcount
    }
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
