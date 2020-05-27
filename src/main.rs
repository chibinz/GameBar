#![allow(non_snake_case)]

mod cpu;
mod ppu;
mod dma;
mod timer;
mod interrupt;
mod memory;
mod console;
mod keyboard;
mod util;
mod debug;

use std::env;
use std::marker::Send;

unsafe impl Send for debug::Debugger {}

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2
    {
        usage();
        return;
    }

    let mut console = console::Console::new();
    console.memory.console = &mut console as *mut console::Console;
    console.irqcnt.cpu = &mut console.cpu as *mut cpu::CPU;
    console.cpu.dma = &mut console.dma as *mut dma::DMA;

    console.memory.load_rom(&args[1]);
    console.memory.load_bios(&"rom/gba_bios.bin".to_string());

    let mut debugger = debug::Debugger::new();
    debugger.console = &mut console as *mut console::Console;

    std::thread::spawn(move || debug(debugger));

    while console.window.is_open()
    {
        let input = keyboard::input(&console.window);
        console.keypad.set_input(input, &mut console.irqcnt);
        console.step_frame();
    }
}

fn usage()
{
    println!("usage: GameBar <rom>");
}

fn debug(mut debugger: debug::Debugger)
{
    debugger.run();
}