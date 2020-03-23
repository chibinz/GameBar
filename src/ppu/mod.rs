pub mod color;
pub mod layer;
pub mod background;
pub mod sprite;
pub mod window;

use crate::util::*;
use crate::memory::Memory;

use color::*;
use layer::Layer;
use background::Background;
use sprite::Sprite;
use window::Window;

pub struct PPU
{
    pub dispcnt   : u16,            // Raw display control register
    pub mode      : u32,            // Video mode
    pub flip      : bool,           // Determine page flipping in bitmap modes
    pub sequential: bool,           // Determine layout of sprites, 1 - 1d, 0 - 2d
    pub fblank    : bool,           // Force blanking
    pub vcount    : u32,            // Line number of current scanline

    pub background: Vec<Background>,   // Background 0 - 3
    pub sprite    : Vec<Sprite>,       // Sprite 0 - 127
    pub window    : Window,

    pub layer     : Vec<Layer>,        // Layer 0 - 3, and an extra layer for backdrop
    pub buffer    : Vec<u32>,          // Frame buffer, 240 * 160
}

impl PPU
{
    pub fn new() -> Self
    {
        let mut p = Self
        {
            dispcnt   : 0,
            mode      : 0,
            flip      : false,
            sequential: false,
            fblank    : false,
            vcount    : 0,

            background  : vec![Background::new(); 4],
            sprite      : vec![Sprite::new(); 128],
            window      : Window::new(),

            layer       : vec![Layer::new(); 5],
            buffer      : vec![0; 240 * 160],
        };

        for i in 0..4
        {
            p.background[i].index = i;
        }

        for i in 0..128
        {
            p.sprite[i].index = i;
        }

        p
    }

    pub fn render(&mut self, memory: &Memory)
    {
        memory.update_ppu(self);

        if self.fblank {self.force_blank()}
        if self.vcount >= 160 {return} // Change to assertion

        // Setup backdrop color
        for pixel in self.layer[4].pixel.iter_mut()
        {
            *pixel = memory.bg_palette(0, 0);
        }

        for i in 0..4
        {
            self.layer[i].clear();
        }

        self.draw_window(memory);

        match self.mode
        {
            0 => self.draw_mode_0(memory),
            1 => self.draw_mode_1(memory),
            2 => self.draw_mode_2(memory),
            3 => self.draw_mode_3(memory),
            4 => self.draw_mode_4(memory),
            5 => self.draw_mode_5(memory),
            _ => unreachable!(),
        }

        self.draw_sprite(memory);

        self.combine_layers();
    }
    
    pub fn combine_layers(&mut self)
    {
        let n = self.vcount as usize * 240;
        let line = &mut self.buffer[n..n+240];

        for i in 0..240
        {
            for j in 0..5
            {
                let pixel = self.layer[j].pixel[i];

                // Render the topmost opaque color
                if pixel != TRANSPARENT
                {
                    line[i] = pixel;
                    break
                }
            }
        }
    }

    pub fn draw_mode_0(&mut self, memory: &Memory)
    {
        // Background is drawn in reverse order to give
        // precedence to ones with lower index.
        self.draw_text_bg(3, memory);
        self.draw_text_bg(2, memory);
        self.draw_text_bg(1, memory);
        self.draw_text_bg(0, memory);
    }

    pub fn draw_mode_1(&mut self, memory: &Memory)
    {
        self.draw_affine_bg(2, memory);
        self.draw_text_bg(1, memory);
        self.draw_text_bg(0, memory);
    }

    pub fn draw_mode_2(&mut self, memory: &Memory)
    {
        self.draw_affine_bg(3, memory);
        self.draw_affine_bg(2, memory);
    }

    pub fn draw_mode_3(&mut self, memory: &Memory)
    {
        debug_assert!(self.dispcnt.bit(10));

        self.background[2].draw_bitmap_3(&self.window, &mut self.layer[0], memory);

    }

    pub fn draw_mode_4(&mut self, memory: &Memory)
    {
        debug_assert!(self.dispcnt.bit(10));

        self.background[2].draw_bitmap_4(self.flip, &self.window, &mut self.layer[0], memory);
    }

    pub fn draw_mode_5(&mut self, memory: &Memory)
    {
        debug_assert!(self.dispcnt.bit(10));

        let line_n = self.vcount as usize;
        if line_n > 127 {return}

        self.background[2].draw_bitmap_5(self.flip, &self.window, &mut self.layer[0], memory);
    }

    pub fn draw_sprite(&mut self, memory: &Memory)
    {
        for sprite in self.sprite.iter_mut().rev()
        {
            memory.update_sprite(sprite);

            let priority = sprite.priority as usize;
            let layer = &mut self.layer[priority];

            sprite.draw(self.vcount, self.sequential, &self.window, layer, memory);
        }
    }

    pub fn draw_window(&mut self, memory: &Memory)
    {
        let window = &mut self.window;
        window.clear();

        if self.dispcnt.bits(15, 13) > 0
        {
            window.draw_winout(memory);
        }

        if self.dispcnt.bit(15)
        {
            let mut layer = Layer::new();
            let mut dummy = Window::new(); // Dummy window to let all sprite be drawn
            dummy.clear();

            for sprite in self.sprite.iter_mut().rev()
            {
                memory.update_sprite(sprite);

                if sprite.mode == 0b10 // Window mode
                {
                    sprite.draw(self.vcount, self.sequential, &dummy, &mut layer, memory);
                }
            }

            window.draw_objwin(&layer, memory)
        }

        if self.dispcnt.bit(14)
        {
            window.draw_winin(self.vcount, 1, memory);
        }

        if self.dispcnt.bit(13)
        {
            window.draw_winin(self.vcount, 0, memory);
        }
    }

    pub fn draw_text_bg(&mut self, i: u32, memory: &Memory)
    {
        let bg = &mut self.background[i as usize];

        if self.dispcnt.bit(8 + i) 
        {
            memory.update_text_bg(bg);

            let priority = bg.priority as usize;
            let layer = &mut self.layer[priority as usize];

            bg.draw_text(&self.window, layer, memory);
        } 
    }

    pub fn draw_affine_bg(&mut self, i: u32, memory: &Memory)
    {
        let bg = &mut self.background[i as usize];
        
        if self.dispcnt.bit(8 + i) 
        {
            memory.update_affine_bg(bg);

            let priority = bg.priority as usize;
            let layer = &mut self.layer[priority as usize];

            bg.draw_affine(&self.window, layer, memory);
        } 
    }

    pub fn force_blank(&mut self)
    {
        for i in self.buffer.iter_mut()
        {
            *i = 0;
        }
    }
}