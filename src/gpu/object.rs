use super::{gpu::{GPU, DISPLAY_WIDTH, DISPLAY_HEIGHT}, rgb15::Rgb15};
use crate::memory::{
    memory_map::MemoryMap, 
    lcd_io_registers::PixelFormat, 
    lcd_io_registers::ObjAttribute0,
    lcd_io_registers::ObjAttribute1,
    lcd_io_registers::ObjAttribute2,
    lcd_io_registers::OBJRotScaleParam
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Object {
    pub attr0: ObjAttribute0,
    pub attr1: ObjAttribute1,
    pub attr2: ObjAttribute2,
}

impl Object {
    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>){
        self.attr0.register(mem);
        self.attr1.register(mem);
        self.attr2.register(mem);
    }

    pub fn position(&self) -> (i32, i32) {
        return (self.attr1.get_x_coordinate() as i16 as i32, self.attr0.get_y_coordinate() as i16 as i32);
    }

    pub fn size(&self) -> (i32,i32){
        match (self.attr1.get_obj_size(), self.attr0.get_obj_shape()) {
            (0, 0)  => (8, 8),
            (1, 0)  => (16, 16),
            (2, 0)  => (32, 32),
            (3, 0)  => (64, 64),
            (0, 1)  => (16, 8),
            (1, 1)  => (32, 8),
            (2, 1)  => (32, 16),
            (3, 1)  => (64, 32),
            (0, 2)  => (8, 16),
            (1, 2)  => (8, 32),
            (2, 2)  => (16, 32),
            (3, 2)  => (32, 64),
            _ => (8, 8)
        }
    }

    pub fn color_format(&self) -> PixelFormat {
        if self.attr0.get_color_flag() == 0 {
            PixelFormat::FourBit
        } else {
            PixelFormat::EightBit
        }
    }

    pub fn tile_size(&self) -> i32 {
        if self.attr0.get_color_flag() == 0 {
            0x20
        } else {
            0x40
        }
    }
}

pub struct AffineMatrix {
    pub pa: OBJRotScaleParam,
    pub pb: OBJRotScaleParam,
    pub pc: OBJRotScaleParam,
    pub pd: OBJRotScaleParam
}

impl AffineMatrix {
    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>){
        self.pa.register(mem);
        self.pb.register(mem);
        self.pc.register(mem);
        self.pd.register(mem);
    }
}

impl GPU {
    pub fn render_obj(&mut self, mem_map: &mut MemoryMap) {
        for i in 0..128 {
            match self.objects[i].attr0.get_obj_mode() {
                0b10 => continue,
                0b00 => self.render_normal_obj(i, mem_map),
                0b01 | 0b11 => self.render_aff_obj(i, mem_map),
                _ => unreachable!() 
            };
        }
    }

    pub fn render_aff_obj(&mut self, sprite_num: usize, mem_map: &mut MemoryMap){
        let sprite = &mut self.objects[sprite_num];
        let current_scanline = self.vertical_count.get_current_scanline() as i32;
        let priority = sprite.attr2.get_priority_rel_to_bg();
        let (obj_x, obj_y) = sprite.position();
        let (obj_w, obj_h) = sprite.size();
        let gfx_mode = sprite.attr0.get_gfx_mode();

        let (bbox_w, bbox_h) = match sprite.attr0.get_obj_mode() {
            0b11 => (2 * obj_w, 2 * obj_h),
            _ => (obj_w, obj_h),
        };

        let mut ref_point_x = obj_x;
        let mut ref_point_y = obj_y;
        if obj_y >= (DISPLAY_HEIGHT as i32) {
            ref_point_y -= 1 << 8;
        }
        if obj_x >= (DISPLAY_WIDTH as i32) {
            ref_point_x -= 1 << 9;
        }

        if !(current_scanline >= ref_point_y && current_scanline < ref_point_y + bbox_h) {
            return;
        }

        let mode = self.display_control.get_bg_mode();

        let tile_index = sprite.attr2.get_character_name();
        let tile_base = (if mode > 2 { 0x06014000 } else { 0x06010000 }) + 0x20 * (tile_index as u32);

        let pixel_format = sprite.color_format();
        let tile_size = sprite.tile_size();

        let palette_bank = match pixel_format {
            PixelFormat::FourBit => sprite.attr2.get_palette_number() as u32,
            PixelFormat::EightBit => 0u32,
        };

        let screen_width = DISPLAY_WIDTH as i32;
        let tile_array_width = if self.display_control.get_obj_charcter_vram_mapping() == 0 {
            let temp = match pixel_format {
                PixelFormat::FourBit => 32,
                PixelFormat::EightBit => 16
            };
            temp
        } else {
            obj_w / 8
        };

        let aff_index = sprite.attr1.get_rotation_scaling_param();
        let aff_matrix = &self.aff_matrices[aff_index as usize];

        let half_width = bbox_w / 2;
        let half_height = bbox_h / 2;
        let iy = current_scanline - (ref_point_y + half_height);
        
        for ix in -half_width..half_width {
            let screen_x = ref_point_x + half_width + ix;
            if screen_x < 0 {
                continue;
            } 
            if screen_x >= screen_width {
                break;
            }

            let obj_buffer_index: usize = (DISPLAY_WIDTH * (current_scanline as u32) + (screen_x as u32)) as usize;
            if self.obj_buffer[obj_buffer_index].1 <= priority {
                continue;
            }

            let trans_x = (aff_matrix.pa.get_aff_param() as i16 as i32 * ix + aff_matrix.pb.get_aff_param() as i16 as i32 * iy) >> 8;
            let trans_y = (aff_matrix.pc.get_aff_param() as i16 as i32 * ix + aff_matrix.pd.get_aff_param() as i16 as i32 * iy) >> 8;
            let texture_x = trans_x + obj_w / 2;
            let texture_y = trans_y + obj_h / 2;
             if texture_x >= 0 && texture_x < obj_w && texture_y >= 0 && texture_y < obj_h {
                let tile_x = texture_x % 8;
                let tile_y = texture_y % 8;
                let tile_addr = tile_base + ((tile_array_width as u32) * ((texture_y as u32) / 8) + ((texture_x as u32) / 8)) * (tile_size as u32);
                let pixel_index = match pixel_format {
                    PixelFormat::EightBit => {
                        let pixel_index_address = tile_addr + (8 * (tile_y as u32) + (tile_x as u32));
                        mem_map.read_u8(pixel_index_address)
                    },
                    PixelFormat::FourBit => {
                        let pixel_index_address = tile_addr + (4 * (tile_y as u32) + ((tile_x as u32) / 2));
                        let value = mem_map.read_u8(pixel_index_address);
                        if tile_x & 1 != 0 {
                            value >> 4
                        } else {
                            value & 0xf
                        }
                    }
                } as u32;

                let color = if pixel_index == 0 || (palette_bank != 0 && pixel_index % 16 == 0) {
                    Rgb15::new(0x8000)
                } else {
                    let palette_ram_index = 0x200 + 2 * pixel_index + 0x20 * palette_bank;
                    Rgb15::new(mem_map.read_u16(palette_ram_index + 0x500_0000u32))
                };

                let obj_buffer_index: usize = (DISPLAY_WIDTH * (current_scanline as u32) + (screen_x as u32)) as usize;
                if !color.is_transparent() {
                    if gfx_mode == 0b10 {
                        let obj_window_index: usize = (DISPLAY_WIDTH * (current_scanline as u32) + (screen_x as u32)) as usize;
                        self.obj_window[obj_window_index] = true;
                        continue;
                    }
                    self.obj_buffer[obj_buffer_index] = (color, priority, gfx_mode);
                }
            }

        }

    }


    pub fn render_normal_obj(&mut self, sprite_num: usize, mem_map: &mut MemoryMap) {
        let sprite = &mut self.objects[sprite_num];
        let current_scanline = self.vertical_count.get_current_scanline() as i32;
        let (mut obj_x, mut obj_y) = sprite.position();
        let gfx_mode = sprite.attr0.get_gfx_mode();
        let priority = sprite.attr2.get_priority_rel_to_bg();

        if obj_y >= (DISPLAY_HEIGHT as i32) {
            obj_y -= 1 << 8;
        }
        if obj_x >= (DISPLAY_WIDTH as i32) {
            obj_x -= 1 << 9;
        }
        let (obj_w, obj_h) = sprite.size();

        if !(current_scanline >= obj_y && current_scanline < obj_y + obj_h) {
            return;
        }

        let mode = self.display_control.get_bg_mode();

        let tile_index = sprite.attr2.get_character_name();
        let tile_base = (if mode > 2 { 0x06014000 } else { 0x06010000 }) + 0x20 * (tile_index as u32);

        let pixel_format = sprite.color_format();
        let tile_size = sprite.tile_size();

        let palette_bank = match pixel_format {
            PixelFormat::FourBit => sprite.attr2.get_palette_number() as u32,
            PixelFormat::EightBit => 0u32,
        };

        let screen_width = DISPLAY_WIDTH as i32;
        let end_x = obj_x + obj_w;
        let tile_array_width = if self.display_control.get_obj_charcter_vram_mapping() == 0 {
            let temp = match pixel_format {
                PixelFormat::FourBit => 32,
                PixelFormat::EightBit => 16
            };
            temp
        } else {
            obj_w / 8
        };

        for x in obj_x..end_x {
            if x < 0 {
                continue;
            } 

            if x >= screen_width {
                break;
            }

            let obj_buffer_index: usize = (DISPLAY_WIDTH * (current_scanline as u32) + (x as u32)) as usize;
            if self.obj_buffer[obj_buffer_index].1 <= priority {
                continue;
            }
            
            let mut sprite_y = current_scanline - obj_y;
            let mut sprite_x = x - obj_x;

            sprite_y = if sprite.attr1.get_vertical_flip() != 0 {
                obj_h - sprite_y - 1
            } else {
                sprite_y
            };

            sprite_x = if sprite.attr1.get_horizontal_flip() != 0 {
                obj_w - sprite_x - 1
            } else {
                sprite_x
            };

            let tile_x = sprite_x % 8;
            let tile_y = sprite_y % 8;
            let tile_addr = tile_base + ((tile_array_width as u32) * ((sprite_y as u32) / 8) + ((sprite_x as u32) / 8)) * (tile_size as u32);
            let pixel_index = match pixel_format {
                PixelFormat::EightBit => {
                    let pixel_index_address = tile_addr + (8 * (tile_y as u32) + (tile_x as u32));
                    mem_map.read_u8(pixel_index_address)
                },
                PixelFormat::FourBit => {
                    let pixel_index_address = tile_addr + (4 * (tile_y as u32) + ((tile_x as u32) / 2));
                    let value = mem_map.read_u8(pixel_index_address);
                    if tile_x & 1 != 0 {
                        value >> 4
                    } else {
                        value & 0xf
                    }
                }
            } as u32;

            let color = if pixel_index == 0 || (palette_bank != 0 && pixel_index % 16 == 0) {
                Rgb15::new(0x8000)
            } else {
                let palette_ram_index = 0x200 + 2 * pixel_index + 0x20 * palette_bank;
                Rgb15::new(mem_map.read_u16(palette_ram_index + 0x500_0000u32))
            };

            let obj_buffer_index: usize = (DISPLAY_WIDTH * (current_scanline as u32) + (x as u32)) as usize;
            if !color.is_transparent() {
                if gfx_mode == 0b10 {
                    let obj_window_index: usize = (DISPLAY_WIDTH * (current_scanline as u32) + (x as u32)) as usize;
                    self.obj_window[obj_window_index] = true;
                    continue;
                }
                self.obj_buffer[obj_buffer_index] = (color, priority, gfx_mode);
            }
        }

    }
}