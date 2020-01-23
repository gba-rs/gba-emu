extern crate gba_emulator;


#[cfg(test)]
mod tests {
    use gba_emulator::cpu::cpu::{CPU, OperatingMode};
    use gba_emulator::cpu::program_status_register::ProgramStatusRegister;
    use gba_emulator::arm_formats::data_processing::DataProcessing;
    use gba_emulator::operations::instruction::Instruction;
    use gba_emulator::memory::{memory_map::MemoryMap};



    #[test]
    fn correct_operation_called_and() {
        let a: DataProcessing = DataProcessing::from(0xE0012002);
        let mut cpu = CPU::new();
        cpu.set_register(1,1);
        cpu.set_register(2,2);
        let mut map = MemoryMap::new();
        a.execute(&mut cpu,&mut map);
        assert_eq!(cpu.get_register(2), 1 & 2);
    }
    #[test]
        fn correct_operation_called_eor() {
        let a: DataProcessing = DataProcessing::from(0xE0212002);
        let mut cpu = CPU::new();
        cpu.set_register(1,1);
        cpu.set_register(2,2);
        let mut map = MemoryMap::new();
        a.execute(&mut cpu,&mut map);
        assert_eq!(cpu.get_register(2), 1 ^ 2);
    }
    #[test]
        fn correct_operation_called_sub() {
        let a: DataProcessing = DataProcessing::from(0xE0412002);
        let mut cpu = CPU::new();
        cpu.set_register(1,2);
        cpu.set_register(2,2);
        let mut map = MemoryMap::new();
        a.execute(&mut cpu,&mut map);
        assert_eq!(cpu.get_register(2), 2 - 2);
        cpu.set_register(2,1);
        assert_eq!(cpu.get_register(2), 2 - 1);
        cpu.set_register(2,3);
        //[0,2,3]
        a.execute(&mut cpu,&mut map);
        assert_eq!(cpu.get_register(2), 2u32.wrapping_sub(3));



    }
    #[test]
        fn correct_operation_called_rsb() {
        let a: DataProcessing = DataProcessing::from(0xE0612002);
        let mut cpu = CPU::new();
        cpu.set_register(1,2);
        cpu.set_register(2,2);
        let mut map = MemoryMap::new();
        a.execute(&mut cpu,&mut map);
        assert_eq!(cpu.get_register(2), 2 - 2);
        cpu.set_register(2,1);
        a.execute(&mut cpu,&mut map);
        assert_eq!(cpu.get_register(2), 1u32.wrapping_sub(2));
    }
    #[test]
        fn correct_operation_called_add() {
        let a: DataProcessing = DataProcessing::from(0xE0812002);
        let mut cpu = CPU::new();
        cpu.set_register(1,2);
        cpu.set_register(2,2);
//        [0,2,2]
        let mut map = MemoryMap::new();
        a.execute(&mut cpu,&mut map);
        assert_eq!(cpu.get_register(2), 2 + 2);
    }
    #[test]
        fn correct_operation_called_adc() {
        let a: DataProcessing = DataProcessing::from(0xE0A12002);
        let mut cpu = CPU::new();
        cpu.set_register(1,2);
        cpu.set_register(2,2);
//        [0,2,2]
        let mut map = MemoryMap::new();
        a.execute(&mut cpu,&mut map);
        assert_eq!(cpu.get_register(2), 2 + 2 + 1);
    }
    #[test]
        fn correct_operation_called_sbc() {
        let a: DataProcessing = DataProcessing::from(0xE0C12002);
        let mut cpu = CPU::new();
        cpu.set_register(1,2);
        cpu.set_register(2,2);
//        [0,2,2]
        let mut map = MemoryMap::new();
        a.execute(&mut cpu,&mut map);
        assert_eq!(cpu.get_register(2), 0u32.wrapping_sub(1));
    }
        #[test]
            fn correct_operation_called_rsc() {
            let a: DataProcessing = DataProcessing::from(0xE0E12002);
            let mut cpu = CPU::new();
            cpu.set_register(1,1);
            cpu.set_register(2,2);
    //        [0,1,2]
            let mut map = MemoryMap::new();
            a.execute(&mut cpu,&mut map);
            assert_eq!(cpu.get_register(2), 2-1-1);
        }

        #[test]
        fn correct_operation_called_mov() {
            let a: DataProcessing = DataProcessing::from(0xE1A11002);
            let mut cpu = CPU::new();
            cpu.set_register(1,1);
            cpu.set_register(2,2);
    //        [0,1,2]
            let mut map = MemoryMap::new();
            a.execute(&mut cpu,&mut map);
            assert_eq!(cpu.get_register(2), cpu.get_register(1)); //moving 2 into 1 and checking that 1 is now equal to 2
        }

        #[test]
        fn correct_operation_called_bic() {
            let a: DataProcessing = DataProcessing::from(0xE1C12002);
            let mut cpu = CPU::new();
            cpu.set_register(1,1);
            cpu.set_register(2,2);
            let mut map = MemoryMap::new();
            a.execute(&mut cpu,&mut map);
            assert_eq!(cpu.get_register(2), 1 & !2);
        }

        #[test]
        fn correct_operation_called_mvn() {
            let a: DataProcessing = DataProcessing::from(0xE1E12002);
            let mut cpu = CPU::new();
            cpu.set_register(1,1);
            cpu.set_register(2,2);
            let mut map = MemoryMap::new();
            a.execute(&mut cpu,&mut map);
            assert_eq!(cpu.get_register(2), !2);
        }

        #[test]
        fn correct_operation_called_mrs_cpsr() {
            let a: DataProcessing = DataProcessing::from(0xE10F_1000);
            let mut cpu = CPU::new();
            cpu.operating_mode = OperatingMode::Supervisor;
            cpu.set_register(1,1);
            let mut map = MemoryMap::new();
            a.execute(&mut cpu,&mut map);
            assert_eq!(cpu.cpsr.control_bits.fiq_disable, ProgramStatusRegister::from(1).control_bits.fiq_disable);
            assert_eq!(cpu.cpsr.control_bits.irq_disable, ProgramStatusRegister::from(1).control_bits.irq_disable);
            assert_eq!(cpu.cpsr.control_bits.mode_bits, ProgramStatusRegister::from(1).control_bits.mode_bits);
            assert_eq!(cpu.cpsr.control_bits.state_bit, ProgramStatusRegister::from(1).control_bits.state_bit);
        }
        #[test]
        fn correct_operation_called_mrs_spsr() {
            let a: DataProcessing = DataProcessing::from(0xE14F_1000);
            let mut cpu = CPU::new();
            cpu.operating_mode = OperatingMode::Supervisor;
            cpu.set_register(1,1);
            let mut map = MemoryMap::new();
            a.execute(&mut cpu,&mut map);
            assert_eq!(cpu.get_spsr().control_bits.fiq_disable, ProgramStatusRegister::from(1).control_bits.fiq_disable);
            assert_eq!(cpu.get_spsr().control_bits.irq_disable, ProgramStatusRegister::from(1).control_bits.irq_disable);
            assert_eq!(cpu.get_spsr().control_bits.mode_bits, ProgramStatusRegister::from(1).control_bits.mode_bits);
            assert_eq!(cpu.get_spsr().control_bits.state_bit, ProgramStatusRegister::from(1).control_bits.state_bit);
        }
        #[test]
        fn correct_operation_called_msr_cpsr() {
            let a: DataProcessing = DataProcessing::from(0xE12F_F001);
            let mut cpu = CPU::new();
            cpu.operating_mode = OperatingMode::Supervisor;
            cpu.set_register(1,1);
            let mut map = MemoryMap::new();
            a.execute(&mut cpu,&mut map);
            assert_eq!(cpu.cpsr.control_bits.fiq_disable, ProgramStatusRegister::from(0).control_bits.fiq_disable);
            assert_eq!(cpu.cpsr.control_bits.irq_disable, ProgramStatusRegister::from(0).control_bits.irq_disable);
            assert_eq!(cpu.cpsr.control_bits.mode_bits, ProgramStatusRegister::from(0).control_bits.mode_bits);
            assert_eq!(cpu.cpsr.control_bits.state_bit, ProgramStatusRegister::from(0).control_bits.state_bit);
        }
        #[test]
        fn correct_operation_called_msr_spsr() {
            let a: DataProcessing = DataProcessing::from(0xE16F_F001);
            let mut cpu = CPU::new();
            cpu.operating_mode = OperatingMode::Supervisor;
            cpu.set_register(1,1);
            cpu.set_register(2,2);
            let mut map = MemoryMap::new();
            a.execute(&mut cpu,&mut map);
            assert_eq!(cpu.get_spsr().control_bits.fiq_disable, ProgramStatusRegister::from(0).control_bits.fiq_disable);
            assert_eq!(cpu.get_spsr().control_bits.irq_disable, ProgramStatusRegister::from(0).control_bits.irq_disable);
            assert_eq!(cpu.get_spsr().control_bits.mode_bits, ProgramStatusRegister::from(0).control_bits.mode_bits);
            assert_eq!(cpu.get_spsr().control_bits.state_bit, ProgramStatusRegister::from(0).control_bits.state_bit);
        }
}