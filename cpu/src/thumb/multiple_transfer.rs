use crate::Bus;
use crate::Cpu;
use util::*;

use crate::arm::block_data_transfer;

#[inline]
pub fn interpret(cpu: &mut Cpu, bus: &mut impl Bus, instr: u16) {
    execute(cpu, bus, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (bool, u32, u32) {
    let l = instr.bit(11);
    let rb = instr.bits(10, 8);
    let rlist = instr.bits(7, 0);

    (l, rb, rlist)
}

#[inline]
fn execute(cpu: &mut Cpu, bus: &mut impl Bus, (l, rb, rlist): (bool, u32, u32)) {
    // P = 0, U = 1, S = 0, W = true, L = l
    block_data_transfer::execute(cpu, bus, (false, true, false, true, l, rb, rlist));
}
