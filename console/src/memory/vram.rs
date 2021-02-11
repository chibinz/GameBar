use super::Memory;

impl Memory {
    pub fn vram_load8(&self, offset: usize) -> u8 {
        self.c().ppu.vram_load8(offset)
    }

    pub fn vram_load16(&self, offset: usize) -> u16 {
        let lo = self.vram_load8(offset) as u16;
        let hi = self.vram_load8(offset + 1) as u16;
        (hi << 8) | lo
    }

    pub fn vram_load32(&self, offset: usize) -> u32 {
        let lo = self.vram_load16(offset) as u32;
        let hi = self.vram_load16(offset + 2) as u32;
        (hi << 16) | lo
    }

    pub fn vram_store8(&mut self, offset: usize, value: u8) {
        self.c().ppu.vram_store8(offset, value);
    }

    pub fn vram_store16(&mut self, offset: usize, value: u16) {
        let halfword = value.to_le_bytes();
        self.vram_store8(offset, halfword[0]);
        self.vram_store8(offset + 1, halfword[1]);
    }

    pub fn vram_store32(&mut self, offset: usize, value: u32) {
        let word = value.to_le_bytes();
        self.vram_store8(offset, word[0]);
        self.vram_store8(offset + 1, word[1]);
        self.vram_store8(offset + 2, word[2]);
        self.vram_store8(offset + 3, word[3]);
    }
}
