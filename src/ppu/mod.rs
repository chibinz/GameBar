pub mod background;

use crate::util::*;
use crate::memory::Memory;

use background::Background;

pub struct PPU
{
    pub buffer: Vec<u32>, // Frame buffer, 240 * 160
    pub background: Vec<Background>,
}

impl PPU
{
    pub fn new() -> Self
    {
        Self
        {
            buffer: vec![0; 240 * 160],
            background: vec!
            [
                Background::new(0),
                Background::new(1),
                Background::new(2),
                Background::new(3)
            ],
        }
    }

    pub fn render(&mut self, memory: &Memory)
    {
        let dispcnt = memory.get_dispcnt();

        // Currently, only mode 0 is supported
        assert_eq!(dispcnt.bits(2, 0), 0);
        
        if dispcnt.bit(7)
        {
            self.force_blank();
        }

        if dispcnt.bits(11, 8) > 0
        {
            if dispcnt.bit(8)  {self.background[0].draw_tile(memory)}
            if dispcnt.bit(9)  {self.background[1].draw_tile(memory)}
            if dispcnt.bit(10) {self.background[2].draw_tile(memory)}
            if dispcnt.bit(11) {self.background[3].draw_tile(memory)}

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

            let line_n = memory.get_vcount() as usize;
            let hofs = memory.get_bghofs(front) as usize;

            let width = self.background[front].width;
            for i in 0..240
            {
                if line_n < 160
                {
                    let x = (hofs + i) % (width * 8) as usize;

                    self.buffer[line_n * 240 + i] = self.background[front].pixel[x];
                }
            }
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