use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::register::PSRBit::C;
use crate::cpu::barrel_shifter::shift_register;
use crate::memory::Memory;

#[inline]
pub fn interpret(cpu: &mut CPU, memory: &mut Memory, instruction: u32)
{
    execute(cpu, memory, decode(instruction));
}

#[inline]
pub fn decode(instruction: u32) -> (bool, bool, bool, bool, u32, u32, u32, u32)
{
    debug_assert_eq!(instruction.bits(27, 26), 0b01);

    let i      = instruction.bit(25);
    let p      = instruction.bit(24);
    let u      = instruction.bit(23);
    let w      = instruction.bit(21);
    let rn     = instruction.bits(19, 16);
    let rd     = instruction.bits(15, 12);
    let offset = instruction.bits(11, 0);

    // Instruction is later on dispatched using the l, b bits
    let lb = (instruction >> 22 & 1) + ((instruction >> 20 & 1) << 1);

    (i, p, u, w, lb, rn, rd, offset)
}

#[inline]
pub fn execute(cpu: &mut CPU, memory: &mut Memory,
    (i, p, u, w, lb, rn, rd, offset): (bool, bool, bool, bool, u32, u32, u32, u32))
{
    // Shifts does not set CPSR C flag
    let carry = cpu.get_cpsr_bit(C);

    // 0 for i means immediate
    let noffset = if !i {offset} else {shift_register(cpu, offset)};
    cpu.set_cpsr_bit(C, carry);

    let post = cpu.r[rn as usize];
    let pre = if u {cpu.r[rn as usize].wrapping_add(noffset)}
              else {cpu.r[rn as usize].wrapping_sub(noffset)};

    let address = if p {pre} else {post};

    // When R15 is the source register, the stored value will be
    // address of the instruction plus 12
    let value = cpu.r[rd as usize] + if rd == 15 {4} else {0};

    // Privileged write back bit not handled
    if w || !p
    {
        cpu.r[rn as usize] = pre;
    }

    // Misaligned word access handled in `memory.rs`
    match lb
    {
        0b00 => memory.store32(address, value),
        0b01 => memory.store8(address, value as u8),
        0b10 => {cpu.r[rd as usize] = memory.load32(address);
                 if rd == 15 {cpu.flush()}},
        0b11 => {cpu.r[rd as usize] = memory.load8(address) as u32;
                 if rd == 15 {cpu.flush()}},
        _    => unreachable!()
    }

    cpu.cycles += 1 + Memory::access_timing(address, if lb.bit(0) {0} else {2});
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn byte_transfer()
    {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        memory.store8(0x02000001, 0xff);
        cpu.r[0] = 0x02000000;

        // Immediate offset, pre-indexing, up offset, write back, load byte
        execute(&mut cpu, &mut memory, (false, true, true, true, 0b11, 0, 1, 1));
        assert_eq!(cpu.r[1], 0xff);
        assert_eq!(cpu.r[0], 0x02000001);

        // Immediate offset, pre-indexing, down offset, no write back, store byte
        execute(&mut cpu, &mut memory, (false, true, false, false, 0b01, 0, 1, 1));
        assert_eq!(memory.load8(0x02000000), 0xff);
    }

    #[test]
    fn word_transfer()
    {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        memory.store32(0x02000000, 0xdeadbeef);
        cpu.r[0] = 0x02000000;

        // Immediate offset, post-indexing, up offset, write back, load word
        execute(&mut cpu, &mut memory, (false, false, true, true, 0b10, 0, 1, 4));
        assert_eq!(cpu.r[1], 0xdeadbeef);
        assert_eq!(cpu.r[0], 0x02000004);

        cpu.r[1] = 0;
        // Immediate offset, pre-indexing, down offset, write back, store word
        execute(&mut cpu, &mut memory, (false, true, false, true, 0b00, 0, 1, 4));
        assert_eq!(memory.load32(0x02000000), 0);
        assert_eq!(cpu.r[0], 0x02000000);
    }
}