mod debug;
mod window;

use std::env;

pub use window::Window;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
       return usage();
    }

    let rom = std::fs::read(&args[1]).unwrap();
    let bios = std::fs::read("rom/gba_bios.bin").unwrap();
    let mut gba = Box::new(gba::Gba::new());

    // Must be called before any operation
    gba.init();
    gba.bus.bios = bios;
    gba.cart.rom = rom;

    let debugger = debug::init_debugger(&mut *gba);
    let mut window = Window::new("GameBar", 240, 160, 2);
    window.topmost(true);

    while window.is_open() {
        gba.step_frame();
        gba.keypad.set_input(window.get_input(), &mut gba.irqcnt);
        window.update_with_buffer(&gba.ppu.buffer);
        debugger.display_palette();
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
