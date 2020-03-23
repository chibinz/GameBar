use crate::util::*;

pub static TRANSPARENT: u32 = 0xff000000;

#[inline]
pub fn RGB(a: u16) -> u32
{
    let r = a.bits(4, 0) << 19;
    let g = a.bits(9, 5) << 11;
    let b = a.bits(14, 10) << 3;

    r | g | b
}