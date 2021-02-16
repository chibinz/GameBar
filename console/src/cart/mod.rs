//! Module for handling gamepak / cartridge functionalities
//! Backup / RTC, etc.

mod flash;
use util::Bus;
use Backup::*;

enum Backup {
    EEPROM,
    Flash(flash::Flash),
    SRAM(Vec<u8>),
}

pub struct Cart {
    rom: Vec<u8>,
    backup: Backup,
}

impl Bus for Cart {
    fn load8(&self, address: usize) -> u8 {
        match address >> 24 {
            0x08..=0x0d => self.rom.load8(address & 0xffffff),
            0xe => match &self.backup {
                Flash(f) => f.load8(address & 0xffff),
                _ => todo!(),
            }
            _ => unreachable!(),
        }
    }

    fn store8(&mut self, address: usize, value: u8) {
        match address >> 24 {
            0xe => match &mut self.backup {
                Flash(f) => f.store8(address & 0xffff, value),
                _ => todo!(),
            }
            _ => Self::unhandled(true, 1, address),
        }
    }
}

impl Cart {
    pub fn with_rom(rom: Vec<u8>) -> Self {

    }
}
