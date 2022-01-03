use minifb::Window;
use minifb::WindowOptions;

use crate::convert_buffer;

pub struct Frame {
    pub window: Window,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Frame {
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
        let window = Window::new(
            name,
            width,
            height,
            WindowOptions {
                scale,
                resize: true,
                scale_mode: minifb::ScaleMode::UpperLeft,
                ..WindowOptions::default()
            },
        )
        .unwrap();

        Self {
            window,
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
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
}
