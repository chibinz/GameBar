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

    let mut console = Box::new(console::Console::new());
    console.init(); // Must be called before any operation

    console.memory.load_rom(&args[1]);
    console.memory.load_bios(&"rom/gba_bios.bin".to_string());
    // console.cpu.backtrace_on_panic();

    let mut window = init_window();
    let mut converted = vec![0; console.ppu.buffer.len()];

    // debug(&mut console as *mut console::Console);

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
fn debug(c: *mut console::Console) {
    let mut debugger = debug::Debugger::new();
    debugger.console = c;

    std::thread::spawn(move || debugger.run());
}