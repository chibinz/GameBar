//! Auxiliary functions for handling misaligned memory access
//! According to gbatek, only LDR, SWP, LDRH, LDRSH behave strangely.
//! Other accesses(store, access multiple) are forcibly align their
//! access address.
//! Current `Memory` implementation always clear lower bits of
//! misaligned addresses.

use super::Cpu;
use util::*;

impl Cpu {
    #[inline]
    pub fn ldr(address: u32, bus: &impl Bus) -> u32 {
        let rotation = (address & 0b11) * 8;

        // Memory loads are forcibly aligned
        let value = bus.load32(address as usize);

        value.rotate_right(rotation)
    }

    #[inline]
    pub fn ldrb(address: u32, bus: &impl Bus) -> u32 {
        bus.load8(address as usize) as u32
    }

    #[inline]
    pub fn ldrh(address: u32, bus: &impl Bus) -> u32 {
        let rotation = (address & 1) * 8;

        let value = bus.load16(address as usize) as u32;

        value.rotate_right(rotation) as u32
    }

    #[inline]
    pub fn ldrsb(address: u32, bus: &impl Bus) -> u32 {
        bus.load8(address as usize) as i8 as i32 as u32
    }

    #[inline]
    pub fn ldrsh(address: u32, bus: &impl Bus) -> u32 {
        if address.bit(0) {
            // Misaligned LDRSH is effectively LDRSB
            bus.load8(address as usize) as i8 as i32 as u32
        } else {
            bus.load16(address as usize) as i16 as i32 as u32
        }
    }

    #[inline]
    pub fn str(address: u32, value: u32, bus: &mut impl Bus) {
        bus.store32(address as usize, value)
    }

    #[inline]
    pub fn strb(address: u32, value: u32, bus: &mut impl Bus) {
        bus.store8(address as usize, value as u8)
    }

    #[inline]
    pub fn strh(address: u32, value: u32, bus: &mut impl Bus) {
        bus.store16(address as usize, value as u16)
    }
}

use std::collections::HashMap;

/// Used in unit tests
pub struct DummyBus {
    map: HashMap<usize, u8>,
}

impl DummyBus {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl Bus for DummyBus {
    fn load8(&self, address: usize) -> u8 {
        self.map[&address]
    }
    fn store8(&mut self, address: usize, value: u8) {
        self.map.insert(address, value);
    }
}
