pub mod sprite;
pub mod background;

use crate::util::*;
use crate::memory::Memory;

use background::Background;

pub struct PPU
{
    pub dispcnt   : u16,            // Raw display control register
    pub mode      : u32,            // Video mode
    pub flip      : bool,           // Determine page flipping in bitmap modes
    pub sequential: bool,           // Determine layout of sprites, 1 - 1d, 0 - 2d
    pub fblank    : bool,           // Force blanking
    pub vcount    : u32,            // Line number of current scanline

    pub buffer    : Vec<u32>,       // Frame buffer, 240 * 160
    pub background: Vec<Background>,
}

impl PPU
{
    pub fn new() -> Self
    {
        Self
        {
            dispcnt   : 0,
            mode      : 0,
            flip      : false,
            sequential: false,
            fblank    : false,
            vcount    : 0,

            buffer    : vec![0; 240 * 160],
            background: vec!
            [
                Background::new(0),
                Background::new(1),
                Background::new(2),
                Background::new(3)
            ],
        }
    }

    pub fn render(&mut self, memory: &mut Memory)
    {
        memory.update_ppu(self);

        let dispcnt = self.dispcnt;
        
        if self.fblank {self.force_blank()}
        if self.vcount >= 160 {return}       // Change to assertion
        if dispcnt.bits(11, 8) == 0 {return} // Transparent background

        match self.mode
        {
            0 => self.draw_mode_0(memory),
            1 => self.draw_mode_1(memory),
            3 => self.draw_mode_3(memory),
            4 => self.draw_mode_4(memory),
            5 => self.draw_mode_5(memory),
            _ => unimplemented!(),
        }
    }

    pub fn draw_mode_0(&mut self, memory: &Memory)
    {
        let dispcnt = self.dispcnt;

        if dispcnt.bit(8)  {self.background[0].draw_text(memory)}
        if dispcnt.bit(9)  {self.background[1].draw_text(memory)}
        if dispcnt.bit(10) {self.background[2].draw_text(memory)}
        if dispcnt.bit(11) {self.background[3].draw_text(memory)}

        let mut min = 0b11;
        let mut front = 0;
        for i in 0..4
        {
            if self.background[i].priority < min
            {
                min = self.background[i].priority;
                front = i;
            }
        }

        let front_bg = &self.background[front];
        let line_n = self.vcount as usize;
        let hscroll = front_bg.hscroll as usize;

        for i in 0..240
        {
            let x = (hscroll + i) % (front_bg.width * 8) as usize;
            self.buffer[line_n * 240 + i] = front_bg.pixel[x];
        }
    }

    pub fn draw_mode_1(&mut self, memory: &mut Memory)
    {
        let dispcnt = self.dispcnt;

        // if dispcnt.bit(8)  {self.background[0].draw_text(memory)}
        // if dispcnt.bit(9)  {self.background[1].draw_text(memory)}
        if dispcnt.bit(10) {self.background[2].draw_affine(memory)}
        // if dispcnt.bit(11) {self.background[3].draw_affine(memory)}

        let front_bg = &self.background[2];
        let line_n = self.vcount as usize;

        for i in 0..240
        {
            let x = (i) % (front_bg.width * 8) as usize;
            self.buffer[line_n * 240 + i] = front_bg.pixel[x];
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