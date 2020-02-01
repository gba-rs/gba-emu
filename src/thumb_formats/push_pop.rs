use crate::operations::instruction::Instruction;
use crate::memory::memory_map::MemoryMap;
use crate::operations::arm_arithmetic;
use crate::cpu::{cpu::CPU, cpu::THUMB_LR, cpu::THUMB_SP, cpu::THUMB_PC};
use std::fmt;

pub struct PushPop {
    pub load: bool,
    pub store_lr: bool,
    pub register_list: Vec<u8>,
}

impl From<u16> for PushPop {
    fn from(value: u16) -> PushPop {
        let mut temp_reg_list: Vec<u8> = vec![];
        for i in 0..8 {
            if ((value >> i) & 0x01) != 0{
                temp_reg_list.push(i as u8);
            }
        }

        return PushPop {
            load: ((value & 0x800) >> 11) != 0,
            store_lr: ((value & 0x100) >> 8) != 0,
            register_list: temp_reg_list,
        };
    }
}

impl Instruction for PushPop {
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let stack_pointer: u32 = cpu.get_register(THUMB_SP);
        let mut offset: i32 = 0;
        if self.load {          // LDMIA (Load Multiple Increment After) = POP

            for reg_num in self.register_list.iter() {
                let (address, _) = arm_arithmetic::add(stack_pointer, offset as u32);
                let value = mem_map.read_u32(address);
                cpu.set_register(*reg_num, value);
                offset += 4;
            }


            if self.store_lr {
                // thumb PC
                let (address, _) = arm_arithmetic::add(stack_pointer, offset as u32);
                let value = mem_map.read_u32(address);
                cpu.set_register(THUMB_PC, value);
                offset += 4;
            }

            // writeback
            let (new_sp, _) = arm_arithmetic::add(stack_pointer, offset as u32);
            cpu.set_register(THUMB_SP, new_sp)
        } else {// STMDB (Store Multiple Decrement Before) = PUSH

            for reg_num in self.register_list.iter() {
                offset -= 4;
                let value = cpu.get_register(*reg_num);
                let (offset_val, _) = arm_arithmetic::add(stack_pointer, offset as u32);
                mem_map.write_u32(offset_val, value);
            }


            if self.store_lr {  
                // thumb lr
                offset -= 4;
                let value = cpu.get_register(THUMB_LR);
                let (offset_val, _) = arm_arithmetic::sub(stack_pointer, offset as u32);
                mem_map.write_u32(offset_val, value);
            }

            // writeback
            let (new_sp, _) = arm_arithmetic::add(stack_pointer, offset as u32);
            cpu.set_register(THUMB_SP, new_sp)
        }
    }

    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 3;} //nS + 1N + 1I
}

impl fmt::Debug for PushPop {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        if self.load {
            write!(f, "POP {{")?;
        } else {
            write!(f, "PUSH {{")?;
        }

        for reg_num in self.register_list.iter() {
            write!(f, " r{} ", *reg_num)?;
        }

        if self.store_lr {
            if self.load {
                write!(f, " pc ")?;
            } else {
                write!(f, " lr ")?;
            }
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
    fn push_test() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        for i in 0..8 {
            gba.cpu.set_register(i, (i as u32) * 100);
        }

        let base = 0x02000020;
        gba.cpu.set_register(THUMB_SP, base);

        // Store, rb = sp, rlist = {1, 3, 5, 7}
        // STMDB sp!, {1, 3, 5, 7}
        let decode_result = gba.cpu.decode(0xB4AA);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(100, gba.memory_bus.mem_map.read_u32(base - 4));
        assert_eq!(300, gba.memory_bus.mem_map.read_u32(base - 8));
        assert_eq!(500, gba.memory_bus.mem_map.read_u32(base - 12));
        assert_eq!(700, gba.memory_bus.mem_map.read_u32(base - 16));
    }

    #[test]
    fn pop_test() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        let base = 0x02000000;

        for i in 0..8 {
           gba.memory_bus.mem_map.write_u32(0x02000000 + (i * 4), (100 * i) as u32);
        }

        gba.cpu.set_register(THUMB_SP, base);

        // Load, rb = sp, rlist = {1, 3, 5, 7}
        // LDMIA sp!, {1, 3, 5, 7}
        let decode_result = gba.cpu.decode(0xBCAA);
        match decode_result {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.memory_bus.mem_map);
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
