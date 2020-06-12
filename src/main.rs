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

use minifb::Window;
use minifb::WindowOptions;

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

    let mut window = init_window();

    // debug(&mut console as *mut console::Console);

    while window.is_open()
    {
        let input = keyboard::input(&window);
        console.keypad.set_input(input, &mut console.irqcnt);
        console.step_frame();
        window.update_with_buffer(&console.ppu.buffer, 240, 160).unwrap();
    }
}

fn usage()
{
    println!("usage: GameBar <rom>");
}

fn init_window() -> Window
{
    Window::new
    (
        "GameBar",
        240,
        160,
        WindowOptions
        {
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        }
    ).unwrap()
}

#[allow(dead_code)]
fn debug(c: *mut console::Console)
{
    let mut debugger = debug::Debugger::new();
    debugger.console = c;

    std::thread::spawn(move || debugger.run());
}