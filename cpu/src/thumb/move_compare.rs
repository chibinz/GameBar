use crate::alu;
use crate::Cpu;
use util::*;

#[inline]
pub fn interpret(cpu: &mut Cpu, instr: u16) {
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
fn execute(cpu: &mut Cpu, (op, rd, offset8): (u32, u32, u32)) {
    let op1 = cpu.r(rd);
    let op2 = offset8;
    let c = cpu.cpsr.c;
    let v = cpu.cpsr.v;

    let (result, flags) = match op {
        0b00 => alu::mov(op1, op2, c, v),
        0b01 => alu::cmp(op1, op2, c, v),
        0b10 => alu::add(op1, op2, c, v),
        0b11 => alu::sub(op1, op2, c, v),
        _ => unreachable!(),
    };
    cpu.set_flags(flags);

    if op != 0b01 {
        cpu.set_r(rd, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_compare() {
        let mut cpu = Cpu::new();

        cpu.set_r(1, 0xffffffff);
        execute(&mut cpu, (0b10, 1, 1));
        assert_eq!(cpu.r(1), 0);
        assert_eq!(cpu.cpsr.c, true);

        cpu.set_r(1, 0x7fffffff);
        execute(&mut cpu, (0b10, 1, 1));
        assert_eq!(cpu.r(1), 0x80000000);
        assert_eq!(cpu.cpsr.v, true);
        assert_eq!(cpu.cpsr.c, false);
        assert_eq!(cpu.cpsr.n, true);

        cpu.set_r(1, 1);
        execute(&mut cpu, (0b01, 1, 2));
        assert_eq!(cpu.r(1), 1);
        assert_eq!(cpu.cpsr.v, false);
        assert_eq!(cpu.cpsr.c, false);
        assert_eq!(cpu.cpsr.n, true);
        assert_eq!(cpu.cpsr.z, false);
    }
}
