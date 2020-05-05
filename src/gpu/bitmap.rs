use crate::{
    memory::{
        memory_map::{MemoryMap, PALETTE_RAM_START},
    },
    operations::bitutils
};
use super::{
    gpu::{GPU, DISPLAY_WIDTH, DISPLAY_HEIGHT}, 
    rgb15::Rgb15, 
};

impl GPU {
    pub fn render_mode_3(&mut self, mem_map: &mut MemoryMap) {
        let map_start_address = 0x06000000;
        let pa = i32::from(&self.bg_affine_components[0].rotation_scaling_param_a);
        let pc = i32::from(&self.bg_affine_components[0].rotation_scaling_param_c);
        let ref_point_x = bitutils::sign_extend_u32(self.bg_affine_components[0].refrence_point_x_internal, 27) as i32;
        let ref_point_y =  bitutils::sign_extend_u32(self.bg_affine_components[0].refrence_point_y_internal, 27) as i32;

        for x in 0..DISPLAY_WIDTH {
            let pixel_x = (ref_point_x + (x as i32) * pa) >> 8;
            let pixel_y = (ref_point_y + (x as i32) * pc) >> 8;            

            if pixel_x < 0 || pixel_x > (DISPLAY_WIDTH as i32) || pixel_y < 0 || pixel_y > (DISPLAY_HEIGHT as i32) {
                self.backgrounds[2].scan_line[x as usize] = Rgb15::new(0x8000);
            }

            let bitmap_index = (DISPLAY_WIDTH as u32) * (pixel_y as u32) + (pixel_x as u32);
            let color = Rgb15::new(mem_map.read_u16((2 * bitmap_index) + map_start_address));
            self.backgrounds[2].scan_line[x as usize] = color;
        }

    }

    pub fn render_mode_4(&mut self, mem_map: &mut MemoryMap) {
        let page_ofs: u32 = match self.display_control.get_display_frame_select() {
            0 => 0x06000000,
            1 => 0x0600A000,
            _ => unreachable!(),
        };

        let pa = i32::from(&self.bg_affine_components[0].rotation_scaling_param_a);
        let pc = i32::from(&self.bg_affine_components[0].rotation_scaling_param_c);
        let ref_point_x = bitutils::sign_extend_u32(self.bg_affine_components[0].refrence_point_x_internal, 27) as i32;
        let ref_point_y =  bitutils::sign_extend_u32(self.bg_affine_components[0].refrence_point_y_internal, 27) as i32;

        for x in 0..DISPLAY_WIDTH {
            let pixel_x = (ref_point_x + (x as i32) * pa) >> 8;
            let pixel_y = (ref_point_y + (x as i32) * pc) >> 8;            
            // TODO check outside of viewport
            if pixel_x < 0 || pixel_x > (DISPLAY_WIDTH as i32) || pixel_y < 0 || pixel_y > (DISPLAY_HEIGHT as i32) {
                self.backgrounds[2].scan_line[x as usize] = Rgb15::new(0x8000);
            }

            let bitmap_index = (DISPLAY_WIDTH as u32) * (pixel_y as u32) + (pixel_x as u32);
            let bitmap_offset = page_ofs + bitmap_index;
            let index = mem_map.read_u8(bitmap_offset) as u32;
            let color = Rgb15::new(mem_map.read_u16((2 * index) + PALETTE_RAM_START));
            self.backgrounds[2].scan_line[x as usize] = color;
        }
    }

    pub fn render_mode_5(&mut self, mem_map: &mut MemoryMap) {
        let page_ofs: u32 = match self.display_control.get_display_frame_select() {
            0 => 0x06000000,
            1 => 0x0600A000,
            _ => unreachable!(),
        };

        let pa = i32::from(&self.bg_affine_components[0].rotation_scaling_param_a);
        let pc = i32::from(&self.bg_affine_components[0].rotation_scaling_param_c);
        let ref_point_x = bitutils::sign_extend_u32(self.bg_affine_components[0].refrence_point_x_internal, 27) as i32;
        let ref_point_y =  bitutils::sign_extend_u32(self.bg_affine_components[0].refrence_point_y_internal, 27) as i32;

        for x in 0..160 {
            let t = ((ref_point_x + (x as i32) * pa) >> 8, (ref_point_y + (x as i32) * pc) >> 8);
            // TODO check outside of viewport
            let bitmap_index = ((DISPLAY_WIDTH as u32) * (t.1 as u32) + (t.0 as u32)) as u32;
            let bitmap_offset = page_ofs + (2 * bitmap_index);
            let color = Rgb15::new(mem_map.read_u16(bitmap_offset));
            self.backgrounds[2].scan_line[x as usize] = color;
        }
    }
}