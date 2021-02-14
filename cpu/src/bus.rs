//! Auxiliary functions for handling misaligned memory access
//! According to gbatek, only LDR, SWP, LDRH, LDRSH behave strangely.
//! Other accesses(store, access multiple) are forcibly align their
//! access address.
//! Current `Memory` implementation always clear lower bits of
//! misaligned addresses.

use super::CPU;
use util::*;

impl CPU {
    #[inline]
    pub fn ldr<T: ?Sized + Bus>(address: u32, bus: &T) -> u32 {
        let rotation = (address & 0b11) * 8;

        // Memory loads are forcibly aligned
        let value = bus.load32(address as usize);

        value.rotate_right(rotation)
    }

    #[inline]
    pub fn ldrb<T: ?Sized + Bus>(address: u32, bus: &T) -> u32 {
        bus.load8(address as usize) as u32
    }

    #[inline]
    pub fn ldrh<T: ?Sized + Bus>(address: u32, bus: &T) -> u32 {
        let rotation = (address & 1) * 8;

        let value = bus.load16(address as usize) as u32;

        value.rotate_right(rotation) as u32
    }

    #[inline]
    pub fn ldrsb<T: ?Sized + Bus>(address: u32, bus: &T) -> u32 {
        bus.load8(address as usize) as i8 as i32 as u32
    }

    #[inline]
    pub fn ldrsh<T: ?Sized + Bus>(address: u32, bus: &T) -> u32 {
        if address.bit(0) {
            // Misaligned LDRSH is effectively LDRSB
            bus.load8(address as usize) as i8 as i32 as u32
        } else {
            bus.load16(address as usize) as i16 as i32 as u32
        }
    }

    #[inline]
    pub fn str<T: ?Sized + Bus>(address: u32, value: u32, bus: &mut T) {
        bus.store32(address as usize, value)
    }

    #[inline]
    pub fn strb<T: ?Sized + Bus>(address: u32, value: u32, bus: &mut T) {
        bus.store8(address as usize, value as u8)
    }

    #[inline]
    pub fn strh<T: ?Sized + Bus>(address: u32, value: u32, bus: &mut T) {
        bus.store16(address as usize, value as u16)
    }
}
