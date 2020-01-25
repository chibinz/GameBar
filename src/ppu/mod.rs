pub mod background;

use crate::util::*;
use crate::memory::Memory;

pub struct PPU
{
    pub buffer: Vec<u32>, // Frame buffer, 240 * 160
}

impl PPU
{
    pub fn new() -> Self
    {
        Self
        {
            buffer: vec![0; 256 * 256],
        }
    }

    pub fn render(&mut self, memory: &Memory)
    {
        let dispcnt = memory.load32(0x04000000);
        
        if dispcnt.bit(7)
        {
            self.blank();
        }

        if dispcnt.bit(8)
        {
            self.render_background(memory);
        }
    }

    pub fn blank(&mut self)
    {
        for i in self.buffer.iter_mut()
        {
            *i = 0;
        }
    }

    pub fn get_vcount(&self, memory: &Memory) -> u32
    {
        memory.load16(0x04000006) as u32
    }

    pub fn set_vcount(&self, vcount: u32, memory: &mut Memory)
    {
        memory.store16(0x04000006, vcount as u16)
    }
}