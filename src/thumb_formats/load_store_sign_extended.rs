use crate::operations::instruction::Instruction;
use crate::cpu::cpu::CPU;
use crate::memory::memory_map::MemoryMap;
use crate::thumb_formats::load_store_halfword::LoadStoreHalfword;
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
    fn execute(&self, cpu: &mut CPU, mem_map: &mut MemoryMap) {
        let address = cpu.get_register(self.base_register) +
            cpu.get_register(self.offset_register);
        if !self.sign_extended && !self.h_flag {
            // store halfword
            let store_halfword = LoadStoreHalfword {
                load: false,
                immediate_offset: cpu.get_register(self.offset_register) as u8,
                rb: self.base_register,
                rd: self.destination_register,
            };
            store_halfword.execute(cpu, mem_map);
        } else if !self.sign_extended && self.h_flag {
            // load halfword
            cpu.set_register(self.base_register, address);
            cpu.set_register(self.destination_register,
                             mem_map.read_u16(address) as u32 & 0x0000_FFFF);
        } else if self.sign_extended && !self.h_flag {
            // load sign-extended byte
            let byte_from_memory = mem_map.read_u8(address);
            let mut value_to_load = byte_from_memory as u32;
            if (byte_from_memory & (1 << 7)) > 0 {
                value_to_load = byte_from_memory as u32 | 0xFFFF_FF00;
            }

            cpu.set_register(self.destination_register, value_to_load);
        } else {
            // load sign-extended halfword
            let halfword_from_memory = mem_map.read_u16(address);
            let value_to_load: u32;
            if (halfword_from_memory & (1 << 15)) > 0 {
                value_to_load = halfword_from_memory as u32 | 0xFFFF_0000;
            } else {
                value_to_load = halfword_from_memory as u32 & 0x0000_FFFF;
            }
            cpu.set_register(self.destination_register, value_to_load);
        }
        cpu.set_register(self.base_register, address);
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
    use crate::memory::work_ram::WorkRam;

    #[test]
    fn test_store_halfword() {
        let format = LoadStoreSignExtended::from(0x52A6);
        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);

        cpu.set_register(2, 0x02);
        cpu.set_register(4, 0x06);
        cpu.set_register(6, 0xF2F1);

        format.execute(&mut cpu, &mut mem_map);
        assert_eq!(format.h_flag, false);
        assert_eq!(format.sign_extended, false);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.base_register, 4);
        assert_eq!(format.destination_register, 6);
        assert_eq!(mem_map.read_u16(0x02 + 0x06), 0xF2F1);
    }

    #[test]
    fn test_load_halfword() {
        let format = LoadStoreSignExtended::from(0x5AA6);
        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);

        cpu.set_register(2, 0x4);
        cpu.set_register(4, 0x8);
        mem_map.write_u32(0x8 + 0x4, 0xF1A1);

        format.execute(&mut cpu, &mut mem_map);

        assert_eq!(format.h_flag, true);
        assert_eq!(format.sign_extended, false);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.base_register, 4);
        assert_eq!(format.destination_register, 6);
        assert_eq!(cpu.get_register(format.destination_register), 0x0000_F1A1)
    }

    #[test]
    fn test_load_sign_extended_byte () {
        let format = LoadStoreSignExtended::from(0x56A6);
        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);

        cpu.set_register(2, 0x4);
        cpu.set_register(4, 0x8);
        mem_map.write_u32(0x8 + 0x4, 0xA1);

        format.execute(&mut cpu, &mut mem_map);

        assert_eq!(format.h_flag, false);
        assert_eq!(format.sign_extended, true);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.base_register, 4);
        assert_eq!(format.destination_register, 6);
        assert_eq!(cpu.get_register(format.destination_register), 0xFFFF_FFA1)
    }

    #[test]
    fn test_load_sign_extended_halfword_negative () {
        let format = LoadStoreSignExtended::from(0x5EA6);
        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);

        cpu.set_register(2, 0x4);
        cpu.set_register(4, 0x8);
        mem_map.write_u32(0x8 + 0x4, 0xFF01);

        format.execute(&mut cpu, &mut mem_map);

        assert_eq!(format.h_flag, true);
        assert_eq!(format.sign_extended, true);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.base_register, 4);
        assert_eq!(format.destination_register, 6);
        assert_eq!(cpu.get_register(format.destination_register), 0xFFFF_FF01);
    }

    #[test]
    fn test_load_sign_extended_halfword_positive () {
        let format = LoadStoreSignExtended::from(0x5EA6);
        let mut cpu = CPU::new();
        let mut mem_map = MemoryMap::new();
        let wram = WorkRam::new(256000, 0);
        mem_map.register_memory(0x0000, 0x00FF, &wram.memory);

        cpu.set_register(2, 0x4);
        cpu.set_register(4, 0x8);
        mem_map.write_u32(0x8 + 0x4, 0x1F01);

        format.execute(&mut cpu, &mut mem_map);

        assert_eq!(format.h_flag, true);
        assert_eq!(format.sign_extended, true);
        assert_eq!(format.offset_register, 2);
        assert_eq!(format.base_register, 4);
        assert_eq!(format.destination_register, 6);
        assert_eq!(cpu.get_register(format.destination_register), 0x0000_1F01);
    }
}
