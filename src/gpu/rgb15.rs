#[derive(Debug, Clone)]
pub struct Rgb15 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub value: u16
}

impl Rgb15 {
    pub fn new(value: u16) -> Rgb15 {
        return Rgb15 {
            red: (value & 0x1F) as u8,
            green: ((value >> 5) & 0x1F) as u8,
            blue: ((value >> 10) & 0x1F) as u8,
            value: value
        }
    }

    pub fn to_0rgb(&self) -> u32 {
        let (r, g, b) = (self.red as u32, self.green as u32, self.blue as u32);
        (r << 19) | (g << 11) | (b << 3)
    }

    pub fn is_transparent(&self) -> bool {
        return self.value == 0x8000;
    }
}