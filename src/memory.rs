use std::io::Read;
use std::fs::File;

pub struct Memory
{
    bios : Vec<u8>,
    ewram: Vec<u8>,
    iwram: Vec<u8>,
    ioram: Vec<u8>,
    param: Vec<u8>,
    vram : Vec<u8>,
    oam  : Vec<u8>,
    rom  : Vec<u8>, // rom1, rom2 are just image of rom, no need to declare extra fields
    sram : Vec<u8>,
}

impl Memory
{
    /// Initializes memory to zeroes
    pub fn new() -> Self
    {
        Memory
        {
            bios : vec![0; 0x00003fff - 0x00000000],
            ewram: vec![0; 0x0203ffff - 0x02000000],
            iwram: vec![0; 0x03007fff - 0x03000000],
            ioram: vec![0; 0x040003ff - 0x04000000],
            param: vec![0; 0x050003ff - 0x05000000],
            vram : vec![0; 0x06017fff - 0x06000000],
            oam  : vec![0; 0x070003ff - 0x07000000],
            rom  : vec![0; 0x09ffffff - 0x08000000], 
            sram : vec![0; 0x0e00ffff - 0x0e000000], 
        }
    }

    /// Load a byte from memory
    pub fn load8(&self, address: u32) -> u8
    {
        let mut offset = (address & 0x00ffffff) as usize;

        match address >> 24
        {
            0x00 => self.bios[offset],
            0x02 => self.ewram[offset],
            0x03 => self.iwram[offset],
            0x04 => self.ioram[offset],
            0x05 => self.param[offset],
            0x06 => self.vram[offset],
            0x07 => self.oam[offset],
            0x08...0x0d =>
            {
                offset = (address % 0x02000000) as usize;
                self.rom[offset]
            },
            0x0e => self.sram[offset],
            _    => panic!("invalid memory address {:08x}", address),
        }
    }
    
    /// Load a halfword from memory
    pub fn load16(&self, address: u32) -> u16
    {
        let offset = (address & 0x00ffffff) as usize;

        let ldh = |mem: &[u8]| mem[offset] as u16 | (mem[offset + 1] as u16) << 8;

        match address >> 24
        {
            0x00 => ldh(self.bios.as_slice()),
            0x02 => ldh(self.ewram.as_slice()),
            0x03 => ldh(self.iwram.as_slice()),
            0x04 => ldh(self.ioram.as_slice()),
            0x05 => ldh(self.param.as_slice()),
            0x06 => ldh(self.vram.as_slice()),
            0x07 => ldh(self.oam.as_slice()),
            0x08...0x0d => 
            {
                ldh(self.rom.as_slice())
            },
            0x0e => ldh(self.sram.as_slice()),
            _    => panic!("invalid memory address {:08x}", address),
        }
    }

    /// Load a word from memory
    pub fn load32(&self, address: u32) -> u32
    {
        let offset = (address & 0x00ffffff) as usize;

        let ld = |mem: &[u8]| mem[offset] as u32 | 
                               (mem[offset + 1] as u32) << 8 | 
                               (mem[offset + 2] as u32) << 16 | 
                               (mem[offset + 3] as u32) << 24;

        match address >> 24
        {
            0x00 => ld(self.bios.as_slice()),
            0x02 => ld(self.ewram.as_slice()),
            0x03 => ld(self.iwram.as_slice()),
            0x04 => ld(self.ioram.as_slice()),
            0x05 => ld(self.param.as_slice()),
            0x06 => ld(self.vram.as_slice()),
            0x07 => ld(self.oam.as_slice()),
            0x08...0x0d => 
            {
                ld(self.rom.as_slice())
            },
            0x0e => ld(self.sram.as_slice()),
            _    => panic!("invalid memory address {:08x}", address),
        }
    }

    /// Store a byte in memory, only EWRAM, IWRAM, IORAM, SRAM are accessible
    pub fn store8(&mut self, address: u32, data: u8)
    {
        let offset = (address & 0x00ffffff) as usize;

        match address >> 24
        {
            0x02 => self.ewram[offset] = data,
            0x03 => self.iwram[offset] = data,
            0x04 => self.ioram[offset] = data,
            0x0e => self.sram[offset]  = data,
            _    => panic!("invalid memory address {:08x}", address),
        };
    }

    /// Store an halfword in memory, BIOS, ROM, SRAM are inaccessible
    pub fn store16(&mut self, address: u32, data: u16)
    {
        let offset = (address & 0x00ffffff) as usize;

        let sth = |mem: &mut [u8]| 
        {
            mem[offset] = (data & 0b11111111) as u8;
            mem[offset + 1] = (data >> 8) as u8;
        };

        match address >> 24
        {
            0x02 => sth(self.ewram.as_mut_slice()),
            0x03 => sth(self.iwram.as_mut_slice()),
            0x04 => sth(self.ioram.as_mut_slice()),
            0x05 => sth(self.param.as_mut_slice()),
            0x06 => sth(self.vram.as_mut_slice()),
            0x07 => sth(self.oam.as_mut_slice()),
            _    => panic!("invalid memory address {:08x}", address),
        };
    }

    /// Store a word in memory, BIOS, ROM, SRAM are inaccessible
    pub fn store32(&mut self, address: u32, data: u32)
    {
        let offset = (address & 0x00ffffff) as usize;

        let sth = |mem: &mut [u8]| 
        {
            mem[offset] = (data & 0b11111111) as u8;
            mem[offset + 1] = (data >> 8 & 0b11111111) as u8;
            mem[offset + 2] = (data >> 16 & 0b11111111) as u8;
            mem[offset + 3] = (data >> 24) as u8;
        };

        match address >> 24
        {
            0x02 => sth(self.ewram.as_mut_slice()),
            0x03 => sth(self.iwram.as_mut_slice()),
            0x04 => sth(self.ioram.as_mut_slice()),
            0x05 => sth(self.param.as_mut_slice()),
            0x06 => sth(self.vram.as_mut_slice()),
            0x07 => sth(self.oam.as_mut_slice()),
            _    => panic!("invalid memory address {:08x}", address),
        };
    }

    /// Load rom from file, take name as a parameter
    pub fn load_rom(&mut self, name: &String)
    {
        self.rom.clear();

        let mut file = File::open(name).unwrap();
        file.read_to_end(&mut self.rom).unwrap();
        
        println!("{}", self.rom[0]);
    }

}