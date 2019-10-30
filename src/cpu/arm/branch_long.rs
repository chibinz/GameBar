use crate::util::*;
use crate::cpu::CPU;

pub fn decode_execute(cpu: &mut CPU, instruction: u32)
{
    // If the link bit is set, the old value of pc is written
    // to the link register, which is R14.
    let l = bit(instruction, 24);

    // The offset is left shifted by 2 and sign extended to 32 bits
    let offset: i32 = sign_extend(bits(instruction, 23, 0) << 2, 25);
    
    if l
    {
        cpu.register.r[14] = cpu.register.r[15];
    }

    cpu.register.r[15] = cpu.register.r[15].wrapping_add(offset as u32);
}