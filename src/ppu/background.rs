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
                self.render_tile(i, j, memory);
            }
        }
    }

    #[inline]
    pub fn render_tile(&mut self, y: usize, x: usize, memory: &Memory)
    {
        let map_entry = memory.tile_map(0x800, x, y);
        let tile_number = (map_entry & 0x3ff) as u32;

        // Pixel row
        for i in 0..8
        {
            let row = memory.tile_row32(0x4000, tile_number, i as u32);
            
            // Pixel column
            for j in 0..8
            {
                let pixel = row.bits((8 - j as u32) * 4 - 1, (7 - j as u32) * 4);

                self.buffer[x * 8 * 256 + y * 8 + i * 256 + j] = memory.palette(pixel);
            }
        }
    }
}