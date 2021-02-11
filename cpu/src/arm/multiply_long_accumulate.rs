use crate::CPU;
use util::*;

use super::multiply_accumulate::count_cycles;

#[inline]
pub fn interpret(cpu: &mut CPU, instr: u32) {
    execute(cpu, decode(instr));
}

#[inline]
pub fn decode(instr: u32) -> (bool, bool, bool, u32, u32, u32, u32) {
    debug_assert_eq!(instr.bits(7, 4), 0b1001);

    let u = instr.bit(22);
    let a = instr.bit(21);
    let s = instr.bit(20);
    let rdhi = instr.bits(19, 16);
    let rdlo = instr.bits(15, 12);
    let rs = instr.bits(11, 8);
    let rm = instr.bits(3, 0);

    // `rdhi`, `rdlo`, and `rm` must all specify different registers.
    debug_assert_ne!(rdhi, rm);
    debug_assert_ne!(rdlo, rm);
    debug_assert_ne!(rdhi, rdlo);

    // `r15` must not be used as an operand or destination register
    debug_assert_ne!(rdhi, 15);
    debug_assert_ne!(rdlo, 15);
    debug_assert_ne!(rs, 15);
    debug_assert_ne!(rm, 15);

    (u, a, s, rdhi, rdlo, rs, rm)
}

#[inline]
pub fn execute(
    cpu: &mut CPU,
    (u, a, s, rdhi, rdlo, rs, rm): (bool, bool, bool, u32, u32, u32, u32),
) {
    let mut result: u64;

    // 0 for u means unsigned
    if !u {
        let operand1 = cpu.r(rm) as u64;
        let operand2 = cpu.r(rs) as u64;

        result = operand1 * operand2;
    } else {
        // Operands are sign extended to 64 bits. `i32` is necessary for sign extension.
        let operand1 = cpu.r(rm) as i32 as i64;
        let operand2 = cpu.r(rs) as i32 as i64;

        result = (operand1 * operand2) as u64;
    }

    if a {
        let hi = (cpu.r(rdhi) as u64) << 32;
        let lo = cpu.r(rdlo) as u64;

        result = result.wrapping_add(hi + lo);

        // Extra cycle for accumulation
        cpu.cycles += 1;
    }

    if s {
        cpu.cpsr.z = result == 0;
        cpu.cpsr.n = result >> 63 == 1;

        // Both the C and V flags are set to meaningless values
    }

    cpu.set_r(rdhi, (result >> 32) as u32);
    cpu.set_r(rdlo, result as u32);

    // Long multiplication consumes one additional internal cycle
    cpu.cycles += 1 + count_cycles(cpu.r(rm), cpu.r(rs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_long_execute() {
        let mut cpu = CPU::new();

        // unsigned
        cpu.set_r(0, 0x10000000);
        cpu.set_r(1, 0x10000000);
        execute(&mut cpu, (false, false, false, 4, 3, 0, 1));
        assert_eq!(cpu.r(3), 0);
        assert_eq!(cpu.r(4), 0x01000000);

        // signed
        cpu.set_r(0, 0xffffffff);
        cpu.set_r(1, 0xffffffff);
        execute(&mut cpu, (true, false, true, 4, 3, 0, 1));
        assert_eq!(cpu.r(3), 1);
        assert_eq!(cpu.r(4), 0);
        assert_eq!(cpu.cpsr.n, false);
    }
}
