use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::register::PSRBit::*;

#[inline]
pub fn decode_execute(cpu: &mut CPU, instruction: u32)
{
    execute(cpu, decode(instruction));
}

#[inline]
pub fn decode(instruction: u32) -> (bool, bool, u32, u32, u32, u32)
{
    debug_assert_eq!(instruction.bits(7, 4), 0b1001);

    let a = instruction.bit(21);
    let s = instruction.bit(20);
    let rd = instruction.bits(19, 16);
    let rn = instruction.bits(15, 12);
    let rs = instruction.bits(11, 8);
    let rm = instruction.bits(3, 0);

    // The destination register rd must not be the same as the operand register rm
    debug_assert_ne!(rd, rm);

    // `r15` must not be used as an operand or destination register
    debug_assert_ne!(rd, 15);
    debug_assert_ne!(rn, 15);
    debug_assert_ne!(rs, 15);
    debug_assert_ne!(rm, 15);

    (a, s, rd, rn, rs, rm)
}

#[inline]
pub fn execute(cpu: &mut CPU, (a, s, rd, rn, rs, rm): (bool, bool, u32, u32, u32, u32))
{
    let mut result = cpu.r[rm as usize].wrapping_mul(cpu.r[rs as usize]);

    // If accumulate bit is set, add rn to result
    if a
    {
        result = result.wrapping_add(cpu.r[rn as usize]);
    }
    else
    {
        // Rn should be set to 0 if not used as accumulate
        debug_assert_eq!(rn, 0);
    }

    if s
    {
        cpu.set_cpsr_bit(Z, result == 0);
        cpu.set_cpsr_bit(N, result.bit(31));

        // The C (Carry) flag is set to a meaningless value.
        // And the V (Overflow) flag is unaffected.
    }

    cpu.r[rd as usize] = result;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn mul_execute()
    {
        let mut cpu = CPU::new();

        cpu.r[0] = 0xfffffff6;
        cpu.r[1] = 0x00000014;
        execute(&mut cpu, (false, false, 3, 0, 0, 1));
        assert_eq!(cpu.r[3], 0xffffff38);

        cpu.r[0] = 0x10;
        cpu.r[1] = 0x10000000;
        execute(&mut cpu, (false, true, 3, 0, 0, 1));
        assert_eq!(cpu.r[3], 0);
        assert_eq!(cpu.get_cpsr_bit(Z), true);
        assert_eq!(cpu.get_cpsr_bit(N), false);
    }
}