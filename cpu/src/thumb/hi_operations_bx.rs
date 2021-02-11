use crate::alu;
use crate::CPU;
use util::*;

#[inline]
pub fn interpret(cpu: &mut CPU, instr: u16) {
    execute(cpu, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (u32, u32, u32) {
    let op = instr.bits(9, 8);

    // h1 and h2 are concatenated as sign bits to rd and rs.
    // rs = h2 || rs, rd = h1 || rd
    let rs = instr.bits(6, 3);
    let rd = instr.bits(2, 0) | (instr as u32 >> 4) & 0b1000;

    (op, rs, rd)
}

#[inline]
fn execute(cpu: &mut CPU, (op, rs, rd): (u32, u32, u32)) {
    let op1 = cpu.r(rd);
    let op2 = cpu.r(rs);
    let c = cpu.cpsr.c;
    let v = cpu.cpsr.v;

    match op {
        0b00 => cpu.set_r(rd, alu::add(op1, op2, c, v).0),
        0b01 => cpu.set_flags(alu::cmp(op1, op2, c, v).1),
        0b10 => cpu.set_r(rd, alu::mov(op1, op2, c, v).0),
        0b11 => {
            cpu.cpsr.t = cpu.r(rs).bit(0);
            cpu.set_r(15, cpu.r(rs));
        }
        _ => unreachable!(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alu_operations() {
        let mut cpu = CPU::new();

        // All operations except CMP does not effect CPSR flags.
        // ADD 2, 0xffffffff
        cpu.set_r(8, 0xffffffff);
        cpu.set_r(9, 2);
        execute(&mut cpu, (0b00, 8, 9));
        assert_eq!(cpu.r(9), 1);
        assert_eq!(cpu.cpsr.c, false);

        // CMP 1, 1
        cpu.set_r(8, 1);
        execute(&mut cpu, (0b01, 8, 9));
        assert_eq!(cpu.r(9), 1);
        assert_eq!(cpu.cpsr.z, true);

        // BX
        cpu.set_r(14, 0xfffffffb);
        execute(&mut cpu, (0b11, 14, 0));
        assert_eq!(cpu.r(15), 0xfffffffc);
    }
}
