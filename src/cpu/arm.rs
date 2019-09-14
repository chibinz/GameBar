pub fn disassemble(opcode: u32) -> String
{
    // use bits 27 to 25 to decode opcode
    let b27_25 = opcode >> 25 & 0b00000111;

    // get fields of opcode
    let b24_20 = || opcode >> 20 & 0b00011111;
    let b74 = || opcode >> 6 & 0b10 | opcode >> 4 & 0b01;
    let b65 = || opcode >> 5 & 0b11;
    let rn   = || opcode >> 16 & 0b00001111;
    let rd   = || opcode >> 12 & 0b00001111;
    let rs   = || opcode >> 8 & 0b00001111;
    let rm   = || opcode & 0b00001111;

    // set condition bit
    let s    = || if opcode >> 20 & 1 == 1 {"S"} else {""};
    // byte bit
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
            format!("R{}, {} #{:#x}", rm(), stype, simmediate)
        }
    };


    // Data Processing / PSR Transfer / branch and exchange
    let data_process_psr_bx = || -> String
    {    

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
        // actually (opcode >> 8 & 0b00001111) << 1, immediate value is rotated twice by this field
        let rotate = opcode >> 7 & 0b00011110;
        let immediate = opcode & 0b11111111;

        match b24_20()
        {
            0b00000 | 0b00001 => format!("AND{}{} R{}, R{}, #{:#x}, #{:#x}", cond(), s(), rd(), rn(), immediate, rotate),
            0b00010 | 0b00011 => format!("EOR{}{} R{}, R{}, #{:#x}, #{:#x}", cond(), s(), rd(), rn(), immediate, rotate),
            0b00100 | 0b00101 => format!("SUB{}{} R{}, R{}, #{:#x}, #{:#x}", cond(), s(), rd(), rn(), immediate, rotate),
            0b00110 | 0b00111 => format!("RSB{}{} R{}, R{}, #{:#x}, #{:#x}", cond(), s(), rd(), rn(), immediate, rotate),
            0b01000 | 0b01001 => format!("ADD{}{} R{}, R{}, #{:#x}, #{:#x}", cond(), s(), rd(), rn(), immediate, rotate),
            0b01010 | 0b01011 => format!("ADC{}{} R{}, R{}, #{:#x}, #{:#x}", cond(), s(), rd(), rn(), immediate, rotate),
            0b01100 | 0b01101 => format!("SBC{}{} R{}, R{}, #{:#x}, #{:#x}", cond(), s(), rd(), rn(), immediate, rotate),
            0b01110 | 0b01111 => format!("RSC{}{} R{}, R{}, #{:#x}, #{:#x}", cond(), s(), rd(), rn(), immediate, rotate),
            0b10001           => format!("TST{}S R{}, #{:#x}, #{:#x}", cond(), rn(), immediate, rotate),
            0b10011           => format!("TEQ{}S R{}, #{:#x}, #{:#x}", cond(), rn(), immediate, rotate),
            0b10101           => format!("CMP{}S R{}, #{:#x}, #{:#x}", cond(), rn(), immediate, rotate),
            0b10111           => format!("CMN{}S R{}, #{:#x}, #{:#x}", cond(), rn(), immediate, rotate),
            0b11000 | 0b11001 => format!("ORR{}{} R{}, R{}, #{:#x}, #{:#x}", cond(), s(), rd(), rn(), immediate, rotate),
            0b11010 | 0b11011 => format!("MOV{}{} R{}, #{:#x}, #{:#x}", cond(), s(), rd(), immediate, rotate),
            0b11100 | 0b11101 => format!("BIC{}{} R{}, R{}, #{:#x}, #{:#x}", cond(), s(), rd(), rn(), immediate, rotate),
            0b11110 | 0b11111 => format!("MVN{}{} R{}, #{:#x}, #{:#x}", cond(), s(), rd(), immediate, rotate),
            
            _                 => format!("undefined"),
        }
    };

    // Multiply / Multiply Long / Single Data Swap
    let multiply_swap = || -> String
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
    
    // Halfword and Signed Data Transfer
    let halfword_signed_data_transfer = || -> String
    {
        // bit 24, 20, 6, 5 concatenated
        let field = opcode >> 21 & 0b1000 | opcode >> 18 & 0b0100 | b65();
        println!("{:b}", field);

        let offset =
        {
            let sign = if opcode >> 23 & 1 == 1 {""} else {"-"};
            if opcode >> 22 & 1 == 1
            {
                format!("#{}{:#x}", sign, opcode >> 4 & 0b11110000 | opcode & 0b00001111)
            }
            else
            {
                format!("{}R{}", sign, rm())
            }
        };
        // write back and pre-indexed
        let w = if opcode >> 21 & 1 == 1 && opcode >> 24 & 1 == 1 {"!"} else {""};

        match field
        {
            0b0001 => format!("STR{}H R{}, [R{}], {}", cond(), rd(), rn(), offset),
            0b0010 => format!("STR{}SB R{}, [R{}], {}", cond(), rd(), rn(), offset),
            0b0011 => format!("STR{}SH R{}, [R{}], {}", cond(), rd(), rn(), offset),
            0b0101 => format!("LDR{}H R{}, [R{}], {}", cond(), rd(), rn(), offset),
            0b0110 => format!("LDR{}SB R{}, [R{}],{}", cond(), rd(), rn(), offset),
            0b0111 => format!("LDR{}SH R{}, [R{}], {}", cond(), rd(), rn(), offset),
            0b1001 => format!("STR{}H R{}, [R{}, {}]{}", cond(), rd(), rn(), offset, w),
            0b1010 => format!("STR{}SB R{}, [R{}, {}]{}", cond(), rd(), rn(), offset, w),
            0b1011 => format!("STR{}SH R{}, [R{}, {}]{}", cond(), rd(), rn(), offset, w),
            0b1101 => format!("LDR{}H R{}, [R{}, {}]{}", cond(), rd(), rn(), offset, w),
            0b1110 => format!("LDR{}SB R{}, [R{}, {}]{}", cond(), rd(), rn(), offset, w),
            0b1111 => format!("LDR{}SH R{}, [R{}, {}]{}", cond(), rd(), rn(), offset, w),
            _      => format!("undefined"),
        }
    };

    // Single Data Transfer
    let single_data_transfer = || -> String
    {
        // concatenate bit
        let field = opcode >> 24 & 0b10 | opcode >> 20 & 0b01;
        let offset =
        {
            let sign = if opcode >> 23 & 1 == 1 {""} else {"-"};
            if opcode >> 25 & 1 == 0
            {
                format!("#{}{:#x}", sign, opcode & 0b0000111111111111)
            }
            else
            {
                format!("{}{}", sign, op2())
            }
        };
        
        let mut w = "";
        let mut t = "";
        // write back
        if opcode >> 21 & 1 == 1
        {
            // pre-indexed / post-indexed
            if opcode >> 24 & 1 == 1 {w = "!";} else {t = "T";}
        }

        match field
        {
            0b00 => format!("STR{}{}{} R{}, [R{}], {}", cond(), b(), t, rd(), rn(), offset),
            0b01 => format!("LDR{}{}{} R{}, [R{}], {}", cond(), b(), t, rd(), rn(), offset),
            0b10 => format!("STR{}{} R{}, [R{}, {}]{}", cond(), b(), rd(), rn(), offset, w),
            0b11 => format!("LDR{}{} R{}, [R{}, {}]{}", cond(), b(), rd(), rn(), offset, w),
            _    => format!("undefined"),
        }

    };

    // Branch / Branch with Link
    let branch = ||
    {
        let l = if opcode >> 24 & 1 == 1 {"L"} else {""};
        
        // sign extension is padding the most signficant bit
        let offset = if (opcode >> 23 & 1) == 1 
                       {((opcode & 0x00ffffff) << 2 | 0xfc000000) as i32} else
                       {((opcode & 0x00ffffff) << 2) as i32};

        format!("B{}{} #{:#x}", l, cond(), offset)
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
                    halfword_signed_data_transfer()
                }
                else
                {
                    multiply_swap()
                }
            }
        },
        0b001 => data_process_imm(),
        0b010 |
        0b011 => single_data_transfer(),
        0b100 => format!("Block Data Transfer"),
        0b101 => branch(),
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
        assert_eq!(disassemble(0b0000_00000000_0100_0010_0000_0000_0111), "ANDEQ R2, R4, R7, LSL #0x0");
        assert_eq!(disassemble(0b0000_00000011_1000_0000_0000_0111_1101), "EOREQS R0, R8, R13, ROR R0");
        assert_eq!(disassemble(0b0000_00011110_0010_0001_1111_0011_1110), "MVNEQ R1, R14, LSR R15");
        assert_eq!(disassemble(0b0000_00000000_0100_1010_1111_1100_1111), "ANDEQ R10, R4, R15, ASR #0x1f");
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

    #[test]
    fn branch()
    {
        assert_eq!(disassemble(0b0000_1011_10000000_00000000_00000000), "BLEQ #0xfe000000");
        assert_eq!(disassemble(0b0000_1010_00000000_00000000_00000001), "BEQ #0x4");
    }

    #[test]
    fn halfword_signed_data_transfer()
    {
        assert_eq!(disassemble(0b0000_00011111_0011_1100_1000_1011_0001), "LDREQH R12, [R3, #0x81]!");
        assert_eq!(disassemble(0b0000_00000010_0011_1100_1000_1111_0001), "STREQSH R12, [R3], -R1");
    }

    #[test]
    fn single_data_transfer()
    {
        assert_eq!(disassemble(0b0000_01111111_1010_0101_100000000000), "LDREQB R5, [R10, R0, LSL #0x10]!");
        assert_eq!(disassemble(0b0000_01000000_1010_0101_100000000000), "STREQ R5, [R10], #-0x800");
    }

}