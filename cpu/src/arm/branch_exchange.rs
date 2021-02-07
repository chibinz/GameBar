use crate::register::PSRBit::T;
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, instr: u32) {
    debug_assert_eq!(instr.bits(27, 4), 0b000100101111111111110001);

    let rn = instr.bits(3, 0);
    debug_assert_ne!(rn, 15);

    // If bit 0 of rn = 1, subsequent instructions are decoded as THUMB instructions
    cpu.set_cpsr_bit(T, cpu.r(rn).bit(0));

    cpu.set_r(15, cpu.r(rn));
}
