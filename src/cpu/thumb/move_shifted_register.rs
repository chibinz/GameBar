use crate::util::BitField;
use crate::cpu::CPU;
use crate::cpu::barrel_shifter::shift;

#[inline]
pub fn decode_execute(cpu: &mut CPU, instruction: u16)
{
    execute(cpu, decode(instruction));
}

#[inline]
fn decode(instruction: u16) -> (u32, u32, u32, u32)
{
    let op = instruction.bits(12, 11);
    let offset5 = instruction.bits(10, 6);
    let rs = instruction.bits(5, 3);
    let rd = instruction.bits(2, 0);

    (op, offset5, rs, rd)
}

#[inline]
fn execute(cpu: &mut CPU, (op, offset5, rs, rd): (u32, u32, u32, u32))
{
    let shifted = shift(cpu, cpu.register.r[rs as usize], offset5, op);

    cpu.register.r[rd as usize] = shifted;
}