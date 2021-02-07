use crate::Bus;
use crate::CPU;
use util::*;

use crate::arm::block_data_transfer;

#[inline]
pub fn interpret(cpu: &mut CPU, bus: &mut impl Bus, instr: u16) {
    execute(cpu, bus, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (bool, bool, u32) {
    debug_assert_eq!(instr.bits(10, 9), 0b10);

    let l = instr.bit(11);
    let r = instr.bit(8);
    let rlist = instr.bits(7, 0);

    (l, r, rlist)
}

#[inline]
fn execute(cpu: &mut CPU, bus: &mut impl Bus, (l, r, rlist): (bool, bool, u32)) {
    // Push the link register, and then registers specified by rlist
    // onto the stack
    if r && !l {
        cpu.set_r(13, cpu.r(13) - 4);
        CPU::str(cpu.r(13), cpu.r(14), bus);
    }

    if rlist != 0 {
        block_data_transfer::execute(cpu, bus, (!l, l, false, true, l, 13, rlist))
    }

    // Pop values off the stack into registers specified by rlist,
    // and then Pop PC off the stack
    if r && l {
        cpu.set_r(15, CPU::ldr(cpu.r(13), bus));
        cpu.set_r(13, cpu.r(13) + 4);
    }
}
