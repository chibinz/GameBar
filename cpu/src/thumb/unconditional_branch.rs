use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, instr: u16) {
    execute(cpu, decode(instr));
}

#[inline]
pub fn decode(instr: u16) -> u32 {
    instr.bits(10, 0)
}

#[inline]
pub fn execute(cpu: &mut CPU, offset11: u32) {
    cpu.r[15] = cpu.r[15].wrapping_add(sign_extend(offset11 << 1, 11) as u32);
    cpu.flush();
}
