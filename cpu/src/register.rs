use std::fmt::*;
use util::*;
use PsrMode::*;

#[derive(Clone, Copy)]
pub struct Cpsr {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,
    pub i: bool,
    pub f: bool,
    pub t: bool,
    pub mode: PsrMode,
}

impl Cpsr {
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

impl From<u32> for Cpsr {
    fn from(word: u32) -> Self {
        Self {
            n: word.bit(31),
            z: word.bit(30),
            c: word.bit(29),
            v: word.bit(28),
            i: word.bit(7),
            f: word.bit(6),
            t: word.bit(5),
            mode: PsrMode::from(word.bits(4, 0)),
        }
    }
}

impl From<Cpsr> for u32 {
    fn from(
        Cpsr {
            n,
            z,
            c,
            v,
            i,
            f,
            t,
            mode,
        }: Cpsr,
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

impl Debug for Cpsr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let raw: u32 = (*self).into();
        write!(
            f,
            "{:08x} [{}{}{}{}{}{}{}] {:?}",
            raw,
            if self.n { "N" } else { "." },
            if self.z { "Z" } else { "." },
            if self.c { "C" } else { "." },
            if self.v { "V" } else { "." },
            if self.i { "I" } else { "." },
            if self.f { "F" } else { "." },
            if self.t { "T" } else { "." },
            self.mode,
        )
    }
}

/// Operating Mode
#[derive(Clone, Copy, Debug)]
pub enum PsrMode {
    User = 0b10000,
    Fiq = 0b10001,
    Irq = 0b10010,
    Supervisor = 0b10011,
    Abort = 0b10111,
    Undefined = 0b11011,
    System = 0b11111,
}

impl From<u32> for PsrMode {
    fn from(mbits: u32) -> Self {
        match mbits {
            0b10000 => User,
            0b10001 => Fiq,
            0b10010 => Irq,
            0b10011 => Supervisor,
            0b10111 => Abort,
            0b11011 => Undefined,
            0b11111 => System,
            _ => panic!("Invalid PSR Mode: {:05b}!", mbits),
        }
    }
}

impl From<PsrMode> for u32 {
    fn from(mode: PsrMode) -> Self {
        mode as u32
    }
}

impl crate::Cpu {
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
            self.cpsr = Cpsr {
                n: r.bit(31),
                z: r.bit(30),
                c: r.bit(29),
                v: r.bit(28),
                ..self.cpsr
            };
        } else {
            // Save state and switch mode
            let psr: Cpsr = r.into();
            self.switch_mode(psr.mode);

            // Change control bits
            self.cpsr = psr;
        };
    }

    #[inline]
    pub fn switch_mode(&mut self, mode: PsrMode) {
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
            Fiq => {
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
            Irq => {
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

        if let Fiq = self.cpsr.mode {
            for i in 0..5 {
                self.bank[i + 7] = self.r[i + 8];
            }
        } else {
            for i in 0..5 {
                self.bank[i] = self.r[i + 8];
            }
        }
    }

    pub fn switch_bank(&mut self, mode: PsrMode) {
        match mode {
            User => {
                self.r[13] = self.bank[5];
                self.r[14] = self.bank[6];
            }
            System => {
                self.r[13] = self.bank[5];
                self.r[14] = self.bank[6];
            }
            Fiq => {
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
            Irq => {
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

        if let Fiq = mode {
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
        let psr: Cpsr = 0b11_1111.into();

        assert_eq!(psr.t, true);
    }

    #[test]
    fn set_cpsr_bit() {
        let mut psr = Cpsr::new();
        psr.f = true;

        assert_eq!(psr.bit(6), true);
    }

    #[test]
    fn check_condition() {
        let mut cpu = crate::Cpu::new();

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
