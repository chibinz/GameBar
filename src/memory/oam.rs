use crate::util::*;
use crate::ppu::sprite::Sprite;
use crate::ppu::sprite::DIMENSION;

use super::Memory;
use super::into16;

impl Memory
{
    /// Return a halfword from oam, offset is in bytes
    #[inline]
    pub fn oam16(&self, offset: u32) -> u16
    {
        let a = offset as usize;
        into16(&self.oam[a..a+2])
    }

    #[inline]
    pub fn update_sprite(&self, sprite: &mut Sprite)
    {
        self.update_attr0(sprite);
        self.update_attr1(sprite);
        self.update_attr2(sprite);
        self.update_matrix(sprite);
    }

    #[inline]
    pub fn update_attr0(&self, sprite: &mut Sprite)
    {
        let attr0 = self.oam16(sprite.index as u32 * 8);

        sprite.ycoord   = attr0.bits(7, 0);
        sprite.affine_f = attr0.bit(8);
        sprite.double_f = attr0.bit(9);
        sprite.mode     = attr0.bits(11, 10);
        sprite.mosaic_f = attr0.bit(12);
    }

    #[inline]
    pub fn update_attr1(&self, sprite: &mut Sprite)
    {
        let attr0 = self.oam16(sprite.index as u32 * 8);
        let attr1 = self.oam16(0x02 + sprite.index as u32 * 8);
        let shape = attr0.bits(15, 14) as usize;
        let size  = attr1.bits(15, 14) as usize;

        sprite.xcoord   = attr1.bits(8, 0);
        sprite.hflip    = !attr0.bit(8) && attr1.bit(12);
        sprite.vflip    = !attr0.bit(8) && attr1.bit(13);
        sprite.affine_i = attr1.bits(11, 10);
        sprite.width    = DIMENSION[shape][size].0;
        sprite.height   = DIMENSION[shape][size].1;
    }

    #[inline]
    pub fn update_attr2(&self, sprite: &mut Sprite)
    {
        let attr2 = self.oam16(0x04 + sprite.index as u32 * 8);

        sprite.tile_n    = attr2.bits(9, 0);
        sprite.priority  = attr2.bits(11, 10);
        sprite.palette_n = attr2.bits(15, 12);
    }

    #[inline]
    pub fn update_matrix(&self, sprite: &mut Sprite)
    {
        let pa = self.oam16(0x06 + sprite.affine_i * 0x20) as i16 as i32;
        let pb = self.oam16(0x0e + sprite.affine_i * 0x20) as i16 as i32;
        let pc = self.oam16(0x16 + sprite.affine_i * 0x20) as i16 as i32;
        let pd = self.oam16(0x1e + sprite.affine_i * 0x20) as i16 as i32;

        sprite.matrix = (pa, pb, pc, pd);
    }
}