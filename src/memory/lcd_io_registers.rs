#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::cell::RefCell;
use std::rc::Rc;
use memory_macros::*;
use crate::operations::bitutils::*;
use super::GbaMem;

#[memory_segment(2, 0x4000000)]
#[bit_field(bg_mode, 0, 3)]
#[bit_field(cgb_mode, 3, 1)]
#[bit_field(display_frame_select, 4, 1)]
#[bit_field(hblank_interval_free, 5, 1)]
#[bit_field(obj_charcter_vram_mapping, 6, 1)]
#[bit_field(forced_blank, 7, 1)]
#[bit_field(screen_display_bg0, 8, 1)]
#[bit_field(screen_display_bg1, 9, 1)]
#[bit_field(screen_display_bg2, 10, 1)]
#[bit_field(screen_display_bg3, 11, 1)]
#[bit_field(screen_display_obj, 12, 1)]
#[bit_field(window_0_display_flag, 13, 1)]
#[bit_field(window_1_display_flag, 14, 1)]
#[bit_field(obj_window_display_flag, 15, 1)]
pub struct DisplayControl {
    pub memory: Rc<RefCell<GbaMem>>,
}

#[memory_segment(2, 0x4000002)]
#[bit_field(green_swap, 0, 1)]
pub struct GreenSwap {
    pub memory: Rc<RefCell<GbaMem>>,
}

#[memory_segment(2, 0x4000004)]
#[bit_field(vblank_flag, 0, 1)]
#[bit_field(hblank_flag, 1, 1)]
#[bit_field(vcounter_flag, 2, 1)]
#[bit_field(vblank_irq_enable, 3, 1)]
#[bit_field(hblank_irq_enable, 4, 1)]
#[bit_field(vcounter_irq_enable, 5, 1)]
#[bit_field(vcount_setting, 8, 8)]
pub struct DisplayStatus {
    pub memory: Rc<RefCell<GbaMem>>,
}

#[memory_segment(2, 0x4000006)]
#[bit_field(current_scanline, 0, 8)]
pub struct VerticalCount {
    pub memory: Rc<RefCell<GbaMem>>,
}

#[multiple_memory_segment(2, 0x4000008, 0x400000A, 0x400000C, 0x400000E)]
#[bit_field(bg_priority, 0, 2)]
#[bit_field(character_base_block, 2, 2)]
#[bit_field(mosaic, 6, 1)]
#[bit_field(colors, 7, 1)]
#[bit_field(screen_base_block, 8, 5)]
#[bit_field(display_area_overflow, 13, 1)]
#[bit_field(screen_size, 14, 2)]
pub struct BG_Control {
    pub memory: Rc<RefCell<GbaMem>>,
    index: usize
}

#[multiple_memory_segment(2, 0x4000010, 0x4000012, 0x4000014, 0x4000016, 0x4000018, 0x400001A, 0x400001C, 0x400001E)]
#[bit_field(offset, 0, 9)]
pub struct BGOffset {
    pub memory: Rc<RefCell<GbaMem>>,
    index: usize
}

#[multiple_memory_segment(4, 0x4000028, 0x400002C, 0x4000038, 0x400003C)]
#[bit_field(fractional_portion, 0, 8)]
#[bit_field(integer_portion, 8, 19)]
#[bit_field(sign, 27, 1)]
pub struct BGRefrencePoint {
    pub memory: Rc<RefCell<GbaMem>>,
    pub index: usize
}

impl From<&BGRefrencePoint> for i32 {
    fn from(value: &BGRefrencePoint) -> i32 {
        return sign_extend_u32(value.get_register(), 27) as i32;
    }
}

#[multiple_memory_segment(2, 0x4000020, 0x4000022, 0x4000024, 0x4000026, 0x4000030, 0x4000032, 0x4000034, 0x4000036)]
#[bit_field(fractional_portion, 0, 8)]
#[bit_field(integer_portion, 8, 7)]
#[bit_field(sign, 15, 1)]
pub struct BGRotScaleParam {
    pub memory: Rc<RefCell<GbaMem>>,
    pub index: usize
}

impl From<&BGRotScaleParam> for i32 {
    fn from(value: &BGRotScaleParam) -> i32 {
        return sign_extend_u32(value.get_register() as u32, 16) as i32;
    }
}

#[multiple_memory_segment(2, 0x4000040, 0x4000042)]
#[bit_field(X2, 0, 8)]
#[bit_field(X1, 8, 8)]
pub struct WindowHorizontalDimension {
    pub memory: Rc<RefCell<GbaMem>>,
    pub index: usize
}

#[multiple_memory_segment(2, 0x4000044, 0x4000046)]
#[bit_field(Y2, 0, 8)]
#[bit_field(Y1, 8, 8)]
pub struct WindowVerticalDimension {
    pub memory: Rc<RefCell<GbaMem>>,
    pub index: usize
}

#[memory_segment(2, 0x4000048)]
#[bit_field(window0_bg_enable_bits, 0, 4)]
#[bit_field(window0_obj_enable_bits, 4, 1)]
#[bit_field(window0_color_special_effect, 5, 1)]
#[bit_field(window1_bg_enable_bits, 8, 4)]
#[bit_field(window1_obj_enable_bits, 12, 1)]
#[bit_field(window1_color_special_effect, 13, 1)]
pub struct ControlWindowInside {
    pub memory: Rc<RefCell<GbaMem>>,
}

#[memory_segment(2, 0x400004A)]
#[bit_field(outside_bg_enable_bits, 0, 4)]
#[bit_field(outside_obj_enable_bits, 4, 1)]
#[bit_field(outside_color_special_effect, 5, 1)]
#[bit_field(obj_window_bg_enable_bits, 8, 4)]
#[bit_field(obj_window_obj_enable_bits, 12, 1)]
#[bit_field(obj_window_color_special_effect, 13, 1)]
pub struct ControlWindowOutside {
    pub memory: Rc<RefCell<GbaMem>>,
} 

#[memory_segment(4, 0x400004C)]
#[bit_field(bg_mosaic_hsize, 0, 4)]
#[bit_field(bg_mosaic_vsize, 4, 4)]
#[bit_field(obj_mosaic_hsize, 8, 4)]
#[bit_field(obj_mosaic_vsize, 12, 4)]
pub struct MosaicSize {
    pub memory: Rc<RefCell<GbaMem>>,
}

#[memory_segment(2, 0x4000050)]
#[bit_field(bg0_1st_target_pixel, 0, 1)]
#[bit_field(bg1_1st_target_pixel, 1, 1)]
#[bit_field(bg2_1st_target_pixel, 2, 1)]
#[bit_field(bg3_1st_target_pixel, 3, 1)]
#[bit_field(obj_1st_target_pixel, 4, 1)]
#[bit_field(bd_1st_target_pixel, 5, 1)]
#[bit_field(color_special_effect, 6, 2)]
#[bit_field(bg0_2nd_target_pixel, 8, 1)]
#[bit_field(bg1_2nd_target_pixel, 9, 1)]
#[bit_field(bg2_2nd_target_pixel, 10, 1)]
#[bit_field(bg3_2nd_target_pixel, 11, 1)]
#[bit_field(obj_2nd_target_pixel, 12, 1)]
#[bit_field(bd_2nd_target_pixel, 13, 1)]
pub struct ColorSpecialEffectsSelection {
    pub memory: Rc<RefCell<GbaMem>>,
}

#[memory_segment(2, 0x4000052)]
#[bit_field(eva_coefficient, 0, 5)]
#[bit_field(evb_coefficient, 8, 5)]
pub struct AlphaBlendingCoefficients {
    pub memory: Rc<RefCell<GbaMem>>,
}

#[memory_segment(4, 0x4000054)]
#[bit_field(evy_coefficient, 0, 5)]
pub struct BrightnessCoefficient {
    pub memory: Rc<RefCell<GbaMem>>,
}