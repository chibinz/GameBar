mod ppu;
mod dma;
mod timer;
mod interrupt;
mod keypad;

use super::Memory;

impl Memory
{
    #[inline]
    pub fn ioram_load8(&self, address: u32) -> u8
    {
        let value = self.ioram_load16(address);
        if address & 1 == 1 {(value >> 8) as u8} else {value as u8}
    }

    pub fn ioram_load16(&self, address: u32) -> u16
    {
        let console = unsafe {& *self.console};
        let ppu     = &console.ppu;
        let dma     = &console.dma;
        let timers  = &console.timers;
        let irqcnt  = &console.irqcnt;
        let keypad  = &console.keypad;

        let ioreg = (address & 0xfffe) as usize;

        match ioreg
        {
            0x000 => ppu.get_dispcnt(),
            0x004 => ppu.get_dispstat(),
            0x006 => ppu.get_vcount(),

            0x008 => ppu.background[0].get_control(),
            0x00a => ppu.background[1].get_control(),
            0x00c => ppu.background[2].get_control(),
            0x00e => ppu.background[3].get_control(),
            // Background offset & rotation registers are write only

            0x040 => ppu.window.get_win0h(),
            0x042 => ppu.window.get_win1h(),
            0x044 => ppu.window.get_win0v(),
            0x046 => ppu.window.get_win1v(),
            0x048 => ppu.window.get_winin(),
            0x04a => ppu.window.get_winout(),
            // Window boundary register are write only

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

            0x130 => keypad.get_input(),
            0x132 => keypad.get_control(),

            0x200 => irqcnt.get_ie(),
            0x202 => irqcnt.get_irf(),
            0x208 => irqcnt.get_ime(),
            _     => {Self::unhandled(true, 2, address); 0},
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
        let ppu     = &mut console.ppu;
        let dma     = &mut console.dma;
        let timers  = &mut console.timers;
        let irqcnt  = &mut console.irqcnt;
        let keypad  = &mut console.keypad;

        assert_eq!(console.magic, 0xdeadbeef);

        let ioreg = (address & 0xfffe) as usize;

        // Seems like match pattern cannot be replaced with macros...
        match ioreg
        {
            0x000 => ppu.set_dispcnt(value),
            0x004 => ppu.set_dispstat(value),
            // vcount is read only

            // Background 0 - 3
            0x008 => ppu.background[0].set_control(value),
            0x00a => ppu.background[1].set_control(value),
            0x00c => ppu.background[2].set_control(value),
            0x00e => ppu.background[3].set_control(value),
            0x010 => ppu.background[0].set_hofs(value),
            0x012 => ppu.background[0].set_vofs(value),
            0x014 => ppu.background[1].set_hofs(value),
            0x016 => ppu.background[1].set_vofs(value),
            0x018 => ppu.background[2].set_hofs(value),
            0x01a => ppu.background[2].set_vofs(value),
            0x01c => ppu.background[3].set_hofs(value),
            0x01e => ppu.background[3].set_vofs(value),

            0x020 => ppu.background[2].set_pa(value),
            0x022 => ppu.background[2].set_pb(value),
            0x024 => ppu.background[2].set_pc(value),
            0x026 => ppu.background[2].set_pd(value),
            0x028 => ppu.background[2].set_x_l(value),
            0x02a => ppu.background[2].set_x_h(value),
            0x02c => ppu.background[2].set_y_l(value),
            0x02e => ppu.background[2].set_y_h(value),

            0x030 => ppu.background[3].set_pa(value),
            0x032 => ppu.background[3].set_pb(value),
            0x034 => ppu.background[3].set_pc(value),
            0x036 => ppu.background[3].set_pd(value),
            0x038 => ppu.background[3].set_x_l(value),
            0x03a => ppu.background[3].set_x_h(value),
            0x03c => ppu.background[3].set_y_l(value),
            0x03e => ppu.background[3].set_y_h(value),

            0x040 => ppu.window.set_win0h(value),
            0x042 => ppu.window.set_win1h(value),
            0x044 => ppu.window.set_win0v(value),
            0x046 => ppu.window.set_win1v(value),
            0x048 => ppu.window.set_winin(value),
            0x04a => ppu.window.set_winout(value),

            // DMA 0 - 3
            0x0b0 => dma.channel[0].set_src_l(value),
            0x0b2 => dma.channel[0].set_src_h(value),
            0x0b4 => dma.channel[0].set_dst_l(value),
            0x0b6 => dma.channel[0].set_dst_h(value),
            0x0b8 => dma.channel[0].set_count(value),
            0x0ba => dma.channel[0].set_control(value),

            0x0bc => dma.channel[1].set_src_l(value),
            0x0be => dma.channel[1].set_src_h(value),
            0x0c0 => dma.channel[1].set_dst_l(value),
            0x0c2 => dma.channel[1].set_dst_h(value),
            0x0c4 => dma.channel[1].set_count(value),
            0x0c6 => dma.channel[1].set_control(value),

            0x0c8 => dma.channel[2].set_src_l(value),
            0x0ca => dma.channel[2].set_src_h(value),
            0x0cc => dma.channel[2].set_dst_l(value),
            0x0ce => dma.channel[2].set_dst_h(value),
            0x0d0 => dma.channel[2].set_count(value),
            0x0d2 => dma.channel[2].set_control(value),

            0x0d4 => dma.channel[3].set_src_l(value),
            0x0d6 => dma.channel[3].set_src_h(value),
            0x0d8 => dma.channel[3].set_dst_l(value),
            0x0da => dma.channel[3].set_dst_h(value),
            0x0dc => dma.channel[3].set_count(value),
            0x0de => dma.channel[3].set_control(value),

            // Timer 0 - 3
            0x100 => timers.timer[0].set_reload(value),
            0x102 => timers.timer[0].set_control(value),
            0x104 => timers.timer[1].set_reload(value),
            0x106 => timers.timer[1].set_control(value),
            0x108 => timers.timer[2].set_reload(value),
            0x10a => timers.timer[2].set_control(value),
            0x10c => timers.timer[3].set_reload(value),
            0x10e => timers.timer[3].set_control(value),

            // Keypad input is read only
            0x132 => keypad.set_control(value),

            // Interrupt Controller
            0x200 => irqcnt.set_ie(value),
            0x202 => irqcnt.ack_irf(value),
            0x208 => irqcnt.set_ime(value),
            _     => Self::unhandled(false, 2, address),
        }
    }

    #[inline]
    pub fn ioram_store32(&mut self, address: u32, value: u32)
    {
        self.ioram_store16(address, value as u16);
        self.ioram_store16(address + 2, (value >> 16) as u16);
    }
}