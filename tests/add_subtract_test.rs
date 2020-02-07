extern crate gba_emulator;

#[cfg(test)]
mod test {
    use gba_emulator::cpu::cpu::{CPU, InstructionSet, ARM_PC, THUMB_PC};
    use gba_emulator::thumb_formats::add_subtract::AddSubtract;
    use gba_emulator::arm_formats::branch_exchange::BranchExchange;
    use gba_emulator::operations::instruction::Instruction;
    use gba_emulator::gba::memory_bus::MemoryBus;


    #[test]
    fn correct_operation_called_add(){
        let a: AddSubtract = AddSubtract::from(0x188F);
        let b: BranchExchange = BranchExchange::from(0xD12F_FF1F);
        let mut cpu = CPU::new();
        let mut bus = MemoryBus::new();
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        cpu.set_register(current_pc, 1);
        b.execute(&mut cpu,&mut bus);
        cpu.set_register(1,1);
        cpu.set_register(2,2);
        a.execute(&mut cpu,&mut bus);
        assert_eq!(cpu.get_register(7), 1 + 2);   
    }
    #[test]
    fn correct_operation_called_sub(){
        let a: AddSubtract = AddSubtract::from(0x1A8F);
        let b: BranchExchange = BranchExchange::from(0xD12F_FF1F);
        let mut cpu = CPU::new();
        let mut bus = MemoryBus::new();
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        cpu.set_register(current_pc, 1);
        b.execute(&mut cpu,&mut bus);
        cpu.set_register(1,5);
        cpu.set_register(2,2);
        a.execute(&mut cpu,&mut bus);
        assert_eq!(cpu.get_register(7), 5 - 2);   
    }
    #[test]
    fn correct_operation_called_add_i(){
        let a: AddSubtract = AddSubtract::from(0x1DCF);
        let b: BranchExchange = BranchExchange::from(0xD12F_FF1F);
        let mut cpu = CPU::new();
        let mut bus = MemoryBus::new();
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        cpu.set_register(current_pc, 1);
        b.execute(&mut cpu,&mut bus);
        cpu.set_register(1,1);
        a.execute(&mut cpu,&mut bus);
        assert_eq!(cpu.get_register(7), 1 + 7);   
    }

    #[test]
    fn correct_operation_called_sub_i(){
        let a: AddSubtract = AddSubtract::from(0x1E4F);
        let b: BranchExchange = BranchExchange::from(0xD12F_FF1F);
        let mut cpu = CPU::new();
        let mut bus = MemoryBus::new();
        let current_pc = if cpu.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        cpu.set_register(current_pc, 1);
        b.execute(&mut cpu,&mut bus);
        cpu.set_register(1,4);
        a.execute(&mut cpu,&mut bus);
        assert_eq!(cpu.get_register(7), 4 - 1);  
    }
}