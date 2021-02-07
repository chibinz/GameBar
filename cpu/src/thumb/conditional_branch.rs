use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, instr: u16) {
    execute(cpu, decode(instr));
}

#[inline]
pub fn decode(instr: u16) -> (u32, u32) {
    let cond = instr.bits(11, 8);
    let soffset8 = instr.bits(7, 0);

    (cond, soffset8)
}

#[inline]
pub fn execute(cpu: &mut CPU, (cond, soffset8): (u32, u32)) {
    if cpu.check_condition(cond) {
        cpu.set_r(15, (cpu.r(15) as i32 + sign_extend(soffset8 << 1, 8)) as u32);
    }
}
