use crate::alu;
use crate::barrel_shifter::shift;
use crate::CPU;
use crate::register::PSRBit::C;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, instr: u16) {
    execute(cpu, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (u32, u32, u32, u32) {
    let op = instr.bits(12, 11);
    let offset5 = instr.bits(10, 6);
    let rs = instr.bits(5, 3);
    let rd = instr.bits(2, 0);

    (op, offset5, rs, rd)
}

#[inline]
fn execute(cpu: &mut CPU, (op, offset5, rs, rd): (u32, u32, u32, u32)) {
    let shifted = 0 ;//shift(cpu, cpu.r[rs as usize], offset5, cpu.get_cpsr_bit(C), op);

    // Use alu's mov instead of direct assignment to set flags
    cpu.r[rd as usize] = alu::mov(cpu, shifted, shifted, true);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::PSRBit::*;

    #[test]
    fn logical_left() {
        let mut cpu = CPU::new();

        cpu.r[0] = 1;
        execute(&mut cpu, (0b00, 0b11111, 0, 1));
        assert_eq!(cpu.r[1], 0x80000000);
        assert_eq!(cpu.get_cpsr_bit(C), false);

        cpu.r[0] = 0b10;
        execute(&mut cpu, (0b00, 0b11111, 0, 1));
        assert_eq!(cpu.r[1], 0);
        assert_eq!(cpu.get_cpsr_bit(C), true);
    }

    #[test]
    fn logical_right() {
        let mut cpu = CPU::new();

        cpu.r[0] = 0x80000000;
        execute(&mut cpu, (0b01, 0b11111, 0, 1));
        assert_eq!(cpu.r[1], 1);
        assert_eq!(cpu.get_cpsr_bit(C), false);

        cpu.r[0] = 0x40000000;
        execute(&mut cpu, (0b01, 0b11111, 0, 1));
        assert_eq!(cpu.r[1], 0);
        assert_eq!(cpu.get_cpsr_bit(C), true);
    }

    #[test]
    fn arithmetic_right() {
        let mut cpu = CPU::new();

        cpu.r[0] = 0x80000000;
        execute(&mut cpu, (0b10, 0b11111, 0, 1));
        assert_eq!(cpu.r[1], 0xffffffff);
        assert_eq!(cpu.get_cpsr_bit(C), false);

        cpu.r[0] = 0x40000000;
        execute(&mut cpu, (0b10, 0b11111, 0, 1));
        assert_eq!(cpu.r[1], 0);
        assert_eq!(cpu.get_cpsr_bit(C), true);
    }
}
