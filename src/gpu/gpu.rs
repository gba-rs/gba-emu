use crate::memory::work_ram::WorkRam;
use crate::memory::lcd_io_registers::*;
use crate::memory::memory_map::MemoryMap;
use super::rgb15::Rgb15;
use crate::operations::bitutils::*;
use log::debug;

pub const DISPLAY_WIDTH: u32 = 240;
pub const DISPLAY_HEIGHT: u32 = 160;
pub const VBLANK_LENGTH: u32 = 68;

pub const HDRAW_CYCLES: u32 = 960;
pub const HBLANK_CYCLES: u32 = 272;
pub const SCANLINE_CYCLES: u32 = 1232;
pub const VDRAW_CYCLES: u32 = 197120;
pub const VBLANK_CYCLES: u32 = 83776;

pub enum GpuState {
    HDraw,
    HBlank,
    VBlank
}

pub struct Background {
    pub control: BG_Control,
    pub horizontal_offset: BGOffset,
    pub vertical_offset: BGOffset, 
}

impl Background {
    pub fn new() -> Background {
        return Background {
            control: BG_Control::new(),
            horizontal_offset: BGOffset::new(),
            vertical_offset: BGOffset::new()
        };
    }
}

pub struct BgAffineComponent {
    pub refrence_point_x_internal: BGRefrencePoint,
    pub refrence_point_x_external: BGRefrencePoint,
    pub refrence_point_y_internal: BGRefrencePoint,
    pub refrence_point_y_external: BGRefrencePoint,
    pub rotation_scaling_param_a: BGRotScaleParam,
    pub rotation_scaling_param_b: BGRotScaleParam,
    pub rotation_scaling_param_c: BGRotScaleParam,
    pub rotation_scaling_param_d: BGRotScaleParam
}

impl BgAffineComponent {
    pub fn new() -> BgAffineComponent {
        return BgAffineComponent {
            refrence_point_x_internal: BGRefrencePoint::new(),
            refrence_point_x_external: BGRefrencePoint::new(),
            refrence_point_y_internal: BGRefrencePoint::new(),
            refrence_point_y_external: BGRefrencePoint::new(),
            rotation_scaling_param_a: BGRotScaleParam::new(),
            rotation_scaling_param_b: BGRotScaleParam::new(),
            rotation_scaling_param_c: BGRotScaleParam::new(),
            rotation_scaling_param_d: BGRotScaleParam::new()
        }
    }
}

pub struct Window {
    pub horizontal_dimensions: WindowHorizontalDimension,
    pub vertical_dimensions: WindowVerticalDimension
}

impl Window {
    pub fn new() -> Window {
        return Window {    
            horizontal_dimensions: WindowHorizontalDimension::new(),
            vertical_dimensions: WindowVerticalDimension::new()
        }    
    }
}

pub struct GPU {
    // Memory
    pub bg_obj_palette_ram: WorkRam,
    pub not_used_mem: WorkRam,
    pub vram: WorkRam,
    pub not_used_mem_2: WorkRam,
    pub oam_obj_attributes: WorkRam,
    pub not_used_mem_3: WorkRam,

    pub display_control: DisplayControl,
    pub green_swap: GreenSwap,
    pub display_status: DisplayStatus,
    pub vertical_count: VerticalCount,

    pub backgrounds: [Background; 4],
    pub bg_affine_components: [BgAffineComponent; 2],
    pub windows: [Window; 2],

    pub control_window_inside: ControlWindowInside,
    pub control_window_outside: ControlWindowOutside,
    pub mosaic_size: MosaicSize,
    pub color_special_effects_selection: ColorSpecialEffectsSelection,

    pub alpha_blending_coefficients: AlphaBlendingCoefficients,
    pub brightness_coefficient: BrightnessCoefficient,

    pub cycles_to_next_state: u32,
    pub current_state: GpuState,
    pub frame_ready: bool,
    pub frame_buffer: Vec<u32>
}

impl GPU {
    pub fn new() -> GPU {
        return GPU {
            // Memory
            bg_obj_palette_ram: WorkRam::new(0x3FF + 1, 0),
            not_used_mem: WorkRam::new(0xFF_FBFF + 1, 0),
            vram: WorkRam::new(0x1_7FFF + 1, 0),
            not_used_mem_2: WorkRam::new(0xFE_7FFF + 1, 0),
            oam_obj_attributes: WorkRam::new(0x3FF + 1, 0),
            not_used_mem_3: WorkRam::new(0xFF_FBFF + 1, 0),

            // Backgrounds
            backgrounds: [Background::new(), Background::new(), Background::new(), Background::new()],
            bg_affine_components: [BgAffineComponent::new(), BgAffineComponent::new()],
            windows: [Window::new(), Window::new()],

            // Registers
            display_control: DisplayControl::new(),
            green_swap: GreenSwap::new(),
            display_status: DisplayStatus::new(),
            vertical_count: VerticalCount::new(),

            control_window_inside: ControlWindowInside::new(),
            control_window_outside: ControlWindowOutside::new(),
            mosaic_size: MosaicSize::new(),
            color_special_effects_selection: ColorSpecialEffectsSelection::new(),

            alpha_blending_coefficients: AlphaBlendingCoefficients::new(),
            brightness_coefficient: BrightnessCoefficient::new(),

            cycles_to_next_state: HDRAW_CYCLES,
            current_state: GpuState::HDraw,
            frame_ready: false,
            frame_buffer: vec![0; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize]
        };
    }

    pub fn step(&mut self, cycles: u32, mem_map: &mut MemoryMap) {
        if self.cycles_to_next_state <= cycles {
            self.transition_state(mem_map);       
        }
    }

    pub fn transition_state(&mut self, mem_map: &mut MemoryMap) {
        let mut current_scanline = self.vertical_count.get_current_scanline() as u32;
        match self.current_state {
            GpuState::HDraw => {
                self.display_status.set_hblank_flag(1);
                self.current_state = GpuState::HBlank;
                self.cycles_to_next_state = HBLANK_CYCLES;
            },
            GpuState::HBlank => {
                self.vertical_count.set_current_scanline((current_scanline as u8) + 1);
                current_scanline += 1;
                self.display_status.set_hblank_flag(0);

                if current_scanline < DISPLAY_HEIGHT {
                    // render scanline
                    let current_mode = self.display_control.get_bg_mode();
                    match current_mode {
                        0 => {
                            // println!("Mode 0");
                        }
                        4 => {
                            // println!("Mode 4");
                            let page_ofs: u32 = match self.display_control.get_display_frame_select() {
                                0 => 0x06000000,
                                1 => 0x0600A000,
                                _ => unreachable!(),
                            };

                            let pa = i32::from(&self.bg_affine_components[0].rotation_scaling_param_a);
                            let pc = i32::from(&self.bg_affine_components[0].rotation_scaling_param_c);
                            let ref_point_x = i32::from(&self.bg_affine_components[0].refrence_point_x_internal);
                            let ref_point_y = i32::from(&self.bg_affine_components[0].refrence_point_y_internal);

                            for x in 0..DISPLAY_WIDTH {
                                let t = ((ref_point_x + (x as i32) * pa) >> 8, (ref_point_y + (x as i32) * pc) >> 8);
                                // TODO check outside of viewport
                                let bitmap_index = ((DISPLAY_WIDTH as u32) * (t.1 as u32) + (t.0 as u32)) as u32;
                                let bitmap_offset = page_ofs + bitmap_index;
                                let index = mem_map.read_u8(bitmap_offset) as u32;
                                let color = Rgb15::new(mem_map.read_u16((2 * index) + 0x05000000));
                                let frame_buffer_index = ((DISPLAY_WIDTH as u32) * (current_scanline as u32) + (x as u32)) as usize;
                                self.frame_buffer[frame_buffer_index] = color.to_0rgb();
                            }

                        }
                        _ => panic!("Unimplemented mode: {}", current_mode)
                    }

                    // update refrence points at end of scanline
                    for i in 0..2 {
                        let internal_x = i32::from(&self.bg_affine_components[i].refrence_point_x_internal);
                        let internal_y = i32::from(&self.bg_affine_components[i].refrence_point_y_internal);
                        let pb = i32::from(&self.bg_affine_components[i].rotation_scaling_param_b);
                        let pd = i32::from(&self.bg_affine_components[i].rotation_scaling_param_d);

                        self.bg_affine_components[i].refrence_point_x_internal.set_register((internal_x + pb) as u32);
                        self.bg_affine_components[i].refrence_point_y_internal.set_register((internal_y + pd) as u32);
                    }

                    self.current_state = GpuState::HDraw;
                    self.cycles_to_next_state = HDRAW_CYCLES;
                } else {                    
                    for i in 0..2 {
                        self.bg_affine_components[i].refrence_point_x_internal.set_register(self.bg_affine_components[i].refrence_point_x_external.get_register());
                        self.bg_affine_components[i].refrence_point_y_internal.set_register(self.bg_affine_components[i].refrence_point_y_external.get_register());
                    }

                    // do irq stuff
                    // do dma stuff
                    self.display_status.set_vblank_flag(1);
                    self.current_state = GpuState::VBlank;
                    self.cycles_to_next_state = SCANLINE_CYCLES;
                }

            },
            GpuState::VBlank => {
                self.vertical_count.set_current_scanline((current_scanline as u8) + 1);
                current_scanline += 1;

                if current_scanline < DISPLAY_HEIGHT + VBLANK_LENGTH - 1 {
                    self.current_state = GpuState::VBlank;
                    self.cycles_to_next_state = SCANLINE_CYCLES;
                } else {
                    self.display_status.set_vblank_flag(0);
                    self.vertical_count.set_current_scanline(0);
                    self.current_state = GpuState::HDraw;
                    self.cycles_to_next_state = HDRAW_CYCLES;
                    self.frame_ready = true;
                }
            }
        }
    }
}