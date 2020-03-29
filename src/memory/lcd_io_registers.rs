#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::cell::RefCell;
use std::rc::Rc;
use crate::operations::bitutils::*;
use super::GbaMem;
use memory_macros::*;

#[repr(u8)]
#[derive(Debug)]
pub enum PixelFormat {
    FourBit,
    EightBit
}

io_register! (
    DisplayControl => 2, 0x4000000,
    bg_mode: 0, 3,
    cgb_mode: 3, 1,
    display_frame_select: 4, 1,
    hblank_interval_free: 5, 1,
    obj_charcter_vram_mapping: 6, 1,
    forced_blank: 7, 1,
    screen_display_bg0: 8, 1,
    screen_display_bg1: 9, 1,
    screen_display_bg2: 10, 1,
    screen_display_bg3: 11, 1,
    screen_display_obj: 12, 1,
    window_0_display_flag: 13, 1,
    window_1_display_flag: 14, 1,
    obj_window_display_flag: 15, 1
);

impl DisplayControl {
    pub fn should_display(&self, bg_num: u8) -> bool {
        return bg_num == 0 && self.get_screen_display_bg0() == 1 ||
               bg_num == 1 && self.get_screen_display_bg1() == 1 ||
               bg_num == 2 && self.get_screen_display_bg2() == 1 ||
               bg_num == 3 && self.get_screen_display_bg3() == 1;
    }
}

io_register! (
    GreenSwap => 2, 0x4000002,
    green_swap: 0, 1
);

io_register! (
    DisplayStatus => 2, 0x4000004,
    vblank_flag: 0, 1,
    hblank_flag: 1, 1,
    vcounter_flag: 2, 1,
    vblank_irq_enable: 3, 1,
    hblank_irq_enable: 4, 1,
    vcounter_irq_enable: 5, 1,
    vcount_setting: 8, 8,
);

io_register! (
    VerticalCount => 2, 0x4000006,
    current_scanline: 0, 8
);

io_register! (
    BG_Control => 2, [0x4000008, 0x400000A, 0x400000C, 0x400000E],
    bg_priority: 0, 2,
    character_base_block: 2, 2,
    mosaic: 6, 1,
    colors: 7, 1,
    screen_base_block: 8, 5,
    display_area_overflow: 13, 1,
    screen_size: 14, 2,
);

impl BG_Control {
    pub fn get_tileset_location(&self) -> u32 {
        return 0x600_0000 + (self.get_character_base_block() as u32) * 0x4000;
    }

    pub fn get_tilemap_location(&self) -> u32 {
        return 0x600_0000 + (self.get_screen_base_block() as u32) * 0x800;
    }

    // pub fn get_background_
    pub fn get_background_dimensions(&self) -> (u32, u32) {
        let bg_size_number = self.get_screen_size() as u32;

        match bg_size_number {
            0 => (256, 256),
            1 => (512, 256),
            2 => (256, 512),
            3 => (512, 512),
            _ => panic!("Invalid screen size")
        }
    }

    pub fn get_pixel_format(&self) -> PixelFormat {
        if self.get_colors() != 0 {
            PixelFormat::EightBit
        } else {
            PixelFormat::FourBit
        }
    }

    pub fn get_tilesize(&self) -> u32 {
        if self.get_colors() != 0 { 64 } else { 32 }
    }
}

io_register! (
    BGOffset => 2, [0x4000010, 0x4000012, 0x4000014, 0x4000016, 0x4000018, 0x400001A, 0x400001C, 0x400001E],
    offset: 0, 9,
);

io_register! (
    BGRefrencePoint => 4, [0x4000028, 0x400002C, 0x4000038, 0x400003C],
    fractional_portion: 0, 8,
    integer_portion: 8, 19,
    sign: 27, 1
);

impl From<&BGRefrencePoint> for i32 {
    fn from(value: &BGRefrencePoint) -> i32 {
        return sign_extend_u32(value.get_register(), 27) as i32;
    }
}

io_register! (
    BGRotScaleParam => 2, [0x4000020, 0x4000022, 0x4000024, 0x4000026, 0x4000030, 0x4000032, 0x4000034, 0x4000036],
    fractional_portion: 0, 8,
    integer_portion: 8, 7,
    sign: 15, 1,
);

impl From<&BGRotScaleParam> for i32 {
    fn from(value: &BGRotScaleParam) -> i32 {
        return sign_extend_u32(value.get_register() as u32, 16) as i32;
    }
}

io_register! (
    WindowHorizontalDimension => 2, [0x4000040, 0x4000042],
    X2: 0, 8,
    X1: 8, 8
);

io_register! (
    WindowVerticalDimension => 2, [0x4000044, 0x4000046],
    Y2: 0, 8,
    Y1: 8, 8,
);

io_register! (
    ControlWindowInside => 2, 0x4000048,
    window0_bg_enable_bits: 0, 4,
    window0_obj_enable_bits: 4, 1,
    window0_color_special_effect: 5, 1,
    window1_bg_enable_bits: 8, 4,
    window1_obj_enable_bits: 12, 1,
    window1_color_special_effect: 13, 1,
);

io_register! (
    ControlWindowOutside => 2, 0x400004A,
    outside_bg_enable_bits: 0, 4,
    outside_obj_enable_bits: 4, 1,
    outside_color_special_effect: 5, 1,
    obj_window_bg_enable_bits: 8, 4,
    obj_window_obj_enable_bits: 12, 1,
    obj_window_color_special_effect: 13, 1,
);

io_register! (
    MosaicSize => 4, 0x400004C,
    bg_mosaic_hsize: 0, 4,
    bg_mosaic_vsize: 4, 4,
    obj_mosaic_hsize: 8, 4,
    obj_mosaic_vsize: 12, 4,
);

io_register! (
    ColorSpecialEffectsSelection => 2, 0x4000050,
    bg0_1st_target_pixel: 0, 1,
    bg1_1st_target_pixel: 1, 1,
    bg2_1st_target_pixel: 2, 1,
    bg3_1st_target_pixel: 3, 1,
    obj_1st_target_pixel: 4, 1,
    bd_1st_target_pixel: 5, 1,
    color_special_effect: 6, 2,
    bg0_2nd_target_pixel: 8, 1,
    bg1_2nd_target_pixel: 9, 1,
    bg2_2nd_target_pixel: 10, 1,
    bg3_2nd_target_pixel: 11, 1,
    obj_2nd_target_pixel: 12, 1,
    bd_2nd_target_pixel: 13, 1
);

io_register! (
    AlphaBlendingCoefficients => 2, 0x4000052,
    eva_coefficient: 0, 5,
    evb_coefficient: 8, 5,
);

io_register! (
    BrightnessCoefficient => 4, 0x4000054,
    evy_coefficient: 0, 5,
);