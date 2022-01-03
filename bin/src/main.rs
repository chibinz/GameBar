mod debug;
mod keyboard;
mod window;

use std::env;
use window::Frame;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage();
        return;
    }

    let rom = std::fs::read(&args[1]).unwrap();
    let bios = std::fs::read("rom/gba_bios.bin").unwrap();
    let mut gba = Box::new(gba::Gba::new());

    // Must be called before any operation
    gba.init();
    gba.bus.bios = bios;
    gba.cart.rom = rom;
    debug::init_debugger(&mut *gba);

    let mut gba_frame = Frame::new("GameBar", 240, 160, 2);
    let mut object_frame = Frame::new("Object", 256, 256, 2);

    while gba_frame.window.is_open() {
        let input = keyboard::input(&gba_frame.window);
        gba.keypad.set_input(input, &mut gba.irqcnt);
        gba.step_frame();
        gba_frame.update_with_buffer(&gba.ppu.buffer);
        // palette_frame.update_with_buffer(&console.ppu.palette);

        let first_object = gba.ppu.decode_sprite(0);
        let (width, height) = gba.ppu.oam.sprite[0].get_dimension();
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
