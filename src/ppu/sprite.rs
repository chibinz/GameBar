use crate::util::*;
use crate::memory::Memory;

#[derive(Clone)]
pub struct Sprite
{
    pub index    : usize,    // Index of sprite, 0 - 127
    pub xcoord   : u32,      // X coordinate, top left for text sprites
    pub ycoord   : u32,      // Y coordinate, center for affine sprites
    pub width    : u32,      // Width in pixels
    pub height   : u32,      // Height in pixels 
    pub mode     : u32,      // 0 - normal, 1 - semi-transparent, 2 - window
    pub affine_f : bool,     // Rotational / scaling flag
    pub double_f : bool,     // Double size flag
    pub mosaic_f : bool,     // Mosaic flag
    pub palette_f: bool,     // Palette type, 1 - 256, 2 - 16
    pub hflip    : bool,     // Horizontal flip bit
    pub vflip    : bool,     // Vertical flip bit
    pub affine_i : u32,      // Rotation / scaling data index
    pub tile_n   : u32,      // Tile number
    pub priority : u32,
    pub palette_n: u32,      // Palette number (for 16 color sprites)

    // Transform matrix used for Rotation and scaling
    pub matrix   : (u16, u16, u16, u16) 
}

impl Sprite
{
    pub fn new() -> Self
    {
        Self
        {
            index    : 0,     
            xcoord   : 0,     
            ycoord   : 0,     
            width    : 0,     
            height   : 0,     
            mode     : 0,     
            affine_f : false, 
            double_f : false, 
            mosaic_f : false, 
            palette_f: false, 
            hflip    : false, 
            vflip    : false, 
            affine_i : 0,     
            tile_n   : 0,     
            priority : 0,
            palette_n: 0,     
            matrix   : (0, 0, 0, 0) 
        }
    }

    pub fn draw_text(&mut self, vcount: u32, sequential: bool, pixel: &mut Vec<u32>, memory: &Memory)
    {
        assert_eq!(self.affine_f, false);
        memory.update_sprite(self);

        if !self.disabled() && self.visible(vcount)
        {
            let tile_y = (vcount - self.ycoord) / 8;
            let r = if !self.vflip {7 - (vcount - self.ycoord) % 8} else {(vcount - self.ycoord) % 8};

            for tile_x in 0..self.width/8
            {
                // Why 8? weird...
                let w = if sequential {self.height / 8} else {8};
                let row = memory.tile_row32(4, self.tile_n + tile_y * w + tile_x, r);

                for j in 0..8
                {
                    let palette = row.bits((8 - j as u32) * 4 - 1, (7 - j as u32) * 4);
                    let c = if self.hflip {7 - j} else {j};

                    if self.xcoord + tile_x * 8 + c >= 240 {break}
                    
                    pixel[(self.xcoord + tile_x * 8 + c) as usize] = 
                        memory.palette(0x100 + palette);
                }
            }
        }
    }

    pub fn draw_text_256(&mut self, vcount: u32, pixel: &mut Vec<u32>, memory: &Memory)
    {
        assert_eq!(self.affine_f, false);
        memory.update_sprite(self);

        if !self.disabled() && self.visible(vcount)
        {
            let tile_y = (vcount - self.ycoord) / 8;
            let r = (vcount - self.ycoord) % 8;


            for tile_x in 0..self.width/8
            {
                let row = memory.tile_row64(4, self.tile_n + tile_y * self.width/8 + tile_x, r);

                for j in 0..8
                {
                    let palette = (row >> ((7 - j) * 8)) as u8 as u32;
                    let c = if !self.hflip {7 - j} else {j};

                    if self.xcoord + tile_x * 8 + c >= 240 {break}
                    
                    pixel[(self.xcoord + tile_x * 8 + c) as usize] = 
                        memory.palette(0x100 + palette);
                }
            }
        }
    }
    

    pub fn disabled(&self) -> bool
    {
        !self.affine_f && self.double_f
    }

    pub fn visible(&self, vcount: u32) -> bool
    {
        self.xcoord < 240 && self.ycoord < vcount 
            && self.ycoord + self.height > vcount
    }
}