#[derive(Debug)]
pub struct PushPop {
    pub load: bool,
    pub store_lr: bool,
    pub register_list: Vec<u8>,
}

impl From<u16> for PushPop {
    fn from(value: u16) -> PushPop {
        let mut temp_reg_list: Vec<u8> = vec![];
        for i in 0..7 {
            if ((value >> i) & 0x01) != 0{
                temp_reg_list.push(i as u8);
            }
        }

        return PushPop {
            load: ((value & 0x800) >> 11) != 0,
            store_lr: ((value & 0x100) >> 8) != 0,
            register_list: temp_reg_list,
        };
    }
}