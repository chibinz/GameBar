use crate::Ppu;
use util::*;

/// Sprite dimension in pixels
pub static DIMENSION: [[(u32, u32); 4]; 3] = [
    // Square
    [(8, 8), (16, 16), (32, 32), (64, 64)],
    // Horizontal
    [(16, 8), (32, 8), (32, 16), (64, 32)],
    // Vertical
    [(8, 16), (8, 32), (16, 32), (32, 64)],
];

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub attr: [u16; 3], // Raw object attributes, Used for fast oam read

    pub xcoord: u32,     // X coordinate, top left for text sprites
    pub ycoord: u32,     // Y coordinate, center for affine sprites
    pub shape: u32,      // 0 - square, 1 - horizontal, 2 - vertical
    pub size: u32,       // 0 - 8, 1 - 16, 2 - 32, 3 - 64 pixels
    pub mode: u32,       // 0 - normal, 1 - semi-transparent, 2 - window
    pub affine_f: bool,  // Rotational / scaling flag
    pub double_f: bool,  // Double size flag
    pub mosaic_f: bool,  // Mosaic flag
    pub palette_f: bool, // Palette type, 1 - 256, 2 - 16
    pub hflip: bool,     // Horizontal flip bit
    pub vflip: bool,     // Vertical flip bit
    pub affine_i: u32,   // Rotation / scaling data index
    pub tile_n: u32,     // Tile number
    pub priority: u32,
    pub palette_n: u32, // Palette number (for 16 color sprites)
}

impl Sprite {
    pub fn new() -> Self {
        Self {
            attr: [0; 3],

            xcoord: 0,
            ycoord: 0,
            shape: 0,
            size: 0,
            mode: 0,
            affine_f: false,
            double_f: false,
            mosaic_f: false,
            palette_f: false,
            hflip: false,
            vflip: false,
            affine_i: 0,
            tile_n: 0,
            priority: 0,
            palette_n: 0,
        }
    }

    #[inline]
    pub fn get_dimension(&self) -> (u32, u32) {
        DIMENSION[self.shape as usize][self.size as usize]
    }

    #[inline]
    pub fn get_affine_matrix(&self, mat: &mut [u16]) -> (i32, i32, i32, i32) {
        let index = self.affine_i as usize * 4;

        let pa = mat[index] as i16 as i32;
        let pb = mat[index + 1] as i16 as i32;
        let pc = mat[index + 2] as i16 as i32;
        let pd = mat[index + 3] as i16 as i32;

        (pa, pb, pc, pd)
    }

    #[inline]
    pub fn disabled(&self) -> bool {
        !self.affine_f && self.double_f
    }

    #[inline]
    pub fn visible(&self, vcount: u32) -> bool {
        let (width, height) = self.get_dimension();

        let v = vcount as i32;
        let mut x = self.xcoord as i32;
        let mut y = self.ycoord as i32;
        let mut w = width as i32;
        let mut h = height as i32;

        // Horizontal and vertical wrap around
        if x + w > 512 {
            x -= 512
        };
        if y + h > 256 {
            y -= 256
        };

        if self.double_f {
            w *= 2;
            h *= 2;
        }

        x < 240 && x + w >= 0 && y <= v as i32 && y + h > v as i32
    }
}

impl Sprite {
    #[inline]
    pub fn set_attr0(&mut self, value: u16) {
        self.attr[0] = value;

        self.ycoord = value.bits(7, 0);
        self.affine_f = value.bit(8);
        self.double_f = value.bit(9);
        self.mode = value.bits(11, 10);
        self.mosaic_f = value.bit(12);
        self.palette_f = value.bit(13);
        self.shape = value.bits(15, 14);
    }

    #[inline]
    pub fn set_attr1(&mut self, value: u16) {
        self.attr[1] = value;

        self.xcoord = value.bits(8, 0);
        self.hflip = value.bit(12);
        self.vflip = value.bit(13);
        self.affine_i = value.bits(13, 9);
        self.size = value.bits(15, 14);
    }

    #[inline]
    pub fn set_attr2(&mut self, value: u16) {
        self.attr[2] = value;

        self.tile_n = value.bits(9, 0);
        self.priority = value.bits(11, 10);
        self.palette_n = value.bits(15, 12);
    }
}

impl Ppu {
    pub fn draw_sprite(&mut self, index: usize) {
        let sprite = &self.oam.sprite[index];
        let vcount = self.vcount as u32;

        if !sprite.disabled() && sprite.visible(vcount) {
            if sprite.affine_f {
                self.draw_affine_sprite(index)
            } else {
                self.draw_text_sprite(index)
            }
        }
    }

    pub fn decode_sprite(&mut self, index: usize) -> Vec<u16> {
        let sprite = &self.oam.sprite[index];
        let (width, height) = sprite.get_dimension();
        let stride = if self.sequential { width / 8 } else { 32 };
        let mut ret = vec![0; (width * height) as usize];

        for tile_y in 0..(height / 8) {
            for tile_x in 0..(width / 8) {
                let tile_b = 4;
                let tile_n = sprite.tile_n + (tile_y * stride + tile_x);

                for pixel_y in 0..8 {
                    for pixel_x in 0..8 {
                        let t = self.tile_data(sprite.palette_f, tile_b, tile_n, pixel_x, pixel_y);
                        let p = self.obj_palette(sprite.palette_n, t);
                        let n = (tile_y * 8 + pixel_y) * width + tile_x * 8 + pixel_x;
                        ret[n as usize] = p;
                    }
                }
            }
        }

        ret
    }

    pub fn draw_text_sprite(&mut self, index: usize) {
        let sprite = &self.oam.sprite[index];
        let vcount = self.vcount as u32;
        let sequential = self.sequential;
        let window = &self.window;
        let (width, height) = sprite.get_dimension();

        // Vertical wrap around
        let y = vcount.wrapping_sub(sprite.ycoord) % 256;
        let w = if sequential { width / 8 } else { 32 };

        let mut tile_y = y / 8;
        let mut pixel_y = y % 8;
        if sprite.vflip {
            tile_y = height / 8 - tile_y - 1;
            pixel_y = 7 - pixel_y;
        }

        for i in 0..width {
            let mut tile_x = i / 8;
            let mut pixel_x = i % 8;
            if sprite.hflip {
                tile_x = width / 8 - tile_x - 1;
                pixel_x = 7 - pixel_x;
            }

            // Sprite tile data starts at 4 * 0x4000 = 0x10000
            let tile_b = 4;
            let tile_n = sprite.tile_n + tile_y * w + tile_x;

            let palette_entry = self.tile_data(sprite.palette_f, tile_b, tile_n, pixel_x, pixel_y);

            // Horizontal wrap around
            let x = (sprite.xcoord + i) % 512;
            let color = self.obj_palette(sprite.palette_n, palette_entry);

            let layer = &mut self.layer[sprite.priority as usize];
            layer.paint(x, color, window, 4);
        }
    }

    #[allow(unused_assignments)]
    pub fn draw_affine_sprite(&mut self, index: usize) {
        let sprite = &self.oam.sprite[index];
        let vcount = self.vcount as u32;
        let sequential = self.sequential;
        let window = &self.window;
        let (width, height) = sprite.get_dimension();

        let mut half_width = width as i32 / 2;
        let mut half_height = height as i32 / 2;

        let mut xcenter = sprite.xcoord as i32 + half_width;
        let mut ycenter = sprite.ycoord as i32 + half_height;

        // Double flag only doubles the viewport size, not the sprite size
        if sprite.double_f {
            xcenter += half_width;
            ycenter += half_height;
            half_width *= 2;
            half_height *= 2;
        }

        // Wrap around
        xcenter %= 512;
        ycenter %= 256;

        let y = vcount as i32 - ycenter;
        let w = if sequential { width / 8 } else { 32 };

        let (pa, pb, pc, pd) = sprite.get_affine_matrix(&mut self.oam.param);

        for x in -half_width..half_width {
            // Due to the linearity of the transform matrix, the origin is preserved.
            // That is, the screen origin overlaps the texture origin.
            // The transform matrix takes relative ONSCREEN distance to the origin as input
            // and transforms it into relative TEXTURE distance to origin.
            let text_x = ((pa * x + pb * y) >> 8) + width as i32 / 2;
            let text_y = ((pc * x + pd * y) >> 8) + height as i32 / 2;

            // Avoid replication
            if text_x < 0 || text_x >= width as i32 || text_y < 0 || text_y >= height as i32 {
                continue;
            }

            let tile_x = text_x as u32 / 8;
            let tile_y = text_y as u32 / 8;
            let pixel_x = text_x as u32 % 8;
            let pixel_y = text_y as u32 % 8;

            let tile_b = 4;
            let tile_n = sprite.tile_n + tile_y * w + tile_x;

            let palette_entry = self.tile_data(sprite.palette_f, tile_b, tile_n, pixel_x, pixel_y);

            let i = (xcenter + x) as u32;
            let color = self.obj_palette(sprite.palette_n, palette_entry);

            let layer = &mut self.layer[sprite.priority as usize];
            layer.paint(i, color, window, 4);
        }
    }
}
