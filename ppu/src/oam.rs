use crate::Sprite;

use util::Bus;

pub struct OAM {
    pub sprite: [Sprite; 128],
    pub param: [u16; 256],
}

impl OAM {
    pub fn new() -> Self {
        Self {
            sprite: [Sprite::new(); 128],
            param: [0; 256],
        }
    }
}

/// Object attribute memory access
impl Bus for OAM {
    #[inline]
    fn store16(&mut self, offset: usize, value: u16) {
        // offset is in bytes
        let obj = (offset / 2) / 4;
        let attr = (offset / 2) % 4;

        match attr {
            0 => self.sprite[obj].set_attr0(value),
            1 => self.sprite[obj].set_attr1(value),
            2 => self.sprite[obj].set_attr2(value),
            _ => self.param[obj] = value,
        }
    }

    #[inline]
    fn load16(&self, offset: usize) -> u16 {
        let obj = (offset / 2) / 4;
        let attr = (offset / 2) % 4;

        match attr {
            0..=2 => self.sprite[obj].attr[attr],
            _ => self.param[obj],
        }
    }
}
