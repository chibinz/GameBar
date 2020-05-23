use crate::util::*;
use crate::memory::Memory;
use crate::ppu::TRANSPARENT;
use crate::ppu::layer::Layer;

pub struct Window
{
    pub cnt: Vec<u8>,
}

impl Window
{
    pub fn new() -> Self
    {
        Self
        {
            cnt: vec![0; 256],
        }
    }

    pub fn draw_winin(&mut self, vcount: u32, index: usize, memory: &Memory)
    {
        let (x1, x2) = memory.get_winh(index);
        let (y1, y2) = memory.get_winv(index);
        let winin = memory.get_winin(index);

        let mut draw_x = ||
        {
            if x1 > x2
            {
                for i in 0..x2
                {
                    self.cnt[i as usize] = winin;
                }

                for i in x1..240
                {
                    self.cnt[i as usize] = winin;
                }
            }
            else
            {
                for i in x1..x2
                {
                    self.cnt[i as usize] = winin;
                }
            }
        };

        if y1 > y2
        {
            if vcount < y2 || vcount >= y1
            {
                draw_x()
            }
        }
        else
        {
            if vcount >= y1 && vcount < y2
            {
                draw_x()
            }
        }
    }

    pub fn draw_winout(&mut self, memory: &Memory)
    {
        let winout = memory.get_winout() as u8;

        for i in 0..self.cnt.len()
        {
            self.cnt[i] = winout;
        }
    }

    pub fn draw_objwin(&mut self, layer: &Layer, memory: &Memory)
    {
        let objwin = (memory.get_winout() >> 8) as u8;

        for i in 0..self.cnt.len()
        {
            if layer.pixel[i] != TRANSPARENT
            {
                self.cnt[i] = objwin;
            }
        }
    }

    pub fn get_display_flag(&self, x: u32, index: u32) -> bool
    {
        if x >= 240
        {
            return false;
        }
        self.cnt[x as usize].bit(index)
    }

    pub fn clear(&mut self)
    {
        for c in self.cnt.iter_mut()
        {
            *c = 0xff;
        }
    }
}
