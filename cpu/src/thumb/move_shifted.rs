use crate::alu;
use crate::barrel_shifter::shift;
use crate::CPU;
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
    let (shifted, carry) = shift(cpu.r(rs), offset5, op, cpu.carry(), true);
    cpu.set_carry(carry);

    let (c, v) = alu::get_cv(cpu);
    let (result, flags) = alu::mov(shifted, shifted, c, v);
    alu::set_flags(cpu, flags);
    cpu.set_r(rd, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_left() {
        let mut cpu = CPU::new();

        cpu.set_r(0, 1);
        execute(&mut cpu, (0b00, 0b11111, 0, 1));
        assert_eq!(cpu.r(1), 0x80000000);
        assert_eq!(cpu.carry(), false);

        // cpu.set_r(0, 0b10);
        // execute(&mut cpu, (0b00, 0b11111, 0, 1));
        // assert_eq!(cpu.r(1), 0);
        // assert_eq!(cpu.carry(), true);
    }

    #[test]
    fn logical_right() {
        let mut cpu = CPU::new();

        cpu.set_r(0, 0x80000000);
        execute(&mut cpu, (0b01, 0b11111, 0, 1));
        assert_eq!(cpu.r(1), 1);
        assert_eq!(cpu.carry(), false);

        cpu.set_r(0, 0x40000000);
        execute(&mut cpu, (0b01, 0b11111, 0, 1));
        assert_eq!(cpu.r(1), 0);
        assert_eq!(cpu.carry(), true);
    }

    #[test]
    fn arithmetic_right() {
        let mut cpu = CPU::new();

        cpu.set_r(0, 0x80000000);
        execute(&mut cpu, (0b10, 0b11111, 0, 1));
        assert_eq!(cpu.r(1), 0xffffffff);
        assert_eq!(cpu.carry(), false);

        cpu.set_r(0, 0x40000000);
        execute(&mut cpu, (0b10, 0b11111, 0, 1));
        assert_eq!(cpu.r(1), 0);
        assert_eq!(cpu.carry(), true);
    }
}
