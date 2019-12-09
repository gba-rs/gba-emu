extern crate gba_emulator;
use gba_emulator::*;

#[cfg(test)]
mod tests {
//    use gba_emulator::formats::data_processing::DataProcessing;
    use crate::{cpu::cpu::CPU, cpu::cpu::InstructionSet,cpu::cpu::ARM_PC,cpu::cpu::ARM_LR,cpu::cpu::THUMB_PC};
    use gba_emulator::arm_formats::branch::Branch;
    use gba_emulator::arm_formats::common::Instruction;
    use gba_emulator::memory::{memory_map::MemoryMap};

    #[test]
    fn check_cpu_branch_backward() {
        let mut a: Branch = Branch::from(0xEAFFFFEE);
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        cpu.set_register(current_pc, 0x105C8);
        a.execute(&mut cpu,&mut map);
        let reg = cpu.get_register(current_pc);
        assert_eq!(reg, 0x10588);
    }

    #[test]
    fn check_cpu_branch_forward() {
        let mut a: Branch = Branch::from(0xEA000003);
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        cpu.set_register(current_pc, 0x10590);
        a.execute(&mut cpu,&mut map);
        let reg = cpu.get_register(current_pc);
        assert_eq!(reg, 0x105a4);
    }

    #[test]
    fn check_cpu_branch_and_link_backward() {
        let mut a: Branch = Branch::from(0xEBFFFFEE);
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        cpu.set_register(current_pc, 0x105C8);
        a.execute(&mut cpu,&mut map);
        let reg = cpu.get_register(current_pc);
        assert_eq!(reg, 0x10588);
        assert_eq!(cpu.get_register(ARM_LR), 0x105CC);
    }

    #[test]
    fn check_cpu_branch_and_link_forward() {
        let mut a: Branch = Branch::from(0xEB000003);
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        cpu.set_register(current_pc, 0x10590);
        a.execute(&mut cpu,&mut map);
        let reg = cpu.get_register(current_pc);
        assert_eq!(reg, 0x105a4);
        assert_eq!(cpu.get_register(ARM_LR), 0x10594);
    }
}