mod ioreg;
mod timing;

use util::Bus;

use crate::Console;

pub struct Memory {
    pub bios: Vec<u8>,
    ewram: [u8; 0x02040000 - 0x02000000],
    iwram: [u8; 0x03008000 - 0x03000000],
    /// Pointer to containing console struct
    pub console: *mut Console,
}

impl Bus for Memory {
    /// Load a byte from memory
    fn load8(&self, address: usize) -> u8 {
        let offset = Self::mirror(address);

        match Self::region(address) {
            0x00 => self.bios.load8(offset),
            0x02 => self.ewram.load8(offset),
            0x03 => self.iwram.load8(offset),
            0x04 => self.ioram_load8(offset),
            0x06 => self.c().ppu.vram.load8(offset),
            0x08..=0x0d => self.c().cart.rom.load8(offset),
            0x0e => self.c().cart.backup.load8(offset),
            _ => Self::unhandled(true, 4, address),
        }
    }

    /// Load a halfword from memory
    fn load16(&self, address: usize) -> u16 {
        let offset = Self::mirror(address) & !0b1;

        match Self::region(address) {
            0x00 => self.bios.load16(offset),
            0x02 => self.ewram.load16(offset),
            0x03 => self.iwram.load16(offset),
            0x04 => self.ioram_load16(offset),
            0x05 => self.c().ppu.palette.load16(offset),
            0x06 => self.c().ppu.vram.load16(offset),
            0x07 => self.c().ppu.oam.load16(offset),
            0x08..=0x0d => self.c().cart.rom.load16(offset),
            0x0e => self.c().cart.backup.load16(offset),
            _ => Self::unhandled(true, 2, address),
        }
    }

    /// Load a word from memory
    fn load32(&self, address: usize) -> u32 {
        let offset = Self::mirror(address) & !0b11;

        match Self::region(address) {
            0x00 => self.bios.load32(offset),
            0x02 => self.ewram.load32(offset),
            0x03 => self.iwram.load32(offset),
            0x04 => self.ioram_load32(offset),
            0x05 => self.c().ppu.palette.load32(offset),
            0x06 => self.c().ppu.vram.load32(offset),
            0x07 => self.c().ppu.oam.load32(offset),
            0x08..=0x0d => self.c().cart.rom.load32(offset),
            0x0e => self.c().cart.backup.load32(offset),
            _ => Self::unhandled(true, 4, address),
        }
    }

    /// Store a byte in memory, only EWRAM, IWRAM, IORAM, SRAM are accessible
    fn store8(&mut self, address: usize, value: u8) {
        let offset = Self::mirror(address);

        match Self::region(address) {
            0x02 => self.ewram.store8(offset, value),
            0x03 => self.iwram.store8(offset, value),
            0x04 => self.ioram_store8(offset, value),
            0x0e => self.c().cart.backup.store8(offset, value),
            _ => Self::unhandled(false, 1, address),
        };
    }

    /// Store an halfword in memory, BIOS, ROM, SRAM are inaccessible
    fn store16(&mut self, address: usize, value: u16) {
        // Accesses are forced to halfword aligned
        let offset = Self::mirror(address) & !0b1;

        match Self::region(address) {
            0x02 => self.ewram.store16(offset, value),
            0x03 => self.iwram.store16(offset, value),
            0x04 => self.ioram_store16(offset, value),
            0x05 => self.c().ppu.palette.store16(offset, value),
            0x06 => self.c().ppu.vram.store16(offset, value),
            0x07 => self.c().ppu.oam.store16(offset, value),
            _ => Self::unhandled(false, 2, address),
        };
    }

    /// Store a word in memory, BIOS, ROM, SRAM are inaccessible
    fn store32(&mut self, address: usize, value: u32) {
        // Accesses are forced to be word aligned
        let offset = Self::mirror(address) & !0b11;

        match Self::region(address) {
            0x02 => self.ewram.store32(offset, value),
            0x03 => self.iwram.store32(offset, value),
            0x04 => self.ioram_store32(offset, value),
            0x05 => self.c().ppu.palette.store32(offset, value),
            0x06 => self.c().ppu.vram.store32(offset, value),
            0x07 => self.c().ppu.oam.store32(offset, value),
            _ => Self::unhandled(false, 4, address),
        };
    }
}

impl Memory {
    /// Initializes memory to zeroes
    pub fn new() -> Self {
        Memory {
            bios: Vec::new(),
            ewram: [0; 0x02040000 - 0x02000000],
            iwram: [0; 0x03008000 - 0x03000000],
            // param:      0x05000400 - 0x05000000
            // vram :      0x06018000 - 0x06000000
            // oam  :      0x07000400 - 0x07000000
            console: 0 as *mut Console,
        }
    }

    /// Return reference to containing console
    pub fn c(&self) -> &mut Console {
        unsafe { &mut *self.console }
    }

    #[inline]
    fn region(address: usize) -> usize {
        // Top nibble of address is ignored
        address >> 24
    }

    /// Return equivalent base address
    fn mirror(address: usize) -> usize {
        let a = match address >> 24 {
            0x02 => address % 0x40000,
            0x03 => address % 0x8000,
            0x04 => address % 0x10000,
            0x05 => address % 0x400,
            0x06 => {
                // vram is mirrored every 0x20000 and
                // 0x6010000 - 0x6017fff is in turn mirrored from
                // 0x6018000 - 0x601ffff
                let b = address % 0x20000;
                if b > 0x17fff {
                    b - 0x8000
                } else {
                    b
                }
            }
            0x07 => address % 0x400,
            0x08..=0x0d => address % 0x01000000, // Should be length of rom instead
            0x0e => address % 0x10000,
            _ => address,
        };

        a as usize
    }
}
