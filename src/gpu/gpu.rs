use crate::memory::work_ram::WorkRam;
use crate::memory::lcd_io_registers::*;

pub struct GPU {
    pub not_used_mem_2: WorkRam,
    pub display_control: DisplayControl,
    pub green_swap: GreenSwap,
    pub display_status: DisplayStatus,
    pub vertical_count: VerticalCount,

    pub bg0_control: BG_0_1_Control,
    pub bg1_control: BG_0_1_Control,
    pub bg2_control: BG_2_3_Control,
    pub bg3_control: BG_2_3_Control,

    pub bg0_horizontal_offset: BGOffset,
    pub bg1_horizontal_offset: BGOffset,
    pub bg2_horizontal_offset: BGOffset,
    pub bg3_horizontal_offset: BGOffset,

    pub bg0_vertical_offset: BGOffset,
    pub bg1_vertical_offset: BGOffset,
    pub bg2_vertical_offset: BGOffset,
    pub bg3_vertical_offset: BGOffset,

    pub bg2_refrence_point_x_internal: BGRefrencePoint,
    pub bg2_refrence_point_y_internal: BGRefrencePoint,
    pub bg2_refrence_point_x_external: BGRefrencePoint,
    pub bg2_refrence_point_y_external: BGRefrencePoint,
    pub bg2_rotation_scaling_param_a: BGRotScaleParam,
    pub bg2_rotation_scaling_param_b: BGRotScaleParam,
    pub bg2_rotation_scaling_param_c: BGRotScaleParam,
    pub bg2_rotation_scaling_param_d: BGRotScaleParam,

    pub bg3_refrence_point_x_internal: BGRefrencePoint,
    pub bg3_refrence_point_y_internal: BGRefrencePoint,
    pub bg3_refrence_point_x_external: BGRefrencePoint,
    pub bg3_refrence_point_y_external: BGRefrencePoint,
    pub bg3_rotation_scaling_param_a: BGRotScaleParam,
    pub bg3_rotation_scaling_param_b: BGRotScaleParam,
    pub bg3_rotation_scaling_param_c: BGRotScaleParam,
    pub bg3_rotation_scaling_param_d: BGRotScaleParam,

    pub window0_horizontal_dimensions: WindowHorizontalDimension,
    pub window1_horizontal_dimensions: WindowHorizontalDimension,
    pub window0_vertical_dimensions: WindowVerticalDimension,
    pub window1_vertical_dimensions: WindowVerticalDimension,

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

            bg0_control: BG_0_1_Control::new(),
            bg1_control: BG_0_1_Control::new(),
            bg2_control: BG_2_3_Control::new(),
            bg3_control: BG_2_3_Control::new(),

            bg0_horizontal_offset: BGOffset::new(),
            bg1_horizontal_offset: BGOffset::new(),
            bg2_horizontal_offset: BGOffset::new(),
            bg3_horizontal_offset: BGOffset::new(),

            bg0_vertical_offset: BGOffset::new(),
            bg1_vertical_offset: BGOffset::new(),
            bg2_vertical_offset: BGOffset::new(),
            bg3_vertical_offset: BGOffset::new(),

            bg2_refrence_point_x_internal: BGRefrencePoint::new(),
            bg2_refrence_point_y_internal: BGRefrencePoint::new(),
            bg2_refrence_point_x_external: BGRefrencePoint::new(),
            bg2_refrence_point_y_external: BGRefrencePoint::new(),
            bg2_rotation_scaling_param_a: BGRotScaleParam::new(),
            bg2_rotation_scaling_param_b: BGRotScaleParam::new(),
            bg2_rotation_scaling_param_c: BGRotScaleParam::new(),
            bg2_rotation_scaling_param_d: BGRotScaleParam::new(),

            bg3_refrence_point_x_internal: BGRefrencePoint::new(),
            bg3_refrence_point_y_internal: BGRefrencePoint::new(),
            bg3_refrence_point_x_external: BGRefrencePoint::new(),
            bg3_refrence_point_y_external: BGRefrencePoint::new(),
            bg3_rotation_scaling_param_a: BGRotScaleParam::new(),
            bg3_rotation_scaling_param_b: BGRotScaleParam::new(),
            bg3_rotation_scaling_param_c: BGRotScaleParam::new(),
            bg3_rotation_scaling_param_d: BGRotScaleParam::new(),

            window0_horizontal_dimensions: WindowHorizontalDimension::new(),
            window1_horizontal_dimensions: WindowHorizontalDimension::new(),
            window0_vertical_dimensions: WindowVerticalDimension::new(),
            window1_vertical_dimensions: WindowVerticalDimension::new(),

            control_window_inside: ControlWindowInside::new(),
            control_window_outside: ControlWindowOutside::new(),
            mosaic_size: MosaicSize::new(),
            color_special_effects_selection: ColorSpecialEffectsSelection::new(),

            alpha_blending_coefficients: AlphaBlendingCoefficients::new(),
            brightness_coefficient: BrightnessCoefficient::new()
        };
    }
}