use crate::operations::instruction::Instruction;
use crate::cpu::{cpu::CPU, cpu::THUMB_SP};
use crate::operations::load_store::{DataTransfer, DataType, data_transfer_execute};
use std::fmt;
use crate::gba::memory_bus::MemoryBus;

pub struct SpLoadStore {
    pub load: bool,
    pub destination: u8,
    pub offset: u16,
}

impl From<u16> for SpLoadStore {
    fn from(value: u16) -> SpLoadStore {
        return SpLoadStore {
            load: ((value & 0x800) >> 11) != 0,
            destination: ((value & 0x700) >> 8) as u8,
            offset: ((value & 0xFF) << 2) as u16
        }
    }
}

impl Instruction for SpLoadStore {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) -> u32 {
        let transfer_info = DataTransfer {
            is_pre_indexed: true,
            write_back: false,
            load: self.load,
            is_signed: false,
            data_type: DataType::Word,
            base_register: THUMB_SP,
            destination: self.destination,
        };

        let target_address = cpu.get_register(THUMB_SP) + self.offset as u32;
        let base = cpu.get_register(THUMB_SP);

        data_transfer_execute(transfer_info, base, target_address, cpu, mem_bus);
        mem_bus.cycle_clock.get_cycles()
    }

    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 3;} // 1s + 1n + 1l

}

impl fmt::Debug for SpLoadStore {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        if self.load {
            write!(f, "LDR r{}, [SP, #0x{:X}]", self.destination, self.offset)
        } else {
            write!(f, "STR r{}, [SP, #0x{:X}]", self.destination, self.offset)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;
    use crate::cpu::{cpu::InstructionSet};
    use std::borrow::{BorrowMut};

    #[test]
    fn ldr_sp_test() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        gba.memory_bus.mem_map.write_u32(0x02000050, 1000);
        gba.cpu.set_register(THUMB_SP, 0x02000000);

        // LDR r4, [SP, 50]
        match gba.cpu.decode(0x9C14) {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(1000, gba.cpu.get_register(4));
    }

    #[test]
    fn str_sp_test() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        gba.cpu.set_register(THUMB_SP, 0x02000000);
        gba.cpu.set_register(4, 1000);

        // str r4, [SP, 50]
        match gba.cpu.decode(0x9414) {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }
        
        assert_eq!(1000, gba.memory_bus.mem_map.read_u32(0x02000050));
    }
}

