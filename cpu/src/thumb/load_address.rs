use crate::Cpu;
use util::*;

#[inline]
pub fn interpret(cpu: &mut Cpu, instr: u16) {
    execute(cpu, decode(instr));
}

#[inline]
fn decode(instr: u16) -> (bool, u32, u32) {
    let sp = instr.bit(11);
    let rd = instr.bits(10, 8);
    let word8 = instr.bits(7, 0);

    (sp, rd, word8)
}

#[inline]
fn execute(cpu: &mut Cpu, (sp, rd, word8): (bool, u32, u32)) {
    if sp {
        cpu.set_r(rd, cpu.r(13) + (word8 << 2));
    } else {
        // Bit 1 of PC is forced to 0.
        // The value of the PC will be 4 bytes greater than the address
        // of the instruction before bit 1 is forced to 0.
        cpu.set_r(rd, (cpu.r(15) & 0xfffffffc) + (word8 << 2));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_address() {
        let mut cpu = Cpu::new();

        cpu.set_r(13, 0xffffff00);
        execute(&mut cpu, (true, 0, 0b00111111));
        assert_eq!(cpu.r(0), 0xfffffffc);
    }
}
