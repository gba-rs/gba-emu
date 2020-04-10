use crate::memory::lcd_io_registers::*;
use crate::memory::memory_map::MemoryMap;
use crate::interrupts::interrupts::Interrupts;
use super::{rgb15::Rgb15, tile_map_entry::TileMapEntry};
use crate::operations::bitutils;
use crate::dma::DMAController;
use std::cell::RefCell;
use std::rc::Rc;
use log::debug;

pub const DISPLAY_WIDTH: u32 = 240;
pub const DISPLAY_HEIGHT: u32 = 160;
pub const VBLANK_LENGTH: u32 = 68;

pub const HDRAW_CYCLES: i64 = 960;
pub const HBLANK_CYCLES: i64 = 272;
pub const SCANLINE_CYCLES: i64 = 1232;
pub const VDRAW_CYCLES: i64 = 197120;
pub const VBLANK_CYCLES: i64 = 83776;

pub enum GpuState {
    HDraw,
    HBlank,
    VBlank
}

pub struct Object {
    pub attr0: ObjAttribute0,
    pub attr1: ObjAttribute1,
    pub attr2: ObjAttribute2
}

impl Object {
    pub fn register(&mut self, mem: &Rc<RefCell<Vec<u8>>>){
        self.attr0.register(mem);
        self.attr1.register(mem);
        self.attr2.register(mem);
    }

    pub fn size(&self) -> (i32,i32){
        match (self.attr1.get_obj_size(), self.attr0.get_obj_shape()) {
            (0, 0)  => (8, 8),
            (1, 0)  => (16, 16),
            (2, 0)  => (32, 32),
            (3, 0)  => (64, 64),
            (0, 1)  => (16, 8),
            (1, 1)  => (32, 8),
            (2, 1)  => (32, 16),
            (3, 1)  => (64, 32),
            (0, 2)  => (8, 16),
            (1, 2)  => (8, 32),
            (2, 2)  => (16, 32),
            (3, 2)  => (32, 64),
            _ => (8, 8)
        }
    }
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

    pub cycles_to_next_state: i64,
    pub current_state: GpuState,
    pub frame_ready: bool,
    pub frame_buffer: Vec<u32>,
    pub obj_buffer: Vec<Rgb15>
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
            objects: [
                Object {
                    attr0: ObjAttribute0::new(0),
                    attr1: ObjAttribute1::new(0),
                    attr2: ObjAttribute2::new(0)
                },
                Object {
                    attr0: ObjAttribute0::new(1),
                    attr1: ObjAttribute1::new(1),
                    attr2: ObjAttribute2::new(1)
                },
                Object {
                    attr0: ObjAttribute0::new(2),
                    attr1: ObjAttribute1::new(2),
                    attr2: ObjAttribute2::new(2)
                },
                Object {
                    attr0: ObjAttribute0::new(3),
                    attr1: ObjAttribute1::new(3),
                    attr2: ObjAttribute2::new(3)
                },
                Object {
                    attr0: ObjAttribute0::new(4),
                    attr1: ObjAttribute1::new(4),
                    attr2: ObjAttribute2::new(4)
                },
                Object {
                    attr0: ObjAttribute0::new(5),
                    attr1: ObjAttribute1::new(5),
                    attr2: ObjAttribute2::new(5)
                },
                Object {
                    attr0: ObjAttribute0::new(6),
                    attr1: ObjAttribute1::new(6),
                    attr2: ObjAttribute2::new(6)
                },
                Object {
                    attr0: ObjAttribute0::new(7),
                    attr1: ObjAttribute1::new(7),
                    attr2: ObjAttribute2::new(7)
                },
                Object {
                    attr0: ObjAttribute0::new(8),
                    attr1: ObjAttribute1::new(8),
                    attr2: ObjAttribute2::new(8)
                },
                Object {
                    attr0: ObjAttribute0::new(9),
                    attr1: ObjAttribute1::new(9),
                    attr2: ObjAttribute2::new(9)
                },
                Object {
                    attr0: ObjAttribute0::new(10),
                    attr1: ObjAttribute1::new(10),
                    attr2: ObjAttribute2::new(10)
                },
                Object {
                    attr0: ObjAttribute0::new(11),
                    attr1: ObjAttribute1::new(11),
                    attr2: ObjAttribute2::new(11)
                },
                Object {
                    attr0: ObjAttribute0::new(12),
                    attr1: ObjAttribute1::new(12),
                    attr2: ObjAttribute2::new(12)
                },
                Object {
                    attr0: ObjAttribute0::new(13),
                    attr1: ObjAttribute1::new(13),
                    attr2: ObjAttribute2::new(13)
                },
                Object {
                    attr0: ObjAttribute0::new(14),
                    attr1: ObjAttribute1::new(14),
                    attr2: ObjAttribute2::new(14)
                },
                Object {
                    attr0: ObjAttribute0::new(15),
                    attr1: ObjAttribute1::new(15),
                    attr2: ObjAttribute2::new(15)
                },
                Object {
                    attr0: ObjAttribute0::new(16),
                    attr1: ObjAttribute1::new(16),
                    attr2: ObjAttribute2::new(16)
                },
                Object {
                    attr0: ObjAttribute0::new(17),
                    attr1: ObjAttribute1::new(17),
                    attr2: ObjAttribute2::new(17)
                },
                Object {
                    attr0: ObjAttribute0::new(18),
                    attr1: ObjAttribute1::new(18),
                    attr2: ObjAttribute2::new(18)
                },
                Object {
                    attr0: ObjAttribute0::new(19),
                    attr1: ObjAttribute1::new(19),
                    attr2: ObjAttribute2::new(19)
                },
                Object {
                    attr0: ObjAttribute0::new(20),
                    attr1: ObjAttribute1::new(20),
                    attr2: ObjAttribute2::new(20)
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
            frame_buffer: vec![0; (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize],
            obj_buffer: vec![Rgb15::new(0x8000); (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize]
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

                dma_ctl.hblanking = false;

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
                        }
                        _ => panic!("Unimplemented mode: {}", current_mode)
                    }

                    if self.display_control.get_screen_display_obj() == 1 {
                        self.render_obj(mem_map);
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
                    self.cycles_to_next_state = SCANLINE_CYCLES;
                } else {
                    self.display_status.set_vblank_flag(0);

                    dma_ctl.vblanking = false;

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
        for i in 0..128 {
            match self.objects[i].attr0.get_obj_mode() {
                0b10 => continue,
                0b00 => self.render_normal_obj(i, mem_map),
                0b01 | 0b11 => continue,
                _ => unreachable!() 
            };
        }
    }
/**
 * Parse out individual attributes
 * Parse pixel data
 * And pixel data for the line
 */
    fn render_normal_obj(&mut self, sprite_num: usize, mem_map: &mut MemoryMap) {
        let mut sprite = &mut self.objects[sprite_num];
        let current_scanline = self.vertical_count.get_current_scanline() as i32;
        let mut obj_x = sprite.attr1.get_x_coordinate() as i16 as i32;
        let mut obj_y = sprite.attr0.get_y_coordinate() as i16 as i32;

        if obj_y >= (DISPLAY_HEIGHT as i32) {
            obj_y -= 1 << 8;
        }
        if obj_x >= (DISPLAY_WIDTH as i32) {
            obj_x -= 1 << 9;
        }
        let (obj_w, obj_h) = sprite.size();

        if !(current_scanline >= obj_y && current_scanline < obj_y + obj_h) {
            return;
        }

        let mode = self.display_control.get_bg_mode();

        let tile_index = sprite.attr2.get_character_name();
        let tile_base = (if mode > 2 { 0x06014000 } else { 0x06010000 }) + 0x20 * (tile_index as u32);

        let pixel_format = if sprite.attr0.get_color_flag() == 0 {
            PixelFormat::FourBit
        } else {
            PixelFormat::EightBit
        };

        let tile_size = if sprite.attr0.get_color_flag() == 0 {
            0x20
        } else {
            0x40
        };

        let palette_bank = match pixel_format {
            PixelFormat::FourBit => sprite.attr2.get_palette_number() as u32,
            PixelFormat::EightBit => 0u32,
        };


        let screen_width = DISPLAY_WIDTH as i32;
        let end_x = obj_x + obj_w;
        let tile_array_width = if self.display_control.get_obj_charcter_vram_mapping() == 0 {
            let temp = match pixel_format {
                PixelFormat::FourBit => 32,
                PixelFormat::EightBit => 16
            };
            temp
        } else {
            obj_w / 8
        };

        for x in obj_x..end_x {
            if x < 0 {
                continue;
            } 
            if x >= screen_width {
                break;
            }
            
            // todo check priority here

            let mut sprite_y = current_scanline - obj_y;
            let mut sprite_x = x - obj_x;

            sprite_y = if sprite.attr1.get_vertical_flip() != 0 {
                obj_h - sprite_y - 1
            } else {
                sprite_y
            };

            sprite_x = if sprite.attr1.get_horizontal_flip() != 0 {
                obj_w - sprite_x - 1
            } else {
                sprite_x
            };

            let tile_x = sprite_x % 8;
            let tile_y = sprite_y % 8;
            let tile_addr = tile_base + ((tile_array_width as u32) * ((sprite_y as u32) / 8) + ((sprite_x as u32) / 8)) * (tile_size as u32);
            let pixel_index = match pixel_format {
                PixelFormat::EightBit => {
                    let pixel_index_address = tile_addr + (8 * (tile_y as u32) + (tile_x as u32));
                    mem_map.read_u8(pixel_index_address)
                },
                PixelFormat::FourBit => {
                    let pixel_index_address = tile_addr + (4 * (tile_y as u32) + ((tile_x as u32) / 2));
                    let value = mem_map.read_u8(pixel_index_address);
                    if tile_x & 1 != 0 {
                        (value >> 4)
                    } else {
                        value & 0xf
                    }
                }
            } as u32;

            let color = if pixel_index == 0 || (palette_bank != 0 && pixel_index % 16 == 0) {
                Rgb15::new(0x8000)
            } else {
                let palette_ram_index = 0x200 + 2 * pixel_index + 0x20 * palette_bank;
                Rgb15::new(mem_map.read_u16(palette_ram_index + 0x500_0000u32))
            };

            let obj_buffer_index: usize = (DISPLAY_WIDTH * (current_scanline as u32) + (x as u32)) as usize;
            self.obj_buffer[obj_buffer_index] = color;
        }

    }
    fn render_semi_transp_obj(&mut self, sprite_num: usize, mem_map: &mut MemoryMap) {

    }
    fn render_obj_window(&mut self, sprite_num: usize, mem_map: &mut MemoryMap) {

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

    pub fn render_aff_bg(&mut self, mem_map: &mut MemoryMap, bg_number: usize) {
        let texture_size = 128 << self.backgrounds[bg_number].control.get_screen_size();

        let ref_point_x = self.bg_affine_components[bg_number - 2].refrence_point_x_internal as i32;
        let ref_point_y = self.bg_affine_components[bg_number - 2].refrence_point_y_internal as i32;

        let pa = self.bg_affine_components[bg_number - 2].rotation_scaling_param_a.get_register() as i16 as i32;
        let pc = self.bg_affine_components[bg_number - 2].rotation_scaling_param_c.get_register() as i16 as i32;

        let screen_block = self.backgrounds[bg_number].control.get_tilemap_location();
        let char_block = self.backgrounds[bg_number].control.get_tileset_location();

        let wraparound = self.backgrounds[bg_number].control.get_display_area_overflow();

        for screen_x in 0..(DISPLAY_WIDTH as i32) {
            let mut point_x = ((ref_point_x + screen_x * pa) >> 8);
            let mut point_y = ((ref_point_y + screen_x * pc) >> 8);

            let map_addr = screen_block + ((texture_size as u32 / 8) * (point_y as u32 / 8) + (point_x as u32 / 8));
            let tile_index = mem_map.read_u8(map_addr) as u32;
            let tile_addr = char_block + tile_index * 0x40;

            let pixel_index_address = tile_addr + (8 * ((point_y % 8) as u32) + ((point_x % 8) as u32));
            let pixel_index = mem_map.read_u8(pixel_index_address) as u32;


            let palette_ram_index = 2 * pixel_index;
            let color = Rgb15::new(mem_map.read_u16(palette_ram_index + 0x500_0000u32));

            self.backgrounds[bg_number].scan_line[screen_x as usize] = color;
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

            let mut color = self.backgrounds[top_layer_index].scan_line[x as usize].to_0rgb();

            let obj_buffer_index: usize = (DISPLAY_WIDTH * (current_scanline as u32) + (x as u32)) as usize;
            
            if !self.obj_buffer[obj_buffer_index].is_transparent() {
                color = self.obj_buffer[obj_buffer_index].to_0rgb();
                // log::info!("We are drawing a sprite pixel: {:?}", self.obj_buffer[obj_buffer_index]);
            }

            let frame_buffer_index = ((DISPLAY_WIDTH as u32) * (current_scanline as u32) + (x as u32)) as usize;
            self.frame_buffer[frame_buffer_index] = color;
        }
    }
}