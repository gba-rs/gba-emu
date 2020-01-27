use crate::operations::instruction::Instruction;
use crate::memory::memory_map::MemoryMap;
use crate::operations::{arm_arithmetic};
use crate::cpu::{cpu::CPU, cpu::InstructionSet, cpu::ARM_PC, cpu::THUMB_PC};
use std::fmt;
use log::{error};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OpCodes {
    ADD = 0,
    CMP = 1,
    MOV  = 2,
    BX = 3,
}

impl From<u8> for OpCodes {
    fn from(value: u8) -> OpCodes {
        match value {
            0b00 => return OpCodes::ADD,
            0b01 => return OpCodes::CMP,
            0b10 => return OpCodes::MOV,
            0b11 => return OpCodes::BX,
            _=> panic!("Error in Hi Register Ops/Branch Exchange Processing Opcode")
        }
    }
}

pub struct HiRegisterOp {
    pub op: OpCodes,
    pub hi_flag_1: bool,
    pub hi_flag_2: bool,
    pub source_register: u8,
    pub destination_register: u8,
}

impl From<u16> for HiRegisterOp {
    fn from(value: u16) -> HiRegisterOp {
        return HiRegisterOp {
            op: OpCodes::from(((value & 0x300) >> 8) as u8),
            hi_flag_1: ((value & 0x80) >> 7) != 0,
            hi_flag_2: ((value & 0x40) >> 6) != 0,
            source_register: ((value & 0x38) >> 3) as u8,
            destination_register: (value & 0x7) as u8,
        };
    }
}

impl Instruction for HiRegisterOp {
    fn execute(&self, cpu: &mut CPU, _mem_map: &mut MemoryMap) {
        match self.op {
            OpCodes::ADD => {
                self.add(cpu);
            },
            OpCodes::CMP => {
                self.cmp(cpu);
            },
            OpCodes::MOV => {
                self.mov(cpu);
            },
            OpCodes::BX => {
                self.bx(cpu);
            }
        }
    }

    fn asm(&self) -> String{
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {
        match self.op {
            OpCodes::BX => {
                return 3; //2s + 1n
            }
            _ => {
                return 1; //1s
            }
        }
    } // 1s or 2s + 1n

}

impl HiRegisterOp {

    /// Gets the destination and source register values
    /// returns `(destination, source)`
    fn get_register_vals(&self, cpu: &CPU) -> (u32, u32) {
        let mut destination: u32;
        if self.hi_flag_1 {
            // r8-r15
            destination = cpu.get_register_unsafe(15 - self.destination_register);
            if self.destination_register == 0 { // R15 special case
                destination = destination + 2;  // Fetch adds the other +2
            }
        } else {
            // r0-r7
            destination = cpu.get_register(self.destination_register);
        }

        let mut source: u32;
        if self.hi_flag_2 {
            // r8-r15
            source = cpu.get_register_unsafe(15 - self.source_register);
            if self.source_register == 0 {  // R15 Special case
                source = source + 2;        // Fetch adds the other +2
            }
        } else {
            // r0-r7
            source = cpu.get_register(self.source_register);
        }

        return (destination, source);
    }

    // Sets the destination register value based on the hi flags
    fn set_destniation_register(&self, cpu: &mut CPU, value: u32) {
        if self.hi_flag_1 {
            cpu.set_register_unsafe(15 - self.destination_register, value);
        } else {
            cpu.set_register(self.destination_register, value);
        }
    }

    fn add(&self, cpu: &mut CPU) {
        let (destination, source) = self.get_register_vals(cpu);
        let (val, _) = arm_arithmetic::add(destination, source);
        self.set_destniation_register(cpu, val);
    }

    fn cmp(&self, cpu: &mut CPU) {
        let (destination, source) = self.get_register_vals(cpu);
        let flags = arm_arithmetic::cmp(destination, source);
        // TODO: make sure that cmp sets all the flags
        cpu.cpsr.flags = flags;
    }

    fn mov(&self, cpu: &mut CPU) {
        let (_, source) = self.get_register_vals(cpu);
        self.set_destniation_register(cpu, source);
    }

    fn bx(&self, cpu: &mut CPU) {
        if self.hi_flag_1 {
            error!("Invalid hi flag combination");
            panic!("Invalid hi flag combination");
        }

        let (_, source) = self.get_register_vals(cpu);
        let mode_bit = (source & 0x1) != 0;
        if mode_bit {
            // Thumb
            cpu.current_instruction_set = InstructionSet::Thumb;
            error!("Setting to thumb");
            cpu.set_register(THUMB_PC, source - 1);
        } else {
            // Arm
            cpu.current_instruction_set = InstructionSet::Arm;
            error!("Setting to arm");
            cpu.set_register(ARM_PC, source);
        }
    }
}

impl fmt::Debug for HiRegisterOp {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        let dest_reg = if self.hi_flag_1 { 15 - self.destination_register } else { self.destination_register };
        let source_reg = if self.hi_flag_2 { 15 - self.source_register } else { self.source_register };
        if self.op == OpCodes::BX {
            write!(f, "BX r{}", source_reg)
        } else {
            write!(f, "{:?} r{}, r{}", self.op, dest_reg, source_reg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::GBA;
    use crate::cpu::{cpu::InstructionSet, cpu::ARM_PC, cpu::THUMB_PC};
    use std::borrow::{BorrowMut};

    #[test]
    fn creation_0s_test() {
        let format = HiRegisterOp::from(0x4400);

        assert_eq!(format.op, OpCodes::ADD);
        assert_eq!(format.hi_flag_1, false);
        assert_eq!(format.hi_flag_2, false);
        assert_eq!(format.source_register, 0);
        assert_eq!(format.destination_register, 0);
    }

    #[test]
    fn creation_test() {
        let format = HiRegisterOp::from(0x4754);

        assert_eq!(format.op, OpCodes::BX);
        assert_eq!(format.hi_flag_1, false);
        assert_eq!(format.hi_flag_2, true);
        assert_eq!(format.source_register, 2);
        assert_eq!(format.destination_register, 4);
    }

    #[test]
    fn creation_2_test() {
        let format = HiRegisterOp::from(0x46B5);

        assert_eq!(format.op, OpCodes::MOV);
        assert_eq!(format.hi_flag_1, true);
        assert_eq!(format.hi_flag_2, false);
        assert_eq!(format.source_register, 6);
        assert_eq!(format.destination_register, 5);
    }

    #[test]
    fn add_test() {
        let mut gba: GBA = GBA::default(); 
        // hs = r12 = 200
        gba.cpu.set_register(12, 200);
        
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        // rd = r3 = 10
        gba.cpu.set_register(3, 10);

        // ADD r3, r12 = 210
        // 0x445B
        match gba.cpu.decode(0x445B) {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(210, gba.cpu.get_register(3));
    }

    #[test]
    fn cmp_test() {
        let mut gba: GBA = GBA::default(); 

        // hd = r12 = 10
        gba.cpu.set_register(12, 10);
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        // rs = r3 = 10
        gba.cpu.set_register(3, 10);

        gba.cpu.cpsr.flags.zero = false;

        // cmp r3, r12 = Zero flag == true
        // 0x459B
        match gba.cpu.decode(0x459B) {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        // TODO maybe add some asserts for the other flags
        assert!(gba.cpu.cpsr.flags.zero);
    }

    #[test]
    fn mov_test() {
        let mut gba: GBA = GBA::default(); 

        // hd = r11 = 10
        gba.cpu.set_register(11, 10);
        // hs = r12 = 200
        gba.cpu.set_register(12, 200);

        gba.cpu.current_instruction_set = InstructionSet::Thumb;



        gba.cpu.cpsr.flags.zero = false;

        // mov r11, r12 -> r11 = r12
        // 0x46DC
        match gba.cpu.decode(0x46DC) {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(200, gba.cpu.get_register_unsafe(11));
    }

    #[test]
    fn bx_arm_test() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        gba.cpu.set_register(3, 200);

        match gba.cpu.decode(0x4718) {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(InstructionSet::Arm, gba.cpu.current_instruction_set);
        assert_eq!(200, gba.cpu.get_register(ARM_PC));
    }

    #[test]
    fn bx_thumb_test() {
        let mut gba: GBA = GBA::default(); 
        gba.cpu.current_instruction_set = InstructionSet::Thumb;

        gba.cpu.set_register(3, 201);

        match gba.cpu.decode(0x4718) {
            Ok(mut instr) => {
                (instr.borrow_mut() as &mut dyn Instruction).execute(&mut gba.cpu, &mut gba.mem_map);
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        assert_eq!(InstructionSet::Thumb, gba.cpu.current_instruction_set);
        assert_eq!(200, gba.cpu.get_register(THUMB_PC));
    }
}
