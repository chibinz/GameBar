#![allow(clippy::new_without_default)]

mod background;
mod io;
mod layer;
mod oam;
mod sprite;
mod window;

use util::*;

use background::Background;
use layer::Layer;
use oam::Oam;
use sprite::Sprite;
use window::Window;

pub static TRANSPARENT: u16 = 0x8000;

pub struct Ppu {
    pub dispcnt: u16,  // Raw display control register
    pub dispstat: u16, // Raw display status
    pub vcount: u16,   // Line number of current scanline

    pub mode: u32,        // Video mode
    pub flip: bool,       // Determine page flipping in bitmap modes
    pub sequential: bool, // Determine layout of sprites, 1 - 1d, 0 - 2d
    pub fblank: bool,     // Force blanking

    pub palette: [u16; 0x200], // 16 bit colors
    pub vram: Vec<u8>,         // Tile mapping
    /// Sprite 0 - 127
    /// Affine sprite rotation / scaling parameter
    pub oam: Oam,

    pub background: [Background; 4], // Background 0 - 3
    pub window: Window,

    pub layer: [Layer; 5], // Layer 0 - 3, and an extra layer for backdrop
    pub buffer: [u16; 240 * 160], // Frame buffer, 240 * 160
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            dispcnt: 0,
            dispstat: 0,
            mode: 0,
            flip: false,
            sequential: false,
            fblank: false,
            vcount: 0,

            palette: [0; 0x200],
            vram: vec![0; 0x18000],
            oam: Oam::new(),

            background: [Background::new(); 4],
            window: Window::new(),

            layer: [Layer::new(); 5],
            buffer: [0; 240 * 160],
        }
    }

    pub fn hdraw(&mut self) {
        if self.fblank {
            self.force_blank()
        }
        assert!(self.vcount < 160);

        // Setup backdrop color
        let bd = self.backdrop();
        for p in self.layer[4].pixel.iter_mut() {
            *p = bd
        }

        for i in 0..4 {
            self.layer[i].clear();
        }

        self.draw_window();

        self.draw_background();

        self.draw_sprites();

        self.combine_layers();
    }

    pub fn hblank(&mut self) -> bool {
        self.dispstat |= 0b10;

        self.dispstat.bit(4)
    }

    pub fn vblank(&mut self) -> bool {
        self.dispstat |= 0b01;

        self.dispstat.bit(3)
    }

    pub fn increment_vcount(&mut self) -> bool {
        self.vcount += 1;
        self.dispstat &= !1;

        self.check_vmatch()
    }

    pub fn check_vmatch(&mut self) -> bool {
        self.dispstat.bit(5) && self.vcount == self.dispstat >> 8
    }

    pub fn rewind(&mut self) {
        assert_eq!(self.vcount, 228);
        self.dispstat &= !0b11;
        self.vcount = 0;
    }

    pub fn combine_layers(&mut self) {
        let n = self.vcount as usize * 240;
        let line = &mut self.buffer[n..n + 240];

        for (i, l) in line.iter_mut().enumerate() {
            for j in 0..5 {
                let pixel = self.layer[j].pixel[i];

                // Render the topmost opaque color
                if pixel != TRANSPARENT {
                    *l = pixel;
                    break;
                }
            }
        }
    }

    pub fn draw_background(&mut self) {
        match self.mode {
            0 => self.draw_mode_0(),
            1 => self.draw_mode_1(),
            2 => self.draw_mode_2(),
            3 => self.draw_bitmap_3(),
            4 => self.draw_bitmap_4(),
            5 => self.draw_bitmap_5(),
            _ => unreachable!(),
        }
    }

    pub fn draw_sprites(&mut self) {
        for i in (0..self.oam.sprite.len()).rev() {
            self.draw_sprite(i);
        }
    }

    pub fn draw_window(&mut self) {
        let window = &mut self.window;
        window.clear();

        if self.dispcnt.bits(15, 13) > 0 {
            window.draw_winout();
        }

        // if self.dispcnt.bit(15)
        // {
        //     let mut layer = Layer::new();
        //     let mut dummy = Window::new(); // Dummy window to let all sprite be drawn
        //     dummy.clear();

        //     for sprite in self.sprite.iter_mut().rev()
        //     {
        //         memory.update_sprite(sprite);

        //         if sprite.mode == 0b10 // Window mode
        //         {
        //             sprite.draw(self.vcount as u32, self.sequential, &dummy, &mut layer, memory);
        //         }
        //     }

        //     window.draw_objwin(&layer)
        // }

        if self.dispcnt.bit(14) {
            window.draw_winin(self.vcount as u32, 1);
        }

        if self.dispcnt.bit(13) {
            window.draw_winin(self.vcount as u32, 0);
        }
    }

    pub fn draw_mode_0(&mut self) {
        // Background is drawn in reverse order to give
        // precedence to ones with lower index.
        if self.dispcnt.bit(11) {
            self.draw_text_background(3)
        }
        if self.dispcnt.bit(10) {
            self.draw_text_background(2)
        }
        if self.dispcnt.bit(9) {
            self.draw_text_background(1)
        }
        if self.dispcnt.bit(8) {
            self.draw_text_background(0)
        }
    }

    pub fn draw_mode_1(&mut self) {
        if self.dispcnt.bit(10) {
            self.draw_affine_background(2)
        }
        if self.dispcnt.bit(9) {
            self.draw_text_background(1)
        }
        if self.dispcnt.bit(8) {
            self.draw_text_background(0)
        }
    }

    pub fn draw_mode_2(&mut self) {
        if self.dispcnt.bit(11) {
            self.draw_affine_background(3)
        }
        if self.dispcnt.bit(10) {
            self.draw_affine_background(2)
        }
    }

    pub fn force_blank(&mut self) {
        for i in self.buffer.iter_mut() {
            // Use grey to distinguish force blanking color
            *i = 0x7fff;
        }
    }
}
