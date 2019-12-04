use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;

pub struct MoveShifted {
    pub op: u8,
    pub offset: u8,
    pub rs: u8,
    pub rd: u8,
}

impl From<u16> for MoveShifted {
    fn from(value: u16) -> MoveShifted {
        return MoveShifted {
            op: ((value & 0x1800) >> 11) as u8,
            offset: ((value & 0x7C0) >> 6) as u8,
            rs: ((value & 0x38) >> 3) as u8,
            rd: (value & 0x7) as u8,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let move_shifted = MoveShifted::from(0x1FFF);
        let move_shifted_1 = MoveShifted::from(0x15AA);

        assert_eq!(move_shifted.op, 0x3);
        assert_eq!(move_shifted.offset, 0x1F);
        assert_eq!(move_shifted.rs, 0x7);
        assert_eq!(move_shifted.rd, 0x7);

        assert_eq!(move_shifted_1.op, 0x3);
        assert_eq!(move_shifted_1.offset, 0x1F);
        assert_eq!(move_shifted_1.rs, 0x7);
        assert_eq!(move_shifted_1.rd, 0x7);
    }
}