use crate::operations::instruction::Instruction;
use crate::cpu::cpu::{CPU, THUMB_PC};
use crate::operations::load_store::{DataTransfer, DataType, data_transfer_execute};
use crate::memory::memory_bus::MemoryBus;
use core::fmt;


pub struct LoadStoreSignExtended {
    h_flag: bool,
    sign_extended: bool,
    offset_register: u8,
    base_register: u8,
    destination_register: u8,
}

impl From<u16> for LoadStoreSignExtended {
    fn from(value: u16) -> LoadStoreSignExtended {
        return LoadStoreSignExtended {
            h_flag: ((value & 0x800) >> 11) != 0,
            sign_extended: ((value & 0x400) >> 10) != 0,
            offset_register: ((value & 0x1C0) >> 6) as u8,
            base_register: ((value & 0x38) >> 3) as u8,
            destination_register: (value & 0x7) as u8,
        };
    }
}

impl fmt::Debug for LoadStoreSignExtended {
    fn fmt( & self, f: & mut fmt::Formatter < '_ > ) -> fmt::Result {
        let instruction = format!("r{:?}, [r{:?}, r{:?}]", self.destination_register, self.base_register, self.offset_register);
        let instr_type;
        if !self.sign_extended && !self.h_flag {
            instr_type = format!("STRH");
        } else if !self.sign_extended && self.h_flag {
            instr_type = format!("LDRH");
        } else if self.sign_extended && !self.h_flag {
            instr_type = format!("LDSB");
        } else {
            instr_type = format!("LDSH");
        }
        write!(f, "{} {}", instr_type, instruction )
    }
}

impl Instruction for LoadStoreSignExtended {
    fn execute(&self, cpu: &mut CPU, mem_bus: &mut MemoryBus) -> u32 {
        let data_type = if !self.h_flag && self.sign_extended { DataType::Byte } else { DataType::Halfword };
        let is_load;
        if !self.sign_extended && !self.h_flag {
            is_load = false;
        } else {
            is_load = true;
        }

        let transfer_info = DataTransfer {
            is_pre_indexed: true,
            write_back: false,
            load: is_load,
            is_signed: self.sign_extended,
            data_type: data_type,
            base_register: self.base_register,
            destination: self.destination_register,
        };

        let target_address = cpu.get_register(self.base_register) + cpu.get_register(self.offset_register);
        let base;
        if self.base_register == THUMB_PC {
            base = cpu.get_register(self.base_register) + 2;
        } else {
            base = cpu.get_register(self.base_register);
        }

        data_transfer_execute(transfer_info, base, target_address, cpu, mem_bus);
        mem_bus.cycle_clock.get_cycles()
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

    #[test]
    fn test_store_halfword() {
        let format = LoadStoreSignExtended::from(0x52A6);
        let mut gba = GBA::default();

        gba.cpu.set_register(2, 0x02);
        gba.cpu.set_register(4, 0x08000006);
        gba.cpu.set_register(6, 0xF2F1);

        format.execute(&mut gba.cpu, &mut gba.memory_bus);
        assert_eq!(format.h_flag, false);
        assert_eq!(format.sign_extended, false);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.base_register, 4);
        assert_eq!(format.destination_register, 6);
        assert_eq!(gba.memory_bus.read_u16(0x02 + 0x08000006), 0xF2F1);
    }

    #[test]
    fn test_load_halfword() {
        let format = LoadStoreSignExtended::from(0x5AA6);
        let mut gba = GBA::default();

        gba.cpu.set_register(2, 0x4);
        gba.cpu.set_register(4, 0x08000008);
        gba.memory_bus.write_u32(0x08000008 + 0x4, 0xF1A1);

        format.execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(format.h_flag, true);
        assert_eq!(format.sign_extended, false);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.base_register, 4);
        assert_eq!(format.destination_register, 6);
        assert_eq!(gba.cpu.get_register(format.destination_register), 0x0000_F1A1)
    }

    #[test]
    fn test_load_sign_extended_byte () {
        let format = LoadStoreSignExtended::from(0x56A6);
        let mut gba = GBA::default();

        gba.cpu.set_register(2, 0x4);
        gba.cpu.set_register(4, 0x08000008);
        gba.memory_bus.write_u32(0x08000008 + 0x4, 0xA1);

        format.execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(format.h_flag, false);
        assert_eq!(format.sign_extended, true);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.base_register, 4);
        assert_eq!(format.destination_register, 6);
        assert_eq!(gba.cpu.get_register(format.destination_register), 0xFFFF_FFA1)
    }

    #[test]
    fn test_load_sign_extended_halfword_negative () {
        let format = LoadStoreSignExtended::from(0x5EA6);
        let mut gba = GBA::default();

        gba.cpu.set_register(2, 0x4);
        gba.cpu.set_register(4, 0x08000008);
        gba.memory_bus.write_u32(0x08000008 + 0x4, 0xFF01);

        format.execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(format.h_flag, true);
        assert_eq!(format.sign_extended, true);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.base_register, 4);
        assert_eq!(format.destination_register, 6);
        assert_eq!(gba.cpu.get_register(format.destination_register), 0xFFFF_FF01);
    }

    #[test]
    fn test_load_sign_extended_halfword_positive () {
        let format = LoadStoreSignExtended::from(0x5EA6);
        let mut gba = GBA::default();

        gba.cpu.set_register(2, 0x4);
        gba.cpu.set_register(4, 0x08000008);
        gba.memory_bus.write_u32(0x08000008 + 0x4, 0x1F01);

        format.execute(&mut gba.cpu, &mut gba.memory_bus);

        assert_eq!(format.h_flag, true);
        assert_eq!(format.sign_extended, true);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.base_register, 4);
        assert_eq!(format.destination_register, 6);
        assert_eq!(gba.cpu.get_register(format.destination_register), 0x0000_1F01);
    }
}
