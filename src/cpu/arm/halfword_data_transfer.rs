use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

pub fn decode_execute(cpu: &mut CPU, memory: &mut Memory, instruction: u32)
{
    execute(cpu, memory, decode(instruction));
}

pub fn decode(instruction: u32) -> (bool, bool, bool, bool, u32, u32, u32 ,u32)
{
    debug_assert_eq!(bits(instruction, 27, 25), 0b000);
    debug_assert_eq!(bit(instruction, 7), true);
    debug_assert_eq!(bit(instruction, 4), true);

    let p = bit(instruction, 24);
    let u = bit(instruction, 23);
    let w = bit(instruction, 22);
    let rn = bits(instruction, 19, 16);
    let rd = bits(instruction, 15, 12);

    // Instruction is later on dispatched by the l, s, h bits
    // Note that the shift operator takes precedence over add operator
    let lsh = ((instruction >> 20 & 1) << 2) + bits(instruction, 6, 5);

    // Offset can be a register or an immediate depending on bit 22
    let i = bit(instruction, 22);
    let offset = (bits(instruction, 11, 8) << 4) + bits(instruction, 3, 0);

    (p, u, i, w, lsh, rn, rd, offset)
}

pub fn execute(cpu: &mut CPU, memory: &mut Memory, 
    (p, u, i, w, lsh, rn, rd, offset): (bool, bool, bool, bool, u32, u32, u32 ,u32))
{
    let noffset = if i {offset} else {cpu.register.r[offset as usize]};

    let post = cpu.register.r[rn as usize];
    let pre = if u {cpu.register.r[rn as usize] + noffset} 
              else {cpu.register.r[rn as usize] - noffset};

    let address = if p {pre} else {post};
    let data = cpu.register.r[rd as usize];

    match lsh
    {
        0b001 => memory.store16(address, data as u16),
        0b010 => memory.store8(address, data as u8),
        0b011 => memory.store16(address, data as u16),
        0b101 => cpu.register.r[rd as usize] = memory.load16(address) as u32,
        0b110 => cpu.register.r[rd as usize] = sign_extend(memory.load8(address) as u32, 7) as u32,
        0b111 => cpu.register.r[rd as usize] = sign_extend(memory.load16(address) as u32, 15) as u32,
        _     => unreachable!()
    }

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
    fn decode_strh()
    {
        let instruction = 0b0000_000_11111_1010_0101_1111_1011_1111;
        assert_eq!(decode(instruction), (true, true, true, true, 0b101, 0b1010, 0b0101, 0b11111111));
    }

    #[test]
    fn halfword_transfer()
    {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        memory.store16(0x02000000, 0xdead);
        cpu.register.r[0] = 0x02000000;

        // Post-indexing, up offset, write back, load unsigned halfword
        // base = r0, dst = r1, offset = 2
        execute(&mut cpu, &mut memory, (false, true, true, true, 0b101, 0, 1, 2));
        assert_eq!(cpu.register.r[1], 0xdead);
        assert_eq!(cpu.register.r[0], 0x02000002);

        memory.store8(0x02000001, 0xff);
        // Pre-indexing, down offset, write back, load signed byte
        // base = r0, src = r1, offset = 1
        execute(&mut cpu, &mut memory, (true, false, true, true, 0b110, 0, 1, 1));
        assert_eq!(cpu.register.r[1], 0xffffffff);
        assert_eq!(cpu.register.r[0], 0x02000001);
    }
}
