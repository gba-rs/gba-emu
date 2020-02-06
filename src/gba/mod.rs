pub mod memory_bus;
use crate::cpu::{cpu::CPU, cpu::OperatingMode, cpu::ARM_SP, cpu::ARM_PC};
use crate::memory::{memory_map::MemoryMap, game_pack_rom::GamePackRom, io_registers::IORegisters, work_ram::WorkRam};
use crate::memory::lcd_io_registers::*;
use crate::gpu::gpu::GPU;
use crate::memory::{interrupt_registers::*, key_input_registers::*, system_control::WaitStateControl};
use crate::operations::timing::{MemAccessSize, CycleClock};
use crate::gba::memory_bus::MemoryBus;
use crate::interrupts::interrupts::Interrupts;


pub struct GBA {
    pub cpu: CPU,
    pub gpu: GPU,
    pub memory_bus: MemoryBus,
    // pub game_pack_memory: [GamePackRom; 3],
    pub io_reg: IORegisters,
    pub key_status: KeyStatus,
    pub ket_interrupt_control: KeyInterruptControl,
    pub interrupt_handler: Interrupts
}

impl Default for GBA {
    fn default() -> Self {
        // let temp_gamepack = [
        //     GamePackRom::new(0),
        //     GamePackRom::new(0),
        //     GamePackRom::new(0),
        // ];

        let mut temp: GBA = GBA {
            cpu: CPU::new(),
            gpu: GPU::new(),
            memory_bus: MemoryBus::new(),
            // game_pack_memory: temp_gamepack,
            io_reg: IORegisters::new(0),
            key_status: KeyStatus::new(),
            ket_interrupt_control: KeyInterruptControl::new(),
            interrupt_handler: Interrupts::new()
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

        return temp;
    }
}

impl GBA {
    pub fn new(pc_address: u32, bios: &Vec<u8>, rom: &Vec<u8>) -> GBA {

        let mut temp: GBA = GBA {
            cpu: CPU::new(),
            gpu: GPU::new(),
            io_reg: IORegisters::new(0),
            memory_bus: MemoryBus::new(),
            key_status: KeyStatus::new(),
            ket_interrupt_control: KeyInterruptControl::new(),
            interrupt_handler: Interrupts::new()  
        };

        temp.gpu.register(&temp.memory_bus.mem_map.memory);
        temp.key_status.register(&temp.memory_bus.mem_map.memory);
        temp.ket_interrupt_control.register(&temp.memory_bus.mem_map.memory);
        temp.interrupt_handler.ime_interrupt.register(&temp.memory_bus.mem_map.memory);
        temp.interrupt_handler.ie_interrupt.register(&temp.memory_bus.mem_map.memory);
        temp.interrupt_handler.if_interrupt.register(&temp.memory_bus.mem_map.memory);

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
            if self.interrupt_handler.enabled() && !self.cpu.cpsr.control_bits.irq_disable {
                self.interrupt_handler.service(&mut self.cpu);
            }
            self.cpu.fetch(&mut self.memory_bus.mem_map);
        }

        self.gpu.step(self.cpu.cycle_count as u32, &mut self.memory_bus.mem_map);
        self.cpu.cycle_count = 0;
    }
}
