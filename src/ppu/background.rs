use crate::util::*;
use crate::ppu::PPU;
use crate::memory::Memory;



impl PPU
{
    pub fn render_mode_0(&mut self, memory: &mut Memory) 
    {
        // 256 * 256 pixels background
        // 8 * 8 pixels tile
        // 32 * 32 tiles
        // tile y
        let tile_data_address: u32 = 0x06004000;
        let tile_map_address: u32 = 0x06000800;
        let palette_address: u32 = 0x05000000;

        for i in 0..32
        {
            // tile x
            for j in 0..32
            {
                let map_entry = memory.load16(tile_map_address + ((i as u32) * 32 + (j as u32)) * 2);
                let tile_number = (map_entry & 0x3ff) as u32;

                let tile_offset = tile_data_address + tile_number * 32;
                // pixel y
                for k in 0..8
                {
                    // pixel x
                    for l in 0..4
                    {
                        let tile_data = memory.load8(tile_offset + (k as u32) * 4 + l as u32) as u32;
                        let left = tile_data.bits(3, 0);
                        let right = tile_data.bits(7, 4);
                        let leftr = memory.load16(palette_address + left * 2);
                        let rightr = memory.load16(palette_address + right * 2);

                        self.buffer[i * 8 * 256 + j * 8 + k * 256 + l * 2] = RGB(leftr);
                        self.buffer[i * 8 * 256 + j * 8 + k * 256 + l * 2 + 1] = RGB(rightr);
                    }
                }
            }
        }
    }
}

fn RGB(a: u16) -> u32
{
    let r = a.bits(4, 0) << 19;
    let g = a.bits(9, 5) << 11;
    let b = a.bits(14, 10) << 3;

    r | g | b
}