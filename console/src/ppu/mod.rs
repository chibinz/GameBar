pub mod background;
pub mod layer;
pub mod sprite;
pub mod window;

use crate::interrupt::IRQController;
use crate::interrupt::Interrupt::*;
use util::*;

use background::Background;
use layer::Layer;
use sprite::Sprite;
use window::Window;

pub static TRANSPARENT: u16 = 0x8000;

pub struct PPU {
    pub dispcnt: u16,  // Raw display control register
    pub dispstat: u16, // Raw display status
    pub vcount: u16,   // Line number of current scanline

    pub mode: u32,        // Video mode
    pub flip: bool,       // Determine page flipping in bitmap modes
    pub sequential: bool, // Determine layout of sprites, 1 - 1d, 0 - 2d
    pub fblank: bool,     // Force blanking

    pub palette: Vec<u16>,   // 16 bit colors
    pub vram: Vec<u8>,       // Tile mapping
    pub obj_param: Vec<u16>, // Affine sprite rotation / scaling parameter

    pub background: Vec<Background>, // Background 0 - 3
    pub sprite: Vec<Sprite>,         // Sprite 0 - 127
    pub window: Window,

    pub layer: Vec<Layer>, // Layer 0 - 3, and an extra layer for backdrop
    pub buffer: Vec<u16>,  // Frame buffer, 240 * 160

    pub state: PPUState,
}

#[derive(Debug)]
pub enum PPUState {
    HDraw,       // HDraw interval
    HBlank,      // HBlank interval
    HBlankStart, // Transition to HBlank, (instant)
    VBlank,      // VBlank interval
    VBlankStart, // Transition to VBlank,
    EndOfLine,   // Scanline Complete
                 // VCount match is not considered as a state because it
                 // may occur simultaneously with other states
}

impl PPU {
    pub fn new() -> Self {
        let mut p = Self {
            dispcnt: 0,
            dispstat: 0,
            mode: 0,
            flip: false,
            sequential: false,
            fblank: false,
            vcount: 227, // VCount is incremented at beginning of each newline

            palette: vec![0; 0x200],
            vram: vec![0; 0x18000],
            obj_param: vec![0; 0x100],

            background: vec![Background::new(); 4],
            sprite: vec![Sprite::new(); 128],
            window: Window::new(),

            layer: vec![Layer::new(); 5],
            buffer: vec![0; 240 * 160],
            state: PPUState::HDraw,
        };

        for i in 0..4 {
            p.background[i].index = i;
        }

        for i in 0..128 {
            p.sprite[i].index = i;
        }

        p
    }

    pub fn hdraw(&mut self) {
        self.dispstat &= !0b11;

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

        self.state = PPUState::HBlankStart;
    }

    pub fn hblank(&mut self, irqcnt: &mut IRQController) {
        self.dispstat |= 0b10;
        self.state = PPUState::HBlank;

        if self.dispstat.bit(4) {
            irqcnt.request(HBlank)
        }
    }

    pub fn vblank(&mut self, irqcnt: &mut IRQController) {
        self.dispstat |= 0b01;
        self.state = PPUState::VBlank;

        if self.dispstat.bit(3) && self.vcount == 160 {
            irqcnt.request(VBlank)
        }
    }

    pub fn increment_vcount(&mut self, irqcnt: &mut IRQController) {
        self.vcount += 1;

        if self.vcount > 227 {
            self.vcount = 0
        }
        self.state = if self.vcount < 160 {
            PPUState::HDraw
        } else {
            PPUState::VBlankStart
        };
        self.check_vmatch(irqcnt);
    }

    pub fn check_vmatch(&mut self, irqcnt: &mut IRQController) {
        if self.dispstat.bit(5) && self.vcount == self.dispstat >> 8 {
            irqcnt.request(VCount)
        }
    }

    pub fn combine_layers(&mut self) {
        let n = self.vcount as usize * 240;
        let line = &mut self.buffer[n..n + 240];

        for i in 0..240 {
            for j in 0..5 {
                let pixel = self.layer[j].pixel[i];

                // Render the topmost opaque color
                if pixel != TRANSPARENT {
                    line[i] = pixel;
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
        for i in (0..self.sprite.len()).rev() {
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
