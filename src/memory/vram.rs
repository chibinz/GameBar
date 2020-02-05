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

    /// Return a word from vram, offset is in bytes
    #[inline]
    pub fn vram64(&self, offset: u32) -> u64
    {
        unsafe
        {
            let ptr = self.vram.as_ptr() as *const u64;

            *ptr.add((offset / 8) as usize)
        }
    }

    /// Return a row of 4-bit tile data as a word
    #[inline]
    pub fn tile_row32(&self, index: u32, tile_number: u32, row: u32) -> u32
    {
        self.vram32(index * 0x4000 + (tile_number * 8 + row) * 4)
    }

    /// Return a row of 4-bit tile data as a word
    #[inline]
    pub fn tile_row64(&self, index: u32, tile_number: u32, row: u32) -> u64
    {
        self.vram64(index * 0x4000 + (tile_number * 8 + row) * 8)
    }

    /// Return tile map entry
    #[inline]
    pub fn tile_map(&self, index: u32, size_r: u32, tile_y: u32, tile_x: u32) -> u16
    {
        let offset = map_entry(size_r, tile_y, tile_x) * 2;

        self.vram16(index * 0x800 + offset)
    }
}

// Referenced from TONC GBA
#[inline]
pub fn map_entry(size_r: u32, tile_y: u32, tile_x: u32) -> u32
{
    let mut n = tile_y * 32 + tile_x;

    if tile_x >= 32
    {
        n += 0x03e0;
    }

    if tile_y >= 32 && size_r == 0b11 // 64x64
    {
        n += 0x0400;
    }
    
    n
}