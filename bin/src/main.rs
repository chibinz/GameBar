mod debug;
mod keyboard;

use minifb::Window;
use minifb::WindowOptions;

use std::env;
use std::marker::Send;

unsafe impl Send for debug::Debugger {}

struct Frame {
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

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage();
        return;
    }

    let rom = std::fs::read(&args[1]).unwrap();
    let bios = std::fs::read("rom/gba_bios.bin").unwrap();
    let mut console = Box::new(gba::Gba::new());
    debug::enable_debugger(&mut *console);

    // Must be called before any operation
    console.init();
    console.bus.bios = bios;
    console.cart.rom = rom;

    let mut gba_frame = Frame::new("GameBar", 240, 160, 2);
    let mut object_frame = Frame::new("Object", 256, 256, 2);

    while gba_frame.window.is_open() {
        let input = keyboard::input(&gba_frame.window);
        console.keypad.set_input(input, &mut console.irqcnt);
        console.step_frame();
        gba_frame.update_with_buffer(&console.ppu.buffer);
        // palette_frame.update_with_buffer(&console.ppu.palette);

        let first_object = console.ppu.decode_sprite(0);
        let (width, height) = console.ppu.oam.sprite[0].get_dimension();
        object_frame.resize(width as usize, height as usize);
        object_frame.update_with_buffer(&first_object);
    }
}

fn convert_buffer(orig: &[u16], new: &mut [u32]) {
    use util::Color;
    for (n, o) in new.iter_mut().zip(orig) {
        *n = o.to_rgb24();
    }
}

fn usage() {
    println!("usage: GameBar <rom>");
}
