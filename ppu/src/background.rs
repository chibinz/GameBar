use util::*;

use super::Ppu;

#[derive(Clone, Copy)]
pub struct Background {
    pub bgcnt: u16,      // Raw background control register
    pub priority: u32,   // Lower priority takes precedence
    pub tile_b: u32,     // Determine base address of tile data
    pub map_b: u32,      // Determine base address of tile map
    pub mosaic_f: bool,  // Mosaic, 1 - on, 0 - off
    pub palette_f: bool, // Palette type, 1 - 256, 0 - 16x16
    pub wrap_f: bool,    // Screen over of rotational backgrounds
    pub size_r: u32,     // Raw bits 15 - 14 of bgcnt

    // Text background registers
    pub hscroll: u16,
    pub vscroll: u16,

    // Affine background registers
    pub matrix: (i32, i32, i32, i32),
    pub coord: (i32, i32),
    pub internal: (i32, i32),
}

impl Background {
    pub fn new() -> Self {
        Self {
            bgcnt: 0,
            priority: 0,
            tile_b: 0,
            map_b: 0,
            mosaic_f: false,
            palette_f: false,
            wrap_f: false,
            size_r: 0,
            hscroll: 0,
            vscroll: 0,
            matrix: (0, 0, 0, 0),
            coord: (0, 0),
            internal: (0, 0),
        }
    }
}

impl Background {
    pub fn get_control(&self) -> u16 {
        self.bgcnt
    }

    pub fn set_control(&mut self, value: u16) {
        self.bgcnt = value;
        self.priority = value.bits(1, 0);
        self.tile_b = value.bits(3, 2);
        self.mosaic_f = value.bit(6);
        self.palette_f = value.bit(7);
        self.map_b = value.bits(12, 8);
        self.wrap_f = value.bit(13);
        self.size_r = value.bits(15, 14);
    }

    pub fn set_hofs(&mut self, value: u16) {
        self.hscroll = value;
    }

    pub fn set_vofs(&mut self, value: u16) {
        self.vscroll = value;
    }

    pub fn set_pa(&mut self, value: u16) {
        self.matrix.0 = value as i16 as i32;
    }

    pub fn set_pb(&mut self, value: u16) {
        self.matrix.1 = value as i16 as i32;
    }

    pub fn set_pc(&mut self, value: u16) {
        self.matrix.2 = value as i16 as i32;
    }

    pub fn set_pd(&mut self, value: u16) {
        self.matrix.3 = value as i16 as i32;
    }

    pub fn set_x_l(&mut self, value: u16) {
        self.coord.0 &= ((self.coord.0 as u32) & 0xffff0000) as i32;
        self.coord.0 |= value as i32;
        self.internal.0 = self.coord.0;
    }

    pub fn set_x_h(&mut self, value: u16) {
        self.coord.0 &= 0x0000ffff;
        self.coord.0 |= ((value as u32) << 16) as i32;
        self.internal.0 = self.coord.0;
    }
    pub fn set_y_l(&mut self, value: u16) {
        self.coord.1 = ((self.coord.0 as u32) & 0xffff0000) as i32;
        self.coord.1 |= value as i32;
        self.internal.1 = self.coord.1;
    }
    pub fn set_y_h(&mut self, value: u16) {
        self.coord.1 &= 0x0000ffff;
        self.coord.1 |= ((value as u32) << 16) as i32;
        self.internal.1 = self.coord.1;
    }
}

impl Ppu {
    pub fn is_background_affine(&self, index: usize) -> bool {
        match (self.mode, index) {
            (0, 0..=3) => false,
            (1, 0..=1) => false,
            (1, 2) => true,
            (2, 2..=3) => true,
            _ => unreachable!(),
        }
    }

    pub fn get_background_dimension(&self, index: usize) -> (u32, u32) {
        match (
            self.is_background_affine(index) as u32,
            self.background[index].size_r,
        ) {
            (0, 0) => (256, 256),
            (0, 1) => (512, 256),
            (0, 2) => (256, 512),
            (0, 3) => (512, 512),
            (1, 0) => (128, 128),
            (1, 1) => (256, 256),
            (1, 2) => (512, 512),
            (1, 3) => (1024, 1024),
            _ => unreachable!(),
        }
    }

    pub fn decode_text_background(&mut self, index: usize) -> Vec<u16> {
        let bg = &self.background[index];
        let (width, height) = self.get_background_dimension(index);
        let mut ret = vec![0; (width * height) as usize];

        for tile_x in 0..(width / 8) {
            for tile_y in 0..(height / 8) {
                let tile_entry = self.text_tile_map(bg.map_b, bg.size_r, tile_x, tile_y);
                let tile_n = tile_entry.bits(9, 0);
                let hflip = tile_entry.bit(10);
                let vflip = tile_entry.bit(11);
                let palette_n = tile_entry.bits(15, 12);

                for y in 0..8 {
                    for x in 0..8 {
                        let pixel_x = if hflip { x } else { 7 - x };
                        let pixel_y = if vflip { y } else { 7 - y };
                        let palette_entry =
                            self.tile_data(bg.palette_f, bg.tile_b, tile_n, pixel_x, pixel_y);
                        let n = (tile_y * 8 + pixel_y) * width + tile_x * 8 + pixel_x;
                        ret[n as usize] = self.bg_palette(bg.palette_f, palette_n, palette_entry);
                    }
                }
            }
        }

        ret
    }

    pub fn decode_affine_background(&mut self, index: usize) -> Vec<u16> {
        let bg = &self.background[index];
        let (width, height) = self.get_background_dimension(index);
        let mut ret = vec![0; (width * height) as usize];

        for tile_x in 0..(width / 8) {
            for tile_y in 0..(height / 8) {
                let tile_n = self.affine_tile_map(bg.map_b, bg.size_r, tile_x, tile_y) as u32;

                for pixel_y in 0..8 {
                    for pixel_x in 0..8 {
                        let palette_entry =
                            self.tile_data(true, bg.tile_b, tile_n, pixel_x, pixel_y);
                        let c = self.palette[palette_entry as usize];
                        let n = (tile_y * 8 + pixel_y) * width + tile_x * 8 + pixel_x;
                        ret[n as usize] = c;
                    }
                }
            }
        }

        ret
    }

    pub fn draw_text_background(&mut self, index: usize) {
        let bg = &self.background[index];
        let vcount = self.vcount;
        let window = &self.window;
        let (width, height) = self.get_background_dimension(index);

        // Vertical wrap around
        let line_n = (vcount.wrapping_add(bg.vscroll)) as u32 % height;

        for i in 0..width {
            let tile_x = i / 8;
            let tile_y = line_n / 8;
            let mut pixel_x = i % 8;
            let mut pixel_y = line_n % 8;

            let tile_entry = self.text_tile_map(bg.map_b, bg.size_r, tile_x, tile_y);

            let tile_n = tile_entry.bits(9, 0);
            let hflip = tile_entry.bit(10);
            let vflip = tile_entry.bit(11);
            let palette_n = tile_entry.bits(15, 12);

            if hflip {
                pixel_x = 7 - pixel_x
            };
            if vflip {
                pixel_y = 7 - pixel_y
            };

            let palette_entry = self.tile_data(bg.palette_f, bg.tile_b, tile_n, pixel_x, pixel_y);

            // Horizontal wrap around
            let x = i.wrapping_sub(bg.hscroll as u32) % width;
            let color = self.bg_palette(bg.palette_f, palette_n, palette_entry);

            let layer = &mut self.layer[bg.priority as usize];
            layer.paint(x, color, window, index);
        }
    }

    pub fn draw_affine_background(&mut self, index: usize) {
        let bg = &self.background[index];
        let vram = &self.vram;
        let window = &self.window;
        let (width, height) = self.get_background_dimension(index);

        for i in 0..width {
            let mut text_x = (bg.matrix.0 * i as i32 + bg.internal.0) >> 8;
            let mut text_y = (bg.matrix.2 * i as i32 + bg.internal.1) >> 8;

            // TODO: Refactor into macro
            if out_of_bound(text_x, width) {
                if bg.wrap_f {
                    text_x = wrap_around(text_x, width)
                } else {
                    continue;
                }
            }

            if out_of_bound(text_y, height) {
                if bg.wrap_f {
                    text_y = wrap_around(text_y, height)
                } else {
                    continue;
                }
            }

            let tile_x = text_x as u32 / 8;
            let tile_y = text_y as u32 / 8;
            let pixel_x = text_x as u32 % 8;
            let pixel_y = text_y as u32 % 8;

            // Can't use self.tile_data() due to borrow checker
            let offset = tile_y * (16 << bg.size_r) + tile_x;
            let tile_n = vram[(bg.map_b * 0x800 + offset) as usize] as u32;
            let palette_entry = self.tile_data(true, bg.tile_b, tile_n, pixel_x, pixel_y);
            let color = self.bg_palette(true, 0, palette_entry);

            let layer = &mut self.layer[bg.priority as usize];
            layer.paint(i, color, window, index);
        }
    }

    pub fn draw_bitmap_3(&mut self) {
        let line_n = self.vcount as u32;
        let window = &self.window;

        for x in 0..240 {
            let pixel = self.vram16((line_n * 240 + x) * 2);
            self.layer[0].paint(x, pixel, window, 2);
        }
    }

    pub fn draw_bitmap_4(&mut self) {
        let start = if self.flip { 0xa000 } else { 0 };
        let line_n = self.vcount as u32;

        for x in 0..240 {
            let palette_entry = self.vram8(start + line_n * 240 + x);
            let color = self.bg_palette(true, 0, palette_entry as u32);
            self.layer[0].paint(x, color, &self.window, 2);
        }
    }

    pub fn draw_bitmap_5(&mut self) {
        let start = if self.flip { 0xa000 } else { 0 };
        let line_n = self.vcount as u32;
        let window = &self.window;
        if line_n > 127 {
            return;
        }

        for x in 0..160 {
            let pixel = self.vram16(start + (line_n * 160 + x) * 2);
            self.layer[0].paint(x, pixel, window, 2);
        }
    }
}

#[inline]
pub fn out_of_bound(a: i32, max: u32) -> bool {
    a < 0 || a >= max as i32
}

#[inline]
pub fn wrap_around(a: i32, max: u32) -> i32 {
    let mut b = a;

    b %= max as i32;

    if a < 0 {
        b += max as i32;
    }

    b as i32
}
