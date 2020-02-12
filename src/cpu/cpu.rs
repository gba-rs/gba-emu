use crate::arm_formats::{data_processing::DataProcessing, software_interrupt::SoftwareInterrupt};
use crate::arm_formats::{halfword_register::HalfwordRegisterOffset, halfword_register::HalfwordImmediateOffset};
use crate::arm_formats::{multiply::Multiply, multiply_long::MultiplyLong};
use crate::arm_formats::{single_data_transfer::SingleDataTransfer};
use crate::arm_formats::{single_data_swap::SingleDataSwap};
use crate::arm_formats::{branch::Branch, branch_exchange::BranchExchange};
use crate::arm_formats::{block_data_transfer::BlockDataTransfer};
use crate::thumb_formats::{add_subtract::AddSubtract,alu::ALU,conditional_branch::ConditionalBranch};
use crate::thumb_formats::{hi_register_ops::HiRegisterOp, immediate_ops::ImmediateOp, load_address::LoadAddress, load_store_halfword::LoadStoreHalfword};
use crate::thumb_formats::{move_shifted_register::MoveShifted, load_store_immediate_offset::LoadStoreImmediateOffset, load_store_sign_extended::LoadStoreSignExtended};
use crate::thumb_formats::{long_branch_link::BL,multiple_load_store::MultipleLoadStore,pc_load::LDR,push_pop::PushPop, software_interrupt::ThumbSoftwareInterrupt};
use crate::thumb_formats::{sp_load_store::SpLoadStore,unconditional_branch::UnconditionalBranch, add_offset_sp::AddOffsetSP, load_store_register_offset::LoadStoreRegisterOffset};
use super::{program_status_register::ProgramStatusRegister};
use super::{arm_instr::ARM_INSTRUCTIONS};
use super::{thumb_instr::THUMB_INSTRUCTIONS};
use super::{decode_error::DecodeError};
use super::{condition::Condition};
use crate::operations::instruction::Instruction;
use std::borrow::{BorrowMut};
use log::{info};
use crate::gba::memory_bus::MemoryBus;


pub const ARM_PC: u8 = 15;
pub const ARM_LR: u8 = 14;
pub const ARM_SP: u8 = 13;
pub const THUMB_PC: u8 = 10;
pub const THUMB_SP: u8 = 8;
pub const THUMB_LR: u8 = 9;


pub const ARM_WORD_SIZE: u8 = 4;
pub const THUMB_WORD_SIZE: u8 = 2;

pub const REG_MAP: [[[usize; 16]; 7]; 2] = [
    // arm
    [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],     // System
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],     // User
        [0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18 , 19, 20, 21, 22, 15],  // FIQ
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 23, 24, 15],     // Supervisor
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 25, 26, 15],     // Abort
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 27, 28, 15],     // IRQ
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 29, 30, 15]      // Undefiend
    ],
    // thumb
    [
        [0, 1, 2, 3, 4, 5, 6, 7, 13, 14, 15, 11, 12, 13, 14, 15],        // System
        [0, 1, 2, 3, 4, 5, 6, 7, 13, 14, 15, 11, 12, 13, 14, 15],        // User
        [0, 1, 2, 3, 4, 5, 6, 7, 21, 22, 15, 11, 12, 13, 14, 15],        // FIQ
        [0, 1, 2, 3, 4, 5, 6, 7, 23, 24, 15, 11, 12, 13, 14, 15],        // Supervisor
        [0, 1, 2, 3, 4, 5, 6, 7, 25, 26, 15, 11, 12, 13, 14, 15],        // Abort
        [0, 1, 2, 3, 4, 5, 6, 7, 27, 28, 15, 11, 12, 13, 14, 15],        // IRQ
        [0, 1, 2, 3, 4, 5, 6, 7, 29, 30, 15, 11, 12, 13, 14, 15]         // Undefiend
    ]
];

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OperatingMode {
    System = 0,
    User = 1,
    FastInterrupt = 2,
    Supervisor = 3,
    Abort = 4,
    Interrupt = 5,
    Undefined = 6
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum InstructionSet {
    Arm,
    Thumb
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum InstructionFormat {
    DataProcessing,
    PsrTransfer,
    Multiply,
    MultiplyLong,
    SingleDataSwap,
    BranchAndExchange,
    HalfwordDataTransfer,
    SingleDataTransfer,
    Undefined,
    BlockDataTransfer,
    Branch,
    CoProcessorDataTransfer,
    CoProcessorDataOperation,
    CoProcessorRegisetTransfer,
    SoftwareInterrupt
}

#[derive(Debug)]
pub enum ThumbInstructionFormat {
    MoveShiftedRegister,
    AddSubtract,
    MoveCompare,
    ALU,
    HiRegister,
    LoadPC,
    LoadStoreOffset,
    LoadStoreExtended,
    LoadStoreImmediateOffset,
    LoadStoreHalfWord,
    LoadStoreSP,
    LoadAddress,
    GetAddress,
    ImmediateOp,
    AddOffsetSP,
    PushPopRegister,
    MultipleLoadStore,
    ConditionalBranch,
    UnConditonalBranch,
    LongBranchLink,
    BreakpointInterrupt,
    SoftwareInterrupt,
    Undefined
}

pub struct CPU {   
    registers: [u32; 31],
    spsr: [ProgramStatusRegister; 7],
    pub cpsr: ProgramStatusRegister,
    // pub wram: WorkRam,
    // pub onchip_wram: WorkRam,
    // pub bios_ram: BiosRam,
    pub operating_mode: OperatingMode,
    pub current_instruction_set: InstructionSet,
    pub last_instruction: String,
    pub cycle_count: usize,
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            registers: [0; 31],
            spsr: [ProgramStatusRegister::from(0); 7],
            cpsr: ProgramStatusRegister::from(0b11111),
            // wram: WorkRam::new(0x40000 + 1, 0),
            // onchip_wram: WorkRam::new(0x7FFF + 1, 0),
            // bios_ram: BiosRam::new(0),
            operating_mode: OperatingMode::System,
            current_instruction_set: InstructionSet::Arm,
            last_instruction: "".to_string(),
            cycle_count: 0
        };
    }

    pub fn decode(&self, instruction: u32) -> Result<Box<dyn Instruction>, DecodeError> {
        if self.current_instruction_set == InstructionSet::Arm {
           return self.decode_arm(instruction);
        } else{
            return self.decode_thumb(instruction);
        }
    }

    pub fn decode_arm(&self, instruction: u32)-> Result<Box<dyn Instruction>, DecodeError> {
        let opcode: u16 = (((instruction >> 16) & 0xFF0) | ((instruction >> 4) & 0x0F)) as u16;
        let instruction_format = ARM_INSTRUCTIONS[opcode as usize];
        match instruction_format {
            InstructionFormat::DataProcessing | InstructionFormat::PsrTransfer => {
                return Ok(Box::new(DataProcessing::from(instruction)));
            },
            InstructionFormat::Multiply => {
                return Ok(Box::new(Multiply::from(instruction)));
            },
            InstructionFormat::MultiplyLong => {
                return Ok(Box::new(MultiplyLong::from(instruction)));
            },
            InstructionFormat::SingleDataSwap => {
                return Ok(Box::new(SingleDataSwap::from(instruction)));
            },
            InstructionFormat::SingleDataTransfer => {
                return Ok(Box::new(SingleDataTransfer::from(instruction)));
            },
            InstructionFormat::BranchAndExchange => {
                return Ok(Box::new(BranchExchange::from(instruction)));
            },
            InstructionFormat::HalfwordDataTransfer => {
                if opcode & 0x40 == 0 {
                    return Ok(Box::new(HalfwordRegisterOffset::from(instruction)));
                } else {
                    return Ok(Box::new(HalfwordImmediateOffset::from(instruction)));
                }
            },
            InstructionFormat::BlockDataTransfer => {
                    return Ok(Box::new(BlockDataTransfer::from(instruction)));
            },
            InstructionFormat::Branch => {
                return Ok(Box::new(Branch::from(instruction)));
            },
            InstructionFormat::SoftwareInterrupt => {
                return Ok(Box::new(SoftwareInterrupt::from(instruction)));
            },
            _ => Err(DecodeError{
                instruction_set: self.current_instruction_set,
                instruction: instruction,
                opcode: opcode
            })
        }
    }

    pub fn decode_thumb(&self, instruction: u32)-> Result<Box<dyn Instruction>, DecodeError> {
        let thumb_instruction: u16 = instruction as u16;
        let opcode: u16 = (((thumb_instruction >> 8) & 0xF0) | ((thumb_instruction >> 8) & 0x0F)) as u16;
        let instruction_format = &THUMB_INSTRUCTIONS[opcode as usize];
        // println!("Format: {:?}, Opcode: {:X}, Instruction: {:X}", instruction_format, opcode, thumb_instruction);
        match instruction_format {
            ThumbInstructionFormat::MoveShiftedRegister => {
                return Ok(Box::new(MoveShifted::from(thumb_instruction)));
            },
            ThumbInstructionFormat::AddSubtract => {
                return Ok(Box::new(AddSubtract::from(thumb_instruction)));
            },
            ThumbInstructionFormat::ALU => {
                return Ok(Box::new(ALU::from(thumb_instruction)));
            },
            ThumbInstructionFormat::ConditionalBranch => {
                return Ok(Box::new(ConditionalBranch::from(thumb_instruction)));
            },
            ThumbInstructionFormat::HiRegister => {
                return Ok(Box::new(HiRegisterOp::from(thumb_instruction)));
            },
            ThumbInstructionFormat::ImmediateOp => {
                return Ok(Box::new(ImmediateOp::from(thumb_instruction))); 
            },
            ThumbInstructionFormat::LoadAddress => {
                return Ok(Box::new(LoadAddress::from(thumb_instruction))); 
            },
            ThumbInstructionFormat::LoadStoreHalfWord => {
                return Ok(Box::new(LoadStoreHalfword::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadStoreImmediateOffset => {
                return Ok(Box::new(LoadStoreImmediateOffset::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadStoreOffset => {
                return Ok(Box::new(LoadStoreRegisterOffset::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadStoreExtended => {
                return Ok(Box::new(LoadStoreSignExtended::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LongBranchLink => {
                return Ok(Box::new(BL::from(thumb_instruction)));
            },
            ThumbInstructionFormat::MultipleLoadStore => {
                return Ok(Box::new(MultipleLoadStore::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadPC => {
                return Ok(Box::new(LDR::from(thumb_instruction)));
            },
            ThumbInstructionFormat::PushPopRegister => {
                return Ok(Box::new(PushPop::from(thumb_instruction)));
            },
            ThumbInstructionFormat::BreakpointInterrupt => {
                return Ok(Box::new(ThumbSoftwareInterrupt::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadStoreSP => {
                return Ok(Box::new(SpLoadStore::from(thumb_instruction)));
            },
            ThumbInstructionFormat::AddOffsetSP => {
                return Ok(Box::new(AddOffsetSP::from(thumb_instruction)));
            }
            ThumbInstructionFormat::UnConditonalBranch => {
                return Ok(Box::new(UnconditionalBranch::from(thumb_instruction)));
            },
            ThumbInstructionFormat::SoftwareInterrupt => {
                return Ok(Box::new(ThumbSoftwareInterrupt::from(thumb_instruction)));
            }
            _ => Err(DecodeError{
                instruction_set: self.current_instruction_set,
                instruction: instruction,
                opcode: opcode
            })
        }
    }

    pub fn fetch(&mut self, bus: &mut MemoryBus) {
        let current_pc = if self.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        let pc_contents = self.get_register(current_pc);
        // log::debug!("PC: {:X}", pc_contents);

        let instruction: u32 = if self.current_instruction_set == InstructionSet::Arm { bus.read_u32(pc_contents) } else { bus.read_u16(pc_contents) as u32 };

        if self.current_instruction_set == InstructionSet::Arm { 
            self.set_register(current_pc, pc_contents + ARM_WORD_SIZE as u32) 
        } else { 
            self.set_register(current_pc, pc_contents + THUMB_WORD_SIZE as u32) 
        };

        let condition = if self.current_instruction_set == InstructionSet::Arm { Condition::from((instruction & 0xF000_0000) >> 28)} else {(Condition::from(0x0))};//THUMB codes don't include conditions 
        let check_condition = if self.current_instruction_set == InstructionSet::Arm { self.check_condition(&condition) } else { true };//fine

        let decode_result = self.decode(instruction);
        match decode_result {
            Ok(mut instr) => {
                // info!("Condition: {}, Instruction: {:?}", check_condition, instr.asm());

                if check_condition {
                    let temp_cycles = (instr.borrow_mut() as &mut dyn Instruction).execute(self, bus);
                    self.cycle_count += ((instr.borrow_mut() as &mut dyn Instruction).cycles() + temp_cycles) as usize;
                }
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    fn check_reg_range(reg_num: &u8, instr_set: &InstructionSet) {
        if *instr_set == InstructionSet::Thumb {
            if *reg_num > 10 {
                panic!("Attempting to get register out of range for Thumb: {}", reg_num);
            }
        } else {
            if *reg_num > 15 {
                panic!("Attempting to get register out of range for Arm: {}", reg_num);
            }
        }
    }

    pub fn get_register(&self, reg_num: u8) -> u32 {
        CPU::check_reg_range(&reg_num, &self.current_instruction_set);
        return self.registers[REG_MAP[self.current_instruction_set as usize][self.operating_mode as usize][reg_num as usize]];
    }

    pub fn set_register(&mut self, reg_num: u8, value: u32) {
        CPU::check_reg_range(&reg_num, &self.current_instruction_set);
        self.registers[REG_MAP[self.current_instruction_set as usize][self.operating_mode as usize][reg_num as usize]] = value;
    }

    pub fn get_register_override_opmode(&self, reg_num: u8, op_mode: OperatingMode) -> u32 {
        CPU::check_reg_range(&reg_num, &self.current_instruction_set);
        return self.registers[REG_MAP[self.current_instruction_set as usize][op_mode as usize][reg_num as usize]];
    }

    pub fn set_register_override_opmode(&mut self, reg_num: u8, op_mode: OperatingMode, value: u32) {
        CPU::check_reg_range(&reg_num, &self.current_instruction_set);
        self.registers[REG_MAP[self.current_instruction_set as usize][op_mode as usize][reg_num as usize]] = value;
    }
    
    pub fn get_register_unsafe(&self, reg_num: u8) -> u32{
        return self.registers[REG_MAP[InstructionSet::Arm as usize][self.operating_mode as usize][reg_num as usize]];
    }

    pub fn set_register_unsafe(&mut self, reg_num: u8, value: u32){
        self.registers[REG_MAP[InstructionSet::Arm as usize][self.operating_mode as usize][reg_num as usize]] = value;
    }

    pub fn check_condition(&self, cond: &Condition) -> bool {
        match cond {
            Condition::EQ => return self.cpsr.flags.zero,
            Condition::NE => return !self.cpsr.flags.zero,
            Condition::CS => return self.cpsr.flags.carry,
            Condition::CC => return !self.cpsr.flags.carry,
            Condition::MI => return self.cpsr.flags.negative,
            Condition::PL => return !self.cpsr.flags.negative,
            Condition::VS => return self.cpsr.flags.signed_overflow,
            Condition::VC => return !self.cpsr.flags.signed_overflow,
            Condition::HI => return self.cpsr.flags.carry && !self.cpsr.flags.zero,
            Condition::LS => return !self.cpsr.flags.carry && self.cpsr.flags.zero,
            Condition::GE => return self.cpsr.flags.negative == self.cpsr.flags.signed_overflow,
            Condition::LT => return self.cpsr.flags.negative != self.cpsr.flags.signed_overflow,
            Condition::GT => return !self.cpsr.flags.zero && (self.cpsr.flags.negative == self.cpsr.flags.signed_overflow),
            Condition::LE => return self.cpsr.flags.zero || (self.cpsr.flags.negative != self.cpsr.flags.signed_overflow),
            Condition::AL => return true,
            Condition::Error => panic!("Condition::Error hit"),
        }
    }

    pub fn get_spsr(&mut self) -> ProgramStatusRegister {
        if self.operating_mode == OperatingMode::User {
            panic!("Invalid operating mode {:?}", self.operating_mode);
        }
        return self.spsr[self.operating_mode as usize];
    }

    pub fn set_spsr(&mut self, psr: ProgramStatusRegister) {
        if self.operating_mode == OperatingMode::User {
            panic!("Invalid operating mode {:?}", self.operating_mode);
        }
        self.spsr[self.operating_mode as usize] = psr;
    }
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug};

    #[test]
    fn test_access_registers(){
        let cpu = CPU::new();
        let _empty_registers: [u32; 31] = [0; 31];
        
        assert_eq!(_empty_registers, cpu.registers);
    }

    #[test]
    fn test_decode_unimplemented(){
        let cpu = CPU::new();
//        let bus = MemoryBus::new();

        let result = cpu.decode(0x00F0F0F0);
        match result {
            Ok(instr) => {
                debug!("{:?}", instr.asm());
                assert!(false);
            },
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_decode(){
//        let mut bus = MemoryBus::new();
//        let cpu = CPU::new();
        // cpu.decode(&mut map, 0xE0812001);
    }

    #[test]
    fn test_fetch(){
        let mut cpu = CPU::new();
        cpu.set_register(15, 0x02000000);
        let mut bus = MemoryBus::new();
        bus.write_u32(0x02000000, 0x012081E0);
        bus.write_u32(0x02000004, 0x012081E0);
        cpu.fetch(&mut bus);
        cpu.fetch(&mut bus);
    }

    #[test]
    fn test_register_access() {
        let mut cpu = CPU::new();
        cpu.set_register(10, 15);
        let spv_reg_10 = cpu.get_register(10);
        cpu.operating_mode = OperatingMode::User;
        cpu.set_register(10, 200);
        let usr_reg_10 = cpu.get_register(10);

        assert_eq!(spv_reg_10, 15);
        assert_eq!(usr_reg_10, 200);
        assert!(spv_reg_10 != usr_reg_10);
    }

    #[test]
    #[should_panic]
    fn test_register_access_invalid() {
        let mut cpu = CPU::new();
        cpu.current_instruction_set = InstructionSet::Thumb;
        let _should_fail = cpu.get_register(11);
    }

    // #[test]
    // fn test_branch_exchange(){
    //     let mut cpu = CPU::new();
    //     cpu.set_register(15, 0x02000000);
    //     let mut map = MemoryBus::new();
    //     map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
    //     map.write_u32(0x02000000, 0x11FF2FE1u32.to_be());
    //     cpu.fetch(&mut map);
    //     assert_eq!(cpu.current_instruction_set, InstructionSet::Thumb);
    // }
}