use crate::{
    memory::{
        memory_map::{MemoryMap},
        lcd_io_registers::PixelFormat,
    },
    operations::bitutils
};
use super::{
    gpu::{GPU, DISPLAY_WIDTH}, 
    rgb15::Rgb15
};

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

impl GPU {
    pub fn render_bg(&mut self, mem_map: &mut MemoryMap, bg_number: usize) {
        let (vertical_offset, horizontal_offset) = self.backgrounds[bg_number].get_offsets();
        let tileset_location = self.backgrounds[bg_number].control.get_tileset_location();
        let tilemap_location = self.backgrounds[bg_number].control.get_tilemap_location();
        let (background_width, background_height) = self.backgrounds[bg_number].control.get_background_dimensions();

        let pixel_format = self.backgrounds[bg_number].control.get_pixel_format();
        let tile_size = self.backgrounds[bg_number].control.get_tilesize();

        let current_scanline = self.vertical_count.get_current_scanline() as u32;
        let mut x = 0;

        let background_x = (x + horizontal_offset) % background_width;
        let background_y = (current_scanline + vertical_offset) % background_height;

        let mut sbb: u32 = 0;
        if background_width == 256 && background_height == 256 {
            sbb = 0;
        } else if background_width == 512 && background_height == 256 {
            sbb = background_x / 256;
        } else if background_width == 256 && background_height == 512 {
            sbb = background_y / 256;
        } else if background_width == 512 && background_height == 512 {
            sbb = (2 * (background_y / 256) + (background_x / 256)) as u32;
        }

        let mut se_row = (background_x / 8) % 32;
        let se_column = (background_y / 8) % 32;

        let mut start_tile_x = background_x % 8;
        let tile_py = background_y % 8;

        loop {
            let mut map_address = tilemap_location + 0x800u32 * sbb + 2u32 * (32 * se_column + se_row);
            for _ in se_row..32 {
                let entry_value = TileMapEntry::from(mem_map.read_u16(map_address));
                let tile_address = tileset_location + (entry_value.tile_index as u32) * tile_size;

                for tile_px in start_tile_x..8 {
                    let pixel_x = if entry_value.vertical_flip { 7 - tile_px } else { tile_px };
                    let pixel_y = if entry_value.horizontal_flip { 7 - tile_py } else { tile_py };
                    let pixel_index = match pixel_format {
                        PixelFormat::EightBit => {
                            let pixel_index_address = tile_address + (8 * pixel_y + pixel_x);
                            mem_map.read_u8(pixel_index_address)
                        },
                        PixelFormat::FourBit => {
                            let pixel_index_address = tile_address + (4 * pixel_y + (pixel_x / 2));
                            let value = mem_map.read_u8(pixel_index_address);
                            if pixel_x & 1 != 0 {
                                value >> 4
                            } else {
                                value & 0xf
                            }
                        }
                    } as u32;

                    let palette_bank = match pixel_format {
                        PixelFormat::FourBit => entry_value.palette_bank as u32,
                        PixelFormat::EightBit => 0u32,
                    };

                    let color = if pixel_index == 0 || (palette_bank != 0 && pixel_index % 16 == 0) {
                        Rgb15::new(0x8000)
                    } else {
                        let palette_ram_index = 2 * pixel_index + 0x20 * palette_bank;
                        Rgb15::new(mem_map.read_u16(palette_ram_index + 0x500_0000u32))
                    };

                    self.backgrounds[bg_number].scan_line[x as usize] = color;
                    x += 1;
                    if DISPLAY_WIDTH == x {
                        return;
                    }
                }
                start_tile_x = 0;
                map_address += 2;
            }
            se_row = 0;
            if background_width == 512 {
                sbb = sbb ^ 1;
            }
        }
    }

    pub fn render_aff_bg(&mut self, mem_map: &mut MemoryMap, bg_number: usize) {
        let texture_size = 128 << self.backgrounds[bg_number].control.get_screen_size();

        let ref_point_x = bitutils::sign_extend_u32(self.bg_affine_components[bg_number - 2].refrence_point_x_internal, 27) as i32;
        let ref_point_y =  bitutils::sign_extend_u32(self.bg_affine_components[bg_number - 2].refrence_point_y_internal, 27) as i32;

        let pa = i32::from(&self.bg_affine_components[bg_number - 2].rotation_scaling_param_a);
        let pc = i32::from(&self.bg_affine_components[bg_number - 2].rotation_scaling_param_c);

        let screen_block = self.backgrounds[bg_number].control.get_tilemap_location();
        let char_block = self.backgrounds[bg_number].control.get_tileset_location();

        let wraparound = self.backgrounds[bg_number].control.get_display_area_overflow();

        for screen_x in 0..(DISPLAY_WIDTH as i32) {
            let mut point_x = (ref_point_x + screen_x * pa) >> 8;
            let mut point_y = (ref_point_y + screen_x * pc) >> 8;

            if !(point_x >= 0 && point_x < texture_size && point_y >= 0 && point_y < texture_size) {
                if wraparound != 0 {
                    point_x = point_x.rem_euclid(texture_size);
                    point_y = point_y.rem_euclid(texture_size);
                } else {
                    self.backgrounds[bg_number].scan_line[screen_x as usize] = Rgb15::new(0x8000);
                    continue;
                }
            }

            let map_addr = screen_block + ((texture_size as u32 / 8) * (point_y as u32 / 8) + (point_x as u32 / 8));
            let tile_index = mem_map.read_u8(map_addr) as u32;
            let tile_addr = char_block + tile_index * 0x40;

            let pixel_index_address = tile_addr + (8 * ((point_y % 8) as u32) + ((point_x % 8) as u32));
            let pixel_index = mem_map.read_u8(pixel_index_address) as u32;


            let color = if pixel_index == 0 {
                Rgb15::new(0x8000)
            } else {
                let palette_ram_index = 2 * pixel_index;
                Rgb15::new(mem_map.read_u16(palette_ram_index + 0x500_0000u32))
            };

            self.backgrounds[bg_number].scan_line[screen_x as usize] = color;
        }

    }
}