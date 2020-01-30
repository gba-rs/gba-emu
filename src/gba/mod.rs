use crate::cpu::{cpu::CPU, cpu::OperatingMode, cpu::ARM_SP, cpu::ARM_PC};
use crate::memory::{memory_map::MemoryMap, game_pack_rom::GamePackRom, io_registers::IORegisters};
use crate::memory::lcd_io_registers::*;
use crate::gpu::gpu::GPU;
use crate::memory::interrupt_registers::*;
use crate::memory::system_control::WaitStateControl;


pub struct GBA {
    pub cpu: CPU,
    pub gpu: GPU,
    pub mem_map: MemoryMap,
    pub game_pack_memory: [GamePackRom; 3],
    pub io_reg: IORegisters,
    pub ime_interrupt: InterruptMasterEnableRegister,
    pub ie_interrupt: InterruptEnableRegister,
    pub if_interrupt: InterruptRequestFlags
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
            mem_map: MemoryMap::new(),
            game_pack_memory: temp_gamepack,
            io_reg: IORegisters::new(0),
            ime_interrupt: InterruptMasterEnableRegister::new(),
            ie_interrupt: InterruptEnableRegister::new(),
            if_interrupt: InterruptRequestFlags::new()
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

        // setup the memory
        temp.mem_map.register_memory(0x00000000, 0x00003FFF, &temp.cpu.bios_ram.memory);
        temp.mem_map.register_memory(0x02000000, 0x0203FFFF, &temp.cpu.wram.memory);
        temp.mem_map.register_memory(0x03000000, 0x03007FFF, &temp.cpu.onchip_wram.memory);
        temp.mem_map.register_memory(0x07000400, 0x07FFFFFF, &temp.gpu.not_used_mem_2.memory);
        temp.mem_map.register_memory(0x08000000, 0x09FFFFFF, &temp.game_pack_memory[0].memory);
        temp.mem_map.register_memory(0x0A000000, 0x0BFFFFFF, &temp.game_pack_memory[1].memory);
        temp.mem_map.register_memory(0x0C000000, 0x0DFFFFFF, &temp.game_pack_memory[2].memory);
        temp.mem_map.register_memory(0x04000000, 0x040003FE, &temp.io_reg.memory);

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
            mem_map: MemoryMap::new(),
            game_pack_memory: temp_gamepack,
            io_reg: IORegisters::new(0),
            ime_interrupt: InterruptMasterEnableRegister::new(),
            ie_interrupt: InterruptEnableRegister::new(),
            if_interrupt: InterruptRequestFlags::new()   
        };

        // setup the PC
        temp.cpu.set_register(ARM_PC, pc_address);

        // setup the SPs'
        temp.cpu.operating_mode = OperatingMode::Interrupt;
        temp.cpu.set_register(ARM_SP, 0x03007FE0);

        temp.cpu.operating_mode = OperatingMode::User;
        temp.cpu.set_register(ARM_SP, 0x03007FE0);

        temp.cpu.operating_mode = OperatingMode::Supervisor;
        temp.cpu.set_register(ARM_SP, 0x03007FE0);

        // setup the memory
        temp.cpu.bios_ram.load(bios);
        temp.game_pack_memory[0].load(rom);
        temp.mem_map.register_memory(0x00000000, 0x00003FFF, &temp.cpu.bios_ram.memory);
        temp.mem_map.register_memory(0x02000000, 0x0203FFFF, &temp.cpu.wram.memory);
        temp.mem_map.register_memory(0x03000000, 0x03007FFF, &temp.cpu.onchip_wram.memory);
        temp.mem_map.register_memory(0x07000400, 0x07FFFFFF, &temp.gpu.not_used_mem_2.memory);
        temp.mem_map.register_memory(0x08000000, 0x09FFFFFF, &temp.game_pack_memory[0].memory);
        temp.mem_map.register_memory(0x0A000000, 0x0BFFFFFF, &temp.game_pack_memory[1].memory);
        temp.mem_map.register_memory(0x0C000000, 0x0DFFFFFF, &temp.game_pack_memory[2].memory);
        temp.mem_map.register_memory(0x04000000, 0x040003FE, &temp.io_reg.memory);

        macro_rules! register_memory_segment {
            ($startval:expr, $name:ident, $variable:expr) => {
                temp.mem_map.register_memory($startval, $startval + ($name::SEGMENT_SIZE as u32), &$variable.memory);
            };
        }
        //Interrupt Memory
        //4000208
        register_memory_segment!(0x4000208, InterruptMasterEnableRegister, temp.ime_interrupt);
        register_memory_segment!(0x4000200, InterruptMasterEnableRegister, temp.ie_interrupt);
        register_memory_segment!(0x4000202, InterruptMasterEnableRegister, temp.if_interrupt);

        // TODO figure out how to register this memory
//         register_memory_segment!(0x4000204, WaitStateControl, temp.mem_map.cycle_clock.wait_state_control);

        // GPU memory
        register_memory_segment!(0x4000000, DisplayControl, temp.gpu.display_control);
        register_memory_segment!(0x4000002, GreenSwap, temp.gpu.green_swap);
        register_memory_segment!(0x4000004, DisplayStatus, temp.gpu.display_status);
        register_memory_segment!(0x4000006, VerticalCount, temp.gpu.vertical_count);

        register_memory_segment!(0x4000008, BG_0_1_Control, temp.gpu.bg0_control);
        register_memory_segment!(0x400000A, BG_0_1_Control, temp.gpu.bg1_control);
        register_memory_segment!(0x400000C, BG_2_3_Control, temp.gpu.bg2_control);
        register_memory_segment!(0x400000E, BG_2_3_Control, temp.gpu.bg3_control);

        register_memory_segment!(0x4000010, BGOffset, temp.gpu.bg0_horizontal_offset);
        register_memory_segment!(0x4000012, BGOffset, temp.gpu.bg0_vertical_offset);
        register_memory_segment!(0x4000014, BGOffset, temp.gpu.bg1_horizontal_offset);
        register_memory_segment!(0x4000016, BGOffset, temp.gpu.bg1_vertical_offset);
        register_memory_segment!(0x4000018, BGOffset, temp.gpu.bg2_horizontal_offset);
        register_memory_segment!(0x400001A, BGOffset, temp.gpu.bg2_vertical_offset);
        register_memory_segment!(0x400001C, BGOffset, temp.gpu.bg3_horizontal_offset);
        register_memory_segment!(0x400001E, BGOffset, temp.gpu.bg3_vertical_offset);

        register_memory_segment!(0x4000020, BGRotScaleParam, temp.gpu.bg2_rotation_scaling_param_a);
        register_memory_segment!(0x4000022, BGRotScaleParam, temp.gpu.bg2_rotation_scaling_param_b);
        register_memory_segment!(0x4000024, BGRotScaleParam, temp.gpu.bg2_rotation_scaling_param_c);
        register_memory_segment!(0x4000026, BGRotScaleParam, temp.gpu.bg2_rotation_scaling_param_d);

        register_memory_segment!(0x4000028, BGRefrencePoint, temp.gpu.bg2_refrence_point_x_external);
        register_memory_segment!(0x400002C, BGRefrencePoint, temp.gpu.bg2_refrence_point_y_external);

        register_memory_segment!(0x4000030, BGRotScaleParam, temp.gpu.bg3_rotation_scaling_param_a);
        register_memory_segment!(0x4000032, BGRotScaleParam, temp.gpu.bg3_rotation_scaling_param_b);
        register_memory_segment!(0x4000034, BGRotScaleParam, temp.gpu.bg3_rotation_scaling_param_c);
        register_memory_segment!(0x4000036, BGRotScaleParam, temp.gpu.bg3_rotation_scaling_param_d);

        register_memory_segment!(0x4000038, BGRefrencePoint, temp.gpu.bg3_refrence_point_x_external);
        register_memory_segment!(0x400003C, BGRefrencePoint, temp.gpu.bg3_refrence_point_y_external);

        register_memory_segment!(0x4000040, WindowHorizontalDimension, temp.gpu.window0_horizontal_dimensions);
        register_memory_segment!(0x4000042, WindowHorizontalDimension, temp.gpu.window1_horizontal_dimensions);
        register_memory_segment!(0x4000044, WindowVerticalDimension, temp.gpu.window0_vertical_dimensions);
        register_memory_segment!(0x4000046, WindowVerticalDimension, temp.gpu.window1_vertical_dimensions);

        register_memory_segment!(0x4000048, ControlWindowInside, temp.gpu.control_window_inside);
        register_memory_segment!(0x400004A, ControlWindowOutside, temp.gpu.control_window_outside);

        register_memory_segment!(0x400004C, MosaicSize, temp.gpu.mosaic_size);
        register_memory_segment!(0x4000050, ColorSpecialEffectsSelection, temp.gpu.color_special_effects_selection);
        register_memory_segment!(0x4000052, AlphaBlendingCoefficients, temp.gpu.alpha_blending_coefficients);
        register_memory_segment!(0x4000054, BrightnessCoefficient, temp.gpu.brightness_coefficient);

        return temp;
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.fetch(&mut self.mem_map);
        }
    }

    pub fn step(&mut self) {
        self.cpu.fetch(&mut self.mem_map);
    }
}
