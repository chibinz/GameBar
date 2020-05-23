use std::io::prelude::*;
use std::process::exit;
use std::collections::HashSet;

use minifb::Window;
use minifb::WindowOptions;
use crate::util::*;
use crate::console::Console;

static WIDTH: usize = 8;
static HEIGHT: usize = 8;

#[allow(dead_code)]
pub struct Debugger
{
    breakpoint: HashSet<u32>,
    command   : Vec<String>,
    buffer    : Vec<u32>,
    window    : Window,

    pub counter   : i32,
    pub console   : *mut Console,
}

#[allow(dead_code)]
impl Debugger
{
    pub fn new() -> Self
    {
        Self
        {
            breakpoint: HashSet::new(),
            command   : vec![String::from("s")],
            buffer    : vec![0; WIDTH * HEIGHT],
            window    : Window::new
            (
                "Close to continue",
                WIDTH,
                HEIGHT,
                WindowOptions
                {
                    scale: minifb::Scale::X32,
                    ..WindowOptions::default()
                },
            ).unwrap(),

            counter   : 0,
            console   : 0 as *mut Console,
        }
    }

    #[inline]
    pub fn c(&self) -> &mut Console
    {
        let c = unsafe {&mut *self.console};
        assert_eq!(c.magic, 0xdeadbeef);

        c
    }

    pub fn run(&mut self)
    {
        loop
        {
            self.step();
        }
    }

    pub fn step(&mut self)
    {
        // self.prompt();
        // self.dispatch();
        if self.counter == 60
        {
            self.display_tile(0x331);
            self.counter = 0;
        }

        self.counter += 1;
    }

    pub fn prompt(&mut self)
    {
        print!("(debug) ");
        std::io::stdout().flush().ok().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if !input.trim().is_empty()
        {
            self.command.clear();

            for str in input.split_whitespace()
            {
                self.command.push(str.to_string());
            }
        }
    }

    pub fn dispatch(&mut self)
    {
        match self.command[0].as_str()
        {
            "s" => self.c().step(),
            // "p" => self.c().print(),
            "c" => self.continue_run(),
            "b" => self.insert_breakpoint(),
            "d" => self.delete_breakpoint(),
            "l" => self.list_breakpoint(),
            "x" => self.examine_memory(),
            "dp" => self.display_palette(),
            "q" => exit(0),
            _   => println!("Invalid input"),
        }
    }

    fn continue_run(&mut self)
    {
        while !self.breakpoint_hit()
        {
            self.c().step()
        }
    }

    fn insert_breakpoint(&mut self)
    {
        if self.command.len() < 2
        {
            println!("Please specify breakpoint")
        }
        else
        {
            match u32::from_str_radix(self.command[1].as_str(), 16)
            {
                Err(_) => println!("Invalid breakpoint"),
                Ok(e)  => {self.breakpoint.insert(e);},
            };
        }
    }

    fn delete_breakpoint(&mut self)
    {
        if self.command.len() < 2
        {
            println!("Please specify breakpoint")
        }
        else
        {
            match u32::from_str_radix(self.command[1].as_str(), 16)
            {
                Err(_) => println!("Invalid breakpoint"),
                Ok(e)  => {self.breakpoint.remove(&e);},
            }
        }
    }

    fn list_breakpoint(&self)
    {
        for b in self.breakpoint.iter()
        {
            println!("{:#8x}", b);
        }
    }

    fn breakpoint_hit(&mut self) -> bool
    {
        self.breakpoint.contains(&(self.c().cpu.r[15]))
    }

    fn examine_memory(&mut self)
    {
        let address = u32::from_str_radix(self.command[1].as_str(), 16).unwrap();

        for i in 0..16
        {
            print!("{:08x}:   ", address + i * 16);
            for j in 0..16
            {
                let value = self.c().memory.load8(address + i * 16 + j);

                print!("{:02x} ", value);
            }
            println!("");
        }
    }

    pub fn display_palette(&mut self)
    {
        for i in 0..0x100
        {
            self.buffer[i] = self.c().memory.bg_palette(0, i as u32).to_rgb24();
        }

        for i in 0..0x100
        {
            self.buffer[i + 0x100] = self.c().memory.obj_palette(0, i as u32).to_rgb24();
        }

        while self.window.is_open()
        {
            self.window.update_with_buffer(&self.buffer, 32, 16).unwrap();
        }
    }

    pub fn display_tile(&mut self, index: usize)
    {
        let palette_num = 0;

        for p in 0..(8*8)
        {
            // 32 bytes per tile
            let index = index * 32 + p / 2 + 0x10000;
            let byte = self.c().memory.vram[index];
            let nibble = if p & 1 == 1 {byte >> 4} else {byte & 0x0f};

            let color = self.c().memory.obj_palette(palette_num, nibble as u32);

            self.buffer[p] = color.to_rgb24();
        }

        self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();
    }
}