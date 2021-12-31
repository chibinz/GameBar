use crate::Cpu;
use util::*;

#[inline]
pub fn interpret(cpu: &mut Cpu, instr: u16) {
    execute(cpu, decode(instr));
}

#[inline]
fn decode(instruction: u16) -> (bool, u32) {
    let s = instruction.bit(7);
    let sword7 = instruction.bits(6, 0);

    (s, sword7)
}

#[inline]
fn execute(cpu: &mut Cpu, (s, sword7): (bool, u32)) {
    let shifted = sword7 << 2;
    let offset = if s { shifted.wrapping_neg() } else { shifted };
    cpu.set_r(13, cpu.r(13).wrapping_add(offset));
}
