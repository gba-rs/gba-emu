use crate::cpu::{cpu::CPU, cpu::InstructionSet, cpu::OperatingMode, cpu::ARM_PC, cpu::ARM_LR, condition::Condition};
use crate::operations::instruction::Instruction;
use crate::gba::memory_bus::MemoryBus;

#[derive(Debug)]
pub struct SoftwareInterrupt {
    pub comment_field_arm: u32,
    pub comment_field_thumb: u32,
    pub condition: Condition,
}

impl From<u32> for SoftwareInterrupt {
    fn from(value: u32) -> SoftwareInterrupt {
        return SoftwareInterrupt {
            comment_field_arm: (value & 0xFF_0000) >> 16,
            comment_field_thumb: value & 0xFF_0000,
            condition: Condition::from((value & 0xF000_0000) >> 28)
        };
    }
}

impl Instruction for SoftwareInterrupt {
    fn execute(&self, cpu: &mut CPU, _mem_bus: &mut MemoryBus) -> u32 {
        let old_cpsr = cpu.cpsr;
        let current_pc = cpu.get_register(ARM_PC);
        cpu.current_instruction_set = InstructionSet::Arm;
        cpu.operating_mode = OperatingMode::Supervisor;
        cpu.cpsr.control_bits.irq_disable = true;
        cpu.cpsr.control_bits.mode_bits = 0b10011;
        cpu.cpsr.control_bits.state_bit = false;
        cpu.set_spsr(old_cpsr);
        let current_pc = cpu.get_register(ARM_PC);
        cpu.set_register(ARM_LR, current_pc + 4); // set LR to the next instruction        
        cpu.set_register(ARM_PC, 0x08);
        _mem_bus.cycle_clock.get_cycles()
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }

    fn cycles(&self) -> u32 { return 3; }
}