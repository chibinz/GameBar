use std::collections::HashSet;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::process::exit;

use gba::Gba;
use minifb::Window;
use util::*;

static mut DEBUGGER: Option<Debugger> = None;

#[allow(dead_code)]
pub fn init_debugger(gba: *mut Gba) {
    unsafe {
        DEBUGGER = Some(Debugger::new(gba));
        (*gba).set_callback(debugger_callback);
        (*gba).cpu.set_callback(debugger_callback);
    }
}

fn debugger_callback() {
    unsafe {
        if let Some(ref mut debugger) = DEBUGGER {
            debugger.step();
        }
    }
}

pub struct Debugger {
    breakpoint: HashSet<u32>,
    command: Vec<String>,
    buffer: Vec<u32>,

    gba: *mut Gba,
}

impl Deref for Debugger {
    type Target = Gba;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.gba }
    }
}
impl DerefMut for Debugger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.gba }
    }
}

#[allow(dead_code)]
impl Debugger {
    pub fn new(gba: *mut Gba) -> Self {
        Self {
            breakpoint: HashSet::new(),
            command: vec![String::from("s")],
            buffer: vec![0; 256 * 256],

            gba
        }
    }

    pub fn step(&mut self) {
        if self.breakpoint_hit() {
            self.prompt();
        }
    }

    pub fn prompt(&mut self) {
        print!("(debug) ");
        std::io::stdout().flush().ok().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if !input.trim().is_empty() {
            self.command.clear();

            for str in input.split_whitespace() {
                self.command.push(str.to_string());
            }
        }
    }

    pub fn dispatch(&mut self) {
        match self.command[0].as_str() {
            "s" => self.step(),
            "c" => (),
            "b" => self.insert_breakpoint(),
            "d" => self.delete_breakpoint(),
            "l" => self.list_breakpoint(),
            "x" => self.examine_memory(),
            // "dp" => self.display_palette(),
            "q" => exit(0),
            _ => println!("Invalid input"),
        }
    }

    fn insert_breakpoint(&mut self) {
        if self.command.len() < 2 {
            println!("Please specify breakpoint")
        } else {
            match u32::from_str_radix(self.command[1].as_str(), 16) {
                Err(_) => println!("Invalid breakpoint"),
                Ok(e) => {
                    self.breakpoint.insert(e);
                }
            };
        }
    }

    fn delete_breakpoint(&mut self) {
        if self.command.len() < 2 {
            println!("Please specify breakpoint")
        } else {
            match u32::from_str_radix(self.command[1].as_str(), 16) {
                Err(_) => println!("Invalid breakpoint"),
                Ok(e) => {
                    self.breakpoint.remove(&e);
                }
            }
        }
    }

    fn list_breakpoint(&self) {
        for b in self.breakpoint.iter() {
            println!("{:#8x}", b);
        }
    }

    fn breakpoint_hit(&mut self) -> bool {
        self.breakpoint.contains(&(self.cpu.r(15)))
    }

    fn examine_memory(&mut self) {
        let address = usize::from_str_radix(self.command[1].as_str(), 16).unwrap();

        for i in 0..16 {
            print!("{:08x}:   ", address + i * 16);
            for j in 0..16 {
                let value = self.bus.load8(address + i * 16 + j);

                print!("{:02x} ", value);
            }
            println!();
        }
    }

    pub fn display_palette(&mut self, window: &mut Window) {
        for i in 0..0x100 {
            self.buffer[i] = self.ppu.bg_palette(0, i as u32).to_rgb24();
        }

        for i in 0..0x100 {
            self.buffer[i + 0x100] = self.ppu.obj_palette(0, i as u32).to_rgb24();
        }

        window.update_with_buffer(&self.buffer, 32, 16).unwrap();
    }

    pub fn display_tile(&mut self, index: usize, window: &mut Window) {
        let palette_num = 0;

        for p in 0..(8 * 8) {
            // 32 bytes per tile
            let index = index * 32 + p / 2 + 0x10000;
            let byte = self.ppu.vram[index];
            let nibble = if p & 1 == 1 { byte >> 4 } else { byte & 0x0f };

            let color = self.ppu.obj_palette(palette_num, nibble as u32);

            self.buffer[p] = color.to_rgb24();
        }
    }
}
