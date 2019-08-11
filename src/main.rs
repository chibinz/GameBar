pub mod cpu;

use std::{env, fs, io};
use std::time::{Duration, Instant};
use cpu::{register::Register, arm, thumb};

fn main() 
{
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1
    {
        panic!("usage: gba <rom>");
    }

    let mut reg = Register::new();
    let mut cpu0 = cpu::CPU::new(); 

    let file = fs::read(args[1].to_string()).unwrap();
    println!("rom size: 0x{:08x} bytes", file.len());

    println!("{:?}", cpu::Mode::ARM);

    let mut i = 0;
    while i < file.len()
    {
        print!("{:02x} ", file[i]);
        i += 1;

        if i % 0x100 == 0 && i > 0
        {
            println!("{}th byte, press any key to continue", i);
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }

        match cpu0.mode
        {
            cpu::Mode::ARM => 
            {
                let word: u32 = ((file[i] as u32)) + 
                                ((file[i + 1] as u32) << 8) + 
                                ((file[i + 2] as u32) << 16) + 
                                ((file[i + 3] as u32) << 24); 
                
                print!("{:08x}: {:08x} | {:032b} ", i, word, word);
                if arm::disassemble(word) == 1
                {
                    cpu0.mode = cpu::Mode::THUMB;
                }
                print!("\n");
                
                i += 4;
            },
            cpu::Mode::THUMB =>
            {
                let halfword: u16 = (file[i] as u16) + ((file[i + 1] as u16) << 8);

                print!("{:08x}: {:04x} | {:016b} ", i, halfword, halfword);
                if thumb::disassemble(halfword) == 0
                {
                    // cpu0.mode = cpu::Mode::ARM;
                }
                print!("\n");

                i += 2;
            }
        };
    }

    println!("time elapsed: {:?}", start.elapsed());
}

