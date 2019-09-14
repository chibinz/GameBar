pub mod cpu;
pub mod memory;

use std::{env, fs, io};
use std::time::Instant;

fn main() 
{
    let args: Vec<String> = env::args().collect();

    if args.len() == 1
    {
        panic!("usage: gba <rom>");
    }

    let mut cpu = cpu::CPU::new(); 
    let mut memory = memory::Memory::new();
    memory.load_rom(&args[1]);

    let file = fs::read(args[1].to_string()).unwrap();
    println!("rom size: 0x{:08x} bytes", file.len());
    
    let start = Instant::now();
    cpu.register.r[15] = 0x08000000;
    while (cpu.register.r[15] as usize) - 0x08000000 < file.len()
    {
        // if cpu.register.r[15] % 0x100 == 0
        // {
        //     println!("{:08x}th byte, press any key to continue", cpu.register.r[15]);
        //     let mut input = String::new();
        //     io::stdin().read_line(&mut input).unwrap();
        // }
        
        let word = memory.load32(cpu.register.r[15]);
        print!("{:08x}: {:08x} | {:032b} ", cpu.register.r[15], word, word);
        println!("{}", cpu::arm::disassemble(word));

        // let halfword: u16 = (file[i] as u16) + ((file[i + 1] as u16) << 8);
        // print!("{:08x}: {:04x} | {:016b} ", i, halfword, halfword);
        // thumb::disassemble(halfword);
        // print!("{}\n", thumb::disassemble(halfword));

        cpu.register.r[15] += 4;
    }

    println!("time elapsed: {:?}", start.elapsed());
}