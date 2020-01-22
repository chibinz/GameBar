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
            buffer: vec![0; 240 * 160],
        }
    }

    pub fn render(&mut self, memory: &mut Memory)
    {
        let color = to_0RGB(memory.load16(0x05000000));

        for i in 0..self.buffer.len()
        {
            self.buffer[i] = color;
        }
    }
}

fn to_0RGB(a: u16) -> u32
{
    let r = a.bits(4, 0) << 19;
    let g = a.bits(9, 5) << 11;
    let b = a.bits(14, 10) << 3;

    r | g | b
}