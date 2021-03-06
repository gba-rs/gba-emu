use crate::operations::instruction::Instruction;
use crate::cpu::{cpu::CPU};
use crate::cpu::cpu::{THUMB_PC};
use std::fmt;
use crate::memory::memory_bus::MemoryBus;

pub struct LDR {
    pub destination: u8,
    pub offset: u16,
}

impl From<u16> for LDR {
    fn from(value: u16) -> LDR {
        return LDR {
            destination: ((value & 0x700) >> 8) as u8,
            offset: (value & 0xFF) << 2,
        }
    }
}

impl fmt::Debug for LDR {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
            write!(f, "LDR r{:?}, [PC, #0x{:X}]", self.destination, self.offset)
    }
}

impl Instruction for LDR {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) -> u32 {
        let mut current_pc = cpu.get_register(THUMB_PC) + 2; // another +2 in
        current_pc &= !0x02;
        let value = mem_bus.read_u32(current_pc + (self.offset as u32));
        cpu.set_register(self.destination, value);
        mem_bus.cycle_clock.get_cycles()
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 3;} // 1s + 1n + 1l

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;
    use crate::cpu::{cpu::InstructionSet, cpu::THUMB_PC};
    use std::borrow::{BorrowMut};

    #[test]
    fn load_non_zero() {
        let b: LDR = LDR::from(0x8800);
        assert_eq!(b.offset, 0);
    }

    #[test]
    fn load_zero() {
        let b: LDR = LDR::from(0x8802);
        assert_eq!(b.offset, 8);
    }

    #[test]
    fn pc_set() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.set_instruction_set(InstructionSet::Thumb);

        gba.cpu.set_register(THUMB_PC, 0x08000000);
        gba.memory_bus.mem_map.write_u32(0x08000000 + 40, 2000);

        // RD = r1, offset = 20
        let decode_result = gba.cpu.decode(0x490A);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(2000, gba.cpu.get_register(1));

    }
}
