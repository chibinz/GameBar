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
    // Hacky monkey patch...
    if rlist == 0 {
        let addr = cpu.r(rb);
        if l {
            cpu.set_r(15, Cpu::ldr(addr & !0b11, bus));
        } else {
            Cpu::str(addr & !0b11, cpu.r(15) + 2, bus);
        }
        cpu.set_r(rb, addr.wrapping_add(0x40));
    } else {
        // P = 0, U = 1, S = 0, W = true, L = l
        block_data_transfer::execute(cpu, bus, (false, true, false, true, l, rb, rlist));
    }
}
