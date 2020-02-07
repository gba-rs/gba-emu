use crate::cpu::{cpu::CPU, condition::Condition};
use crate::operations::load_store::{apply_offset, DataType, DataTransfer, data_transfer_execute};
use crate::operations::shift::{Shift, apply_shift};
use log::{debug};
use crate::operations::instruction::Instruction;
use std::fmt;
use crate::gba::memory_bus::MemoryBus;

pub struct SingleDataTransfer {
    pub offset: SingleDataTransferOperand,
    pub destination_register: u8,
    pub op1_register: u8,
    pub load: bool,
    pub write_back: bool,
    pub is_byte: bool,
    pub up_down: bool,
    pub is_pre_indexed: bool,
    pub offset_is_register: bool,
    pub condition: Condition,
    pub data_type: DataType,
}

impl fmt::Debug for SingleDataTransfer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.load {
            write!(f, "LDR{:?}", self.condition)?;
        } else {
            write!(f, "STR{:?}", self.condition)?;
        }

        if self.is_byte {
            write!(f, "B ")?;
        } else {
            write!(f, " ")?;
        }

        write!(f, "r{}, [r{}", self.destination_register, self.op1_register)?;

        if self.offset_is_register {
            if self.up_down {
                write!(f, ", +r{}, {:?}", self.offset.rm, self.offset.shift)?;
            } else {
                write!(f, ", -r{}, {:?}", self.offset.rm, self.offset.shift)?;
            }
        } else {
            if self.up_down {
                write!(f, ", +#0x{:X}", self.offset.immediate_value)?;
            } else {
                write!(f, ", -#0x{:X}", self.offset.immediate_value)?;
            }
        }

        if self.write_back {
            if self.is_pre_indexed {
                write!(f, "]!")
            } else {
                write!(f, "] r{}", self.op1_register)
            }
        } else {
            write!(f, "]")
        }
    }
}

impl From<u32> for SingleDataTransfer {
    fn from(value: u32) -> SingleDataTransfer {
        let is_byte: bool = ((value & 0x40_0000) >> 22) != 0;
        let data_type;
        if is_byte {
            data_type = DataType::Byte
        } else {
            data_type = DataType::Word
        }

        return SingleDataTransfer {
            offset: SingleDataTransferOperand::from(value),
            destination_register: ((value & 0xF000) >> 12) as u8,
            op1_register: ((value & 0xF_0000) >> 16) as u8,
            load: ((value & 0x10_0000) >> 20) != 0,
            write_back: ((value & 0x20_0000) >> 21) != 0,
            is_byte,
            up_down: ((value & 0x80_0000) >> 23) != 0,
            is_pre_indexed: ((value & 0x100_0000) >> 24) != 0,
            offset_is_register: ((value & 0x200_0000) >> 25) != 0, //offset is an immediate value if = 0
            condition: Condition::from((value & 0xF000_0000) >> 28),
            data_type,
        };
    }
}

#[derive(Debug)]
pub struct SingleDataTransferOperand {
    pub shift: Shift,
    pub rm: u8,
    pub immediate_value: u16,
    pub immediate: bool,
}

impl From<u32> for SingleDataTransferOperand {
    fn from(value: u32) -> SingleDataTransferOperand {
        return SingleDataTransferOperand {
            shift: Shift::from(value),
            rm: (value & 0xF) as u8,
            immediate_value: (value & 0xFFF) as u16,
            immediate: ((value & 0x0200_0000) >> 25) == 0,
        };
    }
}

impl Instruction for SingleDataTransfer {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) {
        let address_with_offset;
        let base;
        if self.op1_register == 15 {
            base = cpu.get_register(self.op1_register) + 4;
        } else {
            base = cpu.get_register(self.op1_register);
        }
        if !self.offset_is_register {
            address_with_offset = apply_offset(base, self.offset.immediate_value as u32, self.up_down, 0);
        } else {
            let (value, _) = apply_shift(cpu.get_register(self.offset.rm), &self.offset.shift, cpu);
            address_with_offset = apply_offset(base, value, self.up_down, 0);
        }

        let transfer_info =  DataTransfer {
            is_pre_indexed: self.is_pre_indexed,
            write_back: self.write_back,
            load: self.load,
            is_signed: false,
            data_type: self.data_type,
            base_register: self.op1_register,
            destination: self.destination_register,
        };
        data_transfer_execute(transfer_info, base, address_with_offset, cpu, mem_bus);
    }

    fn asm(&self) -> String {
        return format!("{:?}", self);
    }
    fn cycles(&self) -> u32 {return 3;}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singledatatransfer_zero() {
        let a: SingleDataTransfer = SingleDataTransfer::from(0x00000000);
        assert_eq!(a.destination_register, 0);
        assert_eq!(a.op1_register, 0);
        assert_eq!(a.is_pre_indexed, false);
        assert_eq!(a.up_down, false);
        assert_eq!(a.is_byte, false);
        assert_eq!(a.load, false);
    }

    #[test]
    fn singledatatransfer_max() {
        let a: SingleDataTransfer = SingleDataTransfer::from(0xFFFFFFFF);
        assert_eq!(a.destination_register, 0b1111);
        assert_eq!(a.op1_register, 0b1111);
        assert_eq!(a.condition, Condition::Error);
        assert_eq!(a.offset_is_register, true);
        assert_eq!(a.is_pre_indexed, true);
        assert_eq!(a.up_down, true);
        assert_eq!(a.is_byte, true);
        assert_eq!(a.load, true);
    }
}