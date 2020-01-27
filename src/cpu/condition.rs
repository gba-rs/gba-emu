use std::fmt;

#[derive(PartialEq)]
pub enum Condition {
    EQ = 0b0000,
    NE = 0b0001,
    CS = 0b0010,
    CC = 0b0011,
    MI = 0b0100,
    PL = 0b0101,
    VS = 0b0110,
    VC = 0b0111,
    HI = 0b1000,
    LS = 0b1001,
    GE = 0b1010,
    LT = 0b1011,
    GT = 0b1100,
    LE = 0b1101,
    AL = 0b1110,
    Error
}

impl fmt::Debug for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EQ => { write!(f, "EQ") },
            NE => { write!(f, "NE") },
            CS => { write!(f, "CS") },
            CC => { write!(f, "CC") },
            MI => { write!(f, "MI") },
            PL => { write!(f, "PL") },
            VS => { write!(f, "VS") },
            VC => { write!(f, "VC") },
            HI => { write!(f, "HI") },
            LS => { write!(f, "LS") },
            GE => { write!(f, "GE") },
            LT => { write!(f, "LT") },
            GT => { write!(f, "GT") },
            LE => { write!(f, "LE") },
            AL => { write!(f, "") },
            Error => { write!(f, "Error") }
        }
    }
}

impl From<u32> for Condition {
    fn from(value: u32) -> Condition {
        match value {
            0b0000 => return Condition::EQ,
            0b0001 => return Condition::NE,
            0b0010 => return Condition::CS,
            0b0011 => return Condition::CC,
            0b0100 => return Condition::MI,
            0b0101 => return Condition::PL,
            0b0110 => return Condition::VS,
            0b0111 => return Condition::VC,
            0b1000 => return Condition::HI,
            0b1001 => return Condition::LS,
            0b1010 => return Condition::GE,
            0b1011 => return Condition::LT,
            0b1100 => return Condition::GT,
            0b1101 => return Condition::LE,
            0b1110 => return Condition::AL,
            _ => return Condition::Error
        }
    }
}
