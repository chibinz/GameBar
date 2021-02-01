use crate::cpu::CPU;
use crate::util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, instruction: u16) {
    execute(cpu, decode(instruction));
}

#[inline]
pub fn decode(instruction: u16) -> u32 {
    instruction.bits(10, 0)
}

#[inline]
pub fn execute(cpu: &mut CPU, offset11: u32) {
    cpu.r[15] = cpu.r[15].wrapping_add(sign_extend(offset11 << 1, 11) as u32);
    cpu.flush();
}
