use super::Memory;

impl Memory {
    pub fn param_load8(&self, offset: usize) -> u8 {
        let value = self.param_load16(offset);
        value.to_le_bytes()[offset as usize & 1]
    }

    pub fn param_load16(&self, offset: usize) -> u16 {
        self.c().ppu.param_load16(offset)
    }

    pub fn param_load32(&self, offset: usize) -> u32 {
        let lo = self.param_load16(offset) as u32;
        let hi = self.param_load16(offset + 2) as u32;
        (hi << 16) | lo
    }

    pub fn param_store16(&self, offset: usize, value: u16) {
        self.c().ppu.param_store16(offset, value);
    }

    pub fn param_store32(&mut self, offset: usize, value: u32) {
        self.param_store16(offset, value as u16);
        self.param_store16(offset + 2, (value >> 16) as u16);
    }
}
