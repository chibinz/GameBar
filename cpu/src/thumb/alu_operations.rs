use crate::alu;
use crate::Cpu;
use util::*;

#[inline]
pub fn interpret(cpu: &mut Cpu, instr: u16) {
    execute(cpu, decode(instr));
}

#[inline]
fn decode(instruction: u16) -> (u32, u32, u32) {
    let op = instruction.bits(9, 6);
    let rs = instruction.bits(5, 3);
    let rd = instruction.bits(2, 0);

    (op, rs, rd)
}

#[inline]
fn execute(cpu: &mut Cpu, (op, rs, rd): (u32, u32, u32)) {
    let op1 = cpu.r(rd);
    let op2 = cpu.r(rs);
    let c = cpu.cpsr.c;
    let v = cpu.cpsr.v;

    let (result, flags) = match op {
        0b0000 => alu::and(op1, op2, c, v),
        0b0001 => alu::eor(op1, op2, c, v),
        0b0010 => alu::lsl(op1, op2, c, v),
        0b0011 => alu::lsr(op1, op2, c, v),
        0b0100 => alu::asr(op1, op2, c, v),
        0b0101 => alu::adc(op1, op2, c, v),
        0b0110 => alu::sbc(op1, op2, c, v),
        0b0111 => alu::ror(op1, op2, c, v),
        0b1000 => alu::tst(op1, op2, c, v),
        0b1001 => alu::neg(op1, op2, c, v),
        0b1010 => alu::cmp(op1, op2, c, v),
        0b1011 => alu::cmn(op1, op2, c, v),
        0b1100 => alu::orr(op1, op2, c, v),
        0b1101 => alu::mul(op1, op2, c, v),
        0b1110 => alu::bic(op1, op2, c, v),
        0b1111 => alu::mvn(op1, op2, c, v),
        _ => unreachable!(),
    };
    cpu.set_flags(flags);

    if op != 0b1000 && op != 0b1010 && op != 0b1011 {
        cpu.set_r(rd, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alu_operations() {
        let mut cpu = Cpu::new();

        // MUL 0x80000000, 0x10000000
        cpu.set_r(0, 0x10000000);
        cpu.set_r(1, 0x80000000);
        execute(&mut cpu, (0b1101, 0, 1));
        assert_eq!(cpu.r(1), 0);
        assert_eq!(cpu.cpsr.z, true);

        // NEG 0xf0f0f0f0
        cpu.set_r(1, 0xf0f0f0f0);
        execute(&mut cpu, (0b1001, 1, 1));
        assert_eq!(cpu.r(1), 0x0f0f0f0f + 1);
        assert_eq!(cpu.cpsr.z, false);

        // BIC 0x0f0f0f0f, 0xffffffff
        cpu.set_r(0, 0xffffffff);
        execute(&mut cpu, (0b1110, 0, 1));
        assert_eq!(cpu.r(1), 0);
        assert_eq!(cpu.cpsr.z, true);
    }
}
