pub mod background;

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

    pub fn render(&mut self, memory: &mut Memory)
    {
        self.render_mode_0(memory);
    }
}