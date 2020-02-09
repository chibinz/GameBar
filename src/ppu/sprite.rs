use crate::memory::Memory;

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

    pub fn draw(&mut self, vcount: u32, sequential: bool, pixel: &mut Vec<u32>, memory: &Memory)
    {
        memory.update_sprite(self);

        if !self.disabled() && self.visible(vcount)
        {
            if self.affine_f
            {
                self.draw_affine(vcount, sequential, pixel, memory)
            }
            else
            {
                self.draw_text(vcount, sequential, pixel, memory)
            }
        }
    }

    pub fn draw_text(&mut self, vcount: u32, sequential: bool, pixel: &mut Vec<u32>, memory: &Memory)
    {
        let w = if sequential {self.width / 8} else {8};
        let y = vcount - self.ycoord;

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

            let palette = memory.tile_data(self.palette_f, tile_b, tile_n, pixel_x, pixel_y);
            pixel[(self.xcoord + i) as usize] = memory.palette(0x100 + (self.palette_n << 4 | palette));
        }
    }

    pub fn draw_affine(&mut self, vcount: u32, sequential: bool, pixel: &mut Vec<u32>, memory: &Memory)
    {
        let mut half_width = self.width as i32 / 2;
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

        let w = if sequential {self.width / 8} else {8};
        let y = vcount as i32 - ycenter;

        if self.visible(vcount)
        {
            for x in -half_width..half_width
            {
                // Due to the linearity of the transform matrix, the origin is preserved.
                // That is, the screen origin overlaps the texture origin.
                // The transform matrix takes relative ONSCREEN distance to the origin as input
                // and transforms it into relative TEXTURE distance to origin.
                let text_x = ((self.matrix.0 * x + self.matrix.1 * y) >> 8) + half_width;
                let text_y = ((self.matrix.2 * x + self.matrix.3 * y) >> 8) + half_height;

                let tile_x = text_x as u32 / 8;
                let tile_y = text_y as u32 / 8;
                let pixel_x = text_x as u32 % 8;
                let pixel_y = text_y as u32 % 8;

                let tile_b = 4;
                let tile_n = self.tile_n + tile_y * w + tile_x;

                let palette_entry = memory.tile_data(self.palette_f, tile_b, tile_n, pixel_x, pixel_y);
                let palette = (self.palette_n << 4) | palette_entry;

                pixel[(xcenter + x) as usize] = memory.palette(0x100 + (self.palette_n << 4 | palette));
            }
        }
    }

    pub fn disabled(&self) -> bool
    {
        !self.affine_f && self.double_f
    }

    pub fn visible(&self, vcount: u32) -> bool
    {
           self.xcoord < 240 
        && self.ycoord <= vcount 
        && self.ycoord + self.height * (self.double_f as u32 + 1) > vcount
    }
}