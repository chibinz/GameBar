use std::collections::{HashSet, VecDeque};
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::process::exit;

use gba::{Cpu, Gba};
use util::*;

use crate::Window;

static mut DEBUGGER: Option<Debugger> = None;

#[allow(dead_code)]
pub fn init_debugger(gba: *mut Gba) -> &'static mut Debugger {
    std::panic::set_hook(Box::new(panic_hook));
    unsafe {
        DEBUGGER = Some(Debugger::new(gba));
        (*gba).set_callback(debugger_callback);
        (*gba).cpu.set_callback(debugger_callback);
        return &mut *DEBUGGER.as_mut().unwrap();
    }
}
fn debugger_callback() {
    unsafe {
        if let Some(ref mut debugger) = DEBUGGER {
            debugger.step();
        }
    }
}
fn panic_hook(p: &std::panic::PanicInfo) {
    unsafe {
        if let Some(ref mut debugger) = DEBUGGER {
            for c in debugger.trace.iter() {
                util::error!("{:?}", c);
            }
            util::error!("{:#?}", p);
            util::error!("\n{:?}", backtrace::Backtrace::new());
        }
    }
}

pub struct Debugger {
    breakpoint: HashSet<u32>,
    command: Vec<String>,
    trace: VecDeque<Cpu>,
    window: Window,

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
            trace: VecDeque::new(),
            window: Window::new("Debugger", 64, 64, 4),

            gba,
        }
    }

    pub fn step(&mut self) {
        self.save_trace();
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

    fn save_trace(&mut self) {
        if self.trace.len() == 4096 {
            self.trace.pop_front();
        }
        self.trace.push_back(self.cpu.clone());
    }

    pub fn display_palette(&mut self) {
        let Self { window, gba, .. } = self;
        let palette = unsafe { &(**gba).ppu.palette };
        window.resize(16, 32, 16);
        window.update_with_buffer(palette);
    }

    pub fn display_background(&mut self, index: usize) {
        let (width, height) = self.ppu.get_background_dimension(index);
        let bg_buffer = if self.ppu.is_background_affine(index) {
            self.ppu.decode_affine_background(index)
        } else {
            self.ppu.decode_text_background(index)
        };

        if self.ppu.dispcnt.bit(8 + index as u32) {
            self.window.resize(width as usize, height as usize, 1);
            self.window.update_with_buffer(&bg_buffer);
        }
    }

    pub fn display_sprite(&mut self, index: usize) {
        let object = self.ppu.decode_sprite(index);
        let (width, height) = self.ppu.oam.sprite[index].get_dimension();
        dbg!(&self.ppu.oam.sprite[index]);

        self.window.resize(width as usize, height as usize, 4);
        self.window.update_with_buffer(&object);
    }

    pub fn display_vram(&mut self, base: usize) {
        self.window.resize(256, 256, 2);
        for i in 0..0x4000 {
            let v = self.ppu.vram[base * 0x4000 + i] as u32;
            self.window.buffer[i] = v << 16 | v << 8 | v;
        }
        self.window.update();
    }
}
