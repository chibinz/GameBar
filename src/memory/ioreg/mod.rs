pub mod ppu;
pub mod dma;
pub mod timer;

use super::Memory;
use super::into16;
use super::into32;

impl Memory
{
    /// Return a halfword from ioram, offset is in bytes
    #[inline]
    pub fn ioram16(&self, offset: usize) -> u16
    {
        let a = offset as usize;
        into16(&self.ioram[a..a+2])
    }

    /// Return a word from vram, offset is in bytes
    #[inline]
    pub fn ioram32(&self, offset: usize) -> u32
    {
        let a = offset as usize;
        into32(&self.ioram[a..a+4])
    }

    /// Store a halfword in vram, offset is in bytes
    #[inline]
    pub fn ioram16_s(&mut self, offset: usize, value: u16)
    {
        let a = value.to_le_bytes();
        self.ioram[offset]     = a[0];
        self.ioram[offset + 1] = a[1];
    }

    pub fn ioram_store16(&mut self, address: u32)
    {
        println!("{:x}", address & 0xffff);

        let console = unsafe {&mut *self.console};
        let dma = &mut console.dma;

        match address & 0xffff
        {
            // DMA 0
            0x0b0 => self.update_dmasad(&mut dma.channel[0]),
            0x0b4 => self.update_dmadad(&mut dma.channel[0]),
            0x0b8 => self.update_dmacnt_l(&mut dma.channel[0]),
            0x0ba => self.update_dmacnt_h(&mut dma.channel[0]),
            
            // DMA 1
            0x0bc => self.update_dmasad(&mut dma.channel[1]),
            0x0c0 => self.update_dmadad(&mut dma.channel[1]),
            0x0c4 => self.update_dmacnt_l(&mut dma.channel[1]),
            0x0c6 => self.update_dmacnt_h(&mut dma.channel[1]),

            // DMA 2
            0x0c8 => self.update_dmasad(&mut dma.channel[2]),
            0x0cc => self.update_dmadad(&mut dma.channel[2]),
            0x0d0 => self.update_dmacnt_l(&mut dma.channel[2]),
            0x0d2 => self.update_dmacnt_h(&mut dma.channel[2]),

            // DMA 3
            0x0d4 => self.update_dmasad(&mut dma.channel[3]),
            0x0d8 => self.update_dmadad(&mut dma.channel[3]),
            0x0dc => self.update_dmacnt_l(&mut dma.channel[3]),
            0x0de => self.update_dmacnt_h(&mut dma.channel[3]),

            _ => (),
        }
    }

    pub fn ioram_store32(&mut self, address: u32)
    {
        self.ioram_store16(address);
        self.ioram_store16(address + 2);
    }

    pub fn get_keyinput(&self) -> u16
    {
        self.ioram16(0x130)
    }

    pub fn set_keyinput(&mut self, input: u16)
    {
        self.ioram16_s(0x130, input);
    }
}