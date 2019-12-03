use crate::util::*;
use crate::cpu::CPU;
use crate::cpu::barrel_shifter::shift_register;
use crate::memory::Memory;

pub fn decode_execute(cpu: &mut CPU, memory: &mut Memory, instruction: u32)
{
    execute(cpu, memory, decode(instruction));
}

#[inline]
pub fn decode(instruction: u32) -> (bool, bool, bool, bool, u32, u32, u32, u32)
{
    debug_assert_eq!(bits(instruction, 27, 26), 0b01);

    let i      = bit(instruction, 25);
    let p      = bit(instruction, 24);
    let u      = bit(instruction, 23);
    let w      = bit(instruction, 21);
    let rn     = bits(instruction, 19, 16);
    let rd     = bits(instruction, 15, 12);
    let offset = bits(instruction, 11, 0);

    // Instruction is later on dispatched using the l, b bits
    let lb = (instruction >> 22 & 1) + (instruction >> 20 & 1) << 1;

    (i, p, u, w, lb, rn, rd, offset)
}

#[inline]
pub fn execute(cpu: &mut CPU, memory: &mut Memory, 
    (i, p, u, w, lb, rn, rd, offset): (bool, bool, bool, bool, u32, u32, u32, u32))
{
    let noffset = if i {offset} else {shift_register(cpu, offset)};

    let post = cpu.register.r[rn as usize];
    let pre = if u {cpu.register.r[rn as usize] + noffset} 
              else {cpu.register.r[rn as usize] - noffset};

    let address = if p {pre} else {post};
    let data = cpu.register.r[rd as usize];

    // Misaligned word access is not handled
    match lb
    {
        0b00 => memory.store32(address, data),
        0b01 => memory.store8(address, data as u8),
        0b10 => cpu.register.r[rd as usize] = memory.load32(address),
        0b11 => cpu.register.r[rd as usize] = memory.load8(address) as u32,
        _    => unreachable!()
    }

    // Privileged write back bit not handled
    if w || !p
    {
        cpu.register.r[rn as usize] = pre;
    }
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
        cpu.register.r[0] = 0x02000000;

        // Immediate offset, pre-indexing, up offset, write back, load byte
        execute(&mut cpu, &mut memory, (true, true, true, true, 0b11, 0, 1, 1));
        assert_eq!(cpu.register.r[1], 0xff);
        assert_eq!(cpu.register.r[0], 0x02000001);

        // Immediate offset, pre-indexing, down offset, no write back, store byte
        execute(&mut cpu, &mut memory, (true, true, false, false, 0b01, 0, 1, 1));
        assert_eq!(memory.load8(0x02000000), 0xff);
    }

    #[test]
    fn word_transfer()
    {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        memory.store32(0x02000000, 0xdeadbeef);
        cpu.register.r[0] = 0x02000000;

        // Immediate offset, post-indexing, up offset, write back, load word
        execute(&mut cpu, &mut memory, (true, false, true, true, 0b10, 0, 1, 4));
        assert_eq!(cpu.register.r[1], 0xdeadbeef);
        assert_eq!(cpu.register.r[0], 0x02000004);

        cpu.register.r[1] = 0;
        // Immediate offset, pre-indexing, down offset, write back, store word
        execute(&mut cpu, &mut memory, (true, true, false, true, 0b00, 0, 1, 4));
        assert_eq!(memory.load32(0x02000000), 0);
        assert_eq!(cpu.register.r[0], 0x02000000);
    }
}