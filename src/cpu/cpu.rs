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
use crate::memory::memory_bus::MemoryBus;
use serde::{Serialize, Deserialize};


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
        [0, 1, 2, 3, 4, 5, 6, 7, 13, 14, 15, 40, 40, 40, 40, 40],        // System
        [0, 1, 2, 3, 4, 5, 6, 7, 13, 14, 15, 40, 40, 40, 40, 40],        // User
        [0, 1, 2, 3, 4, 5, 6, 7, 21, 22, 15, 40, 40, 40, 40, 40],        // FIQ
        [0, 1, 2, 3, 4, 5, 6, 7, 23, 24, 15, 40, 40, 40, 40, 40],        // Supervisor
        [0, 1, 2, 3, 4, 5, 6, 7, 25, 26, 15, 40, 40, 40, 40, 40],        // Abort
        [0, 1, 2, 3, 4, 5, 6, 7, 27, 28, 15, 40, 40, 40, 40, 40],        // IRQ
        [0, 1, 2, 3, 4, 5, 6, 7, 29, 30, 15, 40, 40, 40, 40, 40]         // Undefiend
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

pub enum DecodedInstruction {
    DataProcessing(DataProcessing),
    Multiply(Multiply),
    MultiplyLong(MultiplyLong),
    SingleDataSwap(SingleDataSwap),
    SingleDataTransfer(SingleDataTransfer),
    BranchExchange(BranchExchange),
    HalfwordRegisterOffset(HalfwordRegisterOffset),
    HalfwordImmediateOffset(HalfwordImmediateOffset),
    BlockDataTransfer(BlockDataTransfer),
    Branch(Branch),
    SoftwareInterrupt(SoftwareInterrupt),
    MoveShifted(MoveShifted),
    AddSubtract(AddSubtract),
    ALU(ALU),
    ConditionalBranch(ConditionalBranch),
    HiRegisterOp(HiRegisterOp),
    ImmediateOp(ImmediateOp),
    LoadAddress(LoadAddress),
    LoadStoreHalfword(LoadStoreHalfword),
    LoadStoreImmediateOffset(LoadStoreImmediateOffset),
    LoadStoreRegisterOffset(LoadStoreRegisterOffset),
    LoadStoreSignExtended(LoadStoreSignExtended),
    BL(BL),
    MultipleLoadStore(MultipleLoadStore),
    LDR(LDR),
    PushPop(PushPop),
    ThumbSoftwareInterrupt(ThumbSoftwareInterrupt),
    SpLoadStore(SpLoadStore),
    AddOffsetSP(AddOffsetSP),
    UnconditionalBranch(UnconditionalBranch),
}

impl DecodedInstruction {
    pub fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) -> u32 {
        match self {
            DecodedInstruction::DataProcessing(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::Multiply(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::MultiplyLong(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::SingleDataSwap(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::SingleDataTransfer(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::BranchExchange(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::HalfwordRegisterOffset(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::HalfwordImmediateOffset(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::BlockDataTransfer(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::Branch(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::SoftwareInterrupt(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::MoveShifted(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::AddSubtract(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::ALU(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::ConditionalBranch(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::HiRegisterOp(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::ImmediateOp(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::LoadAddress(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::LoadStoreHalfword(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::LoadStoreImmediateOffset(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::LoadStoreRegisterOffset(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::LoadStoreSignExtended(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::BL(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::MultipleLoadStore(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::LDR(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::PushPop(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::ThumbSoftwareInterrupt(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::SpLoadStore(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::AddOffsetSP(i) => i.execute(cpu, mem_bus),
            DecodedInstruction::UnconditionalBranch(i) => i.execute(cpu, mem_bus),
        }
    }

    pub fn cycles(&self) -> u32 {
        match self {
            DecodedInstruction::DataProcessing(i) => i.cycles(),
            DecodedInstruction::Multiply(i) => i.cycles(),
            DecodedInstruction::MultiplyLong(i) => i.cycles(),
            DecodedInstruction::SingleDataSwap(i) => i.cycles(),
            DecodedInstruction::SingleDataTransfer(i) => i.cycles(),
            DecodedInstruction::BranchExchange(i) => i.cycles(),
            DecodedInstruction::HalfwordRegisterOffset(i) => i.cycles(),
            DecodedInstruction::HalfwordImmediateOffset(i) => i.cycles(),
            DecodedInstruction::BlockDataTransfer(i) => i.cycles(),
            DecodedInstruction::Branch(i) => i.cycles(),
            DecodedInstruction::SoftwareInterrupt(i) => i.cycles(),
            DecodedInstruction::MoveShifted(i) => i.cycles(),
            DecodedInstruction::AddSubtract(i) => i.cycles(),
            DecodedInstruction::ALU(i) => i.cycles(),
            DecodedInstruction::ConditionalBranch(i) => i.cycles(),
            DecodedInstruction::HiRegisterOp(i) => i.cycles(),
            DecodedInstruction::ImmediateOp(i) => i.cycles(),
            DecodedInstruction::LoadAddress(i) => i.cycles(),
            DecodedInstruction::LoadStoreHalfword(i) => i.cycles(),
            DecodedInstruction::LoadStoreImmediateOffset(i) => i.cycles(),
            DecodedInstruction::LoadStoreRegisterOffset(i) => i.cycles(),
            DecodedInstruction::LoadStoreSignExtended(i) => i.cycles(),
            DecodedInstruction::BL(i) => i.cycles(),
            DecodedInstruction::MultipleLoadStore(i) => i.cycles(),
            DecodedInstruction::LDR(i) => i.cycles(),
            DecodedInstruction::PushPop(i) => i.cycles(),
            DecodedInstruction::ThumbSoftwareInterrupt(i) => i.cycles(),
            DecodedInstruction::SpLoadStore(i) => i.cycles(),
            DecodedInstruction::AddOffsetSP(i) => i.cycles(),
            DecodedInstruction::UnconditionalBranch(i) => i.cycles(),
        }
    }

    pub fn asm(&self) -> String {
        match self {
            DecodedInstruction::DataProcessing(i) => i.asm(),
            DecodedInstruction::Multiply(i) => i.asm(),
            DecodedInstruction::MultiplyLong(i) => i.asm(),
            DecodedInstruction::SingleDataSwap(i) => i.asm(),
            DecodedInstruction::SingleDataTransfer(i) => i.asm(),
            DecodedInstruction::BranchExchange(i) => i.asm(),
            DecodedInstruction::HalfwordRegisterOffset(i) => i.asm(),
            DecodedInstruction::HalfwordImmediateOffset(i) => i.asm(),
            DecodedInstruction::BlockDataTransfer(i) => i.asm(),
            DecodedInstruction::Branch(i) => i.asm(),
            DecodedInstruction::SoftwareInterrupt(i) => i.asm(),
            DecodedInstruction::MoveShifted(i) => i.asm(),
            DecodedInstruction::AddSubtract(i) => i.asm(),
            DecodedInstruction::ALU(i) => i.asm(),
            DecodedInstruction::ConditionalBranch(i) => i.asm(),
            DecodedInstruction::HiRegisterOp(i) => i.asm(),
            DecodedInstruction::ImmediateOp(i) => i.asm(),
            DecodedInstruction::LoadAddress(i) => i.asm(),
            DecodedInstruction::LoadStoreHalfword(i) => i.asm(),
            DecodedInstruction::LoadStoreImmediateOffset(i) => i.asm(),
            DecodedInstruction::LoadStoreRegisterOffset(i) => i.asm(),
            DecodedInstruction::LoadStoreSignExtended(i) => i.asm(),
            DecodedInstruction::BL(i) => i.asm(),
            DecodedInstruction::MultipleLoadStore(i) => i.asm(),
            DecodedInstruction::LDR(i) => i.asm(),
            DecodedInstruction::PushPop(i) => i.asm(),
            DecodedInstruction::ThumbSoftwareInterrupt(i) => i.asm(),
            DecodedInstruction::SpLoadStore(i) => i.asm(),
            DecodedInstruction::AddOffsetSP(i) => i.asm(),
            DecodedInstruction::UnconditionalBranch(i) => i.asm(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CPU {
    registers: [u32; 31],
    spsr: [ProgramStatusRegister; 7],
    pub cpsr: ProgramStatusRegister,
    pub last_instruction: String,
    // One-instruction-deep prefetch: the next instruction's word, already fetched before the
    // current instruction executes. Real ARM7TDMI hardware works this way, and some games
    // (e.g. GBA's "Classic NES Series") deliberately self-modify the next instruction and check
    // whether the stale (already-fetched) or fresh byte executes, as an anti-emulation probe.
    #[serde(skip)]
    prefetch_cache: u32,
    #[serde(skip)]
    prefetch_primed: bool,
}

impl CPU {
    pub fn new() -> CPU {
        return CPU {
            registers: [0; 31],
            spsr: [ProgramStatusRegister::from(0); 7],
            cpsr: ProgramStatusRegister::from(0b011111),
            last_instruction: "".to_string(),
            prefetch_cache: 0,
            prefetch_primed: false,
        };
    }

    pub fn flush_prefetch(&mut self) {
        self.prefetch_primed = false;
    }

    pub fn decode(&self, instruction: u32) -> Result<DecodedInstruction, DecodeError> {
        if self.get_instruction_set() == InstructionSet::Arm {
           return self.decode_arm(instruction);
        } else{
            return self.decode_thumb(instruction);
        }
    }

    pub fn decode_arm(&self, instruction: u32)-> Result<DecodedInstruction, DecodeError> {
        let opcode: u16 = (((instruction >> 16) & 0xFF0) | ((instruction >> 4) & 0x0F)) as u16;
        let instruction_format = ARM_INSTRUCTIONS[opcode as usize];
        match instruction_format {
            InstructionFormat::DataProcessing | InstructionFormat::PsrTransfer => {
                return Ok(DecodedInstruction::DataProcessing(DataProcessing::from(instruction)));
            },
            InstructionFormat::Multiply => {
                return Ok(DecodedInstruction::Multiply(Multiply::from(instruction)));
            },
            InstructionFormat::MultiplyLong => {
                return Ok(DecodedInstruction::MultiplyLong(MultiplyLong::from(instruction)));
            },
            InstructionFormat::SingleDataSwap => {
                return Ok(DecodedInstruction::SingleDataSwap(SingleDataSwap::from(instruction)));
            },
            InstructionFormat::SingleDataTransfer => {
                return Ok(DecodedInstruction::SingleDataTransfer(SingleDataTransfer::from(instruction)));
            },
            InstructionFormat::BranchAndExchange => {
                return Ok(DecodedInstruction::BranchExchange(BranchExchange::from(instruction)));
            },
            InstructionFormat::HalfwordDataTransfer => {
                if opcode & 0x40 == 0 {
                    return Ok(DecodedInstruction::HalfwordRegisterOffset(HalfwordRegisterOffset::from(instruction)));
                } else {
                    return Ok(DecodedInstruction::HalfwordImmediateOffset(HalfwordImmediateOffset::from(instruction)));
                }
            },
            InstructionFormat::BlockDataTransfer => {
                    return Ok(DecodedInstruction::BlockDataTransfer(BlockDataTransfer::from(instruction)));
            },
            InstructionFormat::Branch => {
                return Ok(DecodedInstruction::Branch(Branch::from(instruction)));
            },
            InstructionFormat::SoftwareInterrupt => {
                return Ok(DecodedInstruction::SoftwareInterrupt(SoftwareInterrupt::from(instruction)));
            },
            _ => Err(DecodeError{
                instruction_set: self.get_instruction_set(),
                instruction: instruction,
                opcode: opcode
            })
        }
    }

    pub fn decode_thumb(&self, instruction: u32)-> Result<DecodedInstruction, DecodeError> {
        let thumb_instruction: u16 = instruction as u16;
        let opcode: u16 = (((thumb_instruction >> 8) & 0xF0) | ((thumb_instruction >> 8) & 0x0F)) as u16;
        let instruction_format = &THUMB_INSTRUCTIONS[opcode as usize];
        // println!("Format: {:?}, Opcode: {:X}, Instruction: {:X}", instruction_format, opcode, thumb_instruction);
        match instruction_format {
            ThumbInstructionFormat::MoveShiftedRegister => {
                return Ok(DecodedInstruction::MoveShifted(MoveShifted::from(thumb_instruction)));
            },
            ThumbInstructionFormat::AddSubtract => {
                return Ok(DecodedInstruction::AddSubtract(AddSubtract::from(thumb_instruction)));
            },
            ThumbInstructionFormat::ALU => {
                return Ok(DecodedInstruction::ALU(ALU::from(thumb_instruction)));
            },
            ThumbInstructionFormat::ConditionalBranch => {
                return Ok(DecodedInstruction::ConditionalBranch(ConditionalBranch::from(thumb_instruction)));
            },
            ThumbInstructionFormat::HiRegister => {
                return Ok(DecodedInstruction::HiRegisterOp(HiRegisterOp::from(thumb_instruction)));
            },
            ThumbInstructionFormat::ImmediateOp => {
                return Ok(DecodedInstruction::ImmediateOp(ImmediateOp::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadAddress => {
                return Ok(DecodedInstruction::LoadAddress(LoadAddress::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadStoreHalfWord => {
                return Ok(DecodedInstruction::LoadStoreHalfword(LoadStoreHalfword::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadStoreImmediateOffset => {
                return Ok(DecodedInstruction::LoadStoreImmediateOffset(LoadStoreImmediateOffset::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadStoreOffset => {
                return Ok(DecodedInstruction::LoadStoreRegisterOffset(LoadStoreRegisterOffset::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadStoreExtended => {
                return Ok(DecodedInstruction::LoadStoreSignExtended(LoadStoreSignExtended::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LongBranchLink => {
                return Ok(DecodedInstruction::BL(BL::from(thumb_instruction)));
            },
            ThumbInstructionFormat::MultipleLoadStore => {
                return Ok(DecodedInstruction::MultipleLoadStore(MultipleLoadStore::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadPC => {
                return Ok(DecodedInstruction::LDR(LDR::from(thumb_instruction)));
            },
            ThumbInstructionFormat::PushPopRegister => {
                return Ok(DecodedInstruction::PushPop(PushPop::from(thumb_instruction)));
            },
            ThumbInstructionFormat::BreakpointInterrupt => {
                return Ok(DecodedInstruction::ThumbSoftwareInterrupt(ThumbSoftwareInterrupt::from(thumb_instruction)));
            },
            ThumbInstructionFormat::LoadStoreSP => {
                return Ok(DecodedInstruction::SpLoadStore(SpLoadStore::from(thumb_instruction)));
            },
            ThumbInstructionFormat::AddOffsetSP => {
                return Ok(DecodedInstruction::AddOffsetSP(AddOffsetSP::from(thumb_instruction)));
            }
            ThumbInstructionFormat::UnConditonalBranch => {
                return Ok(DecodedInstruction::UnconditionalBranch(UnconditionalBranch::from(thumb_instruction)));
            },
            ThumbInstructionFormat::SoftwareInterrupt => {
                return Ok(DecodedInstruction::ThumbSoftwareInterrupt(ThumbSoftwareInterrupt::from(thumb_instruction)));
            }
            _ => Err(DecodeError{
                instruction_set: self.get_instruction_set(),
                instruction: instruction,
                opcode: opcode
            })
        }
    }

    pub fn get_pc(&self) -> u32 {
        let current_pc = if self.get_instruction_set() == InstructionSet::Arm { ARM_PC } else { THUMB_PC };
        let pc_contents = self.get_register(current_pc);
        return pc_contents;
    }

    pub fn fetch(&mut self, bus: &mut MemoryBus) -> usize {
        let is_arm = self.get_instruction_set() == InstructionSet::Arm;
        let current_pc = if is_arm { ARM_PC } else { THUMB_PC };
        let word_size = if is_arm { ARM_WORD_SIZE } else { THUMB_WORD_SIZE } as u32;
        let pc_contents = self.get_register(current_pc);
        let pc_after_advance = pc_contents + word_size;
        // log::debug!("PC: {:X}", pc_contents);
        crate::memory::memory_map::CURRENT_INSTR_PC.with(|pc| pc.set(pc_contents));
        crate::memory::memory_map::CURRENT_INSTR_IS_THUMB.with(|t| t.set(self.get_instruction_set() != InstructionSet::Arm));

        let read_word = |addr: u32, bus: &mut MemoryBus| -> u32 {
            if is_arm { bus.read_u32(addr) } else { bus.read_u16(addr) as u32 }
        };

        let instruction: u32 = if self.prefetch_primed { self.prefetch_cache } else { read_word(pc_contents, bus) };

        // Fetch the next instruction's word now, before this instruction executes -- matching
        // real hardware's pipeline, where the next instruction is already fetched by the time
        // the current one runs. This must happen before execute() so a self-modifying write to
        // the next instruction's address (the anti-emulation probe some games use) doesn't
        // affect what we cache here.
        let lookahead = read_word(pc_after_advance, bus);

        self.set_register(current_pc, pc_after_advance);

        let condition = if self.get_instruction_set() == InstructionSet::Arm { Condition::from((instruction & 0xF000_0000) >> 28)} else {Condition::from(0x0)};//THUMB codes don't include conditions 
        let check_condition = if self.get_instruction_set() == InstructionSet::Arm { self.check_condition(&condition) } else { true };//fine

        let decode_result = self.decode(instruction);
        let cycles: usize = match decode_result {
            Ok(instr) => {
                // info!("{:?}, {:?}, {:X}, {:X}, {:?}", self.get_operating_mode(), self.get_instruction_set(), pc_contents, instruction, instr.asm());
                // info!("r0={:X} r1={:X} r2={:X} r3={:X} r4={:X} r5={:X} r6={:X} r7={:X} r8={:X} r9={:X} r10={:X} r11={:X} r12={:X} r13={:X} r14={:X} r15={:X}", 
                //         self.get_register_unsafe(0), 
                //         self.get_register_unsafe(1), 
                //         self.get_register_unsafe(2), 
                //         self.get_register_unsafe(3), 
                //         self.get_register_unsafe(4), 
                //         self.get_register_unsafe(5), 
                //         self.get_register_unsafe(6), 
                //         self.get_register_unsafe(7), 
                //         self.get_register_unsafe(8), 
                //         self.get_register_unsafe(9), 
                //         self.get_register_unsafe(10), 
                //         self.get_register_unsafe(11), 
                //         self.get_register_unsafe(12), 
                //         self.get_register_unsafe(13), 
                //         self.get_register_unsafe(14), 
                //         self.get_register_unsafe(15));


                if check_condition {
                    let temp_cycles = instr.execute(self, bus);
                    let unclaimed_cycles = bus.cycle_clock.get_cycles();
                    (instr.cycles() + temp_cycles + unclaimed_cycles) as usize
                } else {
                    let unclaimed_cycles = bus.cycle_clock.get_cycles();
                    1usize + unclaimed_cycles as usize
                }
            },
            Err(e) => {
                panic!("{:?}", e);
            }
        };

        // Use get_pc() (not the captured current_pc index), and also check the instruction set
        // wasn't switched (e.g. by BX), since either changes which register slot the PC lives in
        // and whether pc_after_advance is even the right comparison.
        let mode_unchanged = (self.get_instruction_set() == InstructionSet::Arm) == is_arm;
        if mode_unchanged && self.get_pc() == pc_after_advance {
            // Sequential flow: the lookahead word we already fetched is genuinely the next
            // instruction, so cache it for next time.
            self.prefetch_cache = lookahead;
            self.prefetch_primed = true;
        } else {
            // Branch/exception redirected PC; the lookahead fetch was speculative and wasted,
            // matching real hardware discarding a prefetch on a taken branch.
            self.prefetch_primed = false;
        }

        return cycles;
    }

    pub fn get_instruction_set(&self) -> InstructionSet {
        if self.cpsr.control_bits.state_bit {
            InstructionSet::Thumb
        } else {
            InstructionSet::Arm
        }
    }

    pub fn set_instruction_set(&mut self, set: InstructionSet) {
        match set {
            InstructionSet::Arm =>      self.cpsr.control_bits.state_bit = false,
            InstructionSet::Thumb =>    self.cpsr.control_bits.state_bit = true
        }
    } 

    pub fn get_operating_mode(&self) -> OperatingMode {
        match self.cpsr.control_bits.mode_bits {
            0b10000 => OperatingMode::User,
            0b10001 => OperatingMode::FastInterrupt,
            0b10010 => OperatingMode::Interrupt,
            0b10011 => OperatingMode::Supervisor,
            0b10111 => OperatingMode::Abort,
            0b11011 => OperatingMode::Undefined,
            0b11111 => OperatingMode::System,
            _ => OperatingMode::System
        }
    }

    pub fn set_operating_mode(&mut self, mode: OperatingMode) {
        match mode {
            OperatingMode::User =>          self.cpsr.control_bits.mode_bits = 0b10000,
            OperatingMode::FastInterrupt => self.cpsr.control_bits.mode_bits = 0b10001,
            OperatingMode::Interrupt =>     self.cpsr.control_bits.mode_bits = 0b10010,
            OperatingMode::Supervisor =>    self.cpsr.control_bits.mode_bits = 0b10011,
            OperatingMode::Abort =>         self.cpsr.control_bits.mode_bits = 0b10111,
            OperatingMode::Undefined =>     self.cpsr.control_bits.mode_bits = 0b11011,
            OperatingMode::System =>        self.cpsr.control_bits.mode_bits = 0b11111,
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
        CPU::check_reg_range(&reg_num, &self.get_instruction_set());
        return self.registers[REG_MAP[self.get_instruction_set() as usize][self.get_operating_mode() as usize][reg_num as usize]];
    }

    pub fn set_register(&mut self, reg_num: u8, value: u32) {
        CPU::check_reg_range(&reg_num, &self.get_instruction_set());
        self.registers[REG_MAP[self.get_instruction_set() as usize][self.get_operating_mode() as usize][reg_num as usize]] = value;
    }

    pub fn get_register_override_opmode(&self, reg_num: u8, op_mode: OperatingMode) -> u32 {
        CPU::check_reg_range(&reg_num, &self.get_instruction_set());
        return self.registers[REG_MAP[self.get_instruction_set() as usize][op_mode as usize][reg_num as usize]];
    }

    pub fn set_register_override_opmode(&mut self, reg_num: u8, op_mode: OperatingMode, value: u32) {
        CPU::check_reg_range(&reg_num, &self.get_instruction_set());
        self.registers[REG_MAP[self.get_instruction_set() as usize][op_mode as usize][reg_num as usize]] = value;
    }
    
    pub fn get_register_unsafe(&self, reg_num: u8) -> u32{
        return self.registers[REG_MAP[InstructionSet::Arm as usize][self.get_operating_mode() as usize][reg_num as usize]];
    }

    pub fn set_register_unsafe(&mut self, reg_num: u8, value: u32){
        self.registers[REG_MAP[InstructionSet::Arm as usize][self.get_operating_mode() as usize][reg_num as usize]] = value;
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
            Condition::LS => return !self.cpsr.flags.carry || self.cpsr.flags.zero,
            Condition::GE => return self.cpsr.flags.negative == self.cpsr.flags.signed_overflow,
            Condition::LT => return self.cpsr.flags.negative != self.cpsr.flags.signed_overflow,
            Condition::GT => return !self.cpsr.flags.zero && (self.cpsr.flags.negative == self.cpsr.flags.signed_overflow),
            Condition::LE => return self.cpsr.flags.zero || (self.cpsr.flags.negative != self.cpsr.flags.signed_overflow),
            Condition::AL => return true,
            Condition::Error => panic!("Condition::Error hit"),
        }
    }

    pub fn get_spsr(&mut self) -> ProgramStatusRegister {
        if self.get_operating_mode() == OperatingMode::User {
            panic!("Invalid operating mode {:?}", self.get_operating_mode());
        }
        return self.spsr[self.get_operating_mode() as usize];
    }

    pub fn set_spsr(&mut self, psr: ProgramStatusRegister) {
        if self.get_operating_mode() == OperatingMode::User {
            panic!("Invalid operating mode {:?}", self.get_operating_mode());
        }
        self.spsr[self.get_operating_mode() as usize] = psr;
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
//        let bus = MemoryBus::new_stub();

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
//        let mut bus = MemoryBus::new_stub();
//        let cpu = CPU::new();
        // cpu.decode(&mut map, 0xE0812001);
    }

    #[test]
    fn test_fetch(){
        let mut cpu = CPU::new();
        cpu.set_register(15, 0x02000000);
        let mut bus = MemoryBus::new_stub();
        bus.write_u32(0x02000000, 0x012081E0);
        bus.write_u32(0x02000004, 0x012081E0);
        cpu.fetch(&mut bus);
        cpu.fetch(&mut bus);
    }

    #[test]
    fn fetch_claims_all_pending_cycle_clock_cost_for_non_memory_instructions() {
        let mut cpu = CPU::new();
        cpu.set_register(15, 0x02000000);
        let mut bus = MemoryBus::new_stub();
        bus.write_u32(0x02000000, 0xE1A00000);
        cpu.fetch(&mut bus);
        assert_eq!(bus.cycle_clock.get_cycles(), 0);
    }

    #[test]
    fn test_register_access() {
        let mut cpu = CPU::new();
        cpu.set_register(10, 15);
        let spv_reg_10 = cpu.get_register(10);
        cpu.set_operating_mode(OperatingMode::User);
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
        cpu.set_instruction_set(InstructionSet::Thumb);
        let _should_fail = cpu.get_register(11);
    }

    // #[test]
    // fn test_branch_exchange(){
    //     let mut cpu = CPU::new();
    //     cpu.set_register(15, 0x02000000);
    //     let mut map = MemoryBus::new_stub();
    //     map.register_memory(0x02000000, 0x0203FFFF, &cpu.wram.memory);
    //     map.write_u32(0x02000000, 0x11FF2FE1u32.to_be());
    //     cpu.fetch(&mut map);
    //     assert_eq!(cpu.get_instruction_set(), InstructionSet::Thumb);
    // }
}
