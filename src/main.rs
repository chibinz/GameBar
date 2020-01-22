#![allow(non_snake_case)]

extern crate minifb;

pub mod cpu;
pub mod ppu;
pub mod memory;
pub mod console;
pub mod util;
pub mod debug;

use std::env;
use minifb::Window;
use minifb::WindowOptions;

fn main() 
{
    let args: Vec<String> = env::args().collect();

    if args.len() == 1
    {
        panic!("usage: gba <rom>");
    }

    let mut console = console::Console::new();
    console.memory.load_rom(&args[1]);
    console.memory.load_bios(&"rom/gba_bios.bin".to_string());
    
    let mut debugger = debug::Debugger::new(&mut console);

    let mut window = Window::new
    (
        "ESC to exit",
        240,
        160,
        WindowOptions::default(),
    ).unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(minifb::Key::Escape)
    {

        debugger.step();

        window.update_with_buffer(&debugger.console.ppu.buffer, 240, 160).unwrap();
    }
}