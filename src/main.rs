pub mod cpu;
pub mod memory;
pub mod debug;
pub mod console;

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
    console.load_gamepak(&args[1]);

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