//! Auxiliary functions for handling misaligned memory access
//! According to gbatek, only LDR, SWP, LDRH, LDRSH behave strangely.
//! Other accesses(store, access multiple) are forcibly align their
//! access address.
//! Current `Memory` implementation always clear lower bits of
//! misaligned addresses.

use crate::memory::Memory;
use crate::util::*;

use super::CPU;

impl CPU
{
    #[inline]
    pub fn ldr(address: u32, memory: &mut Memory) -> u32
    {
        let rotation = (address & 0b11) * 8;

        // Memory loads are forcibly aligned
        let value = memory.load32(address);

        value.rotate_right(rotation)
    }

    #[inline]
    pub fn ldrh(address: u32, memory: &mut Memory) -> u32
    {
        let rotation = (address & 1) * 8;

        let value = memory.load16(address);

        value.rotate_right(rotation) as u32
    }

    #[inline]
    pub fn ldrsh(address: u32, memory: &mut Memory) -> u32
    {
        if address.bit(0)
        {
            // Misaligned LDRSH is effectively LDRSB
            memory.load8(address) as i8 as i32 as u32
        }
        else
        {
            memory.load16(address) as i16 as i32 as u32
        }
    }
}
