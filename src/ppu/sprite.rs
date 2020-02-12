use crate::util::*;
use crate::memory::Memory;

use super::layer::Layer;

/// Sprite dimension in pixels
pub static DIMENSION: [[(u32, u32); 4]; 3] =
[
    // Square
    [
        ( 8,  8),
        (16, 16),
        (32, 32),
        (64, 64),
    ],
    // Horizontal
    [
        (16,  8),
        (32,  8),
        (32, 16),
        (64, 32),
    ],
    // Vertical
    [
        ( 8, 16),
        ( 8, 32),
        (16, 32),
        (32, 64),
    ],
];

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
    pub matrix   : (i32, i32, i32, i32) 
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

    pub fn draw(&mut self, vcount: u32, sequential: bool, layer: &mut Layer, memory: &Memory)
    {
        if !self.disabled() && self.visible(vcount)
        {
            if self.affine_f
            {
                self.draw_affine(vcount, sequential, layer, memory)
            }
            else
            {
                self.draw_text(vcount, sequential, layer, memory)
            }
        }
    }

    pub fn draw_text(&mut self, vcount: u32, sequential: bool, layer: &mut Layer, memory: &Memory)
    {
        // Vertical wrap around
        let y = (vcount - self.ycoord) % 256; 
        let w = if sequential {self.width / 8} else {8};

        let mut tile_y  = y / 8;
        let mut pixel_y = y % 8;
        if self.vflip
        {
            tile_y  = self.height / 8 - tile_y - 1;
            pixel_y = 7 - pixel_y;
        }

        for i in 0..self.width
        {
            let mut tile_x  = i / 8;
            let mut pixel_x = i % 8;
            if self.hflip
            {
                tile_x  = self.width / 8 - tile_x - 1;
                pixel_x = 7 - pixel_x;
            }

            // Sprite tile data starts at 4 * 0x4000 = 0x10000
            let tile_b = 4;
            let tile_n = self.tile_n + tile_y * w + tile_x;

            let palette_entry = memory.tile_data(self.palette_f, tile_b, tile_n, pixel_x, pixel_y);

            // Horizontal wrap around
            let x = (self.xcoord + i) % 512;
            let color = memory.obj_palette(self.palette_n, palette_entry);

            layer.paint(x, color);
        }
    }

    #[allow(unused_assignments)]
    pub fn draw_affine(&mut self, vcount: u32, sequential: bool, layer: &mut Layer, memory: &Memory)
    {
        let mut half_width = self.width as i32/ 2;
        let mut half_height = self.height as i32 / 2;

        let mut xcenter = self.xcoord as i32 + half_width;
        let mut ycenter = self.ycoord as i32 + half_height;

        // Double flag only doubles the viewport size, not the sprite size
        if self.double_f
        {
            xcenter     += half_width;
            ycenter     += half_height;
            half_width  *= 2;
            half_height *= 2;
        }

        // Wrap around
        xcenter %= 512;
        ycenter %= 256;

        let y = vcount as i32 - ycenter;
        let w = if sequential {self.width / 8} else {8};

        for x in -half_width..half_width
        {
            // Due to the linearity of the transform matrix, the origin is preserved.
            // That is, the screen origin overlaps the texture origin.
            // The transform matrix takes relative ONSCREEN distance to the origin as input
            // and transforms it into relative TEXTURE distance to origin.
            let text_x = ((self.matrix.0 * x + self.matrix.1 * y) >> 8) + self.width as i32 / 2;
            let text_y = ((self.matrix.2 * x + self.matrix.3 * y) >> 8) + self.height as i32 / 2;

            // Avoid replication
            if text_x < 0 || text_x >= self.width as i32
            || text_y < 0 || text_y >= self.height as i32
            {
                continue;
            }

            let tile_x = text_x as u32 / 8;
            let tile_y = text_y as u32 / 8;
            let pixel_x = text_x as u32 % 8;
            let pixel_y = text_y as u32 % 8;

            let tile_b = 4;
            let tile_n = self.tile_n + tile_y * w + tile_x;

            let palette_entry = memory.tile_data(self.palette_f, tile_b, tile_n, pixel_x, pixel_y);

            let i = (xcenter + x) as u32;
            let color = memory.obj_palette(self.palette_n, palette_entry);
            
            layer.paint(i, color);
        }
    }

    pub fn disabled(&self) -> bool
    {
        !self.affine_f && self.double_f
    }

    pub fn visible(&self, vcount: u32) -> bool
    {
        let x = sign_extend(self.xcoord, 8);
        let y = sign_extend(self.ycoord, 7);
        let mut w = self.width as i32;
        let mut h = self.height as i32;
        
        if self.double_f
        {
            w *= 2;
            h *= 2;
        }

           x < 240
        && x + w >= 0
        && y <= vcount as i32
        && y + h > vcount as i32
    }
}