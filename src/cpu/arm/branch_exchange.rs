use util::*;
use cpu::CPU;
use cpu::register::PSRBit::T;
use cpu::register::set_cpsr_bit;

pub fn decode_execute(cpu: &mut CPU, instruction: u32)
{
    debug_assert_eq!(bits(instruction, 27, 4), 0b000100101111111111110001);

    let rn = bits(instruction, 3, 0);
    debug_assert_ne!(rn, 15);

    // If bit 0 of rn = 1, subsequent instructions are decoded as THUMB instructions
    cpu.register.set_cpsr_bit(T, bit(cpu.register.r[rn]));

    cpu.register.r[15] = cpu.register.r[rn];
}