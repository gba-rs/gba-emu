use std::io::prelude::*;
use std::fs::File;
use serde::{Serialize, Deserialize};

pub mod flash;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum BackupType {
    Sram,
    Eeprom,
    Flash64K,
    Flash128K,
    Error
}

#[derive(Serialize, Deserialize)]
pub struct GamePack {
    #[serde(skip)]
    pub rom: Vec<u8>,
    #[serde(skip)]
    pub bios: Vec<u8>,
    #[serde(skip)]
    pub save_data: Vec<u8>,
    pub title: String,
    pub game_code: String,
    pub maker_code: String,
    pub backup_type: BackupType,
}

pub const MEM_STRINGS: [&str; 5] = ["SRAM", "EEPROM", "FLASH_", "FLASH512_", "FLASH1M_"];

impl GamePack {
    pub fn new(bios_file_path: &str, rom_file_path: &str) -> GamePack {
        let mut rom = File::open(rom_file_path);
        let mut rom_bytes = Vec::new();

        match &mut rom {
            Ok(val) => {
                let _ = val.read_to_end(&mut rom_bytes);
            },
            Err(_) => {
                panic!("Error loading rom: {}", rom_file_path);
            }
        }

        let mut bios = File::open(&bios_file_path);
        let mut bios_bytes = Vec::new();

        match &mut bios {
            Ok(val) => {
                let _ = val.read_to_end(&mut bios_bytes);
            },
            Err(_) => {
                panic!("Error loading bios: {}", bios_file_path);
            }
        }

        let mut title = "";
        match std::str::from_utf8(&rom_bytes[0xA0..0xAC]) {
            Ok(val) => {
                title = val; 
            },
            Err(_) => {
                log::info!("Title could not be parsed");
            }
        }

        let mut game_code = "";
        match std::str::from_utf8(&rom_bytes[0xAC..0xB0]) {
            Ok(val) => {
                game_code = val; 
            },
            Err(_) => {
                log::info!("Game Code could not be parsed");
            }
        }

        let mut make_code = "";
        match std::str::from_utf8(&rom_bytes[0xB0..0xB2]) {
            Ok(val) => {
                make_code = val; 
            },
            Err(_) => {
                log::info!("Maker Code could not be parsed");
            }
        }

        let backup = GamePack::detect_backup_type(&rom_bytes);

        return GamePack {
            rom: rom_bytes.clone(),
            bios: bios_bytes,
            save_data: Vec::new(),
            title: String::from(title),
            game_code: String::from(game_code),
            maker_code: String::from(make_code),
            backup_type: backup
        };
    }

    pub fn read_title(&mut self) {
        let mut title = "";
        match std::str::from_utf8(&self.rom[0xA0..0xAC]) {
            Ok(val) => {
                title = val; 
            },
            Err(_) => {
                log::info!("Title could not be parsed");
            }
        }
        self.title = String::from(title);
    }

    pub fn default() -> GamePack {
        return GamePack {
            rom: Vec::new(),
            bios: Vec::new(),
            save_data: Vec::new(),
            title: String::from(""),
            game_code: String::from(""),
            maker_code: String::from(""),
            backup_type: BackupType::Error
        };
    }

    pub fn load_save_data(&mut self, save_data_file_path: &str) {
        let mut save_data = File::open(&save_data_file_path);
        let mut save_data_bytes = Vec::new();

        match &mut save_data {
            Ok(val) => {
                let _ = val.read_to_end(&mut save_data_bytes);
            },
            Err(_) => {
                panic!("Error loading bios: {}", save_data_file_path);
            }
        }

        // todo put a check in here to see if the save data matches the size of the backup type

        self.save_data = save_data_bytes;
    }

    pub fn detect_backup_type(rom: &Vec<u8>) -> BackupType {
        for i in 0..5 {
            let mem_string_bytes = MEM_STRINGS[i].as_bytes();
            let result = rom.windows(mem_string_bytes.len()).position(|window| window == mem_string_bytes);
            match result {
                Some(_) => {
                    // string exists
                    log::info!("Found backup type: {}", MEM_STRINGS[i]);
                    match MEM_STRINGS[i] {
                        "SRAM" => return BackupType::Sram,
                        "EEPROM" => return BackupType::Eeprom,
                        "FLASH_" => return BackupType::Flash64K,
                        "FLASH512_" => return BackupType::Flash64K,
                        "FLASH1M_" => return BackupType::Flash128K,
                        _ => return BackupType::Error
                    }
                },
                None => {
                    // string doesn't exist
                }
            }
        }

        return BackupType::Error;
    }
}
