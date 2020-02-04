pub mod memory_bus;
use crate::cpu::{cpu::CPU, cpu::OperatingMode, cpu::ARM_SP, cpu::ARM_PC};
use crate::memory::{memory_map::MemoryMap, game_pack_rom::GamePackRom, io_registers::IORegisters, work_ram::WorkRam};
use crate::memory::lcd_io_registers::*;
use crate::gpu::gpu::GPU;
use crate::memory::{interrupt_registers::*, key_input_registers::*, system_control::WaitStateControl};
use crate::operations::timing::{MemAccessSize, CycleClock};
use crate::gba::memory_bus::MemoryBus;

pub struct GBA {
    pub cpu: CPU,
    pub gpu: GPU,
    pub memory_bus: MemoryBus,
    pub game_pack_memory: [GamePackRom; 3],
    pub io_reg: IORegisters,
    pub ime_interrupt: InterruptMasterEnableRegister,
    pub ie_interrupt: InterruptEnableRegister,
    pub if_interrupt: InterruptRequestFlags,
    pub key_status: KeyStatus,
    pub ket_interrupt_control: KeyInterruptControl
}

impl Default for GBA {
    fn default() -> Self {
        let temp_gamepack = [
            GamePackRom::new(0),
            GamePackRom::new(0),
            GamePackRom::new(0),
        ];

        let mut temp: GBA = GBA {
            cpu: CPU::new(),
            gpu: GPU::new(),
            memory_bus: MemoryBus::new(),
            game_pack_memory: temp_gamepack,
            io_reg: IORegisters::new(0),
            ime_interrupt: InterruptMasterEnableRegister::new(),
            ie_interrupt: InterruptEnableRegister::new(),
            if_interrupt: InterruptRequestFlags::new(),
            key_status: KeyStatus::new(),
            ket_interrupt_control: KeyInterruptControl::new()
        };

        // setup the PC
        temp.cpu.set_register(ARM_PC, 0x08000000);

        // setup the SPs'
        temp.cpu.operating_mode = OperatingMode::Interrupt;
        temp.cpu.set_register(ARM_SP, 0x03007FE0);

        temp.cpu.operating_mode = OperatingMode::User;
        temp.cpu.set_register(ARM_SP, 0x03007FE0);

        temp.cpu.operating_mode = OperatingMode::Supervisor;
        temp.cpu.set_register(ARM_SP, 0x03007FE0);

        temp.cpu.operating_mode = OperatingMode::System;

        // setup the memory
        temp.memory_bus.mem_map.register_memory(0x00000000, 0x00003FFF, &temp.cpu.bios_ram.memory);
        temp.memory_bus.mem_map.register_memory(0x02000000, 0x0203FFFF, &temp.cpu.wram.memory);
        temp.memory_bus.mem_map.register_memory(0x03000000, 0x03007FFF, &temp.cpu.onchip_wram.memory);
        temp.memory_bus.mem_map.register_memory(0x07000400, 0x07FFFFFF, &temp.gpu.not_used_mem_2.memory);
        temp.memory_bus.mem_map.register_memory(0x08000000, 0x09FFFFFF, &temp.game_pack_memory[0].memory);
        temp.memory_bus.mem_map.register_memory(0x0A000000, 0x0BFFFFFF, &temp.game_pack_memory[1].memory);
        temp.memory_bus.mem_map.register_memory(0x0C000000, 0x0DFFFFFF, &temp.game_pack_memory[2].memory);
        temp.memory_bus.mem_map.register_memory(0x04000000, 0x040003FE, &temp.io_reg.memory);

        return temp;
    }
}

impl GBA {
    pub fn new(pc_address: u32, bios: &Vec<u8>, rom: &Vec<u8>) -> GBA {
        let temp_gamepack = [
            GamePackRom::new(0),
            GamePackRom::new(0),
            GamePackRom::new(0),
        ];

        let mut temp: GBA = GBA {
            cpu: CPU::new(),
            gpu: GPU::new(),
            memory_bus: MemoryBus::new(),
            game_pack_memory: temp_gamepack,
            io_reg: IORegisters::new(0),
            ime_interrupt: InterruptMasterEnableRegister::new(),
            ie_interrupt: InterruptEnableRegister::new(),
            if_interrupt: InterruptRequestFlags::new(),
            key_status: KeyStatus::new(),
            ket_interrupt_control: KeyInterruptControl::new()
        };

        // setup the PC
        temp.cpu.set_register(ARM_PC, pc_address);
        temp.cpu.set_register(ARM_SP, 0x03007F00);

        // setup the SPs'
        temp.cpu.operating_mode = OperatingMode::Interrupt;
        temp.cpu.set_register(ARM_SP, 0x03007FA0);

        temp.cpu.operating_mode = OperatingMode::FastInterrupt;
        temp.cpu.set_register(ARM_SP, 0x03007F00);

        temp.cpu.operating_mode = OperatingMode::User;
        temp.cpu.set_register(ARM_SP, 0x03007F00);

        temp.cpu.operating_mode = OperatingMode::Supervisor;
        temp.cpu.set_register(ARM_SP, 0x03007FE0);

        temp.cpu.operating_mode = OperatingMode::Abort;
        temp.cpu.set_register(ARM_SP, 0x03007F00);

        temp.cpu.operating_mode = OperatingMode::Undefined;
        temp.cpu.set_register(ARM_SP, 0x03007F00);

        temp.cpu.operating_mode = OperatingMode::Supervisor;

        temp.key_status.set_register(0xFFFF);

        for i in 0..2 {
            temp.gpu.bg_affine_components[i].rotation_scaling_param_a.set_register(0x100);
            temp.gpu.bg_affine_components[i].rotation_scaling_param_b.set_register(0);
            temp.gpu.bg_affine_components[i].rotation_scaling_param_c.set_register(0);
            temp.gpu.bg_affine_components[i].rotation_scaling_param_d.set_register(0x100);
        }

        // setup the memory
        // General INternal Memory
        temp.cpu.bios_ram.load(bios);
        temp.memory_bus.mem_map.register_memory(0x00000000, 0x00003FFF, &temp.cpu.bios_ram.memory);
        temp.memory_bus.mem_map.register_memory(0x02000000, 0x0203FFFF, &temp.cpu.wram.memory);
        temp.memory_bus.mem_map.register_memory(0x03000000, 0x03007FFF, &temp.cpu.onchip_wram.memory);

        // Internal Display Memory
        temp.memory_bus.mem_map.register_memory(0x05000000, 0x050003FF, &temp.gpu.bg_obj_palette_ram.memory);
        temp.memory_bus.mem_map.register_memory(0x05000400, 0x05FFFFFF, &temp.gpu.not_used_mem.memory);
        temp.memory_bus.mem_map.register_memory(0x06000000, 0x06017FFF, &temp.gpu.vram.memory);
        temp.memory_bus.mem_map.register_memory(0x06018000, 0x06FFFFFF, &temp.gpu.not_used_mem_2.memory);
        temp.memory_bus.mem_map.register_memory(0x07000000, 0x070003FF, &temp.gpu.oam_obj_attributes.memory);
        temp.memory_bus.mem_map.register_memory(0x07000400, 0x07FFFFFF, &temp.gpu.not_used_mem_3.memory);

        // Game Pack Memory
        temp.game_pack_memory[0].load(rom);
        temp.memory_bus.mem_map.register_memory(0x08000000, 0x09FFFFFF, &temp.game_pack_memory[0].memory);
        temp.memory_bus.mem_map.register_memory(0x0A000000, 0x0BFFFFFF, &temp.game_pack_memory[1].memory);
        temp.memory_bus.mem_map.register_memory(0x0C000000, 0x0DFFFFFF, &temp.game_pack_memory[2].memory);

        macro_rules! register_memory_segment {
            ($startval:expr, $name:ident, $variable:expr) => {
                temp.memory_bus.mem_map.register_memory($startval, $startval + ($name::SEGMENT_SIZE as u32) - 1, &$variable.memory);
            }
        }
        //Interrupt Memory
        //4000208
        register_memory_segment!(0x4000208, InterruptMasterEnableRegister, temp.ime_interrupt);

        register_memory_segment!(0x4000200, InterruptMasterEnableRegister, temp.ie_interrupt);
        register_memory_segment!(0x4000202, InterruptMasterEnableRegister, temp.if_interrupt);

        // System Control memory
        register_memory_segment!(0x4000204, WaitStateControl, temp.memory_bus.cycle_clock.wait_state_control);

        // GPU memory
        register_memory_segment!(0x4000000, DisplayControl, temp.gpu.display_control);
        register_memory_segment!(0x4000002, GreenSwap, temp.gpu.green_swap);
        register_memory_segment!(0x4000004, DisplayStatus, temp.gpu.display_status);
        register_memory_segment!(0x4000006, VerticalCount, temp.gpu.vertical_count);

        register_memory_segment!(0x4000008, BG_Control, temp.gpu.backgrounds[0].control);
        register_memory_segment!(0x400000A, BG_Control, temp.gpu.backgrounds[1].control);
        register_memory_segment!(0x400000C, BG_Control, temp.gpu.backgrounds[2].control);
        register_memory_segment!(0x400000E, BG_Control, temp.gpu.backgrounds[3].control);

        register_memory_segment!(0x4000010, BGOffset, temp.gpu.backgrounds[0].horizontal_offset);
        register_memory_segment!(0x4000012, BGOffset, temp.gpu.backgrounds[0].vertical_offset);
        register_memory_segment!(0x4000014, BGOffset, temp.gpu.backgrounds[1].horizontal_offset);
        register_memory_segment!(0x4000016, BGOffset, temp.gpu.backgrounds[1].vertical_offset);
        register_memory_segment!(0x4000018, BGOffset, temp.gpu.backgrounds[2].horizontal_offset);
        register_memory_segment!(0x400001A, BGOffset, temp.gpu.backgrounds[2].vertical_offset);
        register_memory_segment!(0x400001C, BGOffset, temp.gpu.backgrounds[3].horizontal_offset);
        register_memory_segment!(0x400001E, BGOffset, temp.gpu.backgrounds[3].vertical_offset);

        register_memory_segment!(0x4000020, BGRotScaleParam, temp.gpu.bg_affine_components[0].rotation_scaling_param_a);
        register_memory_segment!(0x4000022, BGRotScaleParam, temp.gpu.bg_affine_components[0].rotation_scaling_param_b);
        register_memory_segment!(0x4000024, BGRotScaleParam, temp.gpu.bg_affine_components[0].rotation_scaling_param_c);
        register_memory_segment!(0x4000026, BGRotScaleParam, temp.gpu.bg_affine_components[0].rotation_scaling_param_d);

        register_memory_segment!(0x4000028, BGRefrencePoint, temp.gpu.bg_affine_components[0].refrence_point_x_internal);
        register_memory_segment!(0x400002C, BGRefrencePoint, temp.gpu.bg_affine_components[0].refrence_point_y_internal);

        register_memory_segment!(0x4000030, BGRotScaleParam, temp.gpu.bg_affine_components[1].rotation_scaling_param_a);
        register_memory_segment!(0x4000032, BGRotScaleParam, temp.gpu.bg_affine_components[1].rotation_scaling_param_b);
        register_memory_segment!(0x4000034, BGRotScaleParam, temp.gpu.bg_affine_components[1].rotation_scaling_param_c);
        register_memory_segment!(0x4000036, BGRotScaleParam, temp.gpu.bg_affine_components[1].rotation_scaling_param_d);

        register_memory_segment!(0x4000038, BGRefrencePoint, temp.gpu.bg_affine_components[1].refrence_point_x_internal);
        register_memory_segment!(0x400003C, BGRefrencePoint, temp.gpu.bg_affine_components[1].refrence_point_y_internal);

        register_memory_segment!(0x4000040, WindowHorizontalDimension, temp.gpu.windows[0].horizontal_dimensions);
        register_memory_segment!(0x4000042, WindowHorizontalDimension, temp.gpu.windows[1].horizontal_dimensions);
        register_memory_segment!(0x4000044, WindowVerticalDimension, temp.gpu.windows[0].vertical_dimensions);
        register_memory_segment!(0x4000046, WindowVerticalDimension, temp.gpu.windows[1].vertical_dimensions);

        register_memory_segment!(0x4000048, ControlWindowInside, temp.gpu.control_window_inside);
        register_memory_segment!(0x400004A, ControlWindowOutside, temp.gpu.control_window_outside);

        register_memory_segment!(0x400004C, MosaicSize, temp.gpu.mosaic_size);
        register_memory_segment!(0x4000050, ColorSpecialEffectsSelection, temp.gpu.color_special_effects_selection);
        register_memory_segment!(0x4000052, AlphaBlendingCoefficients, temp.gpu.alpha_blending_coefficients);
        register_memory_segment!(0x4000054, BrightnessCoefficient, temp.gpu.brightness_coefficient);

        register_memory_segment!(0x4000130, KeyStatus, temp.key_status);

        return temp;
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.fetch(&mut self.memory_bus.mem_map);
        }
    }

    pub fn frame(&mut self) {
        while !self.gpu.frame_ready {
            self.step();
        }

        self.gpu.frame_ready = false;
    }

    pub fn single_step(&mut self) {
        if self.cpu.cycle_count < (self.gpu.cycles_to_next_state as usize) {
            self.cpu.fetch(&mut self.memory_bus.mem_map);

        } else {
            self.gpu.step(self.cpu.cycle_count as u32, &mut self.memory_bus.mem_map);
            self.cpu.cycle_count = 0;
        }
    }

    pub fn step(&mut self) {
        while self.cpu.cycle_count < (self.gpu.cycles_to_next_state as usize) {
            self.cpu.fetch(&mut self.memory_bus.mem_map);
        }

        self.gpu.step(self.cpu.cycle_count as u32, &mut self.memory_bus.mem_map);
        self.cpu.cycle_count = 0;
    }
}
