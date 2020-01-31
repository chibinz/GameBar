#![allow(non_snake_case)]

pub mod cpu;
pub mod ppu;
pub mod memory;
pub mod console;
pub mod util;
pub mod debug;

use std::env;

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

    while debugger.console.window.is_open() 
      && !debugger.console.window.is_key_down(minifb::Key::Escape)
    {
        debugger.step();
    }
}