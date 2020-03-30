#![allow(non_snake_case)]

pub mod cpu;
pub mod ppu;
pub mod dma;
pub mod timer;
pub mod interrupt;
pub mod memory;
pub mod console;
pub mod keyboard;
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
    console.memory.console = &mut console as *mut console::Console;
    console.irqcnt.cpu = &mut console.cpu as *mut cpu::CPU;

    console.memory.load_rom(&args[1]);
    console.memory.load_bios(&"rom/gba_bios.bin".to_string());

    // let mut debugger = debug::Debugger::new(&mut console);

    while console.window.is_open()
    {
        let input = keyboard::input(&console.window);
        console.keypad.set_input(input, &mut console.irqcnt);
        console.step_frame();

        // debugger.step();
    }
}