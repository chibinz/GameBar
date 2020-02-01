use crate::util::*;
use crate::ppu::PPU;
use crate::memory::Memory;
use crate::memory::palette::RGB;

pub struct Background
{
    pub pixel   : Vec<u32>,   // Line buffer
    pub index   : usize,      // 0 - 3
    pub priority: u32,
    pub width   : u32,
    pub height  : u32,
}

impl Background
{
    pub fn new(i: usize) -> Self
    {
        Self
        {
            pixel   : vec![0; 1024],   // The largest width of a background is 1024 pixels
            index   : i,
            priority: 0,
            width   : 0,               // Width in tiles, x8 to get number of pixels
            height  : 0,               // Height in tiles
        }
    }

    pub fn draw_text(&mut self, memory: &Memory)
    {
        let bgcnt = memory.get_bgcnt(self.index);

        let s = bgcnt.bits(3, 2);
        // let c = bgcnt.bit(6);
        // let a = bgcnt.bit(7);
        let m = bgcnt.bits(12, 8);

        self.priority = bgcnt.bits(1, 0);
        self.width = get_size(bgcnt, false).0;
        self.height = get_size(bgcnt, false).1;

        let line_n = (memory.get_vcount() + memory.get_bgvofs(self.index)) as u32;
        let tile_y = (line_n / 8) % self.height;
        
        // Tile column
        for tile_x in 0..self.width
        {
            let tile_entry = memory.tile_map(m, tile_y, tile_x);

            let tile_number     = tile_entry.bits(9, 0);
            let horizontal_flip = tile_entry.bit(10);
            let vertical_flip   = tile_entry.bit(11);
            let palette_number  = tile_entry.bits(15, 12) << 4;
    
            let r = if vertical_flip {7 - (line_n % 8)} else {line_n % 8};
            let row = memory.tile_row32(s, tile_number, r);

            // Pixel column
            for j in 0..8
            {
                let palette = row.bits((8 - j as u32) * 4 - 1, (7 - j as u32) * 4);
                let c = if !horizontal_flip {7 - j} else {j};
                
                self.pixel[(tile_x * 8 + c) as usize] = 
                    memory.palette(palette_number | palette);
            }
        }
    }
    
    pub fn draw_bitmap3(&mut self, memory: &Memory)
    {
        let line_n = (memory.get_vcount() + memory.get_bgvofs(self.index)) as u32;

        for x in 0..240
        {
            let pixel = memory.load16(0x06000000 + (line_n * 240 + x) * 2);
            
            self.pixel[x as usize] = RGB(pixel);
        }
    }

    pub fn draw_bitmap4(&mut self, memory: &Memory)
    {
        let line_n = (memory.get_vcount() + memory.get_bgvofs(self.index)) as u32;
        
        for x in 0..240
        {
            let palette_entry = memory.load8(0x06000000 + line_n * 240 + x);
            self.pixel[x as usize] = memory.palette(palette_entry as u32);
        }
    }
}

#[inline]
fn get_size(bgcnt: u16, affine: bool) -> (u32, u32)
{
    let z = bgcnt.bits(15, 14);

    if affine
    {
        match z
        {
            0b00 => (16, 16),
            0b01 => (32, 32),
            0b10 => (64, 64),
            0b11 => (128, 128),
            _  => unreachable!(),
        }
    }
    else
    {
        match z
        {
            0b00 => (32, 32),
            0b01 => (64, 32),
            0b10 => (32, 64),
            0b11 => (64, 64),
            _  => unreachable!(),
        }
    }
}

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
    pub fn render_text_tile(&mut self, y: u32, x: u32, memory: &Memory)
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

                self.buffer[(x * 8 * 256 + y * 8 + r * 256 + c) as usize] = 
                    memory.palette(palette_number | palette);
            }
        }
    }
}