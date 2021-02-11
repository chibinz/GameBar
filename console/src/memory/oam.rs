use super::Memory;

impl Memory {
    pub fn oam_load8(&self, offset: usize) -> u8 {
        let value = self.oam_load16(offset);
        value.to_le_bytes()[offset as usize & 1]
    }

    pub fn oam_load16(&self, offset: usize) -> u16 {
        self.c().ppu.oam_load16(offset)
    }

    pub fn oam_load32(&self, offset: usize) -> u32 {
        let lo = self.oam_load16(offset) as u32;
        let hi = self.oam_load16(offset + 2) as u32;
        (hi << 16) | lo
    }

    pub fn oam_store16(&self, offset: usize, value: u16) {
        self.c().ppu.oam_store16(offset, value);
    }

    pub fn oam_store32(&mut self, offset: usize, value: u32) {
        self.oam_store16(offset, value as u16);
        self.oam_store16(offset + 2, (value >> 16) as u16);
    }
}
