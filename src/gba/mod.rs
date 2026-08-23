use crate::cpu::{cpu::CPU, cpu::OperatingMode, cpu::ARM_SP, cpu::ARM_PC, cpu::THUMB_PC, cpu::InstructionSet};
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

        temp.key_status.set_register(0xFFFF);

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

    /// The authentic GBA BIOS's SWI dispatcher (interrupt vector 0x08) saves
    /// only `{r2, lr}` around the call into the actual SWI handler — see the
    /// Cult-of-GBA BIOS reimplementation's `exception_swi`. Several common
    /// `gba_bios.bin` dumps instead have `{r2, r3, lr}` at this exact spot (a
    /// one-bit difference in the STM/LDM register-list field, at BIOS
    /// offsets 0x88 and 0x94), which silently discards r3 across every SWI
    /// call. That breaks `SWI 06h`/`07h` (Div/DivArm) in particular, since
    /// GBATEK documents r3 as carrying `abs(quotient)` back to the caller —
    /// e.g. Minish Cap's file-select screen reads r3 right after a Div call
    /// to pick a movement direction, gets garbage instead, and the affected
    /// entity's position never converges, which keeps the screen's
    /// `isTransitioning` flag pinned and blocks all input forever.
    /// Only patches when the known-bad bytes are found, so a correctly
    /// dumped BIOS is left untouched.
    fn patch_known_bad_bios_swi_dispatcher(&mut self) {
        // BIOS space isn't writable through write_u8/write_u32 (real
        // hardware can't write its own BIOS either), so this pokes the
        // backing memory directly, the same way load_bios/load_rom do via
        // write_block.
        const BAD_STMFD: [u8; 4] = [0x0C, 0x40, 0x2D, 0xE9];
        const GOOD_STMFD: [u8; 4] = [0x04, 0x40, 0x2D, 0xE9];
        const BAD_LDMFD: [u8; 4] = [0x0C, 0x40, 0xBD, 0xE8];
        const GOOD_LDMFD: [u8; 4] = [0x04, 0x40, 0xBD, 0xE8];

        let mut mem = self.memory_bus.mem_map.memory.borrow_mut();
        if mem[0x88..0x8C] == BAD_STMFD {
            mem[0x88..0x8C].copy_from_slice(&GOOD_STMFD);
        }
        if mem[0x94..0x98] == BAD_LDMFD {
            mem[0x94..0x98].copy_from_slice(&GOOD_LDMFD);
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

        // Diagnostic: a cheap checksum of the fully composited frame,
        // logged once per frame.
        let checksum = self.gpu.frame_buffer.iter().fold(0u32, |acc, &p| acc.wrapping_add(p).rotate_left(1));
        let pc = self.cpu.get_register(if self.cpu.get_instruction_set() == InstructionSet::Arm { ARM_PC } else { THUMB_PC });
        log::info!(
            "FRAME: checksum={:#010X} halt={:?} pc={:#010X} ime={:#X} ie={:#06X} if={:#06X}",
            checksum, self.memory_bus.mem_map.halt_state, pc,
            self.interrupt_handler.ime_interrupt.get_register(),
            self.interrupt_handler.ie_interrupt.get_register(),
            self.interrupt_handler.if_interrupt.get_register()
        );

        self.gpu.frame_ready = false;
        self.gpu.obj_buffer.iter_mut().for_each(|m|{*m = (Rgb15::new(0x8000), 4, 0)});
        self.gpu.obj_window = [false; (DISPLAY_WIDTH as usize) * (DISPLAY_HEIGHT as usize)];
    }

    pub fn single_step(&mut self) {
        // log::info!("Single stepping");
        let cycles = if self.memory_bus.mem_map.halt_state == HaltState::Running {
            // log::info!("Stepping cpu");
            self.cpu.fetch(&mut self.memory_bus)
        } else {
            // log::info!("Skippig cpu {:?}", self.memory_bus.mem_map.halt_state);
            self.gpu.cycles_to_next_state as usize
            // 1
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

/// Per GBATEK's KEYCNT (Key Interrupt Control, 0x4000132): bit 14 enables
/// the keypad IRQ; bit 15 selects the combination logic over whichever
/// buttons are selected in bits 0-9 (same bit layout as KEYINPUT/KeyStatus)
/// — 0=Logical OR (any selected button pressed), 1=Logical AND (all
/// selected buttons pressed simultaneously). KEYINPUT is active-low
/// (0=pressed, 1=released), so it's inverted before comparing against the
/// KEYCNT selection mask. This was previously never evaluated anywhere —
/// KeyInterruptControl was a fully modeled, readable/writable I/O register
/// that nothing ever read, so any game relying on a keypad-IRQ-driven
/// input wait (rather than plain per-frame KEYINPUT polling) would HALT
/// forever no matter what was pressed.
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
        let key_cnt = BUTTON_A; // selected, but bit 14 (enable) is 0
        let a_pressed = ALL_RELEASED & !BUTTON_A;
        assert!(!keypad_interrupt_condition_met(a_pressed, key_cnt));
    }

    #[test]
    fn or_mode_fires_when_any_selected_button_is_pressed() {
        let key_cnt = IRQ_ENABLE | BUTTON_A | BUTTON_START; // OR (bit 15 = 0)
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
        // Selection mask (bits 0-9) is all zero; enable/condition bits alone
        // must not vacuously satisfy an AND-mode check over an empty set.
        let key_cnt = IRQ_ENABLE | IRQ_CONDITION_AND;
        assert!(!keypad_interrupt_condition_met(ALL_RELEASED, key_cnt));
        assert!(!keypad_interrupt_condition_met(0, key_cnt));
    }

    #[test]
    fn unselected_buttons_being_pressed_is_irrelevant() {
        let key_cnt = IRQ_ENABLE | BUTTON_A; // only A selected, OR mode
        let only_start_pressed = ALL_RELEASED & !BUTTON_START;
        assert!(!keypad_interrupt_condition_met(only_start_pressed, key_cnt));
    }
}
