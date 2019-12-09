use crate::formats::{data_processing::DataProcessing, common::Instruction, common::Condition, software_interrupt::SoftwareInterrupt};
use crate::formats::{halfword_register::HalfwordRegisterOffset, halfword_register::HalfwordImmediateOffset};
use crate::formats::{multiply::Multiply, multiply_long::MultiplyLong};
use crate::formats::{single_data_transfer::SingleDataTransfer};
use crate::formats::{branch::Branch, branch_exchange::BranchExchange};
use crate::formats::{block_data_transfer::BlockDataTransfer};
use crate::formats::{debug::Debug};
use crate::memory::{work_ram::WorkRam, bios_ram::BiosRam, memory_map::MemoryMap};
use super::{program_status_register::ProgramStatusRegister};
use super::{arm_instr::ARM_INSTRUCTIONS};


pub const ARM_PC: u8 = 15;
pub const ARM_LR: u8 = 14;
pub const ARM_SP: u8 = 13;
pub const THUMB_PC: u8 = 10;
pub const THUMB_SP: u8 = 8;
pub const THUMB_LR: u8 = 9;

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
        [0, 1, 2, 3, 4, 5, 6, 7, 13, 14, 15, 0, 0, 0, 0, 0],        // System
        [0, 1, 2, 3, 4, 5, 6, 7, 13, 14, 15, 0, 0, 0, 0, 0],        // User
        [0, 1, 2, 3, 4, 5, 6, 7, 21, 22, 15, 0, 0, 0, 0, 0],        // FIQ
        [0, 1, 2, 3, 4, 5, 6, 7, 23, 24, 15, 0, 0, 0, 0, 0],        // Supervisor
        [0, 1, 2, 3, 4, 5, 6, 7, 25, 26, 15, 0, 0, 0, 0, 0],        // Abort
        [0, 1, 2, 3, 4, 5, 6, 7, 27, 28, 15, 0, 0, 0, 0, 0],        // IRQ
        [0, 1, 2, 3, 4, 5, 6, 7, 29, 30, 15, 0, 0, 0, 0, 0]         // Undefiend
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
    Undefiend,
    BlockDataTransfer,
    Branch,
    CoProcessorDataTransfer,
    CoProcessorDataOperation,
    CoProcessorRegisetTransfer,
    SoftwareInterrupt
}

pub struct CPU {   
    registers: [u32; 31],
    spsr: [ProgramStatusRegister; 7],
    pub cpsr: ProgramStatusRegister,
    pub wram: WorkRam,
    pub onchip_wram: WorkRam,
    pub bios_ram: BiosRam,
    pub operating_mode: OperatingMode,
    pub current_instruction_set: InstructionSet
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            registers: [0; 31],
            spsr: [ProgramStatusRegister::from(0); 7],
            cpsr: ProgramStatusRegister::from(0),
            wram: WorkRam::new(256000, 0),
            onchip_wram: WorkRam::new(0x7FFF, 0),
            bios_ram: BiosRam::new(0),
            operating_mode: OperatingMode::Supervisor,
            current_instruction_set: InstructionSet::Arm
        };
    }

    pub fn decode(&mut self, mem_map: &mut MemoryMap, instruction: u32) {
        let opcode: u16 = (((instruction >> 16) & 0xFF0) | ((instruction >> 4) & 0x0F)) as u16;
        let format = ARM_INSTRUCTIONS[opcode as usize];
        let condition = Condition::from((instruction & 0xF000_0000) >> 28);
        let check_condition = self.check_condition(&condition);
        println!("Decoding {:X} Cond {:?} = {:?}: {:X} = {:?}", instruction, condition, check_condition, opcode, format);       
        if check_condition {
            match format {
                InstructionFormat::DataProcessing | InstructionFormat::PsrTransfer => {
                    let mut format: DataProcessing = DataProcessing::from(instruction);
                    format.execute(self, mem_map);
                },
                InstructionFormat::Multiply => {
                    let mut format: Multiply = Multiply::from(instruction);
                    format.execute(self, mem_map);
                },
                InstructionFormat::MultiplyLong => {
                    let mut format: MultiplyLong = MultiplyLong::from(instruction);
                    format.execute(self, mem_map);
                },
                InstructionFormat::SingleDataSwap => {
                    panic!("Single data swap not implemented");
                },
                InstructionFormat::SingleDataTransfer => {
                    let mut format: SingleDataTransfer = SingleDataTransfer::from(instruction);
                    format.execute(self, mem_map);
                },
                InstructionFormat::BranchAndExchange => {
                    let mut format: BranchExchange = BranchExchange::from(instruction);
                    format.execute(self, mem_map);
                },
                InstructionFormat::HalfwordDataTransfer => {
                    if opcode & 0x40 == 0 {
                        let mut format = HalfwordRegisterOffset::from(instruction);
                        format.execute(self, mem_map);
                    } else {
                        let mut format = HalfwordImmediateOffset::from(instruction);
                        format.execute(self, mem_map);
                    }
                },
                InstructionFormat::Undefiend => {
                    let mut format = Debug::from(instruction);
                    format.execute(self, mem_map);
                    // panic!("Got an undefined format: {:X}",opcode);
                },
                InstructionFormat::BlockDataTransfer => {
                     let mut format: BlockDataTransfer = BlockDataTransfer::from(instruction);
                     format.execute(self, mem_map);
                },
                InstructionFormat::Branch => {
                    let mut format: Branch = Branch::from(instruction);
                    format.execute(self, mem_map);
                },
                InstructionFormat::SoftwareInterrupt => {
                    let mut format: SoftwareInterrupt = SoftwareInterrupt::from(instruction);
                    format.execute(self, mem_map);
                },
                _ => panic!("Got a bad format {:?} = {:X}", format, opcode)
            }
        }

    }


    pub fn fetch(&mut self, map: &mut MemoryMap) {
        let instruction: u32 = map.read_u32(self.registers[15]);
        let current_pc = if self.current_instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        let pc_contents = self.get_register(current_pc);
        self.set_register(current_pc, pc_contents + 4);
        self.decode(map, instruction.to_be());
    }

    pub fn get_register(&mut self, reg_num: u8) -> u32 {
        if self.current_instruction_set == InstructionSet::Thumb {
            if reg_num > 10 {
                panic!("Attempting to get register out of range for Thumb: {}", reg_num);
            }
        } else {
            if reg_num > 15 {
                panic!("Attempting to get register out of range for Arm: {}", reg_num);
            }
        }
        return self.registers[REG_MAP[self.current_instruction_set as usize][self.operating_mode as usize][reg_num as usize]];
    }

    pub fn set_register(&mut self, reg_num: u8, value: u32) {
        if self.current_instruction_set == InstructionSet::Thumb {
            if reg_num > 10 {
                panic!("Attempting to set register out of range for Thumb: {}", reg_num);
            }
        } else {
            if reg_num > 15 {
                panic!("Attempting to set register out of range for Arm: {}", reg_num);
            }
        }
        self.registers[REG_MAP[self.current_instruction_set as usize][self.operating_mode as usize][reg_num as usize]] = value;
    }

    pub fn get_register_override(&mut self, reg_num: u8, op_mode: OperatingMode) -> u32 {
        if self.current_instruction_set == InstructionSet::Thumb {
            if reg_num > 10 {
                panic!("Attempting to get register out of range for Thumb: {}", reg_num);
            }
        } else {
            if reg_num > 15 {
                panic!("Attempting to get register out of range for Arm: {}", reg_num);
            }
        }
        return self.registers[REG_MAP[self.current_instruction_set as usize][op_mode as usize][reg_num as usize]];
    }

    pub fn set_register_override(&mut self, reg_num: u8, op_mode: OperatingMode, value: u32) {
        if self.current_instruction_set == InstructionSet::Thumb {
            if reg_num > 10 {
                panic!("Attempting to set register out of range for Thumb: {}", reg_num);
            }
        } else {
            if reg_num > 15 {
                panic!("Attempting to set register out of range for Arm: {}", reg_num);
            }
        }
        self.registers[REG_MAP[self.current_instruction_set as usize][op_mode as usize][reg_num as usize]] = value;
    }

    pub fn check_condition(&mut self, cond: &Condition) -> bool {
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
            Condition::LE => return self.cpsr.flags.zero && (self.cpsr.flags.negative != self.cpsr.flags.signed_overflow),
            Condition::AL => return true,
            Condition::Error => panic!("Condition::Error hit"),
        }
    }

    pub fn get_spsr(&mut self) -> ProgramStatusRegister {
        if self.operating_mode == OperatingMode::System || self.operating_mode == OperatingMode::User {
            panic!("Invalid operating mode {:?}", self.operating_mode);
        }
        return self.spsr[self.operating_mode as usize];
    }

    pub fn set_spsr(&mut self, psr: ProgramStatusRegister) {
        if self.operating_mode == OperatingMode::System || self.operating_mode == OperatingMode::User {
            panic!("Invalid operating mode {:?}", self.operating_mode);
        }
        self.spsr[self.operating_mode as usize] = psr;
    }
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_registers(){
        let cpu = CPU::new();
        let _empty_registers: [u32; 31] = [0; 31];
        
        assert_eq!(_empty_registers, cpu.registers);
    }

    #[test]
    #[should_panic]
    fn test_decode_unimplemented(){
        let mut cpu = CPU::new();
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
        cpu.decode(&mut map, 0xE5000000);
    }

    #[test]
    fn test_decode(){
        let mut map = MemoryMap::new();
        let mut cpu = CPU::new();
        map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
        cpu.decode(&mut map, 0xE0812001);
    }

    #[test]
    fn test_fetch(){
        let mut cpu = CPU::new();
        cpu.set_register(15, 0x02000000);
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
        map.write_u32(0x02000000, 0x012081E0);
        map.write_u32(0x02000004, 0x012081E0);
        cpu.fetch(&mut map);
        cpu.fetch(&mut map);
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

    #[test]
    fn test_branch_exchange(){
        let mut cpu = CPU::new();
        cpu.set_register(15, 0x02000000);
        let mut map = MemoryMap::new();
        map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
        map.write_u32(0x02000000, 0x11FF2FE1);
        cpu.fetch(&mut map);
        assert_eq!(cpu.current_instruction_set, InstructionSet::Thumb);
    }
}