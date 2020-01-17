use crate::operations::instruction::Instruction;
use crate::memory::memory_map::MemoryMap;
use crate::cpu::{cpu::CPU};
use std::fmt;


pub struct MultipleLoadStore {
    pub opcode: u8,
    pub rb: u8,
    pub register_list: Vec<u8>,
    pub load: bool
}

impl From<u16> for MultipleLoadStore {
    fn from(value: u16) -> MultipleLoadStore {
        let mut temp_reg_list: Vec<u8> = vec![];
        for i in 0..8 {
            if ((value >> i) & 0x01) != 0{
                temp_reg_list.push(i as u8);
            }
        }
        return MultipleLoadStore {
            register_list: temp_reg_list,
            rb: ((value >> 8) & 0x7) as u8,
            opcode: ((value >> 11) & 0x1) as u8,
            load: ((value & 0x800) >> 11) != 0
        };
    }
}

impl Instruction for MultipleLoadStore {
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let base = cpu.get_register(self.rb);
        let mut offset = 0;
        if self.load {
            for reg_num in self.register_list.iter() {
                let value = mem_map.read_u32(base + offset);
                cpu.set_register(*reg_num, value);
                offset += 4;
            }
            cpu.set_register(self.rb, base + offset);
        } else {
            for reg_num in self.register_list.iter() {
                let value = cpu.get_register(*reg_num);
                mem_map.write_u32(base + offset, value);
                offset += 4;
            }
            cpu.set_register(self.rb, base + offset);
        }
    }

    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
}

impl fmt::Debug for MultipleLoadStore {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        if self.load {
            write!(f, "LDMIA r{}!, {{", self.rb)?;
        } else {
            write!(f, "STMIA r{}!, {{", self.rb)?;
        }

        for reg_num in self.register_list.iter() {
            write!(f, " r{} ", *reg_num)?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;
    use crate::cpu::{cpu::InstructionSet, cpu::THUMB_PC};
    use std::borrow::{BorrowMut};

    #[test]
    fn stmia_test() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        for i in 0..8 {
            gba.cpu.set_register(i, (i as u32) * 100);
        }

        let base = 0x02000000;
        gba.cpu.set_register(2, base);

        // Store, rb = 2, rlist = {1, 3, 5, 7}
        // STMIA r2!, {1, 3, 5, 7}
        let decode_result = gba.cpu.decode(0xC2AA);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(100, gba.mem_map.read_u32(base));
        assert_eq!(300, gba.mem_map.read_u32(base + 4));
        assert_eq!(500, gba.mem_map.read_u32(base + 8));
        assert_eq!(700, gba.mem_map.read_u32(base + 12));
    }

    #[test]
    fn ldmia_test() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        let base = 0x02000000;

        for i in 0..8 {
           gba.mem_map.write_u32(0x02000000 + (i * 4), (100 * i) as u32);
        }

        gba.cpu.set_register(2, base);

        // Load, rb = 2, rlist = {1, 3, 5, 7}
        // LDMIA r2!, {1, 3, 5, 7}
        let decode_result = gba.cpu.decode(0xCAAA);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(0, gba.cpu.get_register(1));
        assert_eq!(100, gba.cpu.get_register(3));
        assert_eq!(200, gba.cpu.get_register(5));
        assert_eq!(300, gba.cpu.get_register(7));
    }


}
