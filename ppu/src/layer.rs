use super::window::Window;
use super::TRANSPARENT;

#[derive(Clone, Copy)]
pub struct Layer {
    pub pixel: [u16; 240],
}

impl Layer {
    pub fn new() -> Self {
        Self {
            pixel: [TRANSPARENT; 240],
        }
    }

    pub fn paint(&mut self, x: u32, color: u16, window: &Window, index: usize) {
        if color == TRANSPARENT || x >= 240 {
            return;
        }

        if window.get_display_flag(x, index) {
            self.pixel[x as usize] = color;
        }
    }

    pub fn clear(&mut self) {
        for p in self.pixel.iter_mut() {
            *p = TRANSPARENT;
        }
    }
}
