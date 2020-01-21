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
pub struct BG0Control {
    pub memory: Rc<RefCell<Vec<u8>>>
}

#[memory_segment(2)]
#[bit_field(bg_priority, 0, 2)]
#[bit_field(character_base_block, 2, 2)]
#[bit_field(mosaic, 6, 1)]
#[bit_field(colors, 7, 1)]
#[bit_field(screen_base_block, 8, 4)]
#[bit_field(screen_size, 14, 2)]
pub struct BG1Control {
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
pub struct BG2Control {
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
pub struct BG3Control {
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
    }
}