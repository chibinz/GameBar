use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, instr: u32) {
    execute(cpu, decode(instr));
}

#[inline]
pub fn decode(instr: u32) -> (bool, bool, u32, u32, u32, u32) {
    debug_assert_eq!(instr.bits(7, 4), 0b1001);

    let a = instr.bit(21);
    let s = instr.bit(20);
    let rd = instr.bits(19, 16);
    let rn = instr.bits(15, 12);
    let rs = instr.bits(11, 8);
    let rm = instr.bits(3, 0);

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
pub fn execute(cpu: &mut CPU, (a, s, rd, rn, rs, rm): (bool, bool, u32, u32, u32, u32)) {
    let op0 = cpu.r(rm);
    let op1 = cpu.r(rs);
    let mut result = op0.wrapping_mul(op1);

    // If accumulate bit is set, add rn to result
    if a {
        result = result.wrapping_add(cpu.r(rn));

        // One extra cycle for accumulation
        cpu.cycles += 1;
    } else {
        // Rn should be set to 0 if not used as accumulate
        debug_assert_eq!(rn, 0);
    }

    if s {
        cpu.cpsr.z = result == 0;
        cpu.cpsr.n = result.bit(31);

        // The C (Carry) flag is set to a meaningless value.
        // And the V (Overflow) flag is unaffected.
    }

    cpu.set_r(rd, result);

    // Internal cycles depending on size of operand
    cpu.cycles += count_cycles(op0, op1);
}

pub fn count_cycles(op0: u32, op1: u32) -> i32 {
    if op0 < 0x100 && op1 < 0x100 {
        1
    } else if op0 < 0x10000 && op1 < 0x10000 {
        2
    } else if op0 < 0x1000000 && op1 < 0x1000000 {
        3
    } else {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_execute() {
        let mut cpu = CPU::new();

        cpu.set_r(0, 0xfffffff6);
        cpu.set_r(1, 0x00000014);
        execute(&mut cpu, (false, false, 3, 0, 0, 1));
        assert_eq!(cpu.r(3), 0xffffff38);

        cpu.set_r(0, 0x10);
        cpu.set_r(1, 0x10000000);
        execute(&mut cpu, (false, true, 3, 0, 0, 1));
        assert_eq!(cpu.r(3), 0);
        assert_eq!(cpu.cpsr.z, true);
        assert_eq!(cpu.cpsr.n, false);
    }
}
