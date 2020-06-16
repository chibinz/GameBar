use crate::util::*;
use crate::cpu::CPU;

#[inline]
pub fn interpret(cpu: &mut CPU, instruction: u32)
{
    // If the link bit is set, the old value of pc is written
    // to the link register, which is R14.
    let l = instruction.bit(24);

    // The offset is left shifted by 2 and sign extended to 32 bits
    let offset: i32 = sign_extend(instruction.bits(23, 0) << 2, 25);

    if l
    {
        cpu.r[14] = cpu.r[15] - 4;
    }

    cpu.r[15] = cpu.r[15].wrapping_add(offset as u32);

    cpu.flush();
}