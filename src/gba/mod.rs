use crate::cpu::{cpu::CPU, cpu::OperatingMode, cpu::ARM_SP, cpu::ARM_PC};
use crate::gpu::{gpu::GPU, gpu::DISPLAY_WIDTH, gpu::DISPLAY_HEIGHT};
use crate::gpu::rgb15::Rgb15;
use crate::memory::{key_input_registers::*};
use crate::memory::{memory_bus::MemoryBus, memory_map::HaltState};
use crate::interrupts::interrupts::Interrupts;
use crate::dma::DMAController;
use crate::timers::timer::TimerHandler;
use crate::apu::Apu;
use crate::{gamepak::GamePack, gamepak::BackupType};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GBA {
    pub cpu: CPU,
    pub gpu: GPU,
    pub memory_bus: MemoryBus,
    pub key_status: KeyStatus,
    pub ket_interrupt_control: KeyInterruptControl,
    pub interrupt_handler: Interrupts,
    pub timer_handler: TimerHandler,
    pub dma_control: DMAController,
    pub apu: Apu
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
            dma_control: DMAController::new(),
            apu: Apu::new()
        };

        temp.register_memory();

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

        temp.key_status.set_register(0x03FF);

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
        temp.memory_bus.mem_map.configure_rom(game_pack.rom.len(), &game_pack.game_code);

        return temp;
    }

    pub fn register_memory(&mut self) {
        self.gpu.register(&self.memory_bus.mem_map.memory);
        self.key_status.register(&self.memory_bus.mem_map.memory);
        self.ket_interrupt_control.register(&self.memory_bus.mem_map.memory);
        self.interrupt_handler.ime_interrupt.register(&self.memory_bus.mem_map.memory);
        self.interrupt_handler.ie_interrupt.register(&self.memory_bus.mem_map.memory);
        self.interrupt_handler.if_interrupt.register(&self.memory_bus.mem_map.memory);
        self.timer_handler.register(&self.memory_bus.mem_map.memory);
        self.memory_bus.cycle_clock.register(&self.memory_bus.mem_map.memory);
        self.dma_control.register(&self.memory_bus.mem_map.memory);
        self.apu.register(&self.memory_bus.mem_map.memory);
    }

    pub fn load_bios(&mut self, bios: &Vec<u8>) {
        self.memory_bus.mem_map.write_block(0, bios);
        self.patch_known_bad_bios_swi_dispatcher();
    }

    fn patch_known_bad_bios_swi_dispatcher(&mut self) {
        const BAD_STMFD: [u8; 4] = [0x0C, 0x40, 0x2D, 0xE9];
        const GOOD_STMFD: [u8; 4] = [0x04, 0x40, 0x2D, 0xE9];
        const BAD_LDMFD: [u8; 4] = [0x0C, 0x40, 0xBD, 0xE8];
        const GOOD_LDMFD: [u8; 4] = [0x04, 0x40, 0xBD, 0xE8];

        let mem = &self.memory_bus.mem_map.memory;
        let read4 = |start: usize| -> [u8; 4] {
            [mem[start].get(), mem[start + 1].get(), mem[start + 2].get(), mem[start + 3].get()]
        };
        let write4 = |start: usize, bytes: [u8; 4]| {
            for (i, b) in bytes.iter().enumerate() {
                mem[start + i].set(*b);
            }
        };
        if read4(0x88) == BAD_STMFD {
            write4(0x88, GOOD_STMFD);
        }
        if read4(0x94) == BAD_LDMFD {
            write4(0x94, GOOD_LDMFD);
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        self.memory_bus.mem_map.write_block(0x08000000, rom)
    }

    pub fn load_save_file(&mut self, save_data: &Vec<u8>) {
        match self.memory_bus.mem_map.backup_type {
            BackupType::Sram | BackupType::Flash64K | BackupType::Flash128K => {
                self.memory_bus.mem_map.write_block(0x0E000000, save_data);
            },
            BackupType::Eeprom => {
                self.memory_bus.mem_map.eeprom.borrow_mut().import_bytes(save_data);
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
            BackupType::Eeprom => {
                return self.memory_bus.mem_map.eeprom.borrow().export_bytes();
            },
            _ => {log::info!("Save data for this type is not implemented")}
        }

        return Vec::new();
    }

    pub fn frame(&mut self) {
        while !self.gpu.frame_ready {
            self.single_step();
        }

        self.gpu.frame_ready = false;
        self.gpu.obj_buffer.iter_mut().for_each(|m|{*m = (Rgb15::new(0x8000), 4, 0)});
        self.gpu.obj_window = [false; (DISPLAY_WIDTH as usize) * (DISPLAY_HEIGHT as usize)];
    }

    pub fn frame_until_breakpoint(&mut self, breakpoints: &std::collections::HashSet<u32>, max_steps: u32) -> bool {
        for _ in 0..max_steps {
            if breakpoints.contains(&self.cpu.get_pc()) {
                return false;
            }

            self.single_step();

            if self.gpu.frame_ready {
                self.gpu.frame_ready = false;
                self.gpu.obj_buffer.iter_mut().for_each(|m| { *m = (Rgb15::new(0x8000), 4, 0) });
                self.gpu.obj_window = [false; (DISPLAY_WIDTH as usize) * (DISPLAY_HEIGHT as usize)];
                return true;
            }
        }

        true
    }

    pub fn single_step(&mut self) {
        // log::info!("Single stepping");
        let cycles = if self.memory_bus.mem_map.halt_state == HaltState::Running {
            // log::info!("Stepping cpu");
            self.cpu.fetch(&mut self.memory_bus, &mut self.dma_control, &mut self.interrupt_handler)
        } else {
            // log::info!("Skippig cpu {:?}", self.memory_bus.mem_map.halt_state);
            let mut skip = self.gpu.cycles_to_next_state.max(0) as usize;
            for timer in self.timer_handler.timers.iter() {
                if let Some(until_overflow) = timer.cycles_until_irq_overflow() {
                    skip = skip.min(until_overflow);
                }
            }
            skip.max(1)
        };

        self.gpu.step(cycles, &mut self.memory_bus.mem_map, &mut self.interrupt_handler, &mut self.dma_control);
        let timer_overflows = self.timer_handler.update(cycles, &mut self.interrupt_handler);
        self.dma_control.update(&mut self.memory_bus, &mut self.interrupt_handler, timer_overflows);
        let timer_periods = [
            self.timer_handler.timers[0].period_cycles(),
            self.timer_handler.timers[1].period_cycles(),
            self.timer_handler.timers[2].period_cycles(),
            self.timer_handler.timers[3].period_cycles(),
        ];
        self.apu.step(cycles, timer_periods, &mut self.memory_bus);

        if keypad_interrupt_condition_met(self.key_status.get_register(), self.ket_interrupt_control.get_register()) {
            self.interrupt_handler.if_interrupt.set_keypad(1);
        }

        self.interrupt_handler.service(&mut self.cpu, &mut self.memory_bus);
    }
}

fn keypad_interrupt_condition_met(key_status_raw: u16, key_cnt_raw: u16) -> bool {
    let irq_enabled = (key_cnt_raw >> 14) & 1 == 1;
    if !irq_enabled {
        return false;
    }

    let selected = key_cnt_raw & 0x3FF;
    if selected == 0 {
        return false;
    }

    let pressed = (!key_status_raw) & 0x3FF;
    let and_mode = (key_cnt_raw >> 15) & 1 == 1;
    if and_mode {
        (pressed & selected) == selected
    } else {
        (pressed & selected) != 0
    }
}

#[cfg(test)]
mod keypad_interrupt_tests {
    use super::keypad_interrupt_condition_met;

    const IRQ_ENABLE: u16 = 1 << 14;
    const IRQ_CONDITION_AND: u16 = 1 << 15;
    const BUTTON_A: u16 = 1 << 0;
    const BUTTON_START: u16 = 1 << 3;
    const ALL_RELEASED: u16 = 0xFFFF;

    #[test]
    fn disabled_never_fires_even_if_selected_buttons_are_pressed() {
        let key_cnt = BUTTON_A;
        let a_pressed = ALL_RELEASED & !BUTTON_A;
        assert!(!keypad_interrupt_condition_met(a_pressed, key_cnt));
    }

    #[test]
    fn or_mode_fires_when_any_selected_button_is_pressed() {
        let key_cnt = IRQ_ENABLE | BUTTON_A | BUTTON_START;
        let only_a_pressed = ALL_RELEASED & !BUTTON_A;
        assert!(keypad_interrupt_condition_met(only_a_pressed, key_cnt));
    }

    #[test]
    fn or_mode_does_not_fire_when_no_selected_button_is_pressed() {
        let key_cnt = IRQ_ENABLE | BUTTON_A | BUTTON_START;
        assert!(!keypad_interrupt_condition_met(ALL_RELEASED, key_cnt));
    }

    #[test]
    fn and_mode_fires_only_when_all_selected_buttons_are_pressed() {
        let key_cnt = IRQ_ENABLE | IRQ_CONDITION_AND | BUTTON_A | BUTTON_START;
        let only_a_pressed = ALL_RELEASED & !BUTTON_A;
        let both_pressed = ALL_RELEASED & !BUTTON_A & !BUTTON_START;
        assert!(!keypad_interrupt_condition_met(only_a_pressed, key_cnt));
        assert!(keypad_interrupt_condition_met(both_pressed, key_cnt));
    }

    #[test]
    fn no_buttons_selected_never_fires() {
        let key_cnt = IRQ_ENABLE | IRQ_CONDITION_AND;
        assert!(!keypad_interrupt_condition_met(ALL_RELEASED, key_cnt));
        assert!(!keypad_interrupt_condition_met(0, key_cnt));
    }

    #[test]
    fn unselected_buttons_being_pressed_is_irrelevant() {
        let key_cnt = IRQ_ENABLE | BUTTON_A;
        let only_start_pressed = ALL_RELEASED & !BUTTON_START;
        assert!(!keypad_interrupt_condition_met(only_start_pressed, key_cnt));
    }
}

#[cfg(test)]
mod single_step_tests {
    use super::GBA;
    use crate::memory::memory_map::HaltState;

    #[test]
    fn halted_with_negative_cycles_to_next_state_does_not_stall() {
        let mut gba = GBA::default();
        gba.memory_bus.mem_map.halt_state = HaltState::Halt;
        gba.gpu.cycles_to_next_state = -100;

        gba.single_step();

        assert!(gba.gpu.cycles_to_next_state.abs() < 1_000_000);
    }

    #[test]
    fn frame_until_breakpoint_stops_before_executing_the_breakpointed_instruction() {
        let mut gba = GBA::default();
        let pc = gba.cpu.get_pc();
        let mut breakpoints = std::collections::HashSet::new();
        breakpoints.insert(pc);

        let completed = gba.frame_until_breakpoint(&breakpoints, 1_000_000);

        assert!(!completed);
        assert_eq!(gba.cpu.get_pc(), pc);
    }

    #[test]
    fn frame_until_breakpoint_runs_a_full_frame_when_no_breakpoint_is_hit() {
        let mut gba = GBA::default();
        let breakpoints = std::collections::HashSet::new();

        let completed = gba.frame_until_breakpoint(&breakpoints, 1_000_000);

        assert!(completed);
        assert!(!gba.gpu.frame_ready);
    }
}
