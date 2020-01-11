use crate::cpu;
use crate::cpu::CPU;
use crate::memory::Memory;

use std::io;
use std::io::prelude::*;

pub struct Console
{
    pub cpu   : CPU,
    pub memory: Memory,
}

impl Console
{
    pub fn new() -> Console
    {
        Self
        {
            cpu   : CPU::new(),
            memory: Memory::new(),
        }
    }

    pub fn step(&mut self)
    {
        self.cpu.step(&mut self.memory);
    }

    pub fn load_gamepak(&mut self, gamepak: &String)
    {
        self.memory.load_rom(gamepak);
        self.cpu.register.r[15] = 0x08000000;
    }

    pub fn debug(&mut self) -> bool
    {
        print!("(debug) ");
        io::stdout().flush().ok().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let command: Vec<&str> = input.split_whitespace().collect();

        if command.is_empty()
        {
            self.step();
            return true;
        }

        match command[0]
        {
            "print"       => self.print(),
            "disassemble" => self.disassemble(),
            "step"        => self.step(),
            "quit"        => return false,
            _             => self.step(),
        };

        true
    }

    pub fn print(&self)
    {
        println!("{}", self.cpu);
    }


    pub fn disassemble(&self)
    {
        if self.cpu.in_thumb_mode()
        {
            let halfword = self.memory.load16(self.cpu.register.r[15]);
            print!("{:08x}: {:04x} | {:016b} ", self.cpu.register.r[15], halfword, halfword);
            println!("{}", cpu::thumb::disassemble::disassemble(halfword));
        }
        else
        {
            let word = self.memory.load32(self.cpu.register.r[15]);
            print!("{:08x}: {:08x} | {:032b} ", self.cpu.register.r[15], word, word);
            println!("{}", cpu::arm::disassemble::disassemble(word));
        }
    }
}