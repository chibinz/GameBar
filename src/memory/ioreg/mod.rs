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

    #[inline]
    pub fn ioram_load8(&self, address: u32) -> u8
    {
        let value = self.ioram_load16(address);
        if address & 1 == 1 {(value >> 8) as u8} else {value as u8}
    }

    pub fn ioram_load16(&self, address: u32) -> u16
    {
        let console = unsafe {& *self.console};
        let dma     = &console.dma;
        let timers  = &console.timers;
        let irqcnt  = &console.irqcnt;

        let ioreg = (address & 0xfffe) as usize;

        match ioreg
        {
            0x0b0 => dma.channel[0].get_src_l(),
            0x0b2 => dma.channel[0].get_src_h(),
            0x0b4 => dma.channel[0].get_dst_l(),
            0x0b6 => dma.channel[0].get_dst_h(),
            0x0b8 => dma.channel[0].get_count(),
            0x0ba => dma.channel[0].get_control(),

            0x0bc => dma.channel[1].get_src_l(),
            0x0be => dma.channel[1].get_src_h(),
            0x0c0 => dma.channel[1].get_dst_l(),
            0x0c2 => dma.channel[1].get_dst_h(),
            0x0c4 => dma.channel[1].get_count(),
            0x0c6 => dma.channel[1].get_control(),

            0x0c8 => dma.channel[2].get_src_l(),
            0x0ca => dma.channel[2].get_src_h(),
            0x0cc => dma.channel[2].get_dst_l(),
            0x0ce => dma.channel[2].get_dst_h(),
            0x0d0 => dma.channel[2].get_count(),
            0x0d2 => dma.channel[2].get_control(),

            0x0d4 => dma.channel[3].get_src_l(),
            0x0d6 => dma.channel[3].get_src_h(),
            0x0d8 => dma.channel[3].get_dst_l(),
            0x0da => dma.channel[3].get_dst_h(),
            0x0dc => dma.channel[3].get_count(),
            0x0de => dma.channel[3].get_control(),

            0x100 => timers.timer[0].get_counter(),
            0x104 => timers.timer[1].get_counter(),
            0x108 => timers.timer[2].get_counter(),
            0x10c => timers.timer[3].get_counter(),
            
            0x200 => irqcnt.get_ie(),
            0x202 => irqcnt.get_irf(),
            0x208 => irqcnt.get_ime(),
            _     => into16(&self.ioram[ioreg..ioreg+2]),
        }
    }

    #[inline]
    pub fn ioram_load32(&self, address: u32) -> u32
    {
        let lo = self.ioram_load16(address) as u32;
        let hi = self.ioram_load16(address + 2) as u32;
        (hi << 16) | lo
    }

    #[inline]
    pub fn ioram_store8(&mut self, address: u32, value: u8)
    {
        self.ioram_store16(address, value as u16);
    }

    pub fn ioram_store16(&mut self, address: u32, value: u16)
    {
        let console = unsafe {&mut *self.console};
        let cpu     = &mut console.cpu;
        let memory  = &mut console.memory;
        let dma     = &mut console.dma;
        let timers  = &mut console.timers;
        let irqcnt  = &mut console.irqcnt;

        assert_eq!(console.magic, 0xdeadbeef);

        let ioreg = (address & 0xfffe) as usize;

        // Seems like match pattern cannot be replaced with macros...
        match ioreg
        {
            // DMA 0 - 3
            0x0b0 => dma.channel[0].set_src_l(value),
            0x0b2 => dma.channel[0].set_src_h(value),
            0x0b4 => dma.channel[0].set_dst_l(value),
            0x0b6 => dma.channel[0].set_dst_h(value),
            0x0b8 => dma.channel[0].set_count(value),
            0x0ba => dma.channel[0].set_control(value, memory),

            0x0bc => dma.channel[1].set_src_l(value),
            0x0be => dma.channel[1].set_src_h(value),
            0x0c0 => dma.channel[1].set_dst_l(value),
            0x0c2 => dma.channel[1].set_dst_h(value),
            0x0c4 => dma.channel[1].set_count(value),
            0x0c6 => dma.channel[1].set_control(value, memory),

            0x0c8 => dma.channel[2].set_src_l(value),
            0x0ca => dma.channel[2].set_src_h(value),
            0x0cc => dma.channel[2].set_dst_l(value),
            0x0ce => dma.channel[2].set_dst_h(value),
            0x0d0 => dma.channel[2].set_count(value),
            0x0d2 => dma.channel[2].set_control(value, memory),

            0x0d4 => dma.channel[3].set_src_l(value),
            0x0d6 => dma.channel[3].set_src_h(value),
            0x0d8 => dma.channel[3].set_dst_l(value),
            0x0da => dma.channel[3].set_dst_h(value),
            0x0dc => dma.channel[3].set_count(value),
            0x0de => dma.channel[3].set_control(value, memory),

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
            0x200 => irqcnt.set_ie(value, cpu),
            0x202 => irqcnt.ack_irf(value),
            0x208 => irqcnt.set_ime(value, cpu),
            _     => Self::unhandled(false, 2, address),
        }

        let a = value.to_le_bytes();
        self.ioram[ioreg]     = a[0];
        self.ioram[ioreg + 1] = a[1];
    }

    #[inline]
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