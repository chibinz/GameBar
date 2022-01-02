mod debug;
mod keyboard;

use minifb::Window;
use minifb::WindowOptions;

use std::env;
use std::marker::Send;

unsafe impl Send for debug::Debugger {}

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

    // Must be called before any operation
    console.init();
    console.bus.bios = bios;
    console.cart.rom = rom;
    console.cpu.backtrace_on_panic();

    let mut window = init_window("GameBar", 240, 160, 2);
    let mut converted = vec![0; console.ppu.buffer.len()];

    let mut background_palette_window = init_window("Background Palette", 16, 16, 16);
    let mut background_palette_buffer = vec![0; 16 * 16];

    while window.is_open() {
        let input = keyboard::input(&window);
        console.keypad.set_input(input, &mut console.irqcnt);
        console.step_frame();
        convert_buffer(&console.ppu.buffer, &mut converted);
        convert_buffer(&console.ppu.palette[..256], &mut background_palette_buffer);
        background_palette_window.update_with_buffer(&background_palette_buffer, 16, 16).unwrap();
        window.update_with_buffer(&converted, 240, 160).unwrap();
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

fn init_window(name: &str, width: usize, height: usize, scale: usize) -> Window {
    let scale = match scale {
        1 => minifb::Scale::X1,
        2 => minifb::Scale::X2,
        4 => minifb::Scale::X4,
        8 => minifb::Scale::X8,
        16 => minifb::Scale::X16,
        _ => minifb::Scale::X1,
    };

    Window::new(
        name,
        width,
        height,
        WindowOptions {
            scale,
            ..WindowOptions::default()
        },
    )
    .unwrap()
}

#[allow(dead_code)]
fn debug(c: *mut gba::Gba) {
    let mut debugger = debug::Debugger::new();
    debugger.console = c;

    std::thread::spawn(move || debugger.run());
}
