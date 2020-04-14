#[derive(Debug)]
pub struct TileMapEntry {
    pub tile_index: u16,
    pub vertical_flip: bool,
    pub horizontal_flip: bool,
    pub palette_bank: u8
}

impl From<u16> for TileMapEntry {
    fn from(value: u16) -> TileMapEntry {
        return TileMapEntry {
            tile_index: (value & 0x3FF) as u16,
            vertical_flip: ((value >> 10) & 0x1) != 0,
            horizontal_flip: ((value >> 11) & 0x1) != 0,
            palette_bank: (value >> 12) as u8
        };
    }
}