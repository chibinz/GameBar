#![allow(non_snake_case)]

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
        256,
        256,
        WindowOptions
        {
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        }
    ).unwrap();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape)
    {

        debugger.step();

        window.update_with_buffer(&debugger.console.ppu.buffer, 256, 256).unwrap();
    }
}