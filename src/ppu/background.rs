use crate::util::*;
use crate::memory::Memory;
use crate::memory::palette::RGB;

/// Background dimension in pixels
pub static DIMENSION: [[(u32, u32); 4]; 2] =
[
    // Text
    [
        (256, 256),
        (512, 256),
        (256, 512),
        (512, 512),
    ],
    // Affine
    [
        ( 128,  128),
        ( 256,  256),
        ( 512,  512),
        (1024, 1024),
    ],
];

#[derive(Clone)]
pub struct Background
{
    pub index    : usize,   // 0 - 3
    pub bgcnt    : u16,     // Raw background control register
    pub priority : u32,     // Lower priority takes precedence
    pub tile_b   : u32,     // Determine base address of tile data
    pub map_b    : u32,     // Determine base address of tile map
    pub affine_f : bool,    // Background type, 1 - affine, 0 - text
    pub mosaic_f : bool,    // Mosaic, 1 - on, 0 - off
    pub palette_f: bool,    // Palette type, 1 - 256, 0 - 16x16
    pub wrap_f   : bool,    // Screen over of rotational backgrounds
    pub size_r   : u32,     // Raw bits 15 - 14 of bgcnt
    pub width    : u32,     // Width in pixels
    pub height   : u32,     // Height in pixels
    pub vcount   : u32,     // Line number of current scanline

    // Text background registers
    pub hscroll  : u32,
    pub vscroll  : u32,

    // Affine background registers
    pub matrix   : (i32, i32, i32, i32),
    pub coord    : (i32, i32),

    // Line buffer
    pub pixel    : Vec<u32>,  
}

impl Background
{
    pub fn new() -> Self
    {
        Self
        {
            index    : 0,
            bgcnt    : 0,
            priority : 0,
            tile_b   : 0,
            map_b    : 0,
            affine_f : false,
            mosaic_f : false,
            palette_f: false,
            wrap_f   : false,
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

        let line_n = self.vcount + self.vscroll;
        
        for i in 0..self.width
        {   
            let tile_x      = i / 8;
            let tile_y      = line_n / 8;
            let mut pixel_x = i % 8;
            let mut pixel_y = line_n % 8;

            let tile_entry = memory.text_tile_map(self.map_b, self.size_r, tile_y, tile_x);

            let tile_n    = tile_entry.bits(9, 0);
            let hflip     = tile_entry.bit(10);
            let vflip     = tile_entry.bit(11);
            let palette_b = tile_entry.bits(15, 12);
    
            if hflip {pixel_x = 7 - pixel_x};
            if vflip {pixel_y = 7 - pixel_y};

            let palette = memory.tile_data(self.palette_f, self.tile_b, tile_n, pixel_x, pixel_y);

            self.pixel[i as usize] = memory.palette((palette_b << 4) | palette);
        }
    }

    pub fn draw_affine(&mut self, memory: &Memory)
    {
        memory.update_affine_bg(self);

        for i in 0..self.width
        {
            let text_x = (self.matrix.0 * i as i32 + self.coord.0) >> 8;
            let text_y = (self.matrix.2 * i as i32 + self.coord.1) >> 8;

            let tile_x  = text_x as u32 / 8;
            let tile_y  = text_y as u32 / 8;
            let pixel_x = text_x as u32 % 8;
            let pixel_y = text_y as u32 % 8;
            
            let tile_n = memory.affine_tile_map(self.map_b, self.size_r, tile_y, tile_x) as u32;

            let palette = memory.tile_data8(self.tile_b, tile_n, pixel_x, pixel_y);

            self.pixel[i as usize] = memory.palette(palette as u32);
        }

        self.coord.0 += self.matrix.1;
        self.coord.1 += self.matrix.3;
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
            let pixel = memory.vram16(start + (line_n * 160 + x) * 2);
            self.pixel[x as usize] = RGB(pixel);
        }
    }
}