use super::{common::Condition, common::Instruction};
use crate::memory::memory_map::MemoryMap;
use crate::cpu::cpu::CPU;
use crate::operations::load_store::{apply_offset, DataType, DataTransfer, data_transfer_execute};
use crate::operations::shift::{Shift, apply_shift};

#[derive(Debug)]
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
    pub immediate_value: u8,
    pub immediate: bool,
}

impl From<u32> for SingleDataTransferOperand {
    fn from(value: u32) -> SingleDataTransferOperand {
        return SingleDataTransferOperand {
            shift: Shift::from(value),
            rm: (value & 0xF) as u8,
            immediate_value: (value & 0xFF) as u8,
            immediate: ((value & 0x200_0000) >> 25) != 0,
        };
    }
}

impl Instruction for SingleDataTransfer {
    fn execute(&mut self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let address_with_offset;
        let base = cpu.get_register(self.op1_register);
        if !self.offset_is_register {
            address_with_offset = apply_offset(base, self.offset.immediate_value, self.up_down);
        } else {
            let shifted_register = apply_shift(self.offset.rm as u32, &self.offset.shift, cpu);
            let offset = cpu.get_register(shifted_register as u8) as u8;
            address_with_offset = apply_offset(base, offset, self.up_down);
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
        data_transfer_execute(transfer_info, base, address_with_offset, cpu, mem_map);
    }
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