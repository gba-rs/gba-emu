#[derive(Debug)]
pub struct Rgb15 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub is_transparent: bool
}

impl Rgb15 {
    pub fn new(value: u16) -> Rgb15 {
        return Rgb15 {
            red: (value & 0x1F) as u8,
            green: ((value >> 5) & 0x1F) as u8,
            blue: ((value >> 10) & 0x1F) as u8,
            is_transparent: (value >> 15) != 0
        }
    }

    pub fn to_0rgb(&self) -> u32 {
        let (r, g, b) = (self.red as u32, self.green as u32, self.blue as u32);
        (r << 19) | (g << 11) | (b << 3)
    }
}