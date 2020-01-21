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

    pub fn print(&self)
    {
        self.cpu.print();

        self.disassemble();
    }

    pub fn disassemble(&self)
    {
        if self.cpu.in_thumb_mode()
        {
            let address = self.cpu.r[15] - 2;
            let instruction = self.memory.load16(address);
            print!("{:08x}: ", address);
            print!("{:04x} ", instruction);
            println!("{}", cpu::thumb::disassemble::disassemble(instruction));
        }
        else
        {
            let address = self.cpu.r[15] - 4;
            let instruction = self.memory.load32(address);
            print!("{:08x}: ", address);
            print!("{:08x} ", instruction);
            println!("{}", cpu::arm::disassemble::disassemble(instruction));
        }
    }

    pub fn debug(&mut self) -> bool
    {
        self.print();
        
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

        self.step();

        true
    }
}