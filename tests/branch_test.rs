extern crate gba_emulator;
use gba_emulator::*;

#[cfg(test)]
mod tests {
//    use gba_emulator::formats::data_processing::DataProcessing;
    use crate::{cpu::cpu::CPU, cpu::cpu::InstructionSet,cpu::cpu::ARM_PC,cpu::cpu::THUMB_PC, cpu::cpu::REG_MAP};
    use gba_emulator::formats::data_processing::DataProcessing;
    use gba_emulator::formats::branch::Branch;
    use gba_emulator::formats::common::Instruction;
    use gba_emulator::memory::{work_ram::WorkRam, bios_ram::BiosRam, memory_map::MemoryMap};



    #[test]
    fn check_cpu_branch() {
//        assert_eq!()
        let mut a: Branch = Branch::from(0x0A2F_FF1F); //0000 1010 __offset ends here_ 0010 1111 1111 1111 0001 1111‬
        let mut cpu = CPU::new();
        cpu.set_register(1,1);
        let mut map = MemoryMap::new();
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        println!("Before: {:X}", cpu.get_register(current_pc));
        a.execute(&mut cpu,&mut map);
        println!("After: {:X}", cpu.get_register(current_pc));
        assert_eq!(cpu.get_register(2),0);
    }
}