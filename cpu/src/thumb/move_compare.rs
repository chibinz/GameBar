use crate::alu;
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, instr: u16) {
    execute(cpu, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (u32, u32, u32) {
    let op = instr.bits(12, 11);
    let rd = instr.bits(10, 8);
    let offset8 = instr.bits(7, 0);

    (op, rd, offset8)
}

#[inline]
fn execute(cpu: &mut CPU, (op, rd, offset8): (u32, u32, u32)) {
    let op1 = cpu.r[rd as usize];
    let op2 = offset8;
    let (c, v) = alu::get_cv(cpu);

    let (result , flags)= match op {
        0b00 => alu::mov(op1, op2, c, v),
        0b01 => alu::cmp(op1, op2, c, v),
        0b10 => alu::add(op1, op2, c, v),
        0b11 => alu::sub(op1, op2, c, v),
        _ => unreachable!(),
    };
    alu::set_flags(cpu, flags);

    if op != 0b01 {
        cpu.r[rd as usize] = result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::PSRBit::*;

    #[test]
    fn move_compare() {
        let mut cpu = CPU::new();

        cpu.r[1] = 0xffffffff;
        execute(&mut cpu, (0b10, 1, 1));
        assert_eq!(cpu.r[1], 0);
        assert_eq!(cpu.get_cpsr_bit(C), true);

        cpu.r[1] = 0x7fffffff;
        execute(&mut cpu, (0b10, 1, 1));
        assert_eq!(cpu.r[1], 0x80000000);
        assert_eq!(cpu.get_cpsr_bit(V), true);
        assert_eq!(cpu.get_cpsr_bit(C), false);
        assert_eq!(cpu.get_cpsr_bit(N), true);

        cpu.r[1] = 1;
        execute(&mut cpu, (0b01, 1, 2));
        assert_eq!(cpu.r[1], 1);
        assert_eq!(cpu.get_cpsr_bit(V), false);
        assert_eq!(cpu.get_cpsr_bit(C), false);
        assert_eq!(cpu.get_cpsr_bit(N), true);
        assert_eq!(cpu.get_cpsr_bit(Z), false);
    }
}
