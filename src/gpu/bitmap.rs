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

            if pixel_x < 0 || pixel_x >= (DISPLAY_WIDTH as i32) || pixel_y < 0 || pixel_y >= (DISPLAY_HEIGHT as i32) {
                self.backgrounds[2].scan_line[x as usize] = Rgb15::new(0x8000);
                continue;
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

            if pixel_x < 0 || pixel_x >= (DISPLAY_WIDTH as i32) || pixel_y < 0 || pixel_y >= (DISPLAY_HEIGHT as i32) {
                self.backgrounds[2].scan_line[x as usize] = Rgb15::new(0x8000);
                continue;
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
            let pixel_x = (ref_point_x + (x as i32) * pa) >> 8;
            let pixel_y = (ref_point_y + (x as i32) * pc) >> 8;

            if pixel_x < 0 || pixel_x >= 160 || pixel_y < 0 || pixel_y >= 128 {
                self.backgrounds[2].scan_line[x as usize] = Rgb15::new(0x8000);
                continue;
            }

            let bitmap_index = (160u32 * (pixel_y as u32) + (pixel_x as u32)) as u32;
            let bitmap_offset = page_ofs + (2 * bitmap_index);
            let color = Rgb15::new(mem_map.read_u16(bitmap_offset));
            self.backgrounds[2].scan_line[x as usize] = color;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gamepak::BackupType;

    fn setup() -> (GPU, MemoryMap) {
        let mem_map = MemoryMap::new(BackupType::Sram);
        let mut gpu = GPU::new();
        gpu.register(&mem_map.memory);
        (gpu, mem_map)
    }

    #[test]
    fn mode_4_out_of_bounds_coordinates_are_transparent_not_garbage() {
        let (mut gpu, mut mem_map) = setup();
        gpu.bg_affine_components[0].rotation_scaling_param_a.set_register(256);
        gpu.bg_affine_components[0].rotation_scaling_param_c.set_register(0);
        gpu.bg_affine_components[0].refrence_point_x_internal = 10_000_000u32;
        gpu.bg_affine_components[0].refrence_point_y_internal = 0;

        gpu.render_mode_4(&mut mem_map);

        assert!(gpu.backgrounds[2].scan_line.iter().all(|c| c.is_transparent()));
    }

    #[test]
    fn mode_4_in_bounds_reads_real_bitmap_pixel() {
        let (mut gpu, mut mem_map) = setup();
        gpu.bg_affine_components[0].rotation_scaling_param_a.set_register(256);
        gpu.bg_affine_components[0].rotation_scaling_param_c.set_register(0);
        gpu.bg_affine_components[0].refrence_point_x_internal = 0;
        gpu.bg_affine_components[0].refrence_point_y_internal = 0;

        mem_map.memory[0x0600_0000].set(5);
        mem_map.memory[0x0500_000A].set(0x34);
        mem_map.memory[0x0500_000B].set(0x12);

        gpu.render_mode_4(&mut mem_map);

        assert_eq!(gpu.backgrounds[2].scan_line[0].value, 0x1234);
    }

    #[test]
    fn mode_3_out_of_bounds_coordinates_are_transparent_not_garbage() {
        let (mut gpu, mut mem_map) = setup();
        gpu.bg_affine_components[0].rotation_scaling_param_a.set_register(256);
        gpu.bg_affine_components[0].rotation_scaling_param_c.set_register(0);
        gpu.bg_affine_components[0].refrence_point_x_internal = 10_000_000u32;
        gpu.bg_affine_components[0].refrence_point_y_internal = 0;

        gpu.render_mode_3(&mut mem_map);

        assert!(gpu.backgrounds[2].scan_line.iter().all(|c| c.is_transparent()));
    }

    #[test]
    fn mode_5_out_of_bounds_coordinates_are_transparent_not_garbage() {
        let (mut gpu, mut mem_map) = setup();
        gpu.bg_affine_components[0].rotation_scaling_param_a.set_register(256);
        gpu.bg_affine_components[0].rotation_scaling_param_c.set_register(0);
        gpu.bg_affine_components[0].refrence_point_x_internal = 10_000_000u32;
        gpu.bg_affine_components[0].refrence_point_y_internal = 0;

        gpu.render_mode_5(&mut mem_map);

        assert!(gpu.backgrounds[2].scan_line[0..160].iter().all(|c| c.is_transparent()));
    }
}