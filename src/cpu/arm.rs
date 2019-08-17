pub fn disassemble(opcode: u32) -> String
{
    /*
        ARM Binary Opcode Format
        |..3 ..................2 ..................1 ..................0|
        |1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0_9_8_7_6_5_4_3_2_1_0|
        |_Cond__|0_0_0|___Op__|S|__Rn___|__Rd___|__Shift__|Typ|0|__Rm___| DataProc
        |_Cond__|0_0_0|___Op__|S|__Rn___|__Rd___|__Rs___|0|Typ|1|__Rm___| DataProc
        |_Cond__|0_0_1|___Op__|S|__Rn___|__Rd___|_Shift_|___Immediate___| DataProc
        |_Cond__|0_0_1_1_0|P|1|0|_Field_|__Rd___|_Shift_|___Immediate___| PSR Imm
        |_Cond__|0_0_0_1_0|P|L|0|_Field_|__Rd___|0_0_0_0|0_0_0_0|__Rm___| PSR Reg
        |_Cond__|0_0_0_0_0_0|A|S|__Rd___|__Rn___|__Rs___|1_0_0_1|__Rm___| Multiply
        |_Cond__|0_0_0_0_1|U|A|S|_RdHi__|_RdLo__|__Rs___|1_0_0_1|__Rm___| MulLong
        |_Cond__|0_0_0_1_0|B|0_0|__Rn___|__Rd___|0_0_0_0|1_0_0_1|__Rm___| TransSwp12
        |_Cond__|0_0_0|P|U|0|W|L|__Rn___|__Rd___|0_0_0_0|1|S|H|1|__Rm___| TransReg10
        |_Cond__|0_0_0|P|U|1|W|L|__Rn___|__Rd___|OffsetH|1|S|H|1|OffsetL| TransImm10
        |_Cond__|0_1_0|P|U|B|W|L|__Rn___|__Rd___|_________Offset________| TransImm9
        |_Cond__|0_1_1|P|U|B|W|L|__Rn___|__Rd___|__Shift__|Typ|0|__Rm___| TransReg9
        |_Cond__|1_0_0|P|U|S|W|L|__Rn___|__________Register_List________| BlockTrans
        |_Cond__|0_0_0_1_0_0_1_0_1_1_1_1_1_1_1_1_1_1_1_1|0_0|L|1|__Rn___| BX,BLX
        |_Cond__|1_0_1|L|___________________Offset______________________| B,BL,BLX
        |_Cond__|1_1_1_1|_____________Ignored_by_Processor______________| SWI
    */


    // use bits 27 to 25 to decode opcode
    let b27_25 = opcode >> 25 & 0b00000111;

    // let bit = |start: u32, end: u32| opcode >> end & (1 << start - end + 1) - 1;
    // assert_eq!(bit(27, 25), b27_25);

    // get fields of opcode
    let b24_20 = || opcode >> 20 & 0b00011111;
    let b74 = || opcode >> 6 & 0b10 | opcode >> 4 & 0b01;
    let b65 = || opcode >> 5 & 0b11;
    let rn   = || opcode >> 16 & 0b00001111;
    let rd   = || opcode >> 12 & 0b00001111;
    let rs   = || opcode >> 8 & 0b00001111;
    let rm   = || opcode & 0b00001111;

    // helper functions that return strings
    let s    = || if opcode >> 20 & 1 == 1 {"S"} else {""};
    let b    = || if opcode >> 22 & 1 == 1 {"B"} else {""};
    let cond = ||
    {
        match opcode >> 28 & 0b00001111
        {
            0b0000 => "EQ",
            0b0001 => "NE",
            0b0010 => "CS",
            0b0011 => "CC",
            0b0100 => "MI",
            0b0101 => "PL",
            0b0110 => "VS",
            0b0111 => "VC",
            0b1000 => "HI",
            0b1001 => "LS",
            0b1010 => "GE",
            0b1011 => "LT",
            0b1100 => "GT",
            0b1101 => "LE",
            0b1110 => "",
            _      => "undefined",
        }
    };


    // Data Processing / PSR Transfer / branch and exchange
    let data_process_psr_bx = || -> String
    {    
        let op2  = ||
        {
            let stype = match b65()
            {
                0b00 => "LSL",
                0b01 => "LSR",
                0b10 => "ASR",
                0b11 => "ROR",
                _    => "undefined",
            };

            if b74() == 0b01
            {
                format!("R{}, {} R{}", rm(), stype, rs())
            }
            else
            {
                let simmediate = opcode >> 7 & 0b00011111;
                format!("R{}, {} #{}", rm(), stype, simmediate)
            }
        };

        match b24_20()
        {
            0b00000 | 0b00001 => format!("AND{}{} R{}, R{}, {}", cond(), s(), rd(), rn(), op2()),
            0b00010 | 0b00011 => format!("EOR{}{} R{}, R{}, {}", cond(), s(), rd(), rn(), op2()),
            0b00100 | 0b00101 => format!("SUB{}{} R{}, R{}, {}", cond(), s(), rd(), rn(), op2()),
            0b00110 | 0b00111 => format!("RSB{}{} R{}, R{}, {}", cond(), s(), rd(), rn(), op2()),
            0b01000 | 0b01001 => format!("ADD{}{} R{}, R{}, {}", cond(), s(), rd(), rn(), op2()),
            0b01010 | 0b01011 => format!("ADC{}{} R{}, R{}, {}", cond(), s(), rd(), rn(), op2()),
            0b01100 | 0b01101 => format!("SBC{}{} R{}, R{}, {}", cond(), s(), rd(), rn(), op2()),
            0b01110 | 0b01111 => format!("RSC{}{} R{}, R{}, {}", cond(), s(), rd(), rn(), op2()),
            0b10001           => format!("TST{}S R{}, {}", cond(), rn(), op2()),
            0b10011           => format!("TEQ{}S R{}, {}", cond(), rn(), op2()),
            0b10101           => format!("CMP{}S R{}, {}", cond(), rn(), op2()),
            0b10111           => format!("CMN{}S R{}, {}", cond(), rn(), op2()),
            0b11000 | 0b11001 => format!("ORR{}{} R{}, R{}, {}", cond(), s(), rd(), rn(), op2()),
            0b11010 | 0b11011 => format!("MOV{}{} R{}, {}", cond(), s(), rd(), op2()),
            0b11100 | 0b11101 => format!("BIC{}{} R{}, R{}, {}", cond(), s(), rd(), rn(), op2()),
            0b11110 | 0b11111 => format!("MVN{}{} R{}, {}", cond(), s(), rd(), op2()),
            
            0b10000           => format!("MRS{} R{}, CPSR", cond(), rd()),
            0b10100           => format!("MRS{} R{}, SPSR", cond(), rd()),
            0b10110           => format!("MSR{} SPSR, R{}", cond(), rm()),
            0b10010           => if b74() == 0 
                                {format!("MSR{} CPSR, R{}", cond(), rm())} else
                                {format!("BX{} R{}", cond(), rm())},

            _                 => format!("undefined"),
        }
    };

    // Data Process With 8-bit Immediate offset
    let data_process_imm = || -> String
    {    
        let shift = opcode >> 7 & 0b00011110;
        let immediate = opcode & 0b11111111;


        match b24_20()
        {
            0b00000 | 0b00001 => format!("AND{}{} R{}, R{}, #{}, #{}", cond(), s(), rd(), rn(), immediate, shift),
            0b00010 | 0b00011 => format!("EOR{}{} R{}, R{}, #{}, #{}", cond(), s(), rd(), rn(), immediate, shift),
            0b00100 | 0b00101 => format!("SUB{}{} R{}, R{}, #{}, #{}", cond(), s(), rd(), rn(), immediate, shift),
            0b00110 | 0b00111 => format!("RSB{}{} R{}, R{}, #{}, #{}", cond(), s(), rd(), rn(), immediate, shift),
            0b01000 | 0b01001 => format!("ADD{}{} R{}, R{}, #{}, #{}", cond(), s(), rd(), rn(), immediate, shift),
            0b01010 | 0b01011 => format!("ADC{}{} R{}, R{}, #{}, #{}", cond(), s(), rd(), rn(), immediate, shift),
            0b01100 | 0b01101 => format!("SBC{}{} R{}, R{}, #{}, #{}", cond(), s(), rd(), rn(), immediate, shift),
            0b01110 | 0b01111 => format!("RSC{}{} R{}, R{}, #{}, #{}", cond(), s(), rd(), rn(), immediate, shift),
            0b10001           => format!("TST{}S R{}, #{}, #{}", cond(), rn(), immediate, shift),
            0b10011           => format!("TEQ{}S R{}, #{}, #{}", cond(), rn(), immediate, shift),
            0b10101           => format!("CMP{}S R{}, #{}, #{}", cond(), rn(), immediate, shift),
            0b10111           => format!("CMN{}S R{}, #{}, #{}", cond(), rn(), immediate, shift),
            0b11000 | 0b11001 => format!("ORR{}{} R{}, R{}, #{}, #{}", cond(), s(), rd(), rn(), immediate, shift),
            0b11010 | 0b11011 => format!("MOV{}{} R{}, #{}, #{}", cond(), s(), rd(), immediate, shift),
            0b11100 | 0b11101 => format!("BIC{}{} R{}, R{}, #{}, #{}", cond(), s(), rd(), rn(), immediate, shift),
            0b11110 | 0b11111 => format!("MVN{}{} R{}, #{}, #{}", cond(), s(), rd(), immediate, shift),
            
            _                 => format!("undefined"),
        }
    };

    // Multiply / Multiply Long / Single Data Swap
    let multiply_swap = ||
    {
        let hi = || opcode >> 16 & 0b00001111;
        let lo = || opcode >> 12 & 0b00001111;

        match b24_20()
        {
            0b00000 | 0b00001 => format!("MUL{}{} R{}, R{}, R{}", cond(), s(), rn(), rm(), rs()),
            0b00010 | 0b00011 => format!("MLA{}{} R{}, R{}, R{}", cond(), s(), rn(), rm(), rs()),
            0b01000 | 0b01001 => format!("UMULL{}{} R{}, R{}, R{}, R{}", cond(), s(), hi(), lo(), rm(), rs()),
            0b01010 | 0b01011 => format!("UMLAL{}{} R{}, R{}, R{}, R{}", cond(), s(), hi(), lo(), rm(), rs()),
            0b01100 | 0b01101 => format!("SMULL{}{} R{}, R{}, R{}, R{}", cond(), s(), hi(), lo(), rm(), rs()),
            0b01110 | 0b01111 => format!("SMLAL{}{} R{}, R{}, R{}, R{}", cond(), s(), hi(), lo(), rm(), rs()),

            0b10000 | 0b10100 => format!("SWP{}{} R{}, R{}, [R{}]", cond(), b(), rd(), rm(), rn()),

            _                 => format!("undefined"),
        }
    };

    match b27_25
    {
        0b000 =>
        {
            if b74() < 0b11
            {
                data_process_psr_bx()
            }
            else
            {
                if b65() > 0
                {
                    format!("Halfword Data Transfer")
                }
                else
                {
                    multiply_swap()
                }
            }
        },
        0b001 => data_process_imm(),
        0b010 => format!("Single Data Transfer"),
        0b011 => format!("Single Data Transfer / undefined"),
        0b100 => format!("Block Data Transfer"),
        0b101 => format!("Branch"),
        0b110 => format!("Coprocessor"),
        0b111 => format!("SWI / Coprocessor"),
        _     => format!("undefined"),
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn data_process_psr_bx()
    {
        assert_eq!(disassemble(0b0000_00000000_0100_0010_0000_0000_0111), "ANDEQ R2, R4, R7, LSL #0");
        assert_eq!(disassemble(0b0000_00000011_1000_0000_0000_0111_1101), "EOREQS R0, R8, R13, ROR R0");
        assert_eq!(disassemble(0b0000_00011110_0010_0001_1111_0011_1110), "MVNEQ R1, R14, LSR R15");
        assert_eq!(disassemble(0b0000_00000000_0100_1010_1111_1100_1111), "ANDEQ R10, R4, R15, ASR #31");
        assert_eq!(disassemble(0b1110_00010000_1111_1111_0000_0000_0000), "MRS R15, CPSR");
        assert_eq!(disassemble(0b1000_00010010_1000_1111_0000_0000_0000), "MSRHI CPSR, R0");
        assert_eq!(disassemble(0b0001_00010010_1111_1111_1111_0001_1000), "BXNE R8");
    }

    #[test]
    fn multiply_swap()
    {
        assert_eq!(disassemble(0b1110_00000011_0001_1000_0000_1001_0100), "MLAS R1, R4, R0");
        assert_eq!(disassemble(0b1110_00001111_0010_0011_0000_1001_0100), "SMLALS R2, R3, R4, R0");
        assert_eq!(disassemble(0b1110_00010100_0001_1000_0000_1001_0100), "SWPB R8, R4, [R1]");
    }

}