use crate::CPU;

use util::*;

use PSRMode::*;

#[derive(Clone, Copy)]
pub struct CPSR {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,
    pub i: bool,
    pub f: bool,
    pub t: bool,
    pub mode: PSRMode,
}

impl CPSR {
    pub const fn new() -> Self {
        Self {
            n: false,
            z: false,
            c: false,
            v: false,
            i: true,
            f: true,
            t: false,
            mode: Supervisor,
        }
    }
}

impl From<u32> for CPSR {
    fn from(word: u32) -> Self {
        Self {
            n: word.bit(31),
            z: word.bit(30),
            c: word.bit(29),
            v: word.bit(28),
            i: word.bit(7),
            f: word.bit(6),
            t: word.bit(5),
            mode: PSRMode::from(word.bits(4, 0)),
        }
    }
}

impl From<CPSR> for u32 {
    fn from(
        CPSR {
            n,
            z,
            c,
            v,
            i,
            f,
            t,
            mode,
        }: CPSR,
    ) -> Self {
        (n as u32) << 31
            | (z as u32) << 30
            | (c as u32) << 29
            | (v as u32) << 28
            | (i as u32) << 7
            | (f as u32) << 6
            | (t as u32) << 5
            | (mode as u32)
    }
}

/// Operating Mode
#[derive(Clone, Copy)]
pub enum PSRMode {
    User = 0b10000,
    FIQ = 0b10001,
    IRQ = 0b10010,
    Supervisor = 0b10011,
    Abort = 0b10111,
    Undefined = 0b11011,
    System = 0b11111,
}

impl From<u32> for PSRMode {
    fn from(mbits: u32) -> Self {
        match mbits {
            0b10000 => User,
            0b10001 => FIQ,
            0b10010 => IRQ,
            0b10011 => Supervisor,
            0b10111 => Abort,
            0b11011 => Undefined,
            0b11111 => System,
            _ => panic!("Invalid PSR Mode: {:05b}!", mbits),
        }
    }
}

impl From<PSRMode> for u32 {
    fn from(mode: PSRMode) -> Self {
        mode as u32
    }
}

impl CPU {
    #[inline]
    pub fn get_cpsr(&self) -> u32 {
        self.cpsr.into()
    }

    /// Set defined bits of CPSR.
    /// If parameter f is set, transfer only the flag bits.
    /// Reserved bits of CPSR are kept intact.
    #[inline]
    pub fn set_cpsr(&mut self, r: u32, f: bool) {
        if f {
            // Set condition code flags only
            self.cpsr = CPSR {
                n: r.bit(31),
                z: r.bit(30),
                c: r.bit(29),
                v: r.bit(28),
                ..self.cpsr
            };
        } else {
            // Save state and switch mode
            let psr: CPSR = r.into();
            self.switch_mode(psr.mode);

            // Change control bits
            self.cpsr = psr;
        };
    }

    #[inline]
    pub fn switch_mode(&mut self, mode: PSRMode) {
        self.save_state();
        self.switch_bank(mode);

        self.cpsr = (mode as u32).into();
    }

    #[inline]
    pub fn restore_cpsr(&mut self) {
        self.set_cpsr(self.spsr, false);
    }

    /// Get SPSR of current mode
    #[inline]
    pub fn get_spsr(&mut self) -> u32 {
        self.spsr
    }

    /// Set defined bits of SPSR of current mode.
    /// If parameter f is set, transfer only the flag bits.
    /// Reserved bits of SPSR are kept intact.
    #[inline]
    pub fn set_spsr(&mut self, r: u32, f: bool) {
        let mask = if f { 0xf0000000 } else { 0xf00000ff };

        self.spsr &= !mask;
        self.spsr |= r & mask;
    }

    #[inline]
    pub fn save_state(&mut self) {
        match self.cpsr.mode {
            User => {
                self.bank[5] = self.r[13];
                self.bank[6] = self.r[14]
            }
            System => {
                self.bank[5] = self.r[13];
                self.bank[6] = self.r[14]
            }
            FIQ => {
                self.bank[12] = self.r[13];
                self.bank[13] = self.r[14];
                self.bank[14] = self.spsr
            }
            Supervisor => {
                self.bank[15] = self.r[13];
                self.bank[16] = self.r[14];
                self.bank[17] = self.spsr
            }
            Abort => {
                self.bank[18] = self.r[13];
                self.bank[19] = self.r[14];
                self.bank[20] = self.spsr
            }
            IRQ => {
                self.bank[21] = self.r[13];
                self.bank[22] = self.r[14];
                self.bank[23] = self.spsr
            }
            Undefined => {
                self.bank[24] = self.r[13];
                self.bank[25] = self.r[14];
                self.bank[26] = self.spsr
            }
        };

        if let FIQ = self.cpsr.mode {
            for i in 0..5 {
                self.bank[i + 7] = self.r[i + 8];
            }
        } else {
            for i in 0..5 {
                self.bank[i] = self.r[i + 8];
            }
        }
    }

    pub fn switch_bank(&mut self, mode: PSRMode) {
        match mode {
            User => {
                self.r[13] = self.bank[5];
                self.r[14] = self.bank[6];
            }
            System => {
                self.r[13] = self.bank[5];
                self.r[14] = self.bank[6];
            }
            FIQ => {
                self.r[13] = self.bank[12];
                self.r[14] = self.bank[13];
                self.spsr = self.bank[14];
            }
            Supervisor => {
                self.r[13] = self.bank[15];
                self.r[14] = self.bank[16];
                self.spsr = self.bank[17];
            }
            Abort => {
                self.r[13] = self.bank[18];
                self.r[14] = self.bank[19];
                self.spsr = self.bank[20];
            }
            IRQ => {
                self.r[13] = self.bank[21];
                self.r[14] = self.bank[22];
                self.spsr = self.bank[23];
            }
            Undefined => {
                self.r[13] = self.bank[24];
                self.r[14] = self.bank[25];
                self.spsr = self.bank[26];
            }
        };

        if let FIQ = mode {
            for i in 0..5 {
                self.r[i + 8] = self.bank[i + 7];
            }
        } else {
            for i in 0..5 {
                self.r[i + 8] = self.bank[i];
            }
        }
    }

    /// Return true if condition is satified
    pub fn check_condition(&self, condition: u32) -> bool {
        let cpsr = self.cpsr;
        match condition {
            0b0000 => cpsr.z,                        // EQ
            0b0001 => !cpsr.z,                       // NE
            0b0010 => cpsr.c,                        // CS
            0b0011 => !cpsr.c,                       // CC
            0b0100 => cpsr.n,                        // MI
            0b0101 => !cpsr.n,                       // PL
            0b0110 => cpsr.v,                        // VS
            0b0111 => !cpsr.v,                       // VC
            0b1000 => cpsr.c && !cpsr.z,             // HI
            0b1001 => !cpsr.c || cpsr.z,             // LS
            0b1010 => cpsr.n == cpsr.v,              // GE
            0b1011 => cpsr.n != cpsr.v,              // LT
            0b1100 => !cpsr.z && (cpsr.n == cpsr.v), // GT
            0b1101 => cpsr.z || (cpsr.n != cpsr.v),  // LE
            0b1110 => true,
            _ => panic!("Invalid Condition Field!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_cpsr_bit() {
        let mut cpu = CPU::new();
        cpu.cpsr = 0b1_11111.into();

        assert_eq!(cpu.cpsr.t, true);
    }

    #[test]
    fn set_cpsr_bit() {
        let mut cpu = CPU::new();
        cpu.cpsr.f = true;

        assert_eq!(cpu.cpsr.bit(6), true);
    }

    #[test]
    fn check_condition() {
        let mut cpu = CPU::new();

        cpu.cpsr.z = true;
        assert!(cpu.check_condition(0b0000));

        cpu.cpsr.c = true;
        assert!(cpu.check_condition(0b0010));

        cpu.cpsr.n = true;
        assert!(cpu.check_condition(0b0100));

        cpu.cpsr.v = true;
        assert!(cpu.check_condition(0b0110));

        assert!(cpu.check_condition(0b1010));

        cpu.cpsr.z = false;
        assert!(cpu.check_condition(0b1100));
    }
}
