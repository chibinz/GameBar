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
    fn load8(&self, address: u32) -> u8;
    fn load16(&self, address: u32) -> u16;
    fn load32(&self, address: u32) -> u32;
    fn store8(&mut self, address: u32, value: u8);
    fn store16(&mut self, address: u32, value: u16);
    fn store32(&mut self, address: u32, value: u32);
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
    fn load16(&self, address: u32) -> u16 {
        u16::from_le_bytes([self.map[&address], self.map[&(address + 1)]])
    }
    fn load32(&self, address: u32) -> u32 {
        u32::from_le_bytes([
            self.map[&address],
            self.map[&(address + 1)],
            self.map[&(address + 2)],
            self.map[&(address + 3)],
        ])
    }
    fn store8(&mut self, address: u32, value: u8) {
        self.map.insert(address, value);
    }
    fn store16(&mut self, address: u32, value: u16) {
        let [v0, v1] = value.to_le_bytes();
        self.map.insert(address, v0);
        self.map.insert(address + 1, v1);
    }
    fn store32(&mut self, address: u32, value: u32) {
        let [v0, v1, v2, v3] = value.to_le_bytes();
        self.map.insert(address, v0);
        self.map.insert(address + 1, v1);
        self.map.insert(address + 2, v2);
        self.map.insert(address + 3, v3);
    }
}

impl CPU {
    #[inline]
    pub fn ldr(address: u32, bus: &mut impl Bus) -> u32 {
        let rotation = (address & 0b11) * 8;

        // Memory loads are forcibly aligned
        let value = bus.load32(address);

        value.rotate_right(rotation)
    }

    #[inline]
    pub fn ldrh(address: u32, bus: &mut impl Bus) -> u32 {
        let rotation = (address & 1) * 8;

        let value = bus.load16(address) as u32;

        value.rotate_right(rotation) as u32
    }

    #[inline]
    pub fn ldrsh(address: u32, bus: &mut impl Bus) -> u32 {
        if address.bit(0) {
            // Misaligned LDRSH is effectively LDRSB
            bus.load8(address) as i8 as i32 as u32
        } else {
            bus.load16(address) as i16 as i32 as u32
        }
    }
}
