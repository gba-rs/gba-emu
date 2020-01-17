use crate::operations::instruction::Instruction;
use crate::cpu::{cpu::CPU};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::cpu::{THUMB_PC};
use std::fmt;

pub struct LDR {
    pub destination: u8,
    pub word8: u16,
}

impl From<u16> for LDR {
    fn from(value: u16) -> LDR {
        return LDR {
            destination: ((value & 0x700) >> 8) as u8,
            word8: (value & 0xFF) << 2,
        }
    }
}

impl fmt::Debug for LDR {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
            write!(f, "LDR {:?}, [PC, #0x{:X}]", self.destination, self.word8)
    }
}

impl Instruction for LDR {
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let current_pc = cpu.get_register(THUMB_PC) + 2; // another +2 in 
        let value = mem_map.read_u32(current_pc + self.word8 as u32);
        //add PC and Word8
        cpu.set_register(self.destination, value);
    }
    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
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
        assert_eq!(b.word8, 0);
    }

    #[test]
    fn load_zero() {
        let b: LDR = LDR::from(0x8802);
        assert_eq!(b.word8, 8);
    }

    #[test]
    fn pc_set() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        gba.cpu.set_register(THUMB_PC, 0x08000000);
        gba.mem_map.write_u32(0x08000000 + 42, 2000);

        // RD = r1, offset = 20
        let decode_result = gba.cpu.decode(0x490A);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(2000, gba.cpu.get_register(1));

    }
}
