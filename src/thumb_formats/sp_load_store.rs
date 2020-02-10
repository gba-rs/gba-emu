use crate::operations::instruction::Instruction;
use crate::operations::arm_arithmetic;
use crate::cpu::{cpu::CPU, cpu::THUMB_SP};
use std::fmt;
use crate::gba::memory_bus::MemoryBus;

pub struct SpLoadStore {
    pub load: bool,
    pub destination: u8,
    pub word8: u16,
}

impl From<u16> for SpLoadStore {
    fn from(value: u16) -> SpLoadStore {
        return SpLoadStore {
            load: ((value & 0x800) >> 11) != 0,
            destination: ((value & 0x700) >> 8) as u8,
            word8: ((value & 0xFF) << 2) as u16
        }
    }
}

impl Instruction for SpLoadStore {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) -> u32 {
        let stack_pointer = cpu.get_register(THUMB_SP);
        let (address, _) = arm_arithmetic::add(stack_pointer, self.word8 as u32);
        if self.load {
            let value = mem_bus.read_u32(address);
            cpu.set_register(self.destination, value);
        } else {
            let value = cpu.get_register(self.destination);
            mem_bus.write_u32(address, value);
        }
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
            write!(f, "LDR r{}, [SP, #0x{:X}]", self.destination, self.word8)
        } else {
            write!(f, "STR r{}, [SP, #0x{:X}]", self.destination, self.word8)
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

