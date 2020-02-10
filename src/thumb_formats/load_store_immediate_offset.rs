use crate::operations::load_store::DataType;
use crate::operations::instruction::Instruction;
use crate::cpu::{cpu::CPU};
use std::fmt;
use crate::gba::memory_bus::MemoryBus;

pub struct LoadStoreImmediateOffset {
    load: bool,
    data_type: DataType,
    offset_register: u8,
    rb: u8,
    rd: u8,
}

impl From<u16> for LoadStoreImmediateOffset {
    fn from(value: u16) -> LoadStoreImmediateOffset {
        let data_type: DataType;

        if ((value & 0x1000) >> 12) == 0 {
            data_type = DataType::Word
        } else {
            data_type = DataType::Byte
        }
        return LoadStoreImmediateOffset {
            load: ((value & 0x800) >> 11) != 0,
            data_type,
            offset_register: ((value & 0x7C0) >> 6) as u8,
            rb: ((value & 0x38) >> 3) as u8,
            rd: (value & 0x7) as u8,
        };
    }
}
impl fmt::Debug for LoadStoreImmediateOffset {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        if !self.load && self.data_type ==  DataType::Word {
            write!(f, "STR r{}, [r{}, #0x{:X}]", self.rd, self.rb, self.offset_register)
        } else if self.load && self.data_type ==  DataType::Word {
            write!(f, "LDR r{}, [r{}, #0x{:X}]", self.rd, self.rb, self.offset_register)
        } else if !self.load && self.data_type ==  DataType::Byte {
            write!(f, "STRB r{}, [r{}, #0x{:X}]", self.rd, self.rb, self.offset_register)
        } else if self.load && self.data_type ==  DataType::Byte {
            write!(f, "LDRB r{}, [r{}, #0x{:X}]", self.rd, self.rb, self.offset_register)
        }
        else {
            write!(f, "error")
        }
    }
}

impl Instruction for LoadStoreImmediateOffset {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) {
        if !self.load && self.data_type ==  DataType::Word { //str
            //calculating target address by adding together Rb and offset. Store Rd at target
            //assuming word is u32 as shown in load_store
            let target_address: u32 = (cpu.get_register(self.rb) + (self.offset_register << 2) as u32) as u32;
            mem_bus.write_u32(target_address as u32,cpu.get_register(self.rd));

        } else if self.load && self.data_type ==  DataType::Word { //ldr
            //calculate source address by adding Rb and offset. Load rd form source
            let source_address: u32 = (cpu.get_register(self.rb) + (self.offset_register << 2) as u32) as u32;
            let response = mem_bus.read_u32(source_address as u32);
            cpu.set_register(self.rd, response);

        } else if !self.load && self.data_type ==  DataType::Byte { //strb
            //calculating target address by adding together Rb and offset. Store Rd at target
            //assuming word is u32 as shown in load_store
            let target_address: u32 = (cpu.get_register(self.rb) + self.offset_register as u32) as u32;
            mem_bus.write_u8(target_address as u32, cpu.get_register(self.rd) as u8);

        } else if self.load && self.data_type ==  DataType::Byte { //ldrb
            //calculate source address by adding Rb and offset. Load rd form source
            let source_address: u8 = (cpu.get_register(self.rb) + self.offset_register as u32) as u8;
            let response = mem_bus.read_u8(source_address as u32);
            cpu.set_register(self.rd, response as u32);
        }
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
        fn cycles(&self) -> u32 {return 3;} // 1s + 1n + 1l
    // unless pc then its 5 2s + 2n + 1l but that isn't known till later.
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;
    use crate::cpu::{cpu::InstructionSet};
    use std::borrow::{BorrowMut};

    #[test]
    fn test_creation_0s() {
        let format = LoadStoreImmediateOffset::from(0x6000);
        assert_eq!(format.load, false);
        assert_eq!(format.data_type, DataType::Word);
        assert_eq!(format.offset_register, 0);
        assert_eq!(format.rb, 0);
        assert_eq!(format.rd, 0);
    }

    #[test]
    fn test_creation() {
        let format = LoadStoreImmediateOffset::from(0x693B);

        assert_eq!(format.load, true);
        assert_eq!(format.data_type, DataType::Word);
        assert_eq!(format.offset_register, 4);
        assert_eq!(format.rb, 7);
        assert_eq!(format.rd, 3);
    }

    #[test]
    fn test_creation_byte() {
        let format = LoadStoreImmediateOffset::from(0x713B);
        assert_eq!(format.load, false);
        assert_eq!(format.data_type, DataType::Byte);
        assert_eq!(format.offset_register, 4);
        assert_eq!(format.rb, 7);
        assert_eq!(format.rd, 3);
    }
    #[test]
    fn test_str() {
        let format = LoadStoreImmediateOffset::from(0x613B);
        assert_eq!(format.load, false);
        assert_eq!(format.data_type, DataType::Word);
        assert_eq!(format.offset_register, 4);
        assert_eq!(format.rb, 7);
        assert_eq!(format.rd, 3);
        let mut gba: GBA = GBA::default();
        gba.cpu.current_instruction_set = InstructionSet::Thumb;
        gba.cpu.set_register(format.rb,1);
        gba.cpu.set_register(format.rd,2);

        let decode_result = gba.cpu.decode(0x613B);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        let target_address: u32 = (gba.cpu.get_register(format.rb) + (format.offset_register << 2) as u32) as u32;
        assert_eq!(2, gba.memory_bus.mem_map.read_u32(target_address));
    }

        #[test]
    fn test_ldr() {
        let format = LoadStoreImmediateOffset::from(0x693B); //ldr
        assert_eq!(format.load, true);
        assert_eq!(format.data_type, DataType::Word);
        assert_eq!(format.offset_register, 4);
        assert_eq!(format.rb, 7);
        assert_eq!(format.rd, 3);
        let mut gba: GBA = GBA::default();
        gba.cpu.current_instruction_set = InstructionSet::Thumb;
        gba.cpu.set_register(format.rb,1);
        gba.cpu.set_register(format.rd,2);

        //let mem address = 3
        let decode_result = gba.cpu.decode(0x613B); //str
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }
        gba.cpu.set_register(3,0); //make sure its changing after ldr

        let decode_result = gba.cpu.decode(0x693B); //ldr
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        // target_address = 23.
        // Taken from 7(rb) + 4(offset) left shifted to 16 --> 23
        assert_eq!(2, gba.cpu.get_register(3));
    }
    #[test]
    fn test_strb() {
        let format = LoadStoreImmediateOffset::from(0x713B); //strb
        assert_eq!(format.load, false);
        assert_eq!(format.data_type, DataType::Byte);
        assert_eq!(format.offset_register, 4);
        assert_eq!(format.rb, 7);
        assert_eq!(format.rd, 3);
        let mut gba: GBA = GBA::default();
        gba.cpu.current_instruction_set = InstructionSet::Thumb;
        gba.cpu.set_register(format.rb,7);
        gba.cpu.set_register(format.rd,3);
        //let mem address = 3
        let decode_result = gba.cpu.decode(0x713B); //strb
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }
        // target_address = 23.
        // Taken from 7(rb) + 4(offset) left shifted to 16 --> 23
        assert_eq!(3, gba.cpu.get_register(3));
    }

    #[test]
    fn test_ldrb() {
        let format = LoadStoreImmediateOffset::from(0x793B);//ldrb
        assert_eq!(format.load, true);
        assert_eq!(format.data_type, DataType::Byte);
        assert_eq!(format.offset_register, 4);
        assert_eq!(format.rb, 7);
        assert_eq!(format.rd, 3);
        let mut gba: GBA = GBA::default();
        gba.cpu.current_instruction_set = InstructionSet::Thumb;
        //let mem address = 3
        gba.cpu.set_register(format.rb,1);
        gba.cpu.set_register(format.rd,2); //value we want to get
        let decode_result = gba.cpu.decode(0x713B); //strb
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }
        gba.cpu.set_register(3,0); //make sure its changing after ldr

        //set address at 3 = 3
        let decode_result = gba.cpu.decode(0x793B); //ldrb
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(2, gba.cpu.get_register(3));

    }
}
