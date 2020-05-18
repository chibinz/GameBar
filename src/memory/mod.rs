mod ioreg;
mod vram;
mod palette;
mod oam;

use std::io::Read;
use std::fs::File;
use std::convert::TryInto;

use super::console::Console;

pub struct Memory
{
    bios : Vec<u8>,
    ewram: Vec<u8>,
    iwram: Vec<u8>,
    ioram: Vec<u8>,
    param: Vec<u8>,
    vram : Vec<u8>,
    oam  : Vec<u8>,
    rom  : Vec<u8>,
    sram : Vec<u8>,

    /// Pointer to containing console struct
    pub console: *mut Console,
}

impl Memory
{
    /// Initializes memory to zeroes
    pub fn new() -> Self
    {
        Memory
        {
            bios : vec![0; 0x00004000 - 0x00000000],
            ewram: vec![0; 0x02040000 - 0x02000000],
            iwram: vec![0; 0x03008000 - 0x03000000],
            ioram: vec![0; 0x04010000 - 0x04000000],
            param: vec![0; 0x05000400 - 0x05000000],
            vram : vec![0; 0x06018000 - 0x06000000],
            oam  : vec![0; 0x07000400 - 0x07000000],
            rom  : vec![0; 0x0a000000 - 0x08000000],
            sram : vec![0; 0x0e010000 - 0x0e000000],

            console: 0 as *mut Console,
        }
    }

    /// Load a byte from memory
    pub fn load8(&self, address: u32) -> u8
    {
        let offset = mirror(address) & 0x00ffffff;

        match address >> 24
        {
            0x00 => self.bios[offset],
            0x02 => self.ewram[offset],
            0x03 => self.iwram[offset],
            0x04 => self.ioram_load8(address),
            0x05 => self.param[offset],
            0x06 => self.vram[offset],
            0x07 => self.oam[offset],
            0x08..=
            0x0d => self.rom[offset],
            0x0e => self.sram[offset],
            _    => {Self::unhandled(true, 4, address); 0},
        }
    }

    /// Load a halfword from memory
    pub fn load16(&self, address: u32) -> u16
    {
        let offset = mirror(address) & 0x00fffffe;

        let ldh = |mem: &[u8]| into16(&mem[offset..offset+2]);

        match address >> 24
        {
            0x00 => ldh(&self.bios),
            0x02 => ldh(&self.ewram),
            0x03 => ldh(&self.iwram),
            0x04 => self.ioram_load16(address),
            0x05 => ldh(&self.param),
            0x06 => ldh(&self.vram),
            0x07 => ldh(&self.oam),
            0x08..=
            0x0d => ldh(&self.rom),
            0x0e => ldh(&self.sram),
            _    => {Self::unhandled(true, 2, address); 0},
        }
    }

    /// Load a word from memory
    pub fn load32(&self, address: u32) -> u32
    {
        let offset = mirror(address) & 0x00fffffc;

        let ld = |mem: &[u8]| into32(&mem[offset..offset+4]);

        let value = match address >> 24
        {
            0x00 => ld(&self.bios),
            0x02 => ld(&self.ewram),
            0x03 => ld(&self.iwram),
            0x04 => self.ioram_load32(address),
            0x05 => ld(&self.param),
            0x06 => ld(&self.vram),
            0x07 => ld(&self.oam),
            0x08..=
            0x0d => ld(&self.rom),
            0x0e => ld(&self.sram),
            _    => {Self::unhandled(true, 4, address); 0},
        };

        let shift = (address & 0b11) * 8;
        return value.rotate_right(shift);
    }

    /// Store a byte in memory, only EWRAM, IWRAM, IORAM, SRAM are accessible
    pub fn store8(&mut self, address: u32, value: u8)
    {
        let offset = mirror(address) & 0x00ffffff;

        match address >> 24
        {
            0x02 => self.ewram[offset] = value,
            0x03 => self.iwram[offset] = value,
            0x04 => self.ioram_store8(address, value),
            0x0e => self.sram[offset]  = value,
            _    => Self::unhandled(false, 1, address),
        };
    }

    /// Store an halfword in memory, BIOS, ROM, SRAM are inaccessible
    pub fn store16(&mut self, address: u32, value: u16)
    {
        // Accesses are forced to halfword aligned
        let offset = mirror(address) & 0x00fffffe;

        let sth = |mem: &mut [u8]|
        {
            let a = value.to_le_bytes();
            mem[offset]     = a[0];
            mem[offset + 1] = a[1];
        };

        match address >> 24
        {
            0x02 => sth(&mut self.ewram),
            0x03 => sth(&mut self.iwram),
            0x04 => self.ioram_store16(address, value),
            0x05 => sth(&mut self.param),
            0x06 => sth(&mut self.vram),
            0x07 => sth(&mut self.oam),
            _    => Self::unhandled(false, 2, address),
        };
    }

    /// Store a word in memory, BIOS, ROM, SRAM are inaccessible
    pub fn store32(&mut self, address: u32, value: u32)
    {
        // Accesses are forced to be word aligned
        let offset = mirror(address) & 0x00fffffc;

        let st = |mem: &mut [u8]|
        {
            let a = value.to_le_bytes();
            mem[offset]     = a[0];
            mem[offset + 1] = a[1];
            mem[offset + 2] = a[2];
            mem[offset + 3] = a[3];
        };

        match address >> 24
        {
            0x02 => st(&mut self.ewram),
            0x03 => st(&mut self.iwram),
            0x04 => self.ioram_store32(address, value),
            0x05 => st(&mut self.param),
            0x06 => st(&mut self.vram),
            0x07 => st(&mut self.oam),
            _    => Self::unhandled(false, 4, address),
        };
    }

    /// Load rom from file, take name as a parameter
    pub fn load_rom(&mut self, name: &String) -> usize
    {
        self.rom.clear();

        let mut file = File::open(name).unwrap();
        file.read_to_end(&mut self.rom).unwrap();

        self.rom.len()
    }

    /// Load bios and return length
    pub fn load_bios(&mut self, name: &String) -> usize
    {
        self.bios.clear();

        let mut file = File::open(name).unwrap();
        file.read_to_end(&mut self.bios).unwrap();

        self.bios.len()
    }

    /// Print invalid memory access
    #[allow(unused_variables)]
    fn unhandled(load: bool, size: u32, address: u32)
    {
        // let s = if load {"load"} else {"store"};

        // println!("Unhandled {}-byte {} at {:#08x}", size, s, address);
    }
}

/// Return equivalent base address
fn mirror(address: u32) -> usize
{
    let a = match address >> 24
    {
        0x02 => address % 0x40000,
        0x03 => address % 0x8000,
        0x04 => address % 0x10000,
        0x05 => address % 0x400,
        0x06 =>
            {
                let b = address % 0x20000;
                if b > 0x06017fff
                {
                    b - 0x8000
                }
                else
                {
                    b
                }
            },
        0x07 => address % 0x400,
        0x08..=
        0x0d => address % 0x02000000,
        _    => address,
    };

    a as usize
}

#[inline]
pub fn into16(a: &[u8]) -> u16
{
    u16::from_le_bytes(a[0..2].try_into().unwrap())
}

#[inline]
pub fn into32(a: &[u8]) -> u32
{
    u32::from_le_bytes(a[0..4].try_into().unwrap())
}