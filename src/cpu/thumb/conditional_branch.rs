use crate::util::*;
use crate::cpu::CPU;

#[inline]
pub fn interpret(cpu: &mut CPU, instruction: u16)
{
    execute(cpu, decode(instruction));
}

#[inline]
pub fn decode(instruction: u16) -> (u32, u32)
{
    let cond = instruction.bits(11, 8);
    let soffset8 = instruction.bits(7, 0);

    (cond, soffset8)
}

#[inline]
pub fn execute(cpu: &mut CPU, (cond, soffset8): (u32, u32))
{
    if cpu.check_condition(cond)
    {
        cpu.r[15] = (cpu.r[15] as i32 + sign_extend(soffset8 << 1, 8)) as u32;
        cpu.flush();
    }
}