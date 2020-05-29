use crate::util::*;
use crate::ppu::PPU;
use crate::ppu::sprite::Sprite;

use super::Memory;

impl Memory
{
    #[inline]
    pub fn oam_load8(&self, offset: usize) -> u8
    {
        let value = self.oam_load16(offset);
        value.to_le_bytes()[offset as usize & 1]
    }

    pub fn oam_load16(&self, offset: usize) -> u16
    {
        self.c().ppu.oam_load16(offset)
    }

    #[inline]
    pub fn oam_load32(&self, offset: usize) -> u32
    {
        let lo = self.oam_load16(offset) as u32;
        let hi = self.oam_load16(offset + 2) as u32;
        (hi << 16) | lo
    }

    pub fn oam_store16(&self, offset: usize, value: u16)
    {
        self.c().ppu.oam_store16(offset, value);
    }

    #[inline]
    pub fn oam_store32(&mut self, offset: usize, value: u32)
    {
        self.oam_store16(offset, value as u16);
        self.oam_store16(offset + 2, (value >> 16) as u16);
    }
}
impl Sprite
{
    #[inline]
    pub fn set_attr0(&mut self, value: u16)
    {
        self.attr[0]  = value;

        self.ycoord   = value.bits(7, 0);
        self.affine_f = value.bit(8);
        self.double_f = value.bit(9);
        self.mode     = value.bits(11, 10);
        self.mosaic_f = value.bit(12);
        self.shape    = value.bits(15, 14);
    }

    #[inline]
    pub fn set_attr1(&mut self, value: u16)
    {
        self.attr[1]  = value;

        self.xcoord   = value.bits(8, 0);
        self.hflip    = value.bit(12);
        self.vflip    = value.bit(13);
        self.affine_i = value.bits(13, 9);
        self.size     = value.bits(15, 14);
    }

    #[inline]
    pub fn set_attr2(&mut self, value: u16)
    {
        self.attr[2]   = value;

        self.tile_n    = value.bits(9, 0);
        self.priority  = value.bits(11, 10);
        self.palette_n = value.bits(15, 12);
    }
}

impl PPU
{
    pub fn oam_store16(&mut self, offset: usize, value: u16)
    {
        let obj_index  = offset / 8;
        let attr_index = offset % 8;

        match attr_index
        {
            0 => self.sprite[obj_index].set_attr0(value),
            1 => self.sprite[obj_index].set_attr1(value),
            2 => self.sprite[obj_index].set_attr2(value),
            _ => self.obj_param[obj_index] = value,
        }
    }

    pub fn oam_load16(&self, offset: usize) -> u16
    {
        let obj_index  = offset / 8;
        let attr_index = offset % 8;

        match attr_index
        {
            0..=
            2 => self.sprite[obj_index].attr[attr_index],
            _ => self.obj_param[obj_index],
        }
    }
}