use crate::memory::work_ram::WorkRam;
use crate::memory::lcd_io_registers::*;

pub struct GPU {
    pub not_used_mem_2: WorkRam,
    pub display_control: DisplayControl,
    pub green_swap: GreenSwap,
    pub display_status: DisplayStatus,
    pub vertical_count: VerticalCount,

    pub bg0_control: BG0Control,
    pub bg1_control: BG1Control,
    pub bg2_control: BG2Control,
    pub bg3_control: BG3Control,

    pub bg0_horizontal_offset: BG0HorizontalOffset,
    pub bg1_horizontal_offset: BG1HorizontalOffset,
    pub bg2_horizontal_offset: BG2HorizontalOffset,
    pub bg3_horizontal_offset: BG3HorizontalOffset,

    pub bg0_vertical_offset: BG0VerticalOffset,
    pub bg1_vertical_offset: BG1VerticalOffset,
    pub bg2_vertical_offset: BG2VerticalOffset,
    pub bg3_vertical_offset: BG3VerticalOffset,

    pub bg2_refrence_point_x_internal: BG2RefrencePointX,
    pub bg2_refrence_point_y_internal: BG2RefrencePointY,
    pub bg2_refrence_point_x_external: BG2RefrencePointX,
    pub bg2_refrence_point_y_external: BG2RefrencePointY,
    pub bg2_rotation_scaling_param_a: BG2RotationScalingParamA,
    pub bg2_rotation_scaling_param_b: BG2RotationScalingParamB,
    pub bg2_rotation_scaling_param_c: BG2RotationScalingParamC,
    pub bg2_rotation_scaling_param_d: BG2RotationScalingParamD,

    pub bg3_refrence_point_x_internal: BG3RefrencePointX,
    pub bg3_refrence_point_y_internal: BG3RefrencePointY,
    pub bg3_refrence_point_x_external: BG3RefrencePointX,
    pub bg3_refrence_point_y_external: BG3RefrencePointY,
    pub bg3_rotation_scaling_param_a: BG3RotationScalingParamA,
    pub bg3_rotation_scaling_param_b: BG3RotationScalingParamB,
    pub bg3_rotation_scaling_param_c: BG3RotationScalingParamC,
    pub bg3_rotation_scaling_param_d: BG3RotationScalingParamD,

    pub window0_horizontal_dimensions: Window0HorizontalDimensions,
    pub window1_horizontal_dimensions: Window1HorizontalDimensions,
    pub window0_vertical_dimensions: Window0VerticalDimensions,
    pub window1_vertical_dimensions: Window1VerticalDimensions,

    pub control_window_inside: ControlWindowInside,
    pub control_window_outside: ControlWindowOutside,
    pub mosaic_size: MosaicSize,
    pub color_special_effects_selection: ColorSpecialEffectsSelection,

    pub alpha_blending_coefficients: AlphaBlendingCoefficients,
    pub brightness_coefficient: BrightnessCoefficient
}

impl GPU {
    pub fn new() -> GPU {
        return GPU {
            not_used_mem_2: WorkRam::new(0xFFFFBF, 0),
            display_control: DisplayControl::new(),
            green_swap: GreenSwap::new(),
            display_status: DisplayStatus::new(),
            vertical_count: VerticalCount::new(),

            bg0_control: BG0Control::new(),
            bg1_control: BG1Control::new(),
            bg2_control: BG2Control::new(),
            bg3_control: BG3Control::new(),

            bg0_horizontal_offset: BG0HorizontalOffset::new(),
            bg1_horizontal_offset: BG1HorizontalOffset::new(),
            bg2_horizontal_offset: BG2HorizontalOffset::new(),
            bg3_horizontal_offset: BG3HorizontalOffset::new(),

            bg0_vertical_offset: BG0VerticalOffset::new(),
            bg1_vertical_offset: BG1VerticalOffset::new(),
            bg2_vertical_offset: BG2VerticalOffset::new(),
            bg3_vertical_offset: BG3VerticalOffset::new(),

            bg2_refrence_point_x_internal: BG2RefrencePointX::new(),
            bg2_refrence_point_y_internal: BG2RefrencePointY::new(),
            bg2_refrence_point_x_external: BG2RefrencePointX::new(),
            bg2_refrence_point_y_external: BG2RefrencePointY::new(),
            bg2_rotation_scaling_param_a: BG2RotationScalingParamA::new(),
            bg2_rotation_scaling_param_b: BG2RotationScalingParamB::new(),
            bg2_rotation_scaling_param_c: BG2RotationScalingParamC::new(),
            bg2_rotation_scaling_param_d: BG2RotationScalingParamD::new(),

            bg3_refrence_point_x_internal: BG3RefrencePointX::new(),
            bg3_refrence_point_y_internal: BG3RefrencePointY::new(),
            bg3_refrence_point_x_external: BG3RefrencePointX::new(),
            bg3_refrence_point_y_external: BG3RefrencePointY::new(),
            bg3_rotation_scaling_param_a: BG3RotationScalingParamA::new(),
            bg3_rotation_scaling_param_b: BG3RotationScalingParamB::new(),
            bg3_rotation_scaling_param_c: BG3RotationScalingParamC::new(),
            bg3_rotation_scaling_param_d: BG3RotationScalingParamD::new(),

            window0_horizontal_dimensions: Window0HorizontalDimensions::new(),
            window1_horizontal_dimensions: Window1HorizontalDimensions::new(),
            window0_vertical_dimensions: Window0VerticalDimensions::new(),
            window1_vertical_dimensions: Window1VerticalDimensions::new(),

            control_window_inside: ControlWindowInside::new(),
            control_window_outside: ControlWindowOutside::new(),
            mosaic_size: MosaicSize::new(),
            color_special_effects_selection: ColorSpecialEffectsSelection::new(),

            alpha_blending_coefficients: AlphaBlendingCoefficients::new(),
            brightness_coefficient: BrightnessCoefficient::new()
        };
    }
}