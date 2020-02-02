use super::Memory;

impl Memory
{   
    /// Return a byte from vram, offset is in bytes
    #[inline]
    pub fn vram8(&self, offset: u32) -> u8
    {
        unsafe
        {
            let ptr = self.vram.as_ptr() as *const u8;

            *ptr.add(offset as usize)
        }
    }

    /// Return a halfword from vram, offset is in bytes
    #[inline]
    pub fn vram16(&self, offset: u32) -> u16
    {
        unsafe
        {
            let ptr = self.vram.as_ptr() as *const u16;

            *ptr.add((offset / 2) as usize)
        }
    }

    /// Return a word from vram, offset is in bytes
    #[inline]
    pub fn vram32(&self, offset: u32) -> u32
    {
        unsafe
        {
            let ptr = self.vram.as_ptr() as *const u32;

            *ptr.add((offset / 4) as usize)
        }
    }

    /// Return a row of 4-bit tile data as a word
    #[inline]
    pub fn tile_row32(&self, index: u32, tile_number: u32, row: u32) -> u32
    {
        self.vram32(index * 0x4000 + (tile_number * 8 + row) * 4)
    }

    /// Return tile map entry
    #[inline]
    pub fn tile_map(&self, index: u32, y: u32, x: u32) -> u16
    {
        self.vram16(index * 0x800 + (y * 32 + x) * 2)
    }
}