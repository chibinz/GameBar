use crate::Cpu;
use util::*;

#[inline]
pub fn interpret(cpu: &mut Cpu, instr: u16) {
    execute(cpu, decode(instr));
}

#[inline]
pub fn decode(instr: u16) -> u32 {
    instr.bits(10, 0)
}

#[inline]
pub fn execute(cpu: &mut Cpu, offset11: u32) {
    let offset = sign_extend(offset11 << 1, 11) as u32;
    cpu.set_r(15, cpu.r(15).wrapping_add(offset));
}
