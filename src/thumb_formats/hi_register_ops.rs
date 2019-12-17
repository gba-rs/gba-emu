pub struct HiRegisterOp {
    pub op: u8,
    pub hi_flag_1: bool,
    pub hi_flag_2: bool,
    pub source_register: u8,
    pub destination_register: u8,
}

impl From<u16> for HiRegisterOp {
    fn from(value: u16) -> HiRegisterOp {
        return HiRegisterOp {
            op: ((value & 0x300) >> 8) as u8,
            hi_flag_1: ((value & 0x80) >> 7) != 0,
            hi_flag_2: ((value & 0x40) >> 6) != 0,
            source_register: ((value & 0x38) >> 3) as u8,
            destination_register: (value & 0x7) as u8,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_0s() {
        let format = HiRegisterOp::from(0x4400);

        assert_eq!(format.op, 0);
        assert_eq!(format.hi_flag_1, false);
        assert_eq!(format.hi_flag_2, false);
        assert_eq!(format.source_register, 0);
        assert_eq!(format.destination_register, 0);
    }

    #[test]
    fn test_creation() {
        let format = HiRegisterOp::from(0x4754);

        assert_eq!(format.op, 3);
        assert_eq!(format.hi_flag_1, false);
        assert_eq!(format.hi_flag_2, true);
        assert_eq!(format.source_register, 2);
        assert_eq!(format.destination_register, 4);
    }

    #[test]
    fn test_creation_2() {
        let format = HiRegisterOp::from(0x46B5);

        assert_eq!(format.op, 2);
        assert_eq!(format.hi_flag_1, true);
        assert_eq!(format.hi_flag_2, false);
        assert_eq!(format.source_register, 6);
        assert_eq!(format.destination_register, 5);
    }
}
