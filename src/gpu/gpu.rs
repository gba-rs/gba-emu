use crate::memory::lcd_io_registers::*;
use crate::memory::memory_map::MemoryMap;
use crate::interrupts::interrupts::Interrupts;
use super::{rgb15::Rgb15, tile_map_entry::TileMapEntry};
use crate::operations::bitutils;
use std::cell::RefCell;
use std::rc::Rc;
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

pub struct ObjAttribute0 {
    pub y_coordinate: u16,
    pub rotation_flag: bool,
    pub double_size_flag: bool, // Both of these occupy bit 9,
    pub obj_disable_flag: bool, // but they depend on rotation flag
    pub obj_mode: u8,
    pub mosaic_flag: bool,
    pub color_flag: bool,
    pub obj_shape: u8
}


pub struct ObjAttribute1 {
    pub x_coordinate: u16,
    pub rotation_scaling_param: _____, // If rotation flag is set, bits 9-13
    pub horizontal_flip: bool,         // If rotation flag is not, bit 12
    pub vertical_flip: bool,           // If rotation flag is not, bit 13
    pub obj_size: u8,
}


pub struct ObjAttribute2 {
    pub character_name: u16,
    pub priority_rel_to_bg: u8,
    pub palette_number: u8
}

pub struct Object {
    pub obj0: ObjAttribute0,
    pub obj1: ObjAttribute1,
    pub obj2: ObjAttribute2
}

pub struct Background {
    pub control: BG_Control,
    pub horizontal_offset: BGOffset,
    pub vertical_offset: BGOffset,
    pub scan_line: Vec<Rgb15>
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
    pub refrence_point_y_internal: u32,
    pub refrence_point_y_external: BGRefrencePoint,
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
}

pub struct GPU {
    pub display_control: DisplayControl,
    pub green_swap: GreenSwap,
    pub display_status: DisplayStatus,
    pub vertical_count: VerticalCount,

    pub backgrounds: [Background; 4],
    pub bg_affine_components: [BgAffineComponent; 2],
    pub windows: [Window; 2],

    pub objects: [Object; 128],

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
            // Backgrounds
            backgrounds: [
                Background {
                    control: BG_Control::new(0),
                    horizontal_offset: BGOffset::new(0),
                    vertical_offset: BGOffset::new(1),
                    scan_line: vec![Rgb15::new(0x8000); DISPLAY_WIDTH as usize]
                },
                Background {
                    control: BG_Control::new(1),
                    horizontal_offset: BGOffset::new(2),
                    vertical_offset: BGOffset::new(3), 
                    scan_line: vec![Rgb15::new(0x8000); DISPLAY_WIDTH as usize]
                },
                Background {
                    control: BG_Control::new(2),
                    horizontal_offset: BGOffset::new(4),
                    vertical_offset: BGOffset::new(5), 
                    scan_line: vec![Rgb15::new(0x8000); DISPLAY_WIDTH as usize]
                },
                Background {
                    control: BG_Control::new(3),
                    horizontal_offset: BGOffset::new(6),
                    vertical_offset: BGOffset::new(7), 
                    scan_line: vec![Rgb15::new(0x8000); DISPLAY_WIDTH as usize]
                }
            ],
            bg_affine_components: [
                BgAffineComponent {
                    refrence_point_x_internal: 0,
                    refrence_point_x_external: BGRefrencePoint::new(0),
                    refrence_point_y_internal: 0,
                    refrence_point_y_external: BGRefrencePoint::new(1),
                    rotation_scaling_param_a: BGRotScaleParam::new(0),
                    rotation_scaling_param_b: BGRotScaleParam::new(1),
                    rotation_scaling_param_c: BGRotScaleParam::new(2),
                    rotation_scaling_param_d: BGRotScaleParam::new(3)
                },
                BgAffineComponent {
                    refrence_point_x_internal: 0,
                    refrence_point_x_external: BGRefrencePoint::new(2),
                    refrence_point_y_internal: 0,
                    refrence_point_y_external: BGRefrencePoint::new(3),
                    rotation_scaling_param_a: BGRotScaleParam::new(4),
                    rotation_scaling_param_b: BGRotScaleParam::new(5),
                    rotation_scaling_param_c: BGRotScaleParam::new(6),
                    rotation_scaling_param_d: BGRotScaleParam::new(7)
                }
            ],
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

    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>) {
        for i in 0..4 {
            self.backgrounds[i].register(mem);
        }

        for i in 0..2 {
            self.bg_affine_components[i].register(mem);
            self.windows[i].register(mem);
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

    pub fn step(&mut self, cycles: u32, mem_map: &mut MemoryMap, irq_ctl: &mut Interrupts) {
        if self.cycles_to_next_state <= cycles {
            self.transition_state(mem_map, irq_ctl);       
        }
    }

    pub fn transition_state(&mut self, mem_map: &mut MemoryMap, irq_ctl: &mut Interrupts) {
        let mut current_scanline = self.vertical_count.get_current_scanline() as u32;
        match self.current_state {
            GpuState::HDraw => {
                self.display_status.set_hblank_flag(1);

                if self.display_status.get_hblank_irq_enable() == 1 {
                    irq_ctl.if_interrupt.set_lcd_h_blank(1);
                }

                self.current_state = GpuState::HBlank;
                self.cycles_to_next_state = HBLANK_CYCLES;
            },
            GpuState::HBlank => {
                self.update_vcount((current_scanline + 1) as u8, irq_ctl);
                current_scanline += 1;
                self.display_status.set_hblank_flag(0);

                if current_scanline < DISPLAY_HEIGHT {
                    // render scanline
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

                        },
                        2 => {

                        },
                        3 => {
                            self.render_mode_3(mem_map);
                        },
                        4 => {
                            self.render_mode_4(mem_map);
                        },
                        5 => {
                            self.render_mode_5(mem_map);
                        }
                        _ => panic!("Unimplemented mode: {}", current_mode)
                    }

                    // composite the backgrounds
                    self.composite_background();

                    // update refrence points at end of scanline
                    for i in 0..2 {
                        let internal_x = bitutils::sign_extend_u32(self.bg_affine_components[i].refrence_point_x_internal, 27) as i32;
                        let internal_y =  bitutils::sign_extend_u32(self.bg_affine_components[i].refrence_point_y_internal, 27) as i32;
                        let pb = i32::from(&self.bg_affine_components[i].rotation_scaling_param_b);
                        let pd = i32::from(&self.bg_affine_components[i].rotation_scaling_param_d);

                        self.bg_affine_components[i].refrence_point_x_internal = (pb + internal_x) as u32; //t_register((internal_x + pb) as u32);
                        self.bg_affine_components[i].refrence_point_y_internal = (pd + internal_y) as u32;
                    }

                    self.current_state = GpuState::HDraw;
                    self.cycles_to_next_state = HDRAW_CYCLES;
                } else {                    
                    for i in 0..2 {
                        self.bg_affine_components[i].refrence_point_x_internal = self.bg_affine_components[i].refrence_point_x_external.get_register();
                        self.bg_affine_components[i].refrence_point_y_internal = self.bg_affine_components[i].refrence_point_y_external.get_register();
                    }

                    // do irq stuff
                    if self.display_status.get_vblank_irq_enable() == 1 {
                        irq_ctl.if_interrupt.set_lcd_v_blank(1);
                    }

                    // do dma stuff
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
                    self.cycles_to_next_state = SCANLINE_CYCLES;
                } else {
                    self.display_status.set_vblank_flag(0);
                    self.update_vcount(0, irq_ctl);
                    current_scanline = 0;
                    self.current_state = GpuState::HDraw;
                    self.cycles_to_next_state = HDRAW_CYCLES;
                    self.frame_ready = true;
                }
            }
        }
    }

    pub fn update_vcount(&mut self, value: u8, irq_ctl: &mut Interrupts) {
        self.vertical_count.set_current_scanline(value);
        let vcount_setting = self.display_status.get_vcount_setting();
        self.display_status.set_vcounter_flag((vcount_setting == self.vertical_count.get_current_scanline()) as u8);

        if self.display_status.get_vcounter_irq_enable() == 1 && self.display_status.get_vcounter_flag() == 1{
            irq_ctl.if_interrupt.set_lcd_v_counter(1);
        }
    }

    pub fn render_mode_3(&mut self, mem_map: &mut MemoryMap) {
        let map_start_address = 0x06000000;
        let pa = i32::from(&self.bg_affine_components[0].rotation_scaling_param_a);
        let pc = i32::from(&self.bg_affine_components[0].rotation_scaling_param_c);
        let ref_point_x = bitutils::sign_extend_u32(self.bg_affine_components[0].refrence_point_x_internal, 27) as i32;
        let ref_point_y =  bitutils::sign_extend_u32(self.bg_affine_components[0].refrence_point_y_internal, 27) as i32;

        for x in 0..DISPLAY_WIDTH {
            let t = ((ref_point_x + (x as i32) * pa) >> 8, (ref_point_y + (x as i32) * pc) >> 8);
            // TODO check outside of viewport
            let bitmap_index = ((DISPLAY_WIDTH as u32) * (t.1 as u32) + (t.0 as u32)) as u32;
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
            let t = ((ref_point_x + (x as i32) * pa) >> 8, (ref_point_y + (x as i32) * pc) >> 8);
            // TODO check outside of viewport
            let bitmap_index = ((DISPLAY_WIDTH as u32) * (t.1 as u32) + (t.0 as u32)) as u32;
            let bitmap_offset = page_ofs + bitmap_index;
            let index = mem_map.read_u8(bitmap_offset) as u32;
            let color = Rgb15::new(mem_map.read_u16((2 * index) + 0x05000000));
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

    pub fn render_obj(&mut self, mem_map: &mut MemoryMap) {

    }

    pub fn render_bg(&mut self, mem_map: &mut MemoryMap, bg_number: usize) {
        let (vertical_offset, horizontal_offset) = self.backgrounds[bg_number].get_offsets();
        let tileset_location = self.backgrounds[bg_number].control.get_tileset_location();
        let tilemap_location = self.backgrounds[bg_number].control.get_tilemap_location();
        let (background_width, background_height) = self.backgrounds[bg_number].control.get_background_dimensions();

        let pixel_format = self.backgrounds[bg_number].control.get_pixel_format();
        let tile_size = self.backgrounds[bg_number].control.get_tilesize();

        let current_scanline = self.vertical_count.get_current_scanline() as u32;
        let mut x = 0;

        let background_x = (x + horizontal_offset) % background_width;
        let background_y = (current_scanline + vertical_offset) % background_height;

        let mut sbb: u32 = 0;
        if background_width == 256 && background_height == 256 {
            sbb = 0;
        } else if background_width == 512 && background_height == 256 {
            sbb = background_x / 256;
        } else if background_width == 256 && background_height == 512 {
            sbb = background_y / 256;
        } else if background_width == 512 && background_height == 512 {
            sbb = (2 * (background_y / 256) + (background_x / 256)) as u32;
        }

        let mut se_row = (background_x / 8) % 32;
        let se_column = (background_y / 8) % 32;

        let mut start_tile_x = background_x % 8;
        let tile_py = background_y % 8;

        loop {
            let mut map_address = tilemap_location + 0x800u32 * sbb + 2u32 * (32 * se_column + se_row);
            for _ in se_row..32 {
                let entry_value = TileMapEntry::from(mem_map.read_u16(map_address));
                let tile_address = tileset_location + (entry_value.tile_index as u32) * tile_size;

                for tile_px in start_tile_x..8 {
                    let pixel_x = if entry_value.vertical_flip { 7 - tile_px } else { tile_px };
                    let pixel_y = if entry_value.horizontal_flip { 7 - tile_py } else { tile_py };
                    let pixel_index = match pixel_format {
                        PixelFormat::EightBit => {
                            let pixel_index_address = tile_address + (8 * pixel_y + pixel_x);
                            mem_map.read_u8(pixel_index_address)
                        },
                        PixelFormat::FourBit => {
                            let pixel_index_address = tile_address + (4 * pixel_y + (pixel_x / 2));
                            let value = mem_map.read_u8(pixel_index_address);
                            if pixel_x & 1 != 0 {
                                (value >> 4)
                            } else {
                                value & 0xf
                            }
                        }
                    } as u32;

                    let palette_bank = match pixel_format {
                        PixelFormat::FourBit => entry_value.palette_bank as u32,
                        PixelFormat::EightBit => 0u32,
                    };

                    let color = if pixel_index == 0 || (palette_bank != 0 && pixel_index % 16 == 0) {
                        Rgb15::new(0x8000)
                    } else {
                        let palette_ram_index = 2 * pixel_index + 0x20 * palette_bank;
                        Rgb15::new(mem_map.read_u16(palette_ram_index + 0x500_0000u32))
                    };

                    self.backgrounds[bg_number].scan_line[x as usize] = color;
                    x += 1;
                    if DISPLAY_WIDTH == x {
                        return;
                    }
                }
                start_tile_x = 0;
                map_address += 2;
            }
            se_row = 0;
            if background_width == 512 {
                sbb = sbb ^ 1;
            }
        }
    }

    pub fn composite_background(&mut self) {
        let current_scanline = self.vertical_count.get_current_scanline() as u32;

        for x in 0..DISPLAY_WIDTH {
            let mut top_layer_index = 3;
            for i in (0..4).rev() {
                if self.display_control.should_display(i as u8) && !self.backgrounds[i].scan_line[x as usize].is_transparent() {
                    top_layer_index = i;
                }
            }
            let frame_buffer_index = ((DISPLAY_WIDTH as u32) * (current_scanline as u32) + (x as u32)) as usize;
            self.frame_buffer[frame_buffer_index] = self.backgrounds[top_layer_index].scan_line[x as usize].to_0rgb();
        }
    }
}