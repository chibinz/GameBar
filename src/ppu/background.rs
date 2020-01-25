use crate::util::*;
use crate::ppu::PPU;
use crate::memory::Memory;

impl PPU
{
    pub fn render_background(&mut self, memory: &Memory) 
    {
        // 256 * 256 pixels background
        // 8 * 8 pixels tile
        // 32 * 32 tiles
        // tile y

        // Tile row
        for i in 0..32
        {
            // Tile column
            for j in 0..32
            {
                self.render_text_tile(i, j, memory);
            }
        }
    }

    #[inline]
    pub fn render_text_tile(&mut self, y: usize, x: usize, memory: &Memory)
    {
        let tile_entry = memory.tile_map(0x800, x, y);

        let tile_number     = tile_entry.bits(9, 0);
        let horizontal_flip = tile_entry.bit(10);
        let vertical_flip   = tile_entry.bit(11);
        let palette_number  = tile_entry.bits(15, 12) << 4;

        // Pixel row
        for i in 0..8
        {
            let row = memory.tile_row32(0x4000, tile_number, i as u32);
            let r = if vertical_flip {7 - i} else {i};

            // Pixel column
            for j in 0..8
            {
                let palette = row.bits((8 - j as u32) * 4 - 1, (7 - j as u32) * 4);
                let c = if horizontal_flip {7 - j} else {j};

                self.buffer[x * 8 * 256 + y * 8 + r * 256 + c] = 
                    memory.palette(palette_number | palette);
            }
        }
    }
}