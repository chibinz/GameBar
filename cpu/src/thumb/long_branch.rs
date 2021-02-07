use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, instr: u16) {
    execute(cpu, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (bool, u32) {
    let h = instr.bit(11);
    let offset = instr.bits(10, 0);

    (h, offset)
}

#[inline]
fn execute(cpu: &mut CPU, (h, offset): (bool, u32)) {
    if h {
        let temp = cpu.r(15) - 2;
        cpu.set_r(15, cpu.r(14).wrapping_add(offset << 1));
        cpu.set_r(14, temp | 1);
    } else {
        cpu.set_r(14, cpu.r(15).wrapping_add((sign_extend(offset, 10) as u32) << 12));
    }
}
