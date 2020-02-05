use crate::util::*;
use crate::ppu::PPU;
use crate::memory::Memory;
use crate::memory::palette::RGB;


pub struct Background
{
    pub index    : usize,   // 0 - 3
    pub bgcnt    : u16,     // Raw background control register
    pub priority : u32,     // Lower priority takes precedence
    pub tile_n   : u32,     // Determine start address of tile data
    pub map_n    : u32,     // Determine start address of tile map
    pub affine_f : bool,    // Background type, 1 - affine, 0 - text
    pub mosaic_f : bool,    // Mosaic, 1 - on, 0 - off
    pub palette_f: bool,    // Palette type, 1 - 256, 0 - 16x16
    pub repeat_f : bool,    // Screen over of rotational backgrounds
    pub size_r   : u32,     // Raw bits 15 - 14 of bgcnt
    pub width    : u32,     // width in tiles
    pub height   : u32,     // height in tiles
    pub vcount   : u32,     // Line number of current scanline

    // Text background registers
    pub hscroll  : u32,
    pub vscroll  : u32,

    // Affine background registers
    pub matrix   : (u16, u16, u16, u16),
    pub coord    : (u32, u32),

    // Line buffer
    pub pixel    : Vec<u32>,  
}

impl Background
{
    pub fn new(i: usize) -> Self
    {
        Self
        {
            index    : i,
            bgcnt    : 0,
            priority : 0,
            tile_n   : 0,
            map_n    : 0,
            affine_f : false,
            mosaic_f : false,
            palette_f: false,
            repeat_f : false,
            size_r   : 0,
            width    : 0,
            height   : 0,
            vcount   : 0,
            hscroll  : 0,
            vscroll  : 0,
            matrix   : (0, 0, 0, 0),
            coord    : (0, 0),

            // The largest width of a background is 1024 pixels.
            // Avoid reallocation when resizing background.
            pixel    : vec![0; 1024],
        }
    }
    
    pub fn draw_text(&mut self, memory: &Memory)
    {
        memory.update_text_bg(self);

        if self.palette_f
        {
            self.draw_text_256(memory)
        }
        else
        {
            self.draw_text_16(memory)
        }
    }

    pub fn draw_text_16(&mut self, memory: &Memory)
    {
        memory.update_text_bg(self);

        let line_n = self.vcount + self.vscroll;
        let tile_y = (line_n / 8) % self.height;
        
        // Tile column
        for tile_x in 0..self.width
        {
            let tile_entry = memory.tile_map(self.map_n, self.size_r, tile_y, tile_x);

            let tile_number     = tile_entry.bits(9, 0);
            let horizontal_flip = tile_entry.bit(10);
            let vertical_flip   = tile_entry.bit(11);
            let palette_number  = tile_entry.bits(15, 12) << 4;
    
            let r = if vertical_flip {7 - (line_n % 8)} else {line_n % 8};
            let row = memory.tile_row32(self.tile_n, tile_number, r);

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

    pub fn draw_text_256(&mut self, memory: &Memory)
    {
        memory.update_text_bg(self);

        let line_n = self.vcount + self.vscroll;
        let tile_y = (line_n / 8) % self.height;
        
        // Tile column
        for tile_x in 0..self.width
        {
            let tile_entry = memory.tile_map(self.map_n, self.size_r, tile_y, tile_x);

            let tile_number     = tile_entry.bits(9, 0);
            let horizontal_flip = tile_entry.bit(10);
            let vertical_flip   = tile_entry.bit(11);
    
            let r = if vertical_flip {7 - (line_n % 8)} else {line_n % 8};
            let row = memory.tile_row64(self.tile_n, tile_number, r);

            // Pixel column
            for j in 0..8
            {
                let palette = (row >> ((7 - j) * 8)) as u8 as u32;
                let c = if !horizontal_flip {7 - j} else {j};
                
                self.pixel[(tile_x * 8 + c) as usize] = 
                    memory.palette(palette);
            }
        }
    }
    
    pub fn draw_bitmap_3(&mut self, memory: &Memory)
    {
        memory.update_affine_bg(self);

        let line_n = self.vcount;

        for x in 0..240
        {
            let pixel = memory.vram16((line_n * 240 + x) * 2);
            self.pixel[x as usize] = RGB(pixel);
        }
    }

    pub fn draw_bitmap_4(&mut self, flip: bool, memory: &Memory)
    {
        memory.update_affine_bg(self);

        let start = if flip {0xa000} else {0};
        let line_n = self.vcount;
        
        for x in 0..240
        {
            let palette_entry = memory.vram8(start + line_n * 240 + x);
            self.pixel[x as usize] = memory.palette(palette_entry as u32);
        }
    }

    pub fn draw_bitmap_5(&mut self, flip: bool, memory: &Memory)
    {
        memory.update_affine_bg(self);

        let start = if flip {0xa000} else {0};
        let line_n = self.vcount;
        
        for x in 0..160
        {
            let pixel = memory.vram16(start + (line_n * 128 + x) * 2);
            self.pixel[x as usize] = RGB(pixel);
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
        let tile_entry = memory.tile_map(1, 0, x, y);

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