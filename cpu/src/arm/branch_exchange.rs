use crate::Cpu;
use util::*;

#[inline]
pub fn interpret(cpu: &mut Cpu, instr: u32) {
    debug_assert_eq!(instr.bits(27, 4), 0b000100101111111111110001);

    let rn = instr.bits(3, 0);
    debug_assert_ne!(rn, 15);

    // If bit 0 of rn = 1, subsequent instructions are decoded as THUMB instructions
    cpu.cpsr.t = cpu.r(rn).bit(0);

    cpu.set_r(15, cpu.r(rn));
}
