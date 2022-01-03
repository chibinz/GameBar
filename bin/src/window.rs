use crate::convert_buffer;

pub struct Window {
    inner: minifb::Window,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
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
    pub fn new(name: &str, width: usize, height: usize, scale: usize) -> Self {
        let buffer = vec![0; width * height];
        let scale = match scale {
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
                scale,
                resize: true,
                scale_mode: minifb::ScaleMode::UpperLeft,
                ..minifb::WindowOptions::default()
            },
        )
        .unwrap();

        Self {
            inner,
            width,
            height,
            buffer,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        assert!(self.buffer.len() >= width * height);
    }

    pub fn update_with_buffer(&mut self, buffer: &[u16]) {
        convert_buffer(buffer, &mut self.buffer);
        self.inner
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
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
