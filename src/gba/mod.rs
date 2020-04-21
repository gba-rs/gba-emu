use crate::cpu::{cpu::CPU, cpu::OperatingMode, cpu::ARM_SP, cpu::ARM_PC};
use crate::gpu::{gpu::GPU, gpu::DISPLAY_WIDTH, gpu::DISPLAY_HEIGHT};
use crate::gpu::rgb15::Rgb15;
use crate::memory::{key_input_registers::*};
use crate::memory::{memory_bus::MemoryBus, memory_map::HaltState};
use crate::interrupts::interrupts::Interrupts;
use crate::dma::DMAController;
use crate::timers::timer::TimerHandler;
use crate::{gamepak::GamePack, gamepak::BackupType};


pub struct GBA {
    pub cpu: CPU,
    pub gpu: GPU,
    pub memory_bus: MemoryBus,
    pub key_status: KeyStatus,
    pub ket_interrupt_control: KeyInterruptControl,
    pub interrupt_handler: Interrupts,
    pub timer_handler: TimerHandler,
    pub dma_control: DMAController
}

impl Default for GBA {
    fn default() -> Self {
        let temp = GamePack::default();
        return GBA::new(0x08000000, &temp);
    }
}

impl GBA {

    pub fn new(pc_address: u32, game_pack: &GamePack) -> GBA {

        let mut temp: GBA = GBA {
            cpu: CPU::new(),
            gpu: GPU::new(),
            memory_bus: MemoryBus::new(game_pack.backup_type),
            key_status: KeyStatus::new(),
            ket_interrupt_control: KeyInterruptControl::new(),
            interrupt_handler: Interrupts::new(),
            timer_handler: TimerHandler::new(),
            dma_control: DMAController::new()
        };

        temp.gpu.register(&temp.memory_bus.mem_map.memory);
        temp.key_status.register(&temp.memory_bus.mem_map.memory);
        temp.ket_interrupt_control.register(&temp.memory_bus.mem_map.memory);
        temp.interrupt_handler.ime_interrupt.register(&temp.memory_bus.mem_map.memory);
        temp.interrupt_handler.ie_interrupt.register(&temp.memory_bus.mem_map.memory);
        temp.interrupt_handler.if_interrupt.register(&temp.memory_bus.mem_map.memory);
        temp.timer_handler.register(&temp.memory_bus.mem_map.memory);
        temp.memory_bus.cycle_clock.register(&temp.memory_bus.mem_map.memory);
        temp.dma_control.register(&temp.memory_bus.mem_map.memory);

        // setup the PC
        temp.cpu.set_register(ARM_PC, pc_address);
        temp.cpu.set_register(ARM_SP, 0x03007F00);

        // setup the SPs'
        temp.cpu.set_operating_mode(OperatingMode::Interrupt);
        temp.cpu.set_register(ARM_SP, 0x03007FA0);

        temp.cpu.set_operating_mode(OperatingMode::FastInterrupt);
        temp.cpu.set_register(ARM_SP, 0x03007F00);

        temp.cpu.set_operating_mode(OperatingMode::User);
        temp.cpu.set_register(ARM_SP, 0x03007F00);

        temp.cpu.set_operating_mode(OperatingMode::Supervisor);
        temp.cpu.set_register(ARM_SP, 0x03007FE0);

        temp.cpu.set_operating_mode(OperatingMode::Abort);
        temp.cpu.set_register(ARM_SP, 0x03007F00);

        temp.cpu.set_operating_mode(OperatingMode::Undefined);
        temp.cpu.set_register(ARM_SP, 0x03007F00);

        temp.cpu.set_operating_mode(OperatingMode::Supervisor);

        temp.key_status.set_register(0x3FF);

        for i in 0..2 {
            temp.gpu.bg_affine_components[i].rotation_scaling_param_a.set_register(0x100);
            temp.gpu.bg_affine_components[i].rotation_scaling_param_b.set_register(0);
            temp.gpu.bg_affine_components[i].rotation_scaling_param_c.set_register(0);
            temp.gpu.bg_affine_components[i].rotation_scaling_param_d.set_register(0x100);
        }

        // setup the memory
        // General INternal Memory
        temp.load_bios(&game_pack.bios);
        temp.load_rom(&game_pack.rom);

        return temp;
    }

    pub fn load_bios(&mut self, bios: &Vec<u8>) {
        self.memory_bus.mem_map.write_block(0, bios)
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        self.memory_bus.mem_map.write_block(0x08000000, rom)
    }

    pub fn load_save_file(&mut self, save_data: &Vec<u8>) {
        match self.memory_bus.mem_map.backup_type {
            BackupType::Sram | BackupType::Flash64K | BackupType::Flash128K => {
                self.memory_bus.mem_map.write_block(0x0E000000, save_data);
            },
            _ => {log::info!("Save data for this type is not implemented")} 
        }
    }
                                                                                                                                                                                                                                                                                                                   
    pub fn get_save_data(&self) -> Vec<u8> {
        match self.memory_bus.mem_map.backup_type {
            BackupType::Sram => {
                return self.memory_bus.mem_map.read_block_raw(0x0E000000, 0xFFFF);
            },
            BackupType::Flash64K => {
                return self.memory_bus.mem_map.read_block_raw(0x0E000000, 0xFFFF);
            },
            BackupType::Flash128K => {
                return self.memory_bus.mem_map.read_block_raw(0x0E000000, 0x20000);
            }
            _ => {log::info!("Save data for this type is not implemented")} 
        }

        return Vec::new();
    }

    pub fn frame(&mut self) {
        while !self.gpu.frame_ready {
            // self.key_status.set_register(0x3FF);
            self.single_step();
        }

        self.gpu.frame_ready = false;
        self.gpu.obj_buffer.iter_mut().for_each(|m|{*m = (Rgb15::new(0x8000), 4, 0)});
    }

    pub fn single_step(&mut self) {

        let cycles = if self.memory_bus.mem_map.halt_state == HaltState::Running {
            self.cpu.fetch(&mut self.memory_bus)
        } else {
            self.gpu.cycles_to_next_state as usize
            // 1
        };

        self.gpu.step(cycles, &mut self.memory_bus.mem_map, &mut self.interrupt_handler, &mut self.dma_control);
        self.timer_handler.update(cycles, &mut self.interrupt_handler);
        self.dma_control.update(&mut self.memory_bus, &mut self.interrupt_handler);
        self.interrupt_handler.service(&mut self.cpu, &mut self.memory_bus);
    }
}
