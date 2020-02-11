pub mod color;
pub mod layer;
pub mod background;
pub mod sprite;

use crate::util::*;
use crate::memory::Memory;

use color::*;
use layer::Layer;
use background::Background;
use sprite::Sprite;

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

    pub layer     : Vec<Layer>,
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
            layer       : vec![Layer::new(); 4],
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

        for i in 0..4
        {
            self.layer[i].clear();
        }

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
            for j in 0..4
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
        for i in (0..4).rev()
        {
            self.draw_text_bg(i, memory);
        }
    }

    pub fn draw_mode_1(&mut self, memory: &Memory)
    {
        self.draw_affine_bg(2, memory);
        
        for i in (0..2).rev()
        {
            self.draw_text_bg(i, memory);
        }
    }

    pub fn draw_mode_2(&mut self, memory: &Memory)
    {
        for i in (2..4).rev()
        {
            self.draw_affine_bg(i, memory);
        }
    }

    pub fn draw_mode_3(&mut self, memory: &Memory)
    {
        debug_assert!(self.dispcnt.bit(10));

        self.background[2].draw_bitmap_3(memory);

        let line_n = self.vcount as usize;
        for i in 0..240
        {
            self.buffer[line_n * 240 + i] = self.background[2].pixel[i];
        }
    }

    pub fn draw_mode_4(&mut self, memory: &Memory)
    {
        debug_assert!(self.dispcnt.bit(10));

        self.background[2].draw_bitmap_4(self.flip, memory);

        let line_n = self.vcount as usize;
        for i in 0..240
        {
            self.buffer[line_n * 240 + i] = self.background[2].pixel[i];
        }
    }

    pub fn draw_mode_5(&mut self, memory: &Memory)
    {
        debug_assert!(self.dispcnt.bit(10));

        self.background[2].draw_bitmap_5(self.flip, memory);

        let line_n = self.vcount as usize;
        if line_n > 127 {return}

        for i in 0..160
        {
            self.buffer[line_n * 240 + i] = self.background[2].pixel[i];
        }
    }

    pub fn draw_sprite(&mut self, memory: &Memory)
    {
        for i in (0..128).rev()
        {
            let sprite = &mut self.sprite[i];
            memory.update_sprite(sprite);

            let priority = sprite.priority as usize;
            let layer = &mut self.layer[priority];

            sprite.draw(self.vcount, self.sequential, layer, memory);
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

            bg.draw_text(layer, memory);
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

            bg.draw_affine(layer, memory);
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