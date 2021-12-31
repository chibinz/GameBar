//! Module for handling gamepak / cartridge functionalities
//! Backup / RTC, etc.

mod flash;
use flash::Flash;
use util::Bus;
use Backup::*;

pub enum Backup {
    Flash(flash::Flash),
    Sram(Vec<u8>),
}

impl Bus for Backup {
    fn load8(&self, address: usize) -> u8 {
        match self {
            Flash(f) => f.load8(address & 0xffff),
            _ => todo!(),
        }
    }

    fn store8(&mut self, address: usize, value: u8) {
        match self {
            Flash(f) => f.store8(address & 0xffff, value),
            _ => todo!(),
        }
    }
}

pub struct Cart {
    pub rom: Vec<u8>,
    pub backup: Backup,
}

impl Cart {
    pub fn with_rom(rom: Vec<u8>) -> Self {
        Self {
            rom,
            backup: Flash(Flash::new()),
        }
    }
}
