pub mod sprite;
pub mod background;

use crate::util::*;
use crate::memory::Memory;

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

        match self.mode
        {
            0 => self.draw_mode_0(memory),
            1 => self.draw_mode_1(memory),
            3 => self.draw_mode_3(memory),
            4 => self.draw_mode_4(memory),
            5 => self.draw_mode_5(memory),
            _ => unimplemented!(),
        }

        // self.draw_sprite(memory);
    }

    pub fn draw_sprite(&mut self, memory: &Memory)
    {
        let mut pixel: Vec<u32> = vec![0; 512]; // Line buffer for sprites

        for _ in 0..128
        {
            self.sprite[0].draw(self.vcount, self.sequential, &mut pixel, memory);
        }

        for i in 0..240
        {
            self.buffer[self.vcount as usize * 240 + i] = pixel[i];
        }
    }

    pub fn draw_mode_0(&mut self, memory: &Memory)
    {
        let mut min = 4;
        for i in 0..1
        {
            if self.dispcnt.bit(8 + i as u32) && self.background[i].priority < min
            {
                min = self.background[i].priority;

                self.background[i].draw_text(memory);
                let bg = &self.background[i];
                let line_n = self.vcount as usize;
                let hscroll = bg.hscroll as usize;

                for i in 0..240
                {
                    let x = (hscroll + i) % (bg.width * 8) as usize;
                    self.buffer[line_n * 240 + i] = bg.pixel[x];
                }
            }
        }
    }

    pub fn draw_mode_1(&mut self, memory: &Memory)
    {
        let mut min = 0b11;
        for i in 0..2
        {
            if self.dispcnt.bit(8 + i as u32) && self.background[i].priority < min
            {
                min = self.background[i].priority;

                self.background[i].draw_text(memory);
                let bg = &self.background[i];
                let line_n = self.vcount as usize;
                let hscroll = bg.hscroll as usize;

                for i in 0..240
                {
                    let x = (hscroll + i) % (bg.width * 8) as usize;
                    self.buffer[line_n * 240 + i] = bg.pixel[x];
                }
            }
        }

        if self.dispcnt.bit(10)
        {
            self.background[2].draw_affine(memory);

            let front_bg = &self.background[2];
            let line_n = self.vcount as usize;

            for i in 0..240
            {
                let x = (i) % (front_bg.width * 8) as usize;
                self.buffer[line_n * 240 + i] = front_bg.pixel[x];
            }
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

    pub fn force_blank(&mut self)
    {
        for i in self.buffer.iter_mut()
        {
            *i = 0;
        }
    }
}