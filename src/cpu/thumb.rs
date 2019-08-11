pub fn disassemble(opcode: u16) -> u8
{
    // decode the instructions using higher 5 bits
    let b15_11: u8 = (opcode >> 11) as u8;

    // use closures to inline calculation
    let offset5 = || -> u8 {(opcode >> 6 & 0b00011111) as u8};
    let offset8 = || -> u8 {opcode as u8};
    let b10_6 = || -> u8 {(opcode >> 6 & 0b00011111) as u8};
    let b10_9 = || -> u8 {(opcode >> 9 & 0b00000011) as u8};
    let rdb = || -> u8 {(opcode >> 8 & 0b00000111) as u8};
    let rn = || -> u8 {(opcode >> 6 & 0b00000111) as u8};
    let rs = || -> u8 {(opcode >> 3 & 0b00000111) as u8};
    let rd = || -> u8 {(opcode & 0b00000111) as u8};

    match b15_11 
    {
        // LSL Rd, Rs, #Offset5
        0b00000 =>
        {
            print!("LSL R{}, R{}, #{}", rd(), rs(), offset5());
        },
        // LSR Rd, Rs, #Offset5
        0b00001 =>
        {
            print!("LSL R{}, R{}, #{}", rd(), rs(), offset5());
        },
        // ASR Rd, Rs, #Offset5
        0b00010 =>
        {
            print!("ASR R{}, R{}, #{}", rd(), rs(), offset5());
        }
        // ADD/SUB
        0b00011 =>
        {
            match b10_9()
            {
                // ADD Rd, Rs, Rn
                0b00 =>
                {
                    print!("ADD R{}, R{}, R{}", rd(), rs(), rn());
                },
                // SUB Rd, Rs, Rn
                0b01 =>
                {
                    print!("SUB R{}, R{}, R{}", rd(), rs(), rn());
                },
                // ADD Rd, Rs, #Offset3
                0b10 =>
                {
                    print!("ADD R{}, R{}, #{}", rd(), rs(), rn());
                },
                // SUB Rd, Rs, #Offset3
                0b11 =>
                {
                    print!("SUB R{}, R{}, #{}", rd(), rs(), rn());
                },
                _    => (),
            }
        },
        // MOV Rd, #Offset8
        0b00100 =>
        {
            print!("MOV R{}, #{}", rdb(), offset8());
        },
        // CMP Rd, #Offset8
        0b00101 =>
        {
            print!("CMP R{}, #{}", rdb(), offset8());
        },
        // ADD Rd, #Offset8
        0b00110 =>
        {
            print!("ADD R{}, #{}", rdb(), offset8());
        },
        // SUB Rd, #Offset8
        0b00111 =>
        {
            print!("SUB R{}, #{}", rdb(), offset8());
        },
        // AluOp/HiReg/BX
        0b01000 =>
        {
            match b10_6()
            {
                // AND Rd, Rs
                0b00000 =>
                {
                    print!("ADD R{}, R{}", rd(), rs());
                },
                // EOR Rd, Rs
                0b00001 =>
                {
                    print!("EOR R{}, R{}", rd(), rs());
                },
                // LSL Rd, Rs
                0b00010 =>
                {
                    print!("LSL R{}, R{}", rd(), rs());
                },
                // LSR Rd, Rs
                0b00011 =>
                {
                    print!("LSR R{}, R{}", rd(), rs());
                },
                // ASR Rd, Rs
                0b00100 =>
                {
                    print!("ASR R{}, R{}", rd(), rs());
                },
                // ADC Rd, Rs
                0b00101 =>
                {
                    print!("ADC R{}, R{}", rd(), rs());
                },
                // SBC Rd, Rs
                0b00110 =>
                {
                    print!("SBC R{}, R{}", rd(), rs());
                },
                // ROR Rd, Rs
                0b00111 =>
                {
                    print!("ROR R{}, R{}", rd(), rs());
                },
                // TST Rd, Rs
                0b01000 =>
                {
                    print!("TST R{}, R{}", rd(), rs());
                },
                // NEG Rd, Rs
                0b01001 =>
                {
                    print!("NEG R{}, R{}", rd(), rs());
                },
                // CMP Rd, Rs
                0b01010 =>
                {
                    print!("CMP R{}, R{}", rd(), rs());
                },
                // CMN Rd, Rs
                0b01011 =>
                {
                    print!("CMN R{}, R{}", rd(), rs());
                },
                // ORR Rd, Rs
                0b01100 =>
                {
                    print!("ORR R{}, R{}", rd(), rs());
                },
                // MUL Rd, Rs
                0b01101 =>
                {
                    print!("MUL R{}, R{}", rd(), rs());
                },
                // BIC Rd, Rs
                0b01110 =>
                {
                    print!("BIC R{}, R{}", rd(), rs());
                },
                // MVN Rd, Rs
                0b01111 =>
                {
                    print!("MVN R{}, R{}", rd(), rs());
                },
                // ADD Rd, Hs 
                0b10001 =>
                {
                    print!("ADD R{}, R{}", rd(), rs());
                }, 
                // ADD Hd, Rs 
                0b10010 =>
                {
                    print!("ADD R{}, R{}", rd(), rs());
                }, 
                // ADD Hd, Hs 
                0b10011 =>
                {
                    print!("ADD R{}, R{}", rd(), rs());
                }, 
                // CMP Rd, Hs 
                0b10101 =>
                {
                    print!("CMP R{}, R{}", rd(), rs());
                }, 
                // CMP Hd, Rs 
                0b10110 =>
                {
                    print!("CMP R{}, R{}", rd(), rs());
                }, 
                // CMP Hd, Hs 
                0b10111 =>
                {
                    print!("CMP R{}, R{}", rd(), rs());
                }, 
                // MOV Rd, Hs 
                0b11001 =>
                {
                    print!("MOV R{}, R{}", rd(), rs());
                }, 
                // MOV Hd, Rs 
                0b11010 =>
                {
                    print!("MOV R{}, R{}", rd(), rs());
                }, 
                // MOV Hd, Hs 
                0b11011 =>
                {
                    print!("MOV R{}, R{}", rd(), rs());
                }, 
                // BX Rs
                0b11100 =>
                {
                    print!("BX R{}", rs());
                }, 
                //  BX Hs
                0b11101 =>
                {
                    print!("BX R{}", rs());
                }, 
                _       => (),
            }
        },
        0b01001 => print!("LDR PC"),
        0b01010 => print!("STR/STRB/STRH/LDSB"),
        0b01011 => print!("LDR/LDRB/LDRH/LDSH"),
        0b01100 => print!("STR imm"),
        0b01101 => print!("STRB imm"),
        0b01110 => print!("LDR imm"),
        0b01111 => print!("LDRB imm"),
        0b10000 => print!("STRH imm"),
        0b10001 => print!("LDRH imm"),
        0b10010 => print!("STR SP"),
        0b10011 => print!("LDR SP"),
        0b10100 => print!("ADD Rd, PC"),
        0b10101 => print!("ADD Rd, SP"),
        0b10110 => print!("ADD SP imm/PUSH"),
        0b10111 => print!("POP"),
        0b11000 => print!("STMIA"),
        0b11001 => print!("LDMIA"),
        0b11010 => print!("BEQ-BVC"),
        0b11011 => print!("BHI-BLE/SWI"),
        0b11100 => print!("B"),
        0b11110 => print!("BL-0"),
        0b11111 => print!("BL-1"),
        _       => print!("invalid opcode!"),
    }

    return 1;
}