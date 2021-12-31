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
    console.init(); // Must be called before any operation
    console.bus.bios = bios;
    console.cart.rom = rom;
    // console.cpu.backtrace_on_panic();

    let mut window = init_window();
    let mut converted = vec![0; console.ppu.buffer.len()];

    while window.is_open() {
        let input = keyboard::input(&window);
        console.keypad.set_input(input, &mut console.irqcnt);
        console.step_frame();
        convert_buffer(&console.ppu.buffer, &mut converted);
        window.update_with_buffer(&converted, 240, 160).unwrap();
    }

    unreachable!();
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

fn init_window() -> Window {
    Window::new(
        "GameBar",
        240,
        160,
        WindowOptions {
            scale: minifb::Scale::X2,
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
