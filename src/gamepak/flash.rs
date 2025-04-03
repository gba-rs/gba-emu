use crate::memory::memory_map::MemoryMap;
use crate::gamepak::BackupType;
use serde::Serialize;

#[derive(Serialize)]
pub enum FlashCommands {
    StartID = 0x90,
    EndID = 0xF0,
    EnableErase = 0x80,
    EraseChip = 0x10,
    EraseSector = 0x30,
    WriteByte = 0xA0,
    SelectBank = 0xb0
}

impl FlashCommands {
    pub fn from(value: u8) -> FlashCommands {
        match value {
            0x90 => FlashCommands::StartID,
            0xF0 => FlashCommands::EndID,
            0x80 => FlashCommands::EnableErase,
            0x10 => FlashCommands::EraseChip,
            0x30 => FlashCommands::EraseSector,
            0xA0 => FlashCommands::WriteByte,
            0xB0 => FlashCommands::SelectBank,
            _ => panic!("Invalid flash command")
        }
    }
}

#[derive(Serialize)]
enum FlashPhase {
    Phase1,
    Phase2,
    Command,
    CommandParameter
}

#[derive(Serialize)]
pub struct Flash {
    phase: FlashPhase,
    enable_id: bool,
    enable_erase: bool,
    enable_write: bool,
    enable_bank_select: bool,
    bank: u8
}

impl Flash {
    pub fn new() -> Flash {
        return Flash {
            phase: FlashPhase::Phase1,
            enable_id: false,
            enable_erase: false,
            enable_write: false,
            enable_bank_select: false,
            bank: 0
        };
    }

    pub fn banked_address(&self, address: u32) -> u32 {
        return ((self.bank as u32) * 65536) + address;
    }
}

impl MemoryMap {
    

    pub fn read_flash(&self, address: u32) -> u8 {
        if self.flash.enable_id && (address & 0xFFFF) < 2 {
            if self.backup_type == BackupType::Flash128K {
                log::info!("Returning id: {:X}", address);
                return if (address & 0xFFFF) == 0 { 0xC2 } else { 0x09 };
            } else {
                return if (address & 0xFFFF) == 0 { 0xBF } else { 0xD4 };
            }
        } 
        return self.memory.borrow()[self.flash.banked_address(address) as usize];
    }

    pub fn write_flash(&mut self, address: u32, value: u8) {
        match self.flash.phase {
            FlashPhase::Phase1 => {
                if address == 0x0E005555 && value == 0xAA {
                    self.flash.phase = FlashPhase::Phase2;
                }
            },
            FlashPhase::Phase2 => {
                if address == 0x0E002AAA && value == 0x55 {
                    self.flash.phase = FlashPhase::Command;
                }
            },
            FlashPhase::Command => {
                self.run_command(address, value);
            },
            FlashPhase::CommandParameter => {
                self.run_command_parameter(address, value);
            }
        }
    }

    

    fn run_command(&mut self, address: u32, value: u8) {
        let flash_command = FlashCommands::from(value);

        if address == 0x0E005555 || (address & !0xF000) == 0x0E000000 {
            
            match flash_command {
                FlashCommands::StartID => {
                    self.flash.enable_id = true;
                    self.flash.phase = FlashPhase::Phase1;
                },
                FlashCommands::EndID => {
                    self.flash.enable_id = false;
                    self.flash.phase = FlashPhase::Phase1;
                },
                FlashCommands::EnableErase => {
                    self.flash.enable_erase = true;
                    self.flash.phase = FlashPhase::Phase1;
                },
                FlashCommands::EraseChip => {
                    if self.flash.enable_erase {
                        if self.backup_type == BackupType::Flash128K {
                            let mut mem = self.memory.borrow_mut();
                            for i in 0..0x2_0000 {
                                mem[(0x0E000000 + i) as usize] = 0xFF;
                            }
                        } else {
                            let mut mem = self.memory.borrow_mut();
                            for i in 0..0x1_0000 {
                                mem[(0x0E000000 + i) as usize] = 0xFF;
                            }
                        }
                    }

                    self.flash.phase = FlashPhase::Phase1;
                },
                FlashCommands::WriteByte => {
                    self.flash.enable_write = true;
                    self.flash.phase = FlashPhase::CommandParameter;
                },
                FlashCommands::SelectBank => {
                    if self.backup_type == BackupType::Flash128K {
                        self.flash.enable_bank_select = true;
                        self.flash.phase = FlashPhase::CommandParameter;
                    } else {
                        self.flash.phase = FlashPhase::Phase1;
                    }
                },
                FlashCommands::EraseSector => {
                    if self.flash.enable_erase && (address & !0xF000) == 0x0E000000 {
                        let mut mem = self.memory.borrow_mut();
                        for i in 0..0x1000 {
                            mem[self.flash.banked_address(address + i) as usize] = 0xFF;
                        }

                        self.flash.enable_erase = false;
                        self.flash.phase = FlashPhase::Phase1;
                    }
                }
            }
        }
    }

    fn run_command_parameter(&mut self, address: u32, value: u8) {
        if self.flash.enable_write {
            self.memory.borrow_mut()[self.flash.banked_address(address) as usize] = value;
            self.flash.enable_write = false;
        } else if self.flash.enable_bank_select && address == 0x0E000000 {
            self.flash.bank = value & 1;
            self.flash.enable_bank_select = false;
        }

        self.flash.phase = FlashPhase::Phase1;
    }
}
