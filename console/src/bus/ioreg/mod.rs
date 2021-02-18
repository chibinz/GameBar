mod dma;
mod interrupt;
mod keypad;
mod timer;

use super::{Bus, GBus};

impl GBus {
    #[inline]
    pub fn ioram_load8(&self, offset: usize) -> u8 {
        let value = self.ioram_load16(offset);
        value.to_le_bytes()[offset as usize & 1]
    }

    pub fn ioram_load16(&self, offset: usize) -> u16 {
        match offset {
            0x000 => self.ppu.get_dispcnt(),
            0x004 => self.ppu.get_dispstat(),
            0x006 => self.ppu.get_vcount(),

            0x008 => self.ppu.background[0].get_control(),
            0x00a => self.ppu.background[1].get_control(),
            0x00c => self.ppu.background[2].get_control(),
            0x00e => self.ppu.background[3].get_control(),
            // Background offset & rotation registers are write only
            0x040 => self.ppu.window.get_win0h(),
            0x042 => self.ppu.window.get_win1h(),
            0x044 => self.ppu.window.get_win0v(),
            0x046 => self.ppu.window.get_win1v(),
            0x048 => self.ppu.window.get_winin(),
            0x04a => self.ppu.window.get_winout(),
            // Window boundary register are write only
            0x0b0 => self.dma.channel[0].get_src_l(),
            0x0b2 => self.dma.channel[0].get_src_h(),
            0x0b4 => self.dma.channel[0].get_dst_l(),
            0x0b6 => self.dma.channel[0].get_dst_h(),
            0x0b8 => self.dma.channel[0].get_count(),
            0x0ba => self.dma.channel[0].get_control(),

            0x0bc => self.dma.channel[1].get_src_l(),
            0x0be => self.dma.channel[1].get_src_h(),
            0x0c0 => self.dma.channel[1].get_dst_l(),
            0x0c2 => self.dma.channel[1].get_dst_h(),
            0x0c4 => self.dma.channel[1].get_count(),
            0x0c6 => self.dma.channel[1].get_control(),

            0x0c8 => self.dma.channel[2].get_src_l(),
            0x0ca => self.dma.channel[2].get_src_h(),
            0x0cc => self.dma.channel[2].get_dst_l(),
            0x0ce => self.dma.channel[2].get_dst_h(),
            0x0d0 => self.dma.channel[2].get_count(),
            0x0d2 => self.dma.channel[2].get_control(),

            0x0d4 => self.dma.channel[3].get_src_l(),
            0x0d6 => self.dma.channel[3].get_src_h(),
            0x0d8 => self.dma.channel[3].get_dst_l(),
            0x0da => self.dma.channel[3].get_dst_h(),
            0x0dc => self.dma.channel[3].get_count(),
            0x0de => self.dma.channel[3].get_control(),

            0x100 => self.timers.timer[0].get_counter(),
            0x102 => self.timers.timer[0].get_control(),
            0x104 => self.timers.timer[1].get_counter(),
            0x106 => self.timers.timer[1].get_control(),
            0x108 => self.timers.timer[2].get_counter(),
            0x10a => self.timers.timer[2].get_control(),
            0x10c => self.timers.timer[3].get_counter(),
            0x10e => self.timers.timer[3].get_control(),

            0x130 => self.keypad.get_input(),
            0x132 => self.keypad.get_control(),

            0x200 => self.irqcnt.get_ie(),
            0x202 => self.irqcnt.get_irf(),
            0x208 => self.irqcnt.get_ime(),
            _ => Self::unhandled(true, 2, (4 << 24) + offset),
        }
    }

    #[inline]
    pub fn ioram_load32(&self, offset: usize) -> u32 {
        let lo = self.ioram_load16(offset) as u32;
        let hi = self.ioram_load16(offset + 2) as u32;
        (hi << 16) | lo
    }

    #[inline]
    pub fn ioram_store8(&mut self, offset: usize, value: u8) {
        let mut old = self.ioram_load16(offset).to_le_bytes();
        old[offset as usize & 1] = value;
        let new = u16::from_le_bytes(old);

        // Beware of side effects
        self.ioram_store16(offset, new);
    }

    pub fn ioram_store16(&mut self, offset: usize, value: u16) {
        // Seems like match patterns cannot be replaced with macros...
        match offset {
            0x000 => self.ppu.set_dispcnt(value),
            0x004 => self.ppu.set_dispstat(value),
            // vcount is read only

            // Background 0 - 3
            0x008 => self.ppu.background[0].set_control(value),
            0x00a => self.ppu.background[1].set_control(value),
            0x00c => self.ppu.background[2].set_control(value),
            0x00e => self.ppu.background[3].set_control(value),
            0x010 => self.ppu.background[0].set_hofs(value),
            0x012 => self.ppu.background[0].set_vofs(value),
            0x014 => self.ppu.background[1].set_hofs(value),
            0x016 => self.ppu.background[1].set_vofs(value),
            0x018 => self.ppu.background[2].set_hofs(value),
            0x01a => self.ppu.background[2].set_vofs(value),
            0x01c => self.ppu.background[3].set_hofs(value),
            0x01e => self.ppu.background[3].set_vofs(value),

            0x020 => self.ppu.background[2].set_pa(value),
            0x022 => self.ppu.background[2].set_pb(value),
            0x024 => self.ppu.background[2].set_pc(value),
            0x026 => self.ppu.background[2].set_pd(value),
            0x028 => self.ppu.background[2].set_x_l(value),
            0x02a => self.ppu.background[2].set_x_h(value),
            0x02c => self.ppu.background[2].set_y_l(value),
            0x02e => self.ppu.background[2].set_y_h(value),

            0x030 => self.ppu.background[3].set_pa(value),
            0x032 => self.ppu.background[3].set_pb(value),
            0x034 => self.ppu.background[3].set_pc(value),
            0x036 => self.ppu.background[3].set_pd(value),
            0x038 => self.ppu.background[3].set_x_l(value),
            0x03a => self.ppu.background[3].set_x_h(value),
            0x03c => self.ppu.background[3].set_y_l(value),
            0x03e => self.ppu.background[3].set_y_h(value),

            0x040 => self.ppu.window.set_win0h(value),
            0x042 => self.ppu.window.set_win1h(value),
            0x044 => self.ppu.window.set_win0v(value),
            0x046 => self.ppu.window.set_win1v(value),
            0x048 => self.ppu.window.set_winin(value),
            0x04a => self.ppu.window.set_winout(value),

            // DMA 0 - 3
            0x0b0 => self.dma.channel[0].set_src_l(value),
            0x0b2 => self.dma.channel[0].set_src_h(value),
            0x0b4 => self.dma.channel[0].set_dst_l(value),
            0x0b6 => self.dma.channel[0].set_dst_h(value),
            0x0b8 => self.dma.channel[0].set_count(value),
            0x0ba => self.dma.channel[0].set_control(value),

            0x0bc => self.dma.channel[1].set_src_l(value),
            0x0be => self.dma.channel[1].set_src_h(value),
            0x0c0 => self.dma.channel[1].set_dst_l(value),
            0x0c2 => self.dma.channel[1].set_dst_h(value),
            0x0c4 => self.dma.channel[1].set_count(value),
            0x0c6 => self.dma.channel[1].set_control(value),

            0x0c8 => self.dma.channel[2].set_src_l(value),
            0x0ca => self.dma.channel[2].set_src_h(value),
            0x0cc => self.dma.channel[2].set_dst_l(value),
            0x0ce => self.dma.channel[2].set_dst_h(value),
            0x0d0 => self.dma.channel[2].set_count(value),
            0x0d2 => self.dma.channel[2].set_control(value),

            0x0d4 => self.dma.channel[3].set_src_l(value),
            0x0d6 => self.dma.channel[3].set_src_h(value),
            0x0d8 => self.dma.channel[3].set_dst_l(value),
            0x0da => self.dma.channel[3].set_dst_h(value),
            0x0dc => self.dma.channel[3].set_count(value),
            0x0de => self.dma.channel[3].set_control(value),

            // Timer 0 - 3
            0x100 => self.timers.timer[0].set_reload(value),
            0x102 => self.timers.timer[0].set_control(value),
            0x104 => self.timers.timer[1].set_reload(value),
            0x106 => self.timers.timer[1].set_control(value),
            0x108 => self.timers.timer[2].set_reload(value),
            0x10a => self.timers.timer[2].set_control(value),
            0x10c => self.timers.timer[3].set_reload(value),
            0x10e => self.timers.timer[3].set_control(value),

            // Keypad input is read only
            0x132 => self.keypad.set_control(value),

            // Interrupt Controller
            0x200 => self.irqcnt.set_ie(value),
            0x202 => self.irqcnt.ack_irf(value),
            0x208 => self.irqcnt.set_ime(value),
            _ => Self::unhandled(false, 2, (4 << 24) + offset),
        }
    }

    #[inline]
    pub fn ioram_store32(&mut self, offset: usize, value: u32) {
        self.ioram_store16(offset, value as u16);
        self.ioram_store16(offset + 2, (value >> 16) as u16);
    }
}
