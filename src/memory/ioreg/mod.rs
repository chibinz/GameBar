pub mod ppu;
pub mod dma;
pub mod timer;
pub mod interrupt;

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

    pub fn ioram_load16(&self, address: u32) -> u16
    {
        let console = unsafe {& *self.console};
        let timers  = &console.timers;
        let irqcnt  = &console.irqcnt;

        let ioreg = (address & 0xfffe) as usize;

        match ioreg
        {
            0x100 => timers.timer[0].get_counter(),
            0x104 => timers.timer[1].get_counter(),
            0x108 => timers.timer[2].get_counter(),
            0x10c => timers.timer[3].get_counter(),
            
            0x200 => irqcnt.ie,
            0x202 => irqcnt.irf,
            0x208 => irqcnt.ime,
            _     => into16(&self.ioram[ioreg..ioreg+2]),
        }
    }

    pub fn ioram_store16(&mut self, address: u32, value: u16)
    {
        let console = unsafe {&mut *self.console};
        let cpu     = &mut console.cpu;
        let dma     = &mut console.dma;
        let timers  = &mut console.timers;
        let irqcnt  = &mut console.irqcnt;

        assert_eq!(console.magic, 0xdeadbeef);

        let ioreg = address & 0xffff;

        match ioreg
        {
            // DMA 0 - 3
            0x0b0 => self.update_dmasad(&mut dma.channel[0]),
            0x0b4 => self.update_dmadad(&mut dma.channel[0]),
            0x0b8 => self.update_dmacnt_l(&mut dma.channel[0]),
            0x0ba => self.update_dmacnt_h(&mut dma.channel[0]),
            0x0bc => self.update_dmasad(&mut dma.channel[1]),
            0x0c0 => self.update_dmadad(&mut dma.channel[1]),
            0x0c4 => self.update_dmacnt_l(&mut dma.channel[1]),
            0x0c6 => self.update_dmacnt_h(&mut dma.channel[1]),
            0x0c8 => self.update_dmasad(&mut dma.channel[2]),
            0x0cc => self.update_dmadad(&mut dma.channel[2]),
            0x0d0 => self.update_dmacnt_l(&mut dma.channel[2]),
            0x0d2 => self.update_dmacnt_h(&mut dma.channel[2]),
            0x0d4 => self.update_dmasad(&mut dma.channel[3]),
            0x0d8 => self.update_dmadad(&mut dma.channel[3]),
            0x0dc => self.update_dmacnt_l(&mut dma.channel[3]),
            0x0de => self.update_dmacnt_h(&mut dma.channel[3]),

            // Timer 0 - 3
            0x100 => timers.timer[0].set_reload(value),
            0x102 => timers.timer[0].set_control(value),
            0x104 => timers.timer[1].set_reload(value),
            0x106 => timers.timer[1].set_control(value),
            0x108 => timers.timer[2].set_reload(value),
            0x10a => timers.timer[2].set_control(value),
            0x10c => timers.timer[3].set_reload(value),
            0x10e => timers.timer[3].set_control(value),

            // Interrupt Controller
            0x200 => self.update_ie(irqcnt),
            0x202 => irqcnt.acknowledge(value),
            0x208 => self.update_ime(irqcnt),
            _     => println!("unhandled address {:x}", ioreg),
        }

        if ioreg >= 0x200 && ioreg <= 0x208 {println!("{:#x} {:#x} {:?}", ioreg, value, &irqcnt); irqcnt.check(cpu);}
    }

    pub fn ioram_store32(&mut self, address: u32, value: u32)
    {
        self.ioram_store16(address, value as u16);
        self.ioram_store16(address + 2, (value >> 16) as u16);
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