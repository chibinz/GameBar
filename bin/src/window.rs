use crate::convert_buffer;

pub struct Window {
    inner: minifb::Window,
    name: &'static str,
    width: usize,
    height: usize,
    scale: usize,
    pub buffer: Vec<u32>,
}

impl std::ops::Deref for Window {
    type Target = minifb::Window;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Window {
    pub fn new(name: &'static str, width: usize, height: usize, scale: usize) -> Self {
        let buffer = vec![0; width * height];
        let minifb_scale = match scale {
            1 => minifb::Scale::X1,
            2 => minifb::Scale::X2,
            4 => minifb::Scale::X4,
            8 => minifb::Scale::X8,
            16 => minifb::Scale::X16,
            _ => minifb::Scale::X1,
        };
        let inner = minifb::Window::new(
            name,
            width,
            height,
            minifb::WindowOptions {
                scale: minifb_scale,
                ..minifb::WindowOptions::default()
            },
        )
        .unwrap();

        Self {
            inner,
            name,
            width,
            height,
            scale,
            buffer,
        }
    }

    pub fn resize(&mut self, w: usize, h: usize, s: usize) {
        let Self {
            name,
            width,
            height,
            scale,
            ..
        } = self;

        if (*width, *height, *scale) != (w, h, s) {
            *self = Self::new(name, w, h, s);
        }
    }

    pub fn update(&mut self) {
        self.inner
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    pub fn update_with_buffer(&mut self, buffer: &[u16]) {
        convert_buffer(buffer, &mut self.buffer);
        self.update();
    }

    pub fn get_input(&self) -> u16 {
        use minifb::Key::*;
        let mut ret = 0x3ff;
        let keys = [X, Z, Backspace, Enter, Right, Left, Up, Down, S, A];

        for (i, k) in keys.iter().enumerate() {
            if self.is_key_down(*k) {
                ret &= !(1 << (i as u16));
            }
        }

        ret
    }
}
