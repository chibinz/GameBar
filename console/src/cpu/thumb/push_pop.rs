use crate::cpu::CPU;
use crate::memory::Memory;
use crate::util::*;

use crate::cpu::arm::block_data_transfer;

#[inline]
pub fn interpret(cpu: &mut CPU, memory: &mut Memory, instruction: u16) {
    execute(cpu, memory, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (bool, bool, u32) {
    debug_assert_eq!(instruction.bits(10, 9), 0b10);

    let l = instruction.bit(11);
    let r = instruction.bit(8);
    let rlist = instruction.bits(7, 0);

    (l, r, rlist)
}

#[inline]
fn execute(cpu: &mut CPU, memory: &mut Memory, (l, r, rlist): (bool, bool, u32)) {
    // Push the link register, and then registers specified by rlist
    // onto the stack
    if r && !l {
        cpu.r[13] -= 4;
        memory.store32(cpu.r[13], cpu.r[14]);
    }

    if rlist != 0 {
        block_data_transfer::execute(cpu, memory, (!l, l, false, true, l, 13, rlist))
    }

    // Pop values off the stack into registers specified by rlist,
    // and then Pop PC off the stack
    if r && l {
        cpu.r[15] = CPU::ldr(cpu.r[13], memory);
        cpu.flush();

        cpu.r[13] += 4;
    }
}
