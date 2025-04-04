#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::cell::RefCell;
use std::rc::Rc;
use crate::operations::bitutils::*;
use super::GbaMem;
use crate::gpu::graphic_effects::{BlendMode, WindowTypes};
use memory_macros::*;
use serde::{Serialize, Deserialize};

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum PixelFormat {
    FourBit,
    EightBit
}

io_register! (
    DisplayControl => 2, 0x4000000,
    bg_mode: 0, 3,
    cgb_mode: 3, 1,
    display_frame_select: 4, 1,
    hblank_interval_free: 5, 1,
    obj_charcter_vram_mapping: 6, 1,
    forced_blank: 7, 1,
    screen_display_bg0: 8, 1,
    screen_display_bg1: 9, 1,
    screen_display_bg2: 10, 1,
    screen_display_bg3: 11, 1,
    screen_display_obj: 12, 1,
    window_0_display_flag: 13, 1,
    window_1_display_flag: 14, 1,
    obj_window_display_flag: 15, 1
);

impl DisplayControl {
    pub fn should_display(&self, bg_num: u8) -> bool {
        return ((self.get_register() >> (bg_num + 8)) & 0x1) != 0;
    }

    pub fn using_windows(&self) -> bool {
        return self.get_window_0_display_flag() != 0 || 
               self.get_window_1_display_flag() != 0 || 
               self.get_obj_window_display_flag() != 0;
    }
}

io_register! (
    GreenSwap => 2, 0x4000002,
    green_swap: 0, 1
);

io_register! (
    DisplayStatus => 2, 0x4000004,
    vblank_flag: 0, 1,
    hblank_flag: 1, 1,
    vcounter_flag: 2, 1,
    vblank_irq_enable: 3, 1,
    hblank_irq_enable: 4, 1,
    vcounter_irq_enable: 5, 1,
    vcount_setting: 8, 8,
);

io_register! (
    VerticalCount => 2, 0x4000006,
    current_scanline: 0, 8
);

io_register! (
    BG_Control => 2, [0x4000008, 0x400000A, 0x400000C, 0x400000E],
    bg_priority: 0, 2,
    character_base_block: 2, 2,
    mosaic: 6, 1,
    colors: 7, 1,
    screen_base_block: 8, 5,
    display_area_overflow: 13, 1,
    screen_size: 14, 2,
);

impl BG_Control {
    pub fn get_tileset_location(&self) -> u32 {
        return 0x600_0000 + (self.get_character_base_block() as u32) * 0x4000;
    }

    pub fn get_tilemap_location(&self) -> u32 {
        return 0x600_0000 + (self.get_screen_base_block() as u32) * 0x800;
    }

    // pub fn get_background_
    pub fn get_background_dimensions(&self) -> (u32, u32) {
        let bg_size_number = self.get_screen_size() as u32;

        match bg_size_number {
            0 => (256, 256),
            1 => (512, 256),
            2 => (256, 512),
            3 => (512, 512),
            _ => panic!("Invalid screen size")
        }
    }

    pub fn get_pixel_format(&self) -> PixelFormat {
        if self.get_colors() != 0 {
            PixelFormat::EightBit
        } else {
            PixelFormat::FourBit
        }
    }

    pub fn get_tilesize(&self) -> u32 {
        if self.get_colors() != 0 { 64 } else { 32 }
    }
}

io_register! (
    BGOffset => 2, [0x4000010, 0x4000012, 0x4000014, 0x4000016, 0x4000018, 0x400001A, 0x400001C, 0x400001E],
    offset: 0, 9,
);

io_register! (
    BGRefrencePoint => 4, [0x4000028, 0x400002C, 0x4000038, 0x400003C],
    fractional_portion: 0, 8,
    integer_portion: 8, 19,
    sign: 27, 1
);

impl From<&BGRefrencePoint> for i32 {
    fn from(value: &BGRefrencePoint) -> i32 {
        return sign_extend_u32(value.get_register(), 27) as i32;
    }
}

io_register! (
    BGRotScaleParam => 2, [0x4000020, 0x4000022, 0x4000024, 0x4000026, 0x4000030, 0x4000032, 0x4000034, 0x4000036],
    fractional_portion: 0, 8,
    integer_portion: 8, 7,
    sign: 15, 1,
);

impl From<&BGRotScaleParam> for i32 {
    fn from(value: &BGRotScaleParam) -> i32 {
        return value.get_register() as i16 as i32;
    }
}

io_register! (
    OBJRotScaleParam => 2, [
        0x7000006,
        0x700000e,
        0x7000016,
        0x700001e,
        0x7000026,
        0x700002e,
        0x7000036,
        0x700003e,
        0x7000046,
        0x700004e,
        0x7000056,
        0x700005e,
        0x7000066,
        0x700006e,
        0x7000076,
        0x700007e,
        0x7000086,
        0x700008e,
        0x7000096,
        0x700009e,
        0x70000a6,
        0x70000ae,
        0x70000b6,
        0x70000be,
        0x70000c6,
        0x70000ce,
        0x70000d6,
        0x70000de,
        0x70000e6,
        0x70000ee,
        0x70000f6,
        0x70000fe,
        0x7000106,
        0x700010e,
        0x7000116,
        0x700011e,
        0x7000126,
        0x700012e,
        0x7000136,
        0x700013e,
        0x7000146,
        0x700014e,
        0x7000156,
        0x700015e,
        0x7000166,
        0x700016e,
        0x7000176,
        0x700017e,
        0x7000186,
        0x700018e,
        0x7000196,
        0x700019e,
        0x70001a6,
        0x70001ae,
        0x70001b6,
        0x70001be,
        0x70001c6,
        0x70001ce,
        0x70001d6,
        0x70001de,
        0x70001e6,
        0x70001ee,
        0x70001f6,
        0x70001fe,
        0x7000206,
        0x700020e,
        0x7000216,
        0x700021e,
        0x7000226,
        0x700022e,
        0x7000236,
        0x700023e,
        0x7000246,
        0x700024e,
        0x7000256,
        0x700025e,
        0x7000266,
        0x700026e,
        0x7000276,
        0x700027e,
        0x7000286,
        0x700028e,
        0x7000296,
        0x700029e,
        0x70002a6,
        0x70002ae,
        0x70002b6,
        0x70002be,
        0x70002c6,
        0x70002ce,
        0x70002d6,
        0x70002de,
        0x70002e6,
        0x70002ee,
        0x70002f6,
        0x70002fe,
        0x7000306,
        0x700030e,
        0x7000316,
        0x700031e,
        0x7000326,
        0x700032e,
        0x7000336,
        0x700033e,
        0x7000346,
        0x700034e,
        0x7000356,
        0x700035e,
        0x7000366,
        0x700036e,
        0x7000376,
        0x700037e,
        0x7000386,
        0x700038e,
        0x7000396,
        0x700039e,
        0x70003a6,
        0x70003ae,
        0x70003b6,
        0x70003be,
        0x70003c6,
        0x70003ce,
        0x70003d6,
        0x70003de,
        0x70003e6,
        0x70003ee,
        0x70003f6,
        0x70003fe,
    ],
    aff_param: 0, 16,
);

io_register! (
    ObjAttribute0 => 2, [
        117440512usize,
        117440520usize,
        117440528usize,
        117440536usize,
        117440544usize,
        117440552usize,
        117440560usize,
        117440568usize,
        117440576usize,
        117440584usize,
        117440592usize,
        117440600usize,
        117440608usize,
        117440616usize,
        117440624usize,
        117440632usize,
        117440640usize,
        117440648usize,
        117440656usize,
        117440664usize,
        117440672usize,
        117440680usize,
        117440688usize,
        117440696usize,
        117440704usize,
        117440712usize,
        117440720usize,
        117440728usize,
        117440736usize,
        117440744usize,
        117440752usize,
        117440760usize,
        117440768usize,
        117440776usize,
        117440784usize,
        117440792usize,
        117440800usize,
        117440808usize,
        117440816usize,
        117440824usize,
        117440832usize,
        117440840usize,
        117440848usize,
        117440856usize,
        117440864usize,
        117440872usize,
        117440880usize,
        117440888usize,
        117440896usize,
        117440904usize,
        117440912usize,
        117440920usize,
        117440928usize,
        117440936usize,
        117440944usize,
        117440952usize,
        117440960usize,
        117440968usize,
        117440976usize,
        117440984usize,
        117440992usize,
        117441000usize,
        117441008usize,
        117441016usize,
        117441024usize,
        117441032usize,
        117441040usize,
        117441048usize,
        117441056usize,
        117441064usize,
        117441072usize,
        117441080usize,
        117441088usize,
        117441096usize,
        117441104usize,
        117441112usize,
        117441120usize,
        117441128usize,
        117441136usize,
        117441144usize,
        117441152usize,
        117441160usize,
        117441168usize,
        117441176usize,
        117441184usize,
        117441192usize,
        117441200usize,
        117441208usize,
        117441216usize,
        117441224usize,
        117441232usize,
        117441240usize,
        117441248usize,
        117441256usize,
        117441264usize,
        117441272usize,
        117441280usize,
        117441288usize,
        117441296usize,
        117441304usize,
        117441312usize,
        117441320usize,
        117441328usize,
        117441336usize,
        117441344usize,
        117441352usize,
        117441360usize,
        117441368usize,
        117441376usize,
        117441384usize,
        117441392usize,
        117441400usize,
        117441408usize,
        117441416usize,
        117441424usize,
        117441432usize,
        117441440usize,
        117441448usize,
        117441456usize,
        117441464usize,
        117441472usize,
        117441480usize,
        117441488usize,
        117441496usize,
        117441504usize,
        117441512usize,
        117441520usize,
        117441528usize,
    ],
    y_coordinate: 0, 8,
    obj_mode: 8, 2,
    gfx_mode: 10, 2,
    mosaic_flag: 12, 1,
    color_flag: 13, 1,
    obj_shape: 14, 2
);

io_register! (
    ObjAttribute1 => 2, [
        117440514usize,
        117440522usize,
        117440530usize,
        117440538usize,
        117440546usize,
        117440554usize,
        117440562usize,
        117440570usize,
        117440578usize,
        117440586usize,
        117440594usize,
        117440602usize,
        117440610usize,
        117440618usize,
        117440626usize,
        117440634usize,
        117440642usize,
        117440650usize,
        117440658usize,
        117440666usize,
        117440674usize,
        117440682usize,
        117440690usize,
        117440698usize,
        117440706usize,
        117440714usize,
        117440722usize,
        117440730usize,
        117440738usize,
        117440746usize,
        117440754usize,
        117440762usize,
        117440770usize,
        117440778usize,
        117440786usize,
        117440794usize,
        117440802usize,
        117440810usize,
        117440818usize,
        117440826usize,
        117440834usize,
        117440842usize,
        117440850usize,
        117440858usize,
        117440866usize,
        117440874usize,
        117440882usize,
        117440890usize,
        117440898usize,
        117440906usize,
        117440914usize,
        117440922usize,
        117440930usize,
        117440938usize,
        117440946usize,
        117440954usize,
        117440962usize,
        117440970usize,
        117440978usize,
        117440986usize,
        117440994usize,
        117441002usize,
        117441010usize,
        117441018usize,
        117441026usize,
        117441034usize,
        117441042usize,
        117441050usize,
        117441058usize,
        117441066usize,
        117441074usize,
        117441082usize,
        117441090usize,
        117441098usize,
        117441106usize,
        117441114usize,
        117441122usize,
        117441130usize,
        117441138usize,
        117441146usize,
        117441154usize,
        117441162usize,
        117441170usize,
        117441178usize,
        117441186usize,
        117441194usize,
        117441202usize,
        117441210usize,
        117441218usize,
        117441226usize,
        117441234usize,
        117441242usize,
        117441250usize,
        117441258usize,
        117441266usize,
        117441274usize,
        117441282usize,
        117441290usize,
        117441298usize,
        117441306usize,
        117441314usize,
        117441322usize,
        117441330usize,
        117441338usize,
        117441346usize,
        117441354usize,
        117441362usize,
        117441370usize,
        117441378usize,
        117441386usize,
        117441394usize,
        117441402usize,
        117441410usize,
        117441418usize,
        117441426usize,
        117441434usize,
        117441442usize,
        117441450usize,
        117441458usize,
        117441466usize,
        117441474usize,
        117441482usize,
        117441490usize,
        117441498usize,
        117441506usize,
        117441514usize,
        117441522usize,
        117441530usize
    ],
    x_coordinate: 0, 9,
    rotation_scaling_param: 9, 4,
    horizontal_flip: 12, 1,
    vertical_flip: 13, 1,
    obj_size: 14, 2
);

io_register! (
    ObjAttribute2 => 2, [
        117440516usize,
        117440524usize,
        117440532usize,
        117440540usize,
        117440548usize,
        117440556usize,
        117440564usize,
        117440572usize,
        117440580usize,
        117440588usize,
        117440596usize,
        117440604usize,
        117440612usize,
        117440620usize,
        117440628usize,
        117440636usize,
        117440644usize,
        117440652usize,
        117440660usize,
        117440668usize,
        117440676usize,
        117440684usize,
        117440692usize,
        117440700usize,
        117440708usize,
        117440716usize,
        117440724usize,
        117440732usize,
        117440740usize,
        117440748usize,
        117440756usize,
        117440764usize,
        117440772usize,
        117440780usize,
        117440788usize,
        117440796usize,
        117440804usize,
        117440812usize,
        117440820usize,
        117440828usize,
        117440836usize,
        117440844usize,
        117440852usize,
        117440860usize,
        117440868usize,
        117440876usize,
        117440884usize,
        117440892usize,
        117440900usize,
        117440908usize,
        117440916usize,
        117440924usize,
        117440932usize,
        117440940usize,
        117440948usize,
        117440956usize,
        117440964usize,
        117440972usize,
        117440980usize,
        117440988usize,
        117440996usize,
        117441004usize,
        117441012usize,
        117441020usize,
        117441028usize,
        117441036usize,
        117441044usize,
        117441052usize,
        117441060usize,
        117441068usize,
        117441076usize,
        117441084usize,
        117441092usize,
        117441100usize,
        117441108usize,
        117441116usize,
        117441124usize,
        117441132usize,
        117441140usize,
        117441148usize,
        117441156usize,
        117441164usize,
        117441172usize,
        117441180usize,
        117441188usize,
        117441196usize,
        117441204usize,
        117441212usize,
        117441220usize,
        117441228usize,
        117441236usize,
        117441244usize,
        117441252usize,
        117441260usize,
        117441268usize,
        117441276usize,
        117441284usize,
        117441292usize,
        117441300usize,
        117441308usize,
        117441316usize,
        117441324usize,
        117441332usize,
        117441340usize,
        117441348usize,
        117441356usize,
        117441364usize,
        117441372usize,
        117441380usize,
        117441388usize,
        117441396usize,
        117441404usize,
        117441412usize,
        117441420usize,
        117441428usize,
        117441436usize,
        117441444usize,
        117441452usize,
        117441460usize,
        117441468usize,
        117441476usize,
        117441484usize,
        117441492usize,
        117441500usize,
        117441508usize,
        117441516usize,
        117441524usize,
        117441532usize,
    ],
    character_name: 0, 10,
    priority_rel_to_bg: 10, 2,
    palette_number: 12, 4,
);


io_register! (
    WindowHorizontalDimension => 2, [0x4000040, 0x4000042],
    X2: 0, 8,
    X1: 8, 8
);

io_register! (
    WindowVerticalDimension => 2, [0x4000044, 0x4000046],
    Y2: 0, 8,
    Y1: 8, 8,
);

io_register! (
    ControlWindowInside => 2, 0x4000048,
    window0_bg_enable_bits: 0, 4,
    window0_obj_enable_bits: 4, 1,
    window0_color_special_effect: 5, 1,
    window1_bg_enable_bits: 8, 4,
    window1_obj_enable_bits: 12, 1,
    window1_color_special_effect: 13, 1,
);

impl ControlWindowInside {
    pub fn bgs_to_display(&self, window_type: &WindowTypes) -> [bool; 4] {
        let bits = match window_type {
            WindowTypes::Window0 => self.get_window0_bg_enable_bits(),
            WindowTypes::Window1 => self.get_window1_bg_enable_bits(),
            _ => panic!("Trying to get an invalid window type for ControlWindowInside: {:?}", window_type)
        };

        return [
            ((bits >> 0) & 0x1) != 0,
            ((bits >> 1) & 0x1) != 0,
            ((bits >> 2) & 0x1) != 0,
            ((bits >> 3) & 0x1) != 0
        ];
    }

    pub fn should_display_obj(&self, window_type: &WindowTypes) -> bool {
        let bit = match window_type {
            WindowTypes::Window0 => self.get_window0_obj_enable_bits(),
            WindowTypes::Window1 => self.get_window1_obj_enable_bits(),
            _ => panic!("Trying to get an invalid window type for ControlWindowInside: {:?}", window_type)
        };

        return bit != 0;
    }

    pub fn should_display_sfx(&self, window_type: &WindowTypes) -> bool {
        let bit = match window_type {
            WindowTypes::Window0 => self.get_window0_color_special_effect(),
            WindowTypes::Window1 => self.get_window1_color_special_effect(),
            _ => panic!("Trying to get an invalid window type for ControlWindowOutside: {:?}", window_type)
        };

        return bit != 0;
    }
}

io_register! (
    ControlWindowOutside => 2, 0x400004A,
    outside_bg_enable_bits: 0, 4,
    outside_obj_enable_bits: 4, 1,
    outside_color_special_effect: 5, 1,
    obj_window_bg_enable_bits: 8, 4,
    obj_window_obj_enable_bits: 12, 1,
    obj_window_color_special_effect: 13, 1,
);

impl ControlWindowOutside {
    pub fn bgs_to_display(&self, window_type: &WindowTypes) -> [bool; 4] {
        let bits = match window_type {
            WindowTypes::WindowOutside => self.get_outside_bg_enable_bits(),
            WindowTypes::WindowObject => self.get_obj_window_bg_enable_bits(),
            _ => panic!("Trying to get an invalid window type for ControlWindowOutside: {:?}", window_type)
        };

        return [
            ((bits >> 0) & 0x1) != 0,
            ((bits >> 1) & 0x1) != 0,
            ((bits >> 2) & 0x1) != 0,
            ((bits >> 3) & 0x1) != 0
        ];
    }

    pub fn should_display_obj(&self, window_type: &WindowTypes) -> bool {
        let bit = match window_type {
            WindowTypes::WindowOutside => self.get_outside_obj_enable_bits(),
            WindowTypes::WindowObject => self.get_obj_window_obj_enable_bits(),
            _ => panic!("Trying to get an invalid window type for ControlWindowOutside: {:?}", window_type)
        };

        return bit != 0;
    }

    pub fn should_display_sfx(&self, window_type: &WindowTypes) -> bool {
        let bit = match window_type {
            WindowTypes::WindowOutside => self.get_outside_color_special_effect(),
            WindowTypes::WindowObject => self.get_obj_window_color_special_effect(),
            _ => panic!("Trying to get an invalid window type for ControlWindowOutside: {:?}", window_type)
        };

        return bit != 0;
    }
}

io_register! (
    MosaicSize => 4, 0x400004C,
    bg_mosaic_hsize: 0, 4,
    bg_mosaic_vsize: 4, 4,
    obj_mosaic_hsize: 8, 4,
    obj_mosaic_vsize: 12, 4,
);

io_register! (
    ColorSpecialEffectsSelection => 2, 0x4000050,
    bg0_1st_target_pixel: 0, 1,
    bg1_1st_target_pixel: 1, 1,
    bg2_1st_target_pixel: 2, 1,
    bg3_1st_target_pixel: 3, 1,
    obj_1st_target_pixel: 4, 1,
    bd_1st_target_pixel: 5, 1,
    color_special_effect: 6, 2,
    bg0_2nd_target_pixel: 8, 1,
    bg1_2nd_target_pixel: 9, 1,
    bg2_2nd_target_pixel: 10, 1,
    bg3_2nd_target_pixel: 11, 1,
    obj_2nd_target_pixel: 12, 1,
    bd_2nd_target_pixel: 13, 1
);

impl ColorSpecialEffectsSelection {
    pub fn get_blendmode(&self) -> BlendMode {
        return BlendMode::from(self.get_color_special_effect());
    }

    pub fn has_destination(&self, index: u8) -> bool {
        return ((self.get_register() >> index) & 0x1) != 0;
    }

    pub fn has_source(&self, index: u8) -> bool {
        return ((self.get_register() >> (index + 8)) & 0x1) != 0;
    }
}

io_register! (
    AlphaBlendingCoefficients => 2, 0x4000052,
    eva_coefficient: 0, 5,
    evb_coefficient: 8, 5,
);

io_register! (
    BrightnessCoefficient => 4, 0x4000054,
    evy_coefficient: 0, 5,
);
