#[derive(Debug)]
pub struct LoadStoreSignExtended {
    h_flag: bool,
    sign_extended: bool,
    offset_register: u8,
    base_register: u8,
    destination_register: u8,
}

impl From<u16> for LoadStoreSignExtended {
    fn from(value: u16) -> LoadStoreSignExtended {
        return LoadStoreSignExtended {
            h_flag: ((value & 0x800) >> 11) != 0,
            sign_extended: ((value & 0x400) >> 10) != 0,
            offset_register: ((value & 0x1C0) >> 6) as u8,
            base_register: ((value & 0x38) >> 3) as u8,
            destination_register: (value & 0x7) as u8
        };
    }
}