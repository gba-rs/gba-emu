use crate::cpu::{cpu::CPU, cpu::OperatingMode, cpu::ARM_SP, cpu::ARM_PC};
use crate::gpu::{gpu::GPU, gpu::DISPLAY_WIDTH, gpu::DISPLAY_HEIGHT};
use crate::gpu::rgb15::Rgb15;
use crate::memory::{key_input_registers::*};
use crate::memory::{memory_bus::MemoryBus, memory_bus::BackupType};
use crate::memory::memory_bus::HaltState;
use crate::interrupts::interrupts::Interrupts;
use crate::dma::DMAController;
use crate::timers::timer::TimerHandler;


pub const MEM_STRINGS: [&str; 5] = ["SRAM", "EEPROM", "FLASH_", "FLASH512_", "FLASH1M_"];

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
        return GBA::new(0x08000000, &Vec::new(), &Vec::new());
    }
}

impl GBA {

    fn detect_backup_type(&mut self, rom: &Vec<u8>) {
        for i in 0..5 {
            let mem_string_bytes = MEM_STRINGS[i].as_bytes();
            let result = rom.windows(mem_string_bytes.len()).position(|window| window == mem_string_bytes);
            match result {
                Some(_) => {
                    // string exists
                    log::info!("Found backup type: {}", MEM_STRINGS[i]);
                },
                None => {
                    // string doesn't exist
                }
            }
        }
    }

    pub fn new(pc_address: u32, bios: &Vec<u8>, rom: &Vec<u8>) -> GBA {

        let mut temp: GBA = GBA {
            cpu: CPU::new(),
            gpu: GPU::new(),
            memory_bus: MemoryBus::new(),
            key_status: KeyStatus::new(),
            ket_interrupt_control: KeyInterruptControl::new(),
            interrupt_handler: Interrupts::new(),
            timer_handler: TimerHandler::new(),
            dma_control: DMAController::new()
        };

        temp.detect_backup_type(rom);

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
        temp.load_bios(bios);
        temp.load_rom(rom);
        return temp;
    }

    pub fn load_bios(&mut self, bios: &Vec<u8>) {
        self.memory_bus.mem_map.write_block(0, bios)
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        self.memory_bus.mem_map.write_block(0x08000000, rom)
    }

    pub fn frame(&mut self) {
        while !self.gpu.frame_ready {
            // self.key_status.set_register(0x3FF);
            self.single_step();
        }

        self.gpu.frame_ready = false;
        self.gpu.obj_buffer = vec![Rgb15::new(0x8000); (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize];
    }

    pub fn single_step(&mut self) {

        let cycles = if self.memory_bus.halt_state == HaltState::Running {
            self.cpu.fetch(&mut self.memory_bus)
        } else {
            self.gpu.cycles_to_next_state as usize
        };

        self.gpu.step(cycles, &mut self.memory_bus.mem_map, &mut self.interrupt_handler, &mut self.dma_control);
        self.timer_handler.update(cycles, &mut self.interrupt_handler);
        self.dma_control.update(&mut self.memory_bus, &mut self.interrupt_handler);
        self.interrupt_handler.service(&mut self.cpu, &mut self.memory_bus);
    }
}
