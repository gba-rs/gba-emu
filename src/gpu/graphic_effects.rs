use crate::gpu::gpu::GPU;
use crate::memory::memory_map::MemoryMap;
use super::{rgb15::Rgb15, gpu::DISPLAY_WIDTH};
use crate::memory::memory_map::PALETTE_RAM_START;
use std::cmp;

#[derive(PartialEq)]
pub enum BlendMode {
    Off,
    Alpha,
    White,
    Black
}

impl From<u8> for BlendMode {
    fn from(value: u8) -> Self {
        return match value {
            0b00 => BlendMode::Off,
            0b01 => BlendMode::Alpha,
            0b10 => BlendMode::White,
            0b11 => BlendMode::Black,
            _ => panic!("Invalid BlendMode: {:b}", value)
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum WindowTypes {
    Window0,
    Window1,
    WindowOutside,
    WindowObject,
}

impl GPU {
    fn get_window_type(&self, x: u32, y: u32) -> Option<WindowTypes>{
        if self.display_control.using_windows() {
            if self.display_control.get_window_0_display_flag() != 0 && self.windows[0].inside(x, y) {
                return Some(WindowTypes::Window0);
            }

            if self.display_control.get_window_1_display_flag() != 0 && self.windows[1].inside(x, y){
                return Some(WindowTypes::Window1);
            }

            let obj_window_index: usize = (DISPLAY_WIDTH * (y as u32) + (x as u32)) as usize;

            // TODO object window
            if self.display_control.get_obj_window_display_flag() != 0 && self.obj_window[obj_window_index] {
                return Some(WindowTypes::WindowObject);
            }

            Some(WindowTypes::WindowOutside) 
        } else {
            None
        }
    }

    fn get_window_flags(&self, window_type: &WindowTypes) -> (bool, bool, [bool; 4]) {
        match window_type {
            WindowTypes::Window0 | WindowTypes::Window1 => {
                return (self.control_window_inside.should_display_sfx(window_type), 
                        self.control_window_inside.should_display_obj(window_type), 
                        self.control_window_inside.bgs_to_display(window_type));
            },
            WindowTypes::WindowObject | WindowTypes::WindowOutside => {
                return (self.control_window_outside.should_display_sfx(window_type), 
                        self.control_window_outside.should_display_obj(window_type), 
                        self.control_window_outside.bgs_to_display(window_type));

            },
        };
    }

    fn blend(&self, target_1: &mut Rgb15, target_2: &Rgb15, blend_mode: &BlendMode) {
        match blend_mode {
            BlendMode::Alpha => {
                let eva = self.alpha_blending_coefficients.get_eva_coefficient();
                let evb = self.alpha_blending_coefficients.get_evb_coefficient();

                target_1.red = cmp::min(31, (((target_1.red as u32) * (eva as u32) + (target_2.red as u32) * (evb as u32)) >> 4) as u8);
                target_1.green = cmp::min(31, (((target_1.green as u32) * (eva as u32) + (target_2.green as u32) * (evb as u32)) >> 4) as u8);
                target_1.blue = cmp::min(31, (((target_1.blue as u32) * (eva as u32) + (target_2.blue as u32) * (evb as u32)) >> 4) as u8);
            },
            BlendMode::Black => {
                let evy = self.brightness_coefficient.get_evy_coefficient();
                let eva = 16 - evy;
                let evb = evy;
                let other = Rgb15::new(0);

                target_1.red = cmp::min(31, (((target_1.red as u32) * (eva as u32) + (other.red as u32) * (evb as u32)) >> 4) as u8);
                target_1.green = cmp::min(31, (((target_1.green as u32) * (eva as u32) + (other.green as u32) * (evb as u32)) >> 4) as u8);
                target_1.blue = cmp::min(31, (((target_1.blue as u32) * (eva as u32) + (other.blue as u32) * (evb as u32)) >> 4) as u8);
            },
            BlendMode::White => {
                let evy = self.brightness_coefficient.get_evy_coefficient();
                let eva = 16 - evy;
                let evb = evy;
                let other = Rgb15::new(0x7FFF);

                target_1.red = cmp::min(31, (((target_1.red as u32) * (eva as u32) + (other.red as u32) * (evb as u32)) >> 4) as u8);
                target_1.green = cmp::min(31, (((target_1.green as u32) * (eva as u32) + (other.green as u32) * (evb as u32)) >> 4) as u8);
                target_1.blue = cmp::min(31, (((target_1.blue as u32) * (eva as u32) + (other.blue as u32) * (evb as u32)) >> 4) as u8);
            },
            BlendMode::Off => {}
        }
    }

    fn sort_backgrounds(&self) -> ([u8;4], usize){
        let mut bg_list: [u8; 4] = [0; 4];
        let mut bg_count: usize = 0;

        for priority in (0..4).rev() {
            for bg in (0..4).rev() {
                if self.display_control.should_display(bg) && 
                   self.backgrounds[bg as usize].control.get_bg_priority() == priority {

                    bg_list[bg_count] = bg;
                    bg_count += 1;
                }
            }
        }

        return (bg_list, bg_count);
    }

    pub fn composite_background(&mut self, mem_map: &mut MemoryMap) {
        let current_scanline = self.vertical_count.get_current_scanline() as u32;

        let (bg_list, bg_count) = self.sort_backgrounds();
        let mut pixel: (Rgb15, Rgb15) = (Rgb15::new(0), Rgb15::new(0));

        for x in 0..DISPLAY_WIDTH {
            let obj_buffer_index: usize = (DISPLAY_WIDTH * (current_scanline as u32) + (x as u32)) as usize;
            let (obj_color, obj_priority, obj_gfx_mode) = self.obj_buffer[obj_buffer_index];
            let window_type = self.get_window_type(x, current_scanline);
            let dsp_ctrl_obj = self.display_control.get_screen_display_obj() != 0;

            let (disp_sfx, disp_obj, bgs_to_disp) = match &window_type {
                Some(val) => self.get_window_flags(&val),
                None => (true, true, [true, true, true, true])
            };

            let mut layer: (u8, u8) = (5, 5);
            let mut priority: (u8, u8) = (4, 4);

            for i in 0..bg_count {
                let bg = bg_list[i as usize];

                if bgs_to_disp[bg as usize] {
                    let pixel_new = self.backgrounds[bg as usize].scan_line[x as usize];
                    if !pixel_new.is_transparent() {
                        layer.1 = layer.0;
                        layer.0 = bg;
                        priority.1 = priority.0;
                        priority.0 = self.backgrounds[bg as usize].control.get_bg_priority();
                    }
                }
            }

            if dsp_ctrl_obj && disp_obj && !obj_color.is_transparent() {
                if obj_priority <= priority.0 {
                    // drop the obj in front
                    layer.1 = layer.0;
                    layer.0 = 4;
                } else if obj_priority <= priority.1 {
                    // drop the obj in between
                    layer.1 = 4;
                }
            }

            match layer.0 {
                0..=3 => pixel.0 = self.backgrounds[layer.0 as usize].scan_line[x as usize],
                4 => pixel.0 = obj_color,
                5 => pixel.0 = Rgb15::new(mem_map.read_u16(PALETTE_RAM_START)),
                _ => panic!("THis should never hit")
            }

            match layer.1 {
                0..=3 => pixel.1 = self.backgrounds[layer.1 as usize].scan_line[x as usize],
                4 => pixel.1 = obj_color,
                5 => pixel.1 = Rgb15::new(mem_map.read_u16(PALETTE_RAM_START)),
                _ => panic!("THis should never hit")
            }

            let is_alpha_obj = layer.0 == 4 && obj_gfx_mode == 0b01;

            if disp_sfx || is_alpha_obj {
                let mut blend_mode = self.color_special_effects_selection.get_blendmode();
                let have_destination = self.color_special_effects_selection.has_destination(layer.0) | is_alpha_obj;
                let have_source = self.color_special_effects_selection.has_source(layer.1);

                if is_alpha_obj && have_source {
                    blend_mode = BlendMode::Alpha;
                }

                if blend_mode != BlendMode::Off && have_destination && (have_source || blend_mode != BlendMode::Alpha) {
                    // blend
                    self.blend(&mut pixel.0, &pixel.1, &blend_mode);
                }
            }

            let frame_buffer_index = ((DISPLAY_WIDTH as u32) * (current_scanline as u32) + (x as u32)) as usize;
            self.frame_buffer[frame_buffer_index] = pixel.0.to_0rgb();
        }
    }
}