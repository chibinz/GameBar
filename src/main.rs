pub mod cpu;
pub mod memory;
pub mod console;
pub mod util;

use std::env;
use std::time::Instant;

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

    let start = Instant::now();
    loop
    {
        if !console.debug()
        {
            break;
        }
    }

    println!("time elapsed: {:?}", start.elapsed());
}