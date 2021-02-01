use crate::ppu::layer::Layer;
use crate::ppu::TRANSPARENT;
use crate::util::*;
use std::marker::Copy;

#[derive(Clone, Copy)]
pub struct Window {
    pub winh: [u16; 2],
    pub winv: [u16; 2],
    pub winin: u16,
    pub winout: u16,
    pub cnt: [u8; 256],
}

impl Window {
    pub fn new() -> Self {
        Self {
            winh: [0; 2],
            winv: [0; 2],
            winin: 0,
            winout: 0,
            cnt: [0; 256],
        }
    }

    pub fn draw_winin(&mut self, vcount: u32, index: usize) {
        let x1 = self.winh[index].bits(15, 8);
        let x2 = self.winh[index].bits(7, 0);
        let y1 = self.winv[index].bits(15, 8);
        let y2 = self.winv[index].bits(7, 0);

        let winin = self.winin.to_le_bytes()[index];

        let mut draw_x = || {
            if x1 > x2 {
                for i in 0..x2 {
                    self.cnt[i as usize] = winin;
                }

                for i in x1..240 {
                    self.cnt[i as usize] = winin;
                }
            } else {
                for i in x1..x2 {
                    self.cnt[i as usize] = winin;
                }
            }
        };

        if y1 > y2 {
            if vcount < y2 || vcount >= y1 {
                draw_x()
            }
        } else {
            if vcount >= y1 && vcount < y2 {
                draw_x()
            }
        }
    }

    pub fn draw_winout(&mut self) {
        for cnt in self.cnt.iter_mut() {
            *cnt = self.winout as u8;
        }
    }

    #[allow(dead_code)]
    pub fn draw_objwin(&mut self, layer: &Layer) {
        for (i, cnt) in self.cnt.iter_mut().enumerate() {
            if layer.pixel[i] != TRANSPARENT {
                *cnt = (self.winout >> 8) as u8;
            }
        }
    }

    pub fn get_display_flag(&self, x: u32, index: u32) -> bool {
        if x >= 240 {
            return false;
        }

        self.cnt[x as usize].bit(index)
    }

    pub fn clear(&mut self) {
        for c in self.cnt.iter_mut() {
            *c = 0xff;
        }
    }
}
