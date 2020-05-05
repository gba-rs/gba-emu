use crate::{
    memory::{
        memory_map::MemoryMap,
        lcd_io_registers::*
    },
    operations::bitutils,
    dma::DMAController,
    interrupts::interrupts::Interrupts
};
use super::{
    rgb15::Rgb15, 
    object::Object,
    object::AffineMatrix
};
use std::{
    cell::RefCell,
    rc::Rc
};
use memory_macros::{gen_aff_matrix_array, gen_obj_array};

gen_obj_array!();
gen_aff_matrix_array!();

pub const DISPLAY_WIDTH: u32 = 240;
pub const DISPLAY_HEIGHT: u32 = 160;
pub const VBLANK_LENGTH: u32 = 68;

pub const HDRAW_CYCLES: i64 = 960;
pub const HBLANK_CYCLES: i64 = 272;
pub const SCANLINE_CYCLES: i64 = 1232;
pub const VDRAW_CYCLES: i64 = 197120;
pub const VBLANK_CYCLES: i64 = 83776;

#[derive(PartialEq)]
pub enum GpuState {
    HDraw,
    HBlank,
    VBlank
}

pub struct Background {
    pub control: BG_Control,
    pub horizontal_offset: BGOffset,
    pub vertical_offset: BGOffset,
    pub scan_line: Vec<Rgb15>,
    pub id: usize
}

impl Background {
    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>) {
        self.control.register(mem);
        self.horizontal_offset.register(mem);
        self.vertical_offset.register(mem);
    }

    pub fn get_offsets(&self) -> (u32, u32) {
        return (self.vertical_offset.get_offset() as u32, self.horizontal_offset.get_offset() as u32);
    }
}

pub struct BgAffineComponent {
    pub refrence_point_x_internal: u32,
    pub refrence_point_x_external: BGRefrencePoint,
    pub refrence_point_x_external_hold: u32,
    pub refrence_point_y_internal: u32,
    pub refrence_point_y_external: BGRefrencePoint,
    pub refrence_point_y_external_hold: u32,
    pub rotation_scaling_param_a: BGRotScaleParam,
    pub rotation_scaling_param_b: BGRotScaleParam,
    pub rotation_scaling_param_c: BGRotScaleParam,
    pub rotation_scaling_param_d: BGRotScaleParam
}

impl BgAffineComponent {
    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>) {
        self.refrence_point_x_external.register(mem);
        self.refrence_point_y_external.register(mem);
        self.rotation_scaling_param_a.register(mem);
        self.rotation_scaling_param_b.register(mem);
        self.rotation_scaling_param_c.register(mem);
        self.rotation_scaling_param_d.register(mem);
    }
}

pub struct Window {
    pub horizontal_dimensions: WindowHorizontalDimension,
    pub vertical_dimensions: WindowVerticalDimension
}

impl Window {
    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>) {
        self.horizontal_dimensions.register(mem);
        self.vertical_dimensions.register(mem);
    }

    pub fn inside(&self, x: u32, y: u32) -> bool {
        let left = self.horizontal_dimensions.get_X1() as u32;
        let mut right = self.horizontal_dimensions.get_X2() as u32;
        let top = self.vertical_dimensions.get_Y1() as u32;
        let mut bottom = self.vertical_dimensions.get_Y2() as u32;

        if right > DISPLAY_WIDTH || right < left {
            right = DISPLAY_WIDTH;
        }
        if bottom > DISPLAY_HEIGHT || bottom < top {
            bottom = DISPLAY_HEIGHT;
        }

        (x >= left && x < right) && (y >= top && y < bottom)
    }
}

pub struct GPU {
    pub display_control: DisplayControl,
    pub green_swap: GreenSwap,
    pub display_status: DisplayStatus,
    pub vertical_count: VerticalCount,

    pub backgrounds: [Background; 4],
    pub bg_affine_components: [BgAffineComponent; 2],
    pub windows: [Window; 2],
    pub obj_window: [bool; (DISPLAY_WIDTH as usize) * (DISPLAY_HEIGHT as usize)],

    pub objects: [Object; 128],
    pub aff_matrices: [AffineMatrix; 32],

    pub control_window_inside: ControlWindowInside,
    pub control_window_outside: ControlWindowOutside,
    pub mosaic_size: MosaicSize,
    pub color_special_effects_selection: ColorSpecialEffectsSelection,

    pub alpha_blending_coefficients: AlphaBlendingCoefficients,
    pub brightness_coefficient: BrightnessCoefficient,

    pub cycles_to_next_state: i64,
    pub current_state: GpuState,
    pub frame_ready: bool,
    pub frame_buffer: Vec<u32>,
    pub obj_buffer: Vec<(Rgb15, u8, u8)>
}

impl GPU {
    pub fn new() -> GPU {
        return GPU {
            // Backgrounds
            backgrounds: [
                Background {
                    control: BG_Control::new(0),
                    horizontal_offset: BGOffset::new(0),
                    vertical_offset: BGOffset::new(1),
                    scan_line: vec![Rgb15::new(0x8000); DISPLAY_WIDTH as usize],
                    id: 0
                },
                Background {
                    control: BG_Control::new(1),
                    horizontal_offset: BGOffset::new(2),
                    vertical_offset: BGOffset::new(3), 
                    scan_line: vec![Rgb15::new(0x8000); DISPLAY_WIDTH as usize],
                    id: 1
                },
                Background {
                    control: BG_Control::new(2),
                    horizontal_offset: BGOffset::new(4),
                    vertical_offset: BGOffset::new(5), 
                    scan_line: vec![Rgb15::new(0x8000); DISPLAY_WIDTH as usize],
                    id: 2
                },
                Background {
                    control: BG_Control::new(3),
                    horizontal_offset: BGOffset::new(6),
                    vertical_offset: BGOffset::new(7), 
                    scan_line: vec![Rgb15::new(0x8000); DISPLAY_WIDTH as usize],
                    id: 3
                }
            ],
            bg_affine_components: [
                BgAffineComponent {
                    refrence_point_x_internal: 0,
                    refrence_point_x_external: BGRefrencePoint::new(0),
                    refrence_point_x_external_hold: 0,
                    refrence_point_y_internal: 0,
                    refrence_point_y_external: BGRefrencePoint::new(1),
                    refrence_point_y_external_hold: 0,
                    rotation_scaling_param_a: BGRotScaleParam::new(0),
                    rotation_scaling_param_b: BGRotScaleParam::new(1),
                    rotation_scaling_param_c: BGRotScaleParam::new(2),
                    rotation_scaling_param_d: BGRotScaleParam::new(3)
                },
                BgAffineComponent {
                    refrence_point_x_internal: 0,
                    refrence_point_x_external: BGRefrencePoint::new(2),
                    refrence_point_x_external_hold: 0,
                    refrence_point_y_internal: 0,
                    refrence_point_y_external: BGRefrencePoint::new(3),
                    refrence_point_y_external_hold: 0,
                    rotation_scaling_param_a: BGRotScaleParam::new(4),
                    rotation_scaling_param_b: BGRotScaleParam::new(5),
                    rotation_scaling_param_c: BGRotScaleParam::new(6),
                    rotation_scaling_param_d: BGRotScaleParam::new(7)
                }
            ],
            objects: obj_array!(),
            aff_matrices: aff_matrix_array!(),
            windows: [
                Window {
                    horizontal_dimensions: WindowHorizontalDimension::new(0),
                    vertical_dimensions: WindowVerticalDimension::new(0)
                }, 
                Window {
                    horizontal_dimensions: WindowHorizontalDimension::new(1),
                    vertical_dimensions: WindowVerticalDimension::new(1)
                }
            ],
            obj_window: [false; (DISPLAY_WIDTH as usize) * (DISPLAY_HEIGHT as usize)],

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
            frame_buffer: vec![0; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize],
            obj_buffer: vec![(Rgb15::new(0x8000), 4, 0); (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize]
        };
    }

    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>) {
        for i in 0..4 {
            self.backgrounds[i].register(mem);
        }

        for i in 0..2 {
            self.bg_affine_components[i].register(mem);
            self.windows[i].register(mem);
        }

        for i in 0..128 {
            self.objects[i].register(mem);
        }

        for i in 0..32 {
            self.aff_matrices[i].register(mem);
        }

        // Registers
        self.display_control.register(mem);
        self.green_swap.register(mem);
        self.display_status.register(mem);
        self.vertical_count.register(mem);

        self.control_window_inside.register(mem);
        self.control_window_outside.register(mem);
        self.mosaic_size.register(mem);
        self.color_special_effects_selection.register(mem);

        self.alpha_blending_coefficients.register(mem);
        self.brightness_coefficient.register(mem);
    }

    pub fn step(&mut self, cycles: usize, mem_map: &mut MemoryMap, irq_ctl: &mut Interrupts, dma_ctl: &mut DMAController) {
        let temp_cycles: i64 = self.cycles_to_next_state - (cycles as i64);

        if temp_cycles <= 0 {
            self.transition_state(mem_map, irq_ctl, dma_ctl);
            self.cycles_to_next_state += temp_cycles;       
        } else {
            self.cycles_to_next_state = temp_cycles;
        }
    }

    fn render_scanline(&mut self, mem_map: &mut MemoryMap) {
        let current_mode = self.display_control.get_bg_mode();
        match current_mode {
            0 => {
                for i in 0..4 {
                    if self.display_control.should_display(i) {
                        self.render_bg(mem_map, i as usize);
                    }
                }
            },
            1 => {
                if self.display_control.should_display(2) {
                    self.render_aff_bg(mem_map, 2);
                }

                if self.display_control.should_display(1) {
                    self.render_bg(mem_map, 1);
                }

                if self.display_control.should_display(0) {
                    self.render_bg(mem_map, 0);
                }
            },
            2 => {
                if self.display_control.should_display(2) {
                    self.render_aff_bg(mem_map, 2);
                }
                
                if self.display_control.should_display(3) {
                    self.render_aff_bg(mem_map, 3);
                }
            },
            3 => {
                self.render_mode_3(mem_map);
            },
            4 => {
                self.render_mode_4(mem_map);
            },
            5 => {
                self.render_mode_5(mem_map);
            },
            _ => panic!("Unimplemented mode: {}", current_mode)
        }

        if self.display_control.get_screen_display_obj() == 1 {
            self.render_obj(mem_map);
        }
    }

    pub fn transition_state(&mut self, mem_map: &mut MemoryMap, irq_ctl: &mut Interrupts, dma_ctl: &mut DMAController) {
        let mut current_scanline = self.vertical_count.get_current_scanline() as u32;
        match self.current_state {
            GpuState::HDraw => {
                self.display_status.set_hblank_flag(1);

                if self.display_status.get_hblank_irq_enable() == 1 {
                    irq_ctl.if_interrupt.set_lcd_h_blank(1);
                }

                dma_ctl.hblanking = true;

                self.current_state = GpuState::HBlank;
                self.cycles_to_next_state = HBLANK_CYCLES;
            },
            GpuState::HBlank => {
                self.update_vcount((current_scanline + 1) as u8, irq_ctl);
                current_scanline += 1;
                self.display_status.set_hblank_flag(0);

                if current_scanline < DISPLAY_HEIGHT {
                    // render scanline
                    self.render_scanline(mem_map);

                    // composite the backgrounds
                    self.composite_background(mem_map);

                    // update refrence points at end of scanline
                    for i in 0..2 {
                        let mut internal_x = bitutils::sign_extend_u32(self.bg_affine_components[i].refrence_point_x_internal, 27) as i32;
                        let mut internal_y =  bitutils::sign_extend_u32(self.bg_affine_components[i].refrence_point_y_internal, 27) as i32;
                        let external_x = self.bg_affine_components[i].refrence_point_x_external.get_register();
                        let external_y = self.bg_affine_components[i].refrence_point_y_external.get_register();
    
                        if self.bg_affine_components[i].refrence_point_x_external_hold != external_x {
                            internal_x = bitutils::sign_extend_u32(external_x, 27) as i32;
                            self.bg_affine_components[i].refrence_point_x_external_hold = external_x;
                        }
    
                        if self.bg_affine_components[i].refrence_point_y_external_hold != external_y {
                            internal_y = bitutils::sign_extend_u32(external_y, 27) as i32;
                            self.bg_affine_components[i].refrence_point_y_external_hold = external_y;
                        }
    
                        let pb = i32::from(&self.bg_affine_components[i].rotation_scaling_param_b);
                        let pd = i32::from(&self.bg_affine_components[i].rotation_scaling_param_d);
    
                        self.bg_affine_components[i].refrence_point_x_internal = (pb + internal_x) as u32; //t_register((internal_x + pb) as u32);
                        self.bg_affine_components[i].refrence_point_y_internal = (pd + internal_y) as u32;
                    }

                    self.current_state = GpuState::HDraw;
                    self.cycles_to_next_state = HDRAW_CYCLES;
                } else {                    
                    for i in 0..2 {
                        self.bg_affine_components[i].refrence_point_x_internal =  self.bg_affine_components[i].refrence_point_x_external.get_register();
                        self.bg_affine_components[i].refrence_point_y_internal =  self.bg_affine_components[i].refrence_point_y_external.get_register();
                        self.bg_affine_components[i].refrence_point_x_external_hold = self.bg_affine_components[i].refrence_point_x_internal;
                        self.bg_affine_components[i].refrence_point_y_external_hold = self.bg_affine_components[i].refrence_point_y_internal;
                    }

                    // do irq stuff
                    if self.display_status.get_vblank_irq_enable() == 1 {
                        irq_ctl.if_interrupt.set_lcd_v_blank(1);
                    }

                    // do dma stuff
                    dma_ctl.vblanking = true;

                    self.display_status.set_vblank_flag(1);
                    self.current_state = GpuState::VBlank;
                    self.cycles_to_next_state = SCANLINE_CYCLES;
                }

            },
            GpuState::VBlank => {
                self.update_vcount((current_scanline + 1) as u8, irq_ctl);
                current_scanline += 1;

                if current_scanline < DISPLAY_HEIGHT + VBLANK_LENGTH - 1 {
                    self.current_state = GpuState::VBlank;

                    self.display_status.set_hblank_flag(1);
                    if self.display_status.get_hblank_irq_enable() == 1 {
                        irq_ctl.if_interrupt.set_lcd_h_blank(1);
                    }
                    
                    self.cycles_to_next_state = SCANLINE_CYCLES;
                } else {
                    self.display_status.set_vblank_flag(0);

                    self.update_vcount(0, irq_ctl);
                    self.current_state = GpuState::HDraw;
                    self.cycles_to_next_state = HDRAW_CYCLES;
                    self.frame_ready = true;
                }
            }  
        }
    }

    fn update_vcount(&mut self, value: u8, irq_ctl: &mut Interrupts) {
        self.vertical_count.set_current_scanline(value);
        let vcount_setting = self.display_status.get_vcount_setting();
        self.display_status.set_vcounter_flag((vcount_setting == self.vertical_count.get_current_scanline()) as u8);

        if self.display_status.get_vcounter_irq_enable() == 1 && self.display_status.get_vcounter_flag() == 1{
            irq_ctl.if_interrupt.set_lcd_v_counter(1);
        }
    }
}