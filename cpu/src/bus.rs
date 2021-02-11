//! Auxiliary functions for handling misaligned memory access
//! According to gbatek, only LDR, SWP, LDRH, LDRSH behave strangely.
//! Other accesses(store, access multiple) are forcibly align their
//! access address.
//! Current `Memory` implementation always clear lower bits of
//! misaligned addresses.

use super::CPU;
use std::collections::HashMap;
use util::*;

pub trait Bus {
    fn new() -> Self;
    #[inline]
    #[allow(unused_variables)]
    fn load8(&self, address: u32) -> u8 {
        unimplemented!()
    }
    #[inline]
    fn load16(&self, address: u32) -> u16 {
        let lo = self.load8(address);
        let hi = self.load8(address + 1);
        u16::from_le_bytes([lo, hi])
    }
    #[inline]
    fn load32(&self, address: u32) -> u32 {
        let lo = self.load16(address);
        let hi = self.load16(address + 2);
        (hi as u32) << 16 | (lo as u32)
    }
    #[inline]
    #[allow(unused_variables)]
    fn store8(&mut self, address: u32, value: u8) {
        unimplemented!()
    }
    #[inline]
    fn store16(&mut self, address: u32, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.store8(address, lo);
        self.store8(address + 1, hi);
    }
    #[inline]
    fn store32(&mut self, address: u32, value: u32) {
        let lo = value as u16;
        let hi = (value >> 16) as u16;
        self.store16(address, lo);
        self.store16(address + 2, hi);
    }
}

pub struct DummyBus {
    map: HashMap<u32, u8>,
}

impl Bus for DummyBus {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    fn load8(&self, address: u32) -> u8 {
        self.map[&address]
    }
    fn store8(&mut self, address: u32, value: u8) {
        self.map.insert(address, value);
    }
}

impl CPU {
    #[inline]
    pub fn ldr(address: u32, bus: &impl Bus) -> u32 {
        let rotation = (address & 0b11) * 8;

        // Memory loads are forcibly aligned
        let value = bus.load32(address);

        value.rotate_right(rotation)
    }

    #[inline]
    pub fn ldrb(address: u32, bus: &impl Bus) -> u32 {
        bus.load8(address) as u32
    }

    #[inline]
    pub fn ldrh(address: u32, bus: &impl Bus) -> u32 {
        let rotation = (address & 1) * 8;

        let value = bus.load16(address) as u32;

        value.rotate_right(rotation) as u32
    }

    #[inline]
    pub fn ldrsb(address: u32, bus: &impl Bus) -> u32 {
        bus.load8(address) as i8 as i32 as u32
    }

    #[inline]
    pub fn ldrsh(address: u32, bus: &impl Bus) -> u32 {
        if address.bit(0) {
            // Misaligned LDRSH is effectively LDRSB
            bus.load8(address) as i8 as i32 as u32
        } else {
            bus.load16(address) as i16 as i32 as u32
        }
    }

    #[inline]
    pub fn str(address: u32, value: u32, bus: &mut impl Bus) {
        bus.store32(address, value)
    }

    #[inline]
    pub fn strb(address: u32, value: u32, bus: &mut impl Bus) {
        bus.store8(address, value as u8)
    }

    #[inline]
    pub fn strh(address: u32, value: u32, bus: &mut impl Bus) {
        bus.store16(address, value as u16)
    }
}
