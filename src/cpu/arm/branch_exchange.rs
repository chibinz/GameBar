use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::register::PSRBit::T;

pub fn decode_execute(cpu: &mut CPU, instruction: u32)
{
    debug_assert_eq!(instruction.bits(27, 4), 0b000100101111111111110001);

    let rn = instruction.bits(3, 0);
    debug_assert_ne!(rn, 15);

    // If bit 0 of rn = 1, subsequent instructions are decoded as THUMB instructions
    cpu.register.set_cpsr_bit(T, cpu.register.r[rn as usize].bit(0));

    if cpu.register.r[rn as usize].bit(0)
    {
        cpu.register.r[15] = cpu.register.r[rn as usize] & 0xfffffffe;
    }
    else
    {
        cpu.register.r[15] = cpu.register.r[rn as usize] & 0xfffffffc;
    }
    cpu.flushed = true;
}