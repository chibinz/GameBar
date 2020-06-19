use crate::util::*;
use crate::cpu::CPU;
use crate::memory::Memory;

#[inline]
pub fn interpret(cpu: &mut CPU, memory: &mut Memory, instruction: u32)
{
    execute(cpu, memory, decode(instruction));
}

#[inline]
pub fn decode(instruction: u32) -> (bool, bool, bool, bool, u32, u32, u32 ,u32)
{
    debug_assert_eq!(instruction.bits(27, 25), 0b000);
    debug_assert_eq!(instruction.bit(7), true);
    debug_assert_eq!(instruction.bit(4), true);

    let p = instruction.bit(24);
    let u = instruction.bit(23);
    // Offset can be a register or an immediate depending on bit 22
    let i = instruction.bit(22);
    let w = instruction.bit(21);
    let rn = instruction.bits(19, 16);
    let rd = instruction.bits(15, 12);

    // Instruction is later on dispatched by the l, s, h bits
    // Note that the shift operator takes precedence over add operator
    let lsh = ((instruction >> 20 & 1) << 2) + instruction.bits(6, 5);

    let offset = (instruction.bits(11, 8) << 4) + instruction.bits(3, 0);

    (p, u, i, w, lsh, rn, rd, offset)
}

#[inline]
pub fn execute(cpu: &mut CPU, memory: &mut Memory,
    (p, u, i, w, lsh, rn, rd, offset): (bool, bool, bool, bool, u32, u32, u32 ,u32))
{
    let noffset = if i {offset} else {cpu.r[offset as usize]};

    let post = cpu.r[rn as usize];
    let pre = if u {cpu.r[rn as usize] + noffset}
              else {cpu.r[rn as usize] - noffset};

    let address = if p {pre} else {post};

    // When R15 is the source register, the stored value will be
    // address of the instruction plus 12
    let value = cpu.r[rd as usize] + if rd == 15 {4} else {0};

    // Writeback may be overwritten if rn = rd
    if w || !p
    {
        cpu.r[rn as usize] = pre;
    }

    match lsh
    {
        0b001 => memory.store16(address, value as u16),
        0b010 => memory.store8(address, value as u8),
        0b011 => memory.store16(address, value as u16),
        0b101 => cpu.r[rd as usize] = memory.load16(address) as u32
                                    + if rd == 15 {4} else {0},
        0b110 => cpu.r[rd as usize] = memory.load8(address) as i8 as i32 as u32
                                    + if rd == 15 {4} else {0},
        0b111 => cpu.r[rd as usize] = memory.load16(address) as i16 as i32 as u32
                                    + if rd == 15 {4} else {0},
        _     => unreachable!()
    }

    // One internal cycle plus memory waitstate
    cpu.cycles += 1 + Memory::access_timing(address, 1);
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
        cpu.r[0] = 0x02000000;

        // Post-indexing, up offset, write back, load unsigned halfword
        // base = r0, dst = r1, offset = 2
        execute(&mut cpu, &mut memory, (false, true, true, true, 0b101, 0, 1, 2));
        assert_eq!(cpu.r[1], 0xdead);
        assert_eq!(cpu.r[0], 0x02000002);

        memory.store8(0x02000001, 0xff);
        // Pre-indexing, down offset, write back, load signed byte
        // base = r0, src = r1, offset = 1
        execute(&mut cpu, &mut memory, (true, false, true, true, 0b110, 0, 1, 1));
        assert_eq!(cpu.r[1], 0xffffffff);
        assert_eq!(cpu.r[0], 0x02000001);
    }
}
