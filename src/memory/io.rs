use crate::util::*;
use crate::ppu::PPU;
use crate::ppu::background::Background;

use super::Memory;

static BG_DIMENSION: [[(u32, u32); 4]; 2] =
[
    // Text
    [
        (32, 32),
        (64, 32),
        (32, 64),
        (64, 64),
    ],
    // Affine
    [
        ( 16,  16),
        ( 32,  32),
        ( 64,  64),
        (128, 128),
    ],
];

impl Memory
{
    /// Return a halfword from ioram, offset is in bytes
    #[inline]
    pub fn ioram16(&self, index: usize) -> u16
    {
        unsafe
        {
            let ptr = self.ioram.as_ptr() as *const u16;

            *ptr.add(index / 2)
        }
    }

    /// Return a word from vram, offset is in bytes
    #[inline]
    pub fn ioram32(&self, index: usize) -> u32
    {
        unsafe
        {
            let ptr = self.ioram.as_ptr() as *const u32;

            *ptr.add(index / 4)
        }
    }

    /// Store a halfword in vram, offset is in bytes
    #[inline]
    pub fn ioram16_s(&mut self, index: usize, value: u16)
    {
        unsafe
        {
            let ptr = self.ioram.as_mut_ptr() as *mut u16;

            *ptr.add(index / 2) = value;
        }
    }

    /// Update fields in struct `PPU`
    pub fn update_ppu(&self, ppu: &mut PPU)
    {
        self.update_dispcnt(ppu);
        ppu.vcount = self.get_vcount() as u32;
    }

    /// Update fields in struct `Background` (h/vscroll)
    pub fn update_text_bg(&self, bg: &mut Background)
    {
        self.update_bgcnt(bg);
        self.update_bgofs(bg);
        bg.vcount = self.get_vcount() as u32;
    }

    /// Update fields in struct `Background` (pa/b/c/d, x/y)
    pub fn update_affine_bg(&self, bg: &mut Background)
    {
        self.update_bgcnt(bg);
        self.update_bgpxy(bg);
        bg.vcount = self.get_vcount() as u32;
    }
    
    pub fn update_dispcnt(&self, ppu: &mut PPU)
    {
        let dispcnt = self.ioram16(0x00);

        ppu.dispcnt    = dispcnt;
        ppu.mode       = dispcnt.bits(2, 0);
        ppu.flip       = dispcnt.bit(4);
        ppu.sequential = dispcnt.bit(6);
        ppu.fblank     = dispcnt.bit(7);
    }

    pub fn set_vblank_flag(&mut self, value: bool)
    {
        let mut dispstat = self.ioram16(0x04);

        if value { dispstat |= 0b01 } else { dispstat &= !0b01 }

        self.ioram16_s(0x04, dispstat);
    }

    pub fn set_hblank_flag(&mut self, value: bool)
    {
        let mut dispstat = self.ioram16(0x04);

        if value { dispstat |= 0b10 } else { dispstat &= !0b10 }

        self.ioram16_s(0x04, dispstat);
    }

    pub fn get_vcount(&self) -> u16
    {
        self.ioram16(0x06)
    }

    pub fn inc_vcount(&mut self)
    {
        let vcount = self.get_vcount();
        self.ioram16_s(0x06, (vcount + 1) as u16);
    }

    pub fn clr_vcount(&mut self)
    {
        self.ioram16_s(0x06, 0);
    }

    pub fn update_vcount(&self, ppu: &mut PPU)
    {
        ppu.vcount = self.ioram16(0x06) as u32;
    }

    pub fn get_bgcnt(&self, index: usize) -> u16
    {
        self.ioram16(0x08 + index * 2)
    }

    pub fn update_bgcnt(&self, bg: &mut Background)
    {
        let bgcnt = self.ioram16(0x08 + bg.index * 2);

        bg.bgcnt     = bgcnt;
        bg.priority  = bgcnt.bits(1, 0);
        bg.tile_n    = bgcnt.bits(3, 2);
        bg.mosaic_f  = bgcnt.bit(6);
        bg.palette_f = bgcnt.bit(7);
        bg.map_n     = bgcnt.bits(12, 8);
        bg.repeat_f  = bgcnt.bit(13);
        bg.size_r    = bgcnt.bits(15, 14);

        bg.width  = BG_DIMENSION[bg.affine_f as usize][bg.size_r as usize].0;
        bg.height = BG_DIMENSION[bg.affine_f as usize][bg.size_r as usize].1;
    }

    pub fn update_bgofs(&self, bg: &mut Background)
    {
        let hofs = self.ioram16(0x10 + bg.index * 4) as u32;
        let vofs = self.ioram16(0x12 + bg.index * 4) as u32;

        bg.hscroll = hofs;
        bg.vscroll = vofs;
    }

    pub fn update_bgpxy(&self, bg: &mut Background)
    {
        let pa = self.ioram16(0x20 + (bg.index - 2) * 16);
        let pb = self.ioram16(0x22 + (bg.index - 2) * 16);
        let pc = self.ioram16(0x24 + (bg.index - 2) * 16);
        let pd = self.ioram16(0x26 + (bg.index - 2) * 16);
        let x  = self.ioram32(0x28 + (bg.index - 2) * 16);
        let y  = self.ioram32(0x2c + (bg.index - 2) * 16);

        bg.matrix = (pa, pb, pc, pd);
        bg.coord  = (x, y);
    }
}

