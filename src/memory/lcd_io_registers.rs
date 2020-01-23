use std::cell::RefCell;
use std::rc::Rc;
use crate::operations::{bitutils::get_bits_u8};
use memory_macros::*;

#[memory_segment(2)]
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
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(green_swap, 0, 1)]
pub struct GreenSwap {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(vblank_flag, 0, 1)]
#[bit_field(hblank_flag, 1, 1)]
#[bit_field(vcounter_flag, 2, 1)]
#[bit_field(vblank_irq_enable, 3, 1)]
#[bit_field(hblank_irq_enable, 4, 1)]
#[bit_field(vcounter_irq_enable, 5, 1)]
#[bit_field(vcount_setting, 8, 8)]
pub struct DisplayStatus {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(current_scanline, 0, 8)]
pub struct VerticalCount {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(bg_priority, 0, 2)]
#[bit_field(character_base_block, 2, 2)]
#[bit_field(mosaic, 6, 1)]
#[bit_field(colors, 7, 1)]
#[bit_field(screen_base_block, 8, 4)]
#[bit_field(screen_size, 14, 2)]
pub struct BG_0_1_Control {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(bg_priority, 0, 2)]
#[bit_field(character_base_block, 2, 2)]
#[bit_field(mosaic, 6, 1)]
#[bit_field(colors, 7, 1)]
#[bit_field(screen_base_block, 8, 4)]
#[bit_field(display_area_overflow, 13, 1)]
#[bit_field(screen_size, 14, 2)]
pub struct BG_2_3_Control {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(offset, 0, 9)]
pub struct BGOffset {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(4)]
#[bit_field(fractional_portion, 0, 8)]
#[bit_field(integer_portion, 8, 19)]
#[bit_field(sign, 27, 1)]
pub struct BGRefrencePoint {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(fractional_portion, 0, 8)]
#[bit_field(integer_portion, 8, 7)]
#[bit_field(sign, 15, 1)]
pub struct BGRotScaleParam {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(X2, 0, 8)]
#[bit_field(X1, 8, 8)]
pub struct WindowHorizontalDimension {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(Y2, 0, 8)]
#[bit_field(Y1, 8, 8)]
pub struct WindowVerticalDimension {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(window0_bg_enable_bits, 0, 4)]
#[bit_field(window0_obj_enable_bits, 4, 1)]
#[bit_field(window0_color_special_effect, 5, 1)]
#[bit_field(window1_bg_enable_bits, 8, 4)]
#[bit_field(window1_obj_enable_bits, 12, 1)]
#[bit_field(window1_color_special_effect, 13, 1)]
pub struct ControlWindowInside {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(outside_bg_enable_bits, 0, 4)]
#[bit_field(outside_obj_enable_bits, 4, 1)]
#[bit_field(outside_color_special_effect, 5, 1)]
#[bit_field(obj_window_bg_enable_bits, 8, 4)]
#[bit_field(obj_window_obj_enable_bits, 12, 1)]
#[bit_field(obj_window_color_special_effect, 13, 1)]
pub struct ControlWindowOutside {
    pub memory: Rc<RefCell<Vec<u8>>>
} 

#[memory_segment(4)]
#[bit_field(bg_mosaic_hsize, 0, 4)]
#[bit_field(bg_mosaic_vsize, 4, 4)]
#[bit_field(obj_mosaic_hsize, 8, 4)]
#[bit_field(obj_mosaic_vsize, 12, 4)]
pub struct MosaicSize {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
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
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(eva_coefficient, 0, 5)]
#[bit_field(evb_coefficient, 8, 5)]
pub struct AlphaBlendingCoefficients {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(4)]
#[bit_field(evy_coefficient, 0, 5)]
pub struct BrightnessCoefficient {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_macro() {
        let mut a = DisplayControl::new();
        let bg_mode = a.get_bg_mode();
        a.set_bg_mode(5);
        assert_ne!(bg_mode, a.get_bg_mode());
        assert_eq!(5, a.get_bg_mode());
        assert_eq!(2, DisplayControl::SEGMENT_SIZE);
        a.set_window_0_display_flag(1);
        assert_eq!(1, a.get_window_0_display_flag());

        let mut b = BGOffset::new();
        b.set_offset(510);
        assert_eq!(510, b.get_offset());
    }
}