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
            objects: [
                Object {
                    attr0: ObjAttribute0::new(0),
                    attr1: ObjAttribute1::new(0),
                    attr2: ObjAttribute2::new(0),
                },
                Object {
                    attr0: ObjAttribute0::new(1),
                    attr1: ObjAttribute1::new(1),
                    attr2: ObjAttribute2::new(1),
                },
                Object {
                    attr0: ObjAttribute0::new(2),
                    attr1: ObjAttribute1::new(2),
                    attr2: ObjAttribute2::new(2),
                },
                Object {
                    attr0: ObjAttribute0::new(3),
                    attr1: ObjAttribute1::new(3),
                    attr2: ObjAttribute2::new(3),
                },
                Object {
                    attr0: ObjAttribute0::new(4),
                    attr1: ObjAttribute1::new(4),
                    attr2: ObjAttribute2::new(4),
                },
                Object {
                    attr0: ObjAttribute0::new(5),
                    attr1: ObjAttribute1::new(5),
                    attr2: ObjAttribute2::new(5),
                },
                Object {
                    attr0: ObjAttribute0::new(6),
                    attr1: ObjAttribute1::new(6),
                    attr2: ObjAttribute2::new(6),
                },
                Object {
                    attr0: ObjAttribute0::new(7),
                    attr1: ObjAttribute1::new(7),
                    attr2: ObjAttribute2::new(7),
                },
                Object {
                    attr0: ObjAttribute0::new(8),
                    attr1: ObjAttribute1::new(8),
                    attr2: ObjAttribute2::new(8),
                },
                Object {
                    attr0: ObjAttribute0::new(9),
                    attr1: ObjAttribute1::new(9),
                    attr2: ObjAttribute2::new(9),
                },
                Object {
                    attr0: ObjAttribute0::new(10),
                    attr1: ObjAttribute1::new(10),
                    attr2: ObjAttribute2::new(10),
                },
                Object {
                    attr0: ObjAttribute0::new(11),
                    attr1: ObjAttribute1::new(11),
                    attr2: ObjAttribute2::new(11),
                },
                Object {
                    attr0: ObjAttribute0::new(12),
                    attr1: ObjAttribute1::new(12),
                    attr2: ObjAttribute2::new(12),
                },
                Object {
                    attr0: ObjAttribute0::new(13),
                    attr1: ObjAttribute1::new(13),
                    attr2: ObjAttribute2::new(13),
                },
                Object {
                    attr0: ObjAttribute0::new(14),
                    attr1: ObjAttribute1::new(14),
                    attr2: ObjAttribute2::new(14),
                },
                Object {
                    attr0: ObjAttribute0::new(15),
                    attr1: ObjAttribute1::new(15),
                    attr2: ObjAttribute2::new(15),
                },
                Object {
                    attr0: ObjAttribute0::new(16),
                    attr1: ObjAttribute1::new(16),
                    attr2: ObjAttribute2::new(16),
                },
                Object {
                    attr0: ObjAttribute0::new(17),
                    attr1: ObjAttribute1::new(17),
                    attr2: ObjAttribute2::new(17),
                },
                Object {
                    attr0: ObjAttribute0::new(18),
                    attr1: ObjAttribute1::new(18),
                    attr2: ObjAttribute2::new(18),
                },
                Object {
                    attr0: ObjAttribute0::new(19),
                    attr1: ObjAttribute1::new(19),
                    attr2: ObjAttribute2::new(19),
                },
                Object {
                    attr0: ObjAttribute0::new(20),
                    attr1: ObjAttribute1::new(20),
                    attr2: ObjAttribute2::new(20),
                },
                Object {
                    attr0: ObjAttribute0::new(21),
                    attr1: ObjAttribute1::new(21),
                    attr2: ObjAttribute2::new(21)
                },
                Object {
                    attr0: ObjAttribute0::new(22),
                    attr1: ObjAttribute1::new(22),
                    attr2: ObjAttribute2::new(22)
                },
                Object {
                    attr0: ObjAttribute0::new(23),
                    attr1: ObjAttribute1::new(23),
                    attr2: ObjAttribute2::new(23)
                },
                Object {
                    attr0: ObjAttribute0::new(24),
                    attr1: ObjAttribute1::new(24),
                    attr2: ObjAttribute2::new(24)
                },
                Object {
                    attr0: ObjAttribute0::new(25),
                    attr1: ObjAttribute1::new(25),
                    attr2: ObjAttribute2::new(25)
                },
                Object {
                    attr0: ObjAttribute0::new(26),
                    attr1: ObjAttribute1::new(26),
                    attr2: ObjAttribute2::new(26)
                },
                Object {
                    attr0: ObjAttribute0::new(27),
                    attr1: ObjAttribute1::new(27),
                    attr2: ObjAttribute2::new(27)
                },
                Object {
                    attr0: ObjAttribute0::new(28),
                    attr1: ObjAttribute1::new(28),
                    attr2: ObjAttribute2::new(28)
                },
                Object {
                    attr0: ObjAttribute0::new(29),
                    attr1: ObjAttribute1::new(29),
                    attr2: ObjAttribute2::new(29)
                },
                Object {
                    attr0: ObjAttribute0::new(30),
                    attr1: ObjAttribute1::new(30),
                    attr2: ObjAttribute2::new(30)
                },
                Object {
                    attr0: ObjAttribute0::new(31),
                    attr1: ObjAttribute1::new(31),
                    attr2: ObjAttribute2::new(31)
                },
                Object {
                    attr0: ObjAttribute0::new(32),
                    attr1: ObjAttribute1::new(32),
                    attr2: ObjAttribute2::new(32)
                },
                Object {
                    attr0: ObjAttribute0::new(33),
                    attr1: ObjAttribute1::new(33),
                    attr2: ObjAttribute2::new(33)
                },
                Object {
                    attr0: ObjAttribute0::new(34),
                    attr1: ObjAttribute1::new(34),
                    attr2: ObjAttribute2::new(34)
                },
                Object {
                    attr0: ObjAttribute0::new(35),
                    attr1: ObjAttribute1::new(35),
                    attr2: ObjAttribute2::new(35)
                },
                Object {
                    attr0: ObjAttribute0::new(36),
                    attr1: ObjAttribute1::new(36),
                    attr2: ObjAttribute2::new(36)
                },
                Object {
                    attr0: ObjAttribute0::new(37),
                    attr1: ObjAttribute1::new(37),
                    attr2: ObjAttribute2::new(37)
                },
                Object {
                    attr0: ObjAttribute0::new(38),
                    attr1: ObjAttribute1::new(38),
                    attr2: ObjAttribute2::new(38)
                },
                Object {
                    attr0: ObjAttribute0::new(39),
                    attr1: ObjAttribute1::new(39),
                    attr2: ObjAttribute2::new(39)
                },
                Object {
                    attr0: ObjAttribute0::new(40),
                    attr1: ObjAttribute1::new(40),
                    attr2: ObjAttribute2::new(40)
                },
                Object {
                    attr0: ObjAttribute0::new(41),
                    attr1: ObjAttribute1::new(41),
                    attr2: ObjAttribute2::new(41)
                },
                Object {
                    attr0: ObjAttribute0::new(42),
                    attr1: ObjAttribute1::new(42),
                    attr2: ObjAttribute2::new(42)
                },
                Object {
                    attr0: ObjAttribute0::new(43),
                    attr1: ObjAttribute1::new(43),
                    attr2: ObjAttribute2::new(43)
                },
                Object {
                    attr0: ObjAttribute0::new(44),
                    attr1: ObjAttribute1::new(44),
                    attr2: ObjAttribute2::new(44)
                },
                Object {
                    attr0: ObjAttribute0::new(45),
                    attr1: ObjAttribute1::new(45),
                    attr2: ObjAttribute2::new(45)
                },
                Object {
                    attr0: ObjAttribute0::new(46),
                    attr1: ObjAttribute1::new(46),
                    attr2: ObjAttribute2::new(46)
                },
                Object {
                    attr0: ObjAttribute0::new(47),
                    attr1: ObjAttribute1::new(47),
                    attr2: ObjAttribute2::new(47)
                },
                Object {
                    attr0: ObjAttribute0::new(48),
                    attr1: ObjAttribute1::new(48),
                    attr2: ObjAttribute2::new(48)
                },
                Object {
                    attr0: ObjAttribute0::new(49),
                    attr1: ObjAttribute1::new(49),
                    attr2: ObjAttribute2::new(49)
                },
                Object {
                    attr0: ObjAttribute0::new(50),
                    attr1: ObjAttribute1::new(50),
                    attr2: ObjAttribute2::new(50)
                },
                Object {
                    attr0: ObjAttribute0::new(51),
                    attr1: ObjAttribute1::new(51),
                    attr2: ObjAttribute2::new(51)
                },
                Object {
                    attr0: ObjAttribute0::new(52),
                    attr1: ObjAttribute1::new(52),
                    attr2: ObjAttribute2::new(52)
                },
                Object {
                    attr0: ObjAttribute0::new(53),
                    attr1: ObjAttribute1::new(53),
                    attr2: ObjAttribute2::new(53)
                },
                Object {
                    attr0: ObjAttribute0::new(54),
                    attr1: ObjAttribute1::new(54),
                    attr2: ObjAttribute2::new(54)
                },
                Object {
                    attr0: ObjAttribute0::new(55),
                    attr1: ObjAttribute1::new(55),
                    attr2: ObjAttribute2::new(55)
                },
                Object {
                    attr0: ObjAttribute0::new(56),
                    attr1: ObjAttribute1::new(56),
                    attr2: ObjAttribute2::new(56)
                },
                Object {
                    attr0: ObjAttribute0::new(57),
                    attr1: ObjAttribute1::new(57),
                    attr2: ObjAttribute2::new(57)
                },
                Object {
                    attr0: ObjAttribute0::new(58),
                    attr1: ObjAttribute1::new(58),
                    attr2: ObjAttribute2::new(58)
                },
                Object {
                    attr0: ObjAttribute0::new(59),
                    attr1: ObjAttribute1::new(59),
                    attr2: ObjAttribute2::new(59)
                },
                Object {
                    attr0: ObjAttribute0::new(60),
                    attr1: ObjAttribute1::new(60),
                    attr2: ObjAttribute2::new(60)
                },
                Object {
                    attr0: ObjAttribute0::new(61),
                    attr1: ObjAttribute1::new(61),
                    attr2: ObjAttribute2::new(61)
                },
                Object {
                    attr0: ObjAttribute0::new(62),
                    attr1: ObjAttribute1::new(62),
                    attr2: ObjAttribute2::new(62)
                },
                Object {
                    attr0: ObjAttribute0::new(63),
                    attr1: ObjAttribute1::new(63),
                    attr2: ObjAttribute2::new(63)
                },
                Object {
                    attr0: ObjAttribute0::new(64),
                    attr1: ObjAttribute1::new(64),
                    attr2: ObjAttribute2::new(64)
                },
                Object {
                    attr0: ObjAttribute0::new(65),
                    attr1: ObjAttribute1::new(65),
                    attr2: ObjAttribute2::new(65)
                },
                Object {
                    attr0: ObjAttribute0::new(66),
                    attr1: ObjAttribute1::new(66),
                    attr2: ObjAttribute2::new(66)
                },
                Object {
                    attr0: ObjAttribute0::new(67),
                    attr1: ObjAttribute1::new(67),
                    attr2: ObjAttribute2::new(67)
                },
                Object {
                    attr0: ObjAttribute0::new(68),
                    attr1: ObjAttribute1::new(68),
                    attr2: ObjAttribute2::new(68)
                },
                Object {
                    attr0: ObjAttribute0::new(69),
                    attr1: ObjAttribute1::new(69),
                    attr2: ObjAttribute2::new(69)
                },
                Object {
                    attr0: ObjAttribute0::new(70),
                    attr1: ObjAttribute1::new(70),
                    attr2: ObjAttribute2::new(70)
                },
                Object {
                    attr0: ObjAttribute0::new(71),
                    attr1: ObjAttribute1::new(71),
                    attr2: ObjAttribute2::new(71)
                },
                Object {
                    attr0: ObjAttribute0::new(72),
                    attr1: ObjAttribute1::new(72),
                    attr2: ObjAttribute2::new(72)
                },
                Object {
                    attr0: ObjAttribute0::new(73),
                    attr1: ObjAttribute1::new(73),
                    attr2: ObjAttribute2::new(73)
                },
                Object {
                    attr0: ObjAttribute0::new(74),
                    attr1: ObjAttribute1::new(74),
                    attr2: ObjAttribute2::new(74)
                },
                Object {
                    attr0: ObjAttribute0::new(75),
                    attr1: ObjAttribute1::new(75),
                    attr2: ObjAttribute2::new(75)
                },
                Object {
                    attr0: ObjAttribute0::new(76),
                    attr1: ObjAttribute1::new(76),
                    attr2: ObjAttribute2::new(76)
                },
                Object {
                    attr0: ObjAttribute0::new(77),
                    attr1: ObjAttribute1::new(77),
                    attr2: ObjAttribute2::new(77)
                },
                Object {
                    attr0: ObjAttribute0::new(78),
                    attr1: ObjAttribute1::new(78),
                    attr2: ObjAttribute2::new(78)
                },
                Object {
                    attr0: ObjAttribute0::new(79),
                    attr1: ObjAttribute1::new(79),
                    attr2: ObjAttribute2::new(79)
                },
                Object {
                    attr0: ObjAttribute0::new(80),
                    attr1: ObjAttribute1::new(80),
                    attr2: ObjAttribute2::new(80)
                },
                Object {
                    attr0: ObjAttribute0::new(81),
                    attr1: ObjAttribute1::new(81),
                    attr2: ObjAttribute2::new(81)
                },
                Object {
                    attr0: ObjAttribute0::new(82),
                    attr1: ObjAttribute1::new(82),
                    attr2: ObjAttribute2::new(82)
                },
                Object {
                    attr0: ObjAttribute0::new(83),
                    attr1: ObjAttribute1::new(83),
                    attr2: ObjAttribute2::new(83)
                },
                Object {
                    attr0: ObjAttribute0::new(84),
                    attr1: ObjAttribute1::new(84),
                    attr2: ObjAttribute2::new(84)
                },
                Object {
                    attr0: ObjAttribute0::new(85),
                    attr1: ObjAttribute1::new(85),
                    attr2: ObjAttribute2::new(85)
                },
                Object {
                    attr0: ObjAttribute0::new(86),
                    attr1: ObjAttribute1::new(86),
                    attr2: ObjAttribute2::new(86)
                },
                Object {
                    attr0: ObjAttribute0::new(87),
                    attr1: ObjAttribute1::new(87),
                    attr2: ObjAttribute2::new(87)
                },
                Object {
                    attr0: ObjAttribute0::new(88),
                    attr1: ObjAttribute1::new(88),
                    attr2: ObjAttribute2::new(88)
                },
                Object {
                    attr0: ObjAttribute0::new(89),
                    attr1: ObjAttribute1::new(89),
                    attr2: ObjAttribute2::new(89)
                },
                Object {
                    attr0: ObjAttribute0::new(90),
                    attr1: ObjAttribute1::new(90),
                    attr2: ObjAttribute2::new(90)
                },
                Object {
                    attr0: ObjAttribute0::new(91),
                    attr1: ObjAttribute1::new(91),
                    attr2: ObjAttribute2::new(91)
                },
                Object {
                    attr0: ObjAttribute0::new(92),
                    attr1: ObjAttribute1::new(92),
                    attr2: ObjAttribute2::new(92)
                },
                Object {
                    attr0: ObjAttribute0::new(93),
                    attr1: ObjAttribute1::new(93),
                    attr2: ObjAttribute2::new(93)
                },
                Object {
                    attr0: ObjAttribute0::new(94),
                    attr1: ObjAttribute1::new(94),
                    attr2: ObjAttribute2::new(94)
                },
                Object {
                    attr0: ObjAttribute0::new(95),
                    attr1: ObjAttribute1::new(95),
                    attr2: ObjAttribute2::new(95)
                },
                Object {
                    attr0: ObjAttribute0::new(96),
                    attr1: ObjAttribute1::new(96),
                    attr2: ObjAttribute2::new(96)
                },
                Object {
                    attr0: ObjAttribute0::new(97),
                    attr1: ObjAttribute1::new(97),
                    attr2: ObjAttribute2::new(97)
                },
                Object {
                    attr0: ObjAttribute0::new(98),
                    attr1: ObjAttribute1::new(98),
                    attr2: ObjAttribute2::new(98)
                },
                Object {
                    attr0: ObjAttribute0::new(99),
                    attr1: ObjAttribute1::new(99),
                    attr2: ObjAttribute2::new(99)
                },
                Object {
                    attr0: ObjAttribute0::new(100),
                    attr1: ObjAttribute1::new(100),
                    attr2: ObjAttribute2::new(100)
                },
                Object {
                    attr0: ObjAttribute0::new(101),
                    attr1: ObjAttribute1::new(101),
                    attr2: ObjAttribute2::new(101)
                },
                Object {
                    attr0: ObjAttribute0::new(102),
                    attr1: ObjAttribute1::new(102),
                    attr2: ObjAttribute2::new(102)
                },
                Object {
                    attr0: ObjAttribute0::new(103),
                    attr1: ObjAttribute1::new(103),
                    attr2: ObjAttribute2::new(103)
                },
                Object {
                    attr0: ObjAttribute0::new(104),
                    attr1: ObjAttribute1::new(104),
                    attr2: ObjAttribute2::new(104)
                },
                Object {
                    attr0: ObjAttribute0::new(105),
                    attr1: ObjAttribute1::new(105),
                    attr2: ObjAttribute2::new(105)
                },
                Object {
                    attr0: ObjAttribute0::new(106),
                    attr1: ObjAttribute1::new(106),
                    attr2: ObjAttribute2::new(106)
                },
                Object {
                    attr0: ObjAttribute0::new(107),
                    attr1: ObjAttribute1::new(107),
                    attr2: ObjAttribute2::new(107)
                },
                Object {
                    attr0: ObjAttribute0::new(108),
                    attr1: ObjAttribute1::new(108),
                    attr2: ObjAttribute2::new(108)
                },
                Object {
                    attr0: ObjAttribute0::new(109),
                    attr1: ObjAttribute1::new(109),
                    attr2: ObjAttribute2::new(109)
                },
                Object {
                    attr0: ObjAttribute0::new(110),
                    attr1: ObjAttribute1::new(110),
                    attr2: ObjAttribute2::new(110)
                },
                Object {
                    attr0: ObjAttribute0::new(111),
                    attr1: ObjAttribute1::new(111),
                    attr2: ObjAttribute2::new(111)
                },                
                Object {
                    attr0: ObjAttribute0::new(112),
                    attr1: ObjAttribute1::new(112),
                    attr2: ObjAttribute2::new(112)
                },
                Object {
                    attr0: ObjAttribute0::new(113),
                    attr1: ObjAttribute1::new(113),
                    attr2: ObjAttribute2::new(113)
                },
                Object {
                    attr0: ObjAttribute0::new(114),
                    attr1: ObjAttribute1::new(114),
                    attr2: ObjAttribute2::new(114)
                },
                Object {
                    attr0: ObjAttribute0::new(115),
                    attr1: ObjAttribute1::new(115),
                    attr2: ObjAttribute2::new(115)
                },
                Object {
                    attr0: ObjAttribute0::new(116),
                    attr1: ObjAttribute1::new(116),
                    attr2: ObjAttribute2::new(116)
                },
                Object {
                    attr0: ObjAttribute0::new(117),
                    attr1: ObjAttribute1::new(117),
                    attr2: ObjAttribute2::new(117)
                },
                Object {
                    attr0: ObjAttribute0::new(118),
                    attr1: ObjAttribute1::new(118),
                    attr2: ObjAttribute2::new(118)
                },
                Object {
                    attr0: ObjAttribute0::new(119),
                    attr1: ObjAttribute1::new(119),
                    attr2: ObjAttribute2::new(119)
                },
                Object {
                    attr0: ObjAttribute0::new(120),
                    attr1: ObjAttribute1::new(120),
                    attr2: ObjAttribute2::new(120)
                },
                Object {
                    attr0: ObjAttribute0::new(121),
                    attr1: ObjAttribute1::new(121),
                    attr2: ObjAttribute2::new(121)
                },
                Object {
                    attr0: ObjAttribute0::new(122),
                    attr1: ObjAttribute1::new(122),
                    attr2: ObjAttribute2::new(122)
                },
                Object {
                    attr0: ObjAttribute0::new(123),
                    attr1: ObjAttribute1::new(123),
                    attr2: ObjAttribute2::new(123)
                },
                Object {
                    attr0: ObjAttribute0::new(124),
                    attr1: ObjAttribute1::new(124),
                    attr2: ObjAttribute2::new(124)
                },
                Object {
                    attr0: ObjAttribute0::new(125),
                    attr1: ObjAttribute1::new(125),
                    attr2: ObjAttribute2::new(125)
                },
                Object {
                    attr0: ObjAttribute0::new(126),
                    attr1: ObjAttribute1::new(126),
                    attr2: ObjAttribute2::new(126)
                },
                Object {
                    attr0: ObjAttribute0::new(127),
                    attr1: ObjAttribute1::new(127),
                    attr2: ObjAttribute2::new(127)
                }
            ],
            aff_matrices: [
                AffineMatrix{
                    pa: OBJRotScaleParam::new(0),
                    pb: OBJRotScaleParam::new(1),
                    pc: OBJRotScaleParam::new(2),
                    pd: OBJRotScaleParam::new(3)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(4),
                    pb: OBJRotScaleParam::new(5),
                    pc: OBJRotScaleParam::new(6),
                    pd: OBJRotScaleParam::new(7)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(8),
                    pb: OBJRotScaleParam::new(9),
                    pc: OBJRotScaleParam::new(10),
                    pd: OBJRotScaleParam::new(11)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(12),
                    pb: OBJRotScaleParam::new(13),
                    pc: OBJRotScaleParam::new(14),
                    pd: OBJRotScaleParam::new(15)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(16),
                    pb: OBJRotScaleParam::new(17),
                    pc: OBJRotScaleParam::new(18),
                    pd: OBJRotScaleParam::new(19)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(20),
                    pb: OBJRotScaleParam::new(21),
                    pc: OBJRotScaleParam::new(22),
                    pd: OBJRotScaleParam::new(23)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(24),
                    pb: OBJRotScaleParam::new(25),
                    pc: OBJRotScaleParam::new(26),
                    pd: OBJRotScaleParam::new(27)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(28),
                    pb: OBJRotScaleParam::new(29),
                    pc: OBJRotScaleParam::new(30),
                    pd: OBJRotScaleParam::new(31)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(32),
                    pb: OBJRotScaleParam::new(33),
                    pc: OBJRotScaleParam::new(34),
                    pd: OBJRotScaleParam::new(35)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(36),
                    pb: OBJRotScaleParam::new(37),
                    pc: OBJRotScaleParam::new(38),
                    pd: OBJRotScaleParam::new(39)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(40),
                    pb: OBJRotScaleParam::new(41),
                    pc: OBJRotScaleParam::new(42),
                    pd: OBJRotScaleParam::new(43)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(44),
                    pb: OBJRotScaleParam::new(45),
                    pc: OBJRotScaleParam::new(46),
                    pd: OBJRotScaleParam::new(47)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(48),
                    pb: OBJRotScaleParam::new(49),
                    pc: OBJRotScaleParam::new(50),
                    pd: OBJRotScaleParam::new(51)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(52),
                    pb: OBJRotScaleParam::new(53),
                    pc: OBJRotScaleParam::new(54),
                    pd: OBJRotScaleParam::new(55)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(56),
                    pb: OBJRotScaleParam::new(57),
                    pc: OBJRotScaleParam::new(58),
                    pd: OBJRotScaleParam::new(59)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(60),
                    pb: OBJRotScaleParam::new(61),
                    pc: OBJRotScaleParam::new(62),
                    pd: OBJRotScaleParam::new(63)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(64),
                    pb: OBJRotScaleParam::new(65),
                    pc: OBJRotScaleParam::new(66),
                    pd: OBJRotScaleParam::new(67)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(68),
                    pb: OBJRotScaleParam::new(69),
                    pc: OBJRotScaleParam::new(70),
                    pd: OBJRotScaleParam::new(71)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(72),
                    pb: OBJRotScaleParam::new(73),
                    pc: OBJRotScaleParam::new(74),
                    pd: OBJRotScaleParam::new(75)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(76),
                    pb: OBJRotScaleParam::new(77),
                    pc: OBJRotScaleParam::new(78),
                    pd: OBJRotScaleParam::new(79)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(80),
                    pb: OBJRotScaleParam::new(81),
                    pc: OBJRotScaleParam::new(82),
                    pd: OBJRotScaleParam::new(83)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(84),
                    pb: OBJRotScaleParam::new(85),
                    pc: OBJRotScaleParam::new(86),
                    pd: OBJRotScaleParam::new(87)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(88),
                    pb: OBJRotScaleParam::new(89),
                    pc: OBJRotScaleParam::new(90),
                    pd: OBJRotScaleParam::new(91)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(92),
                    pb: OBJRotScaleParam::new(93),
                    pc: OBJRotScaleParam::new(94),
                    pd: OBJRotScaleParam::new(95)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(96),
                    pb: OBJRotScaleParam::new(97),
                    pc: OBJRotScaleParam::new(98),
                    pd: OBJRotScaleParam::new(99)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(100),
                    pb: OBJRotScaleParam::new(101),
                    pc: OBJRotScaleParam::new(102),
                    pd: OBJRotScaleParam::new(103)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(104),
                    pb: OBJRotScaleParam::new(105),
                    pc: OBJRotScaleParam::new(106),
                    pd: OBJRotScaleParam::new(107)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(108),
                    pb: OBJRotScaleParam::new(109),
                    pc: OBJRotScaleParam::new(110),
                    pd: OBJRotScaleParam::new(111)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(112),
                    pb: OBJRotScaleParam::new(113),
                    pc: OBJRotScaleParam::new(114),
                    pd: OBJRotScaleParam::new(115)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(116),
                    pb: OBJRotScaleParam::new(117),
                    pc: OBJRotScaleParam::new(118),
                    pd: OBJRotScaleParam::new(119)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(120),
                    pb: OBJRotScaleParam::new(121),
                    pc: OBJRotScaleParam::new(122),
                    pd: OBJRotScaleParam::new(123)
                },
                AffineMatrix{
                    pa: OBJRotScaleParam::new(124),
                    pb: OBJRotScaleParam::new(125),
                    pc: OBJRotScaleParam::new(126),
                    pd: OBJRotScaleParam::new(127)
                },
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