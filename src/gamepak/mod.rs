use serde::{Serialize, Deserialize};
use std::fmt;

pub mod flash;

/// Errors that can occur while loading a [`GamePack`] from disk.
///
/// Kept separate from the wasm-safe, byte-based constructors below so that
/// this crate's core (`GamePack::from_bytes`) can compile and run on
/// `wasm32-unknown-unknown`, where `std::fs` is unavailable. Only the
/// native-only loader in this module depends on file I/O.
#[derive(Debug)]
pub enum GamePackError {
    /// Failed to read the ROM file from the given path.
    RomReadFailed { path: String, source: std::io::Error },
    /// Failed to read the BIOS file from the given path.
    BiosReadFailed { path: String, source: std::io::Error },
    /// Failed to read a save-data file from the given path.
    SaveDataReadFailed { path: String, source: std::io::Error },
}

impl fmt::Display for GamePackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GamePackError::RomReadFailed { path, source } =>
                write!(f, "failed to read ROM file '{}': {}", path, source),
            GamePackError::BiosReadFailed { path, source } =>
                write!(f, "failed to read BIOS file '{}': {}", path, source),
            GamePackError::SaveDataReadFailed { path, source } =>
                write!(f, "failed to read save data file '{}': {}", path, source),
        }
    }
}

impl std::error::Error for GamePackError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GamePackError::RomReadFailed { source, .. } => Some(source),
            GamePackError::BiosReadFailed { source, .. } => Some(source),
            GamePackError::SaveDataReadFailed { source, .. } => Some(source),
        }
    }
}

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
    /// Builds a `GamePack` directly from in-memory ROM/BIOS bytes.
    ///
    /// This is the portable, allocation-only constructor: it performs no
    /// file I/O, so it compiles and runs the same way on native targets and
    /// on `wasm32-unknown-unknown` (e.g. the `web-frontend`, which loads ROM
    /// bytes via a `<input type="file">`/`fetch` and hands them straight to
    /// this function instead of going through the filesystem).
    pub fn from_bytes(rom_bytes: Vec<u8>, bios_bytes: Vec<u8>) -> GamePack {
        let title = GamePack::parse_header_str(&rom_bytes, 0xA0, 0xAC, "Title");
        let game_code = GamePack::parse_header_str(&rom_bytes, 0xAC, 0xB0, "Game Code");
        let maker_code = GamePack::parse_header_str(&rom_bytes, 0xB0, 0xB2, "Maker Code");
        let backup_type = GamePack::detect_backup_type(&rom_bytes);

        GamePack {
            rom: rom_bytes,
            bios: bios_bytes,
            save_data: Vec::new(),
            title,
            game_code,
            maker_code,
            backup_type,
        }
    }

    /// Loads a `GamePack` from files on disk.
    ///
    /// Native-only: reads files via `std::fs`, which is not available on
    /// `wasm32-unknown-unknown`. Frontends that run in the browser should
    /// use [`GamePack::from_bytes`] instead, after fetching the ROM/BIOS
    /// bytes themselves.
    ///
    /// Returns a [`GamePackError`] on I/O failure instead of panicking, so a
    /// bad path (a very ordinary, recoverable situation for a frontend to
    /// hit) doesn't take down the whole process.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load(bios_file_path: &str, rom_file_path: &str) -> Result<GamePack, GamePackError> {
        use std::fs;

        let rom_bytes = fs::read(rom_file_path).map_err(|source| GamePackError::RomReadFailed {
            path: rom_file_path.to_string(),
            source,
        })?;

        let bios_bytes = fs::read(bios_file_path).map_err(|source| GamePackError::BiosReadFailed {
            path: bios_file_path.to_string(),
            source,
        })?;

        Ok(GamePack::from_bytes(rom_bytes, bios_bytes))
    }

    /// Deprecated alias for [`GamePack::load`], kept so existing native
    /// callers (e.g. `minifb-frontend`) don't break immediately. Panics on
    /// failure, matching the previous behavior; prefer `load` in new code.
    #[cfg(not(target_arch = "wasm32"))]
    #[deprecated(note = "use GamePack::load, which returns a Result instead of panicking")]
    pub fn new(bios_file_path: &str, rom_file_path: &str) -> GamePack {
        match GamePack::load(bios_file_path, rom_file_path) {
            Ok(pack) => pack,
            Err(e) => panic!("{}", e),
        }
    }

    fn parse_header_str(rom_bytes: &[u8], start: usize, end: usize, field_name: &str) -> String {
        if rom_bytes.len() < end {
            log::info!("{} could not be parsed: ROM shorter than header", field_name);
            return String::new();
        }
        match std::str::from_utf8(&rom_bytes[start..end]) {
            Ok(val) => String::from(val),
            Err(_) => {
                log::info!("{} could not be parsed", field_name);
                String::new()
            }
        }
    }

    pub fn read_title(&mut self) {
        self.title = GamePack::parse_header_str(&self.rom, 0xA0, 0xAC, "Title");
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

    /// Sets the pack's save data directly from in-memory bytes. Portable
    /// (no file I/O) — use this from the web frontend.
    pub fn set_save_data(&mut self, save_data: Vec<u8>) {
        // todo put a check in here to see if the save data matches the size of the backup type
        self.save_data = save_data;
    }

    /// Loads save data from a file on disk. Native-only; see
    /// [`GamePack::load`] for why.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_save_data(&mut self, save_data_file_path: &str) -> Result<(), GamePackError> {
        let save_data_bytes = std::fs::read(save_data_file_path).map_err(|source| {
            GamePackError::SaveDataReadFailed {
                path: save_data_file_path.to_string(),
                source,
            }
        })?;

        self.set_save_data(save_data_bytes);
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes_never_panics_on_short_rom() {
        // Regression test: header parsing used to index the ROM directly
        // (`&rom_bytes[0xA0..0xAC]`), which panics on any ROM shorter than
        // the header. from_bytes must degrade gracefully instead.
        let pack = GamePack::from_bytes(vec![0u8; 4], vec![]);
        assert_eq!(pack.title, "");
        assert_eq!(pack.game_code, "");
        assert_eq!(pack.maker_code, "");
    }

    #[test]
    fn from_bytes_parses_header_fields() {
        let mut rom = vec![0u8; 0xC0];
        rom[0xA0..0xAC].copy_from_slice(b"TESTGAME\0\0\0\0");
        rom[0xAC..0xB0].copy_from_slice(b"ABCD");
        rom[0xB0..0xB2].copy_from_slice(b"01");

        let pack = GamePack::from_bytes(rom, vec![]);
        assert_eq!(pack.game_code, "ABCD");
        assert_eq!(pack.maker_code, "01");
    }

    #[test]
    fn detect_backup_type_finds_flash1m() {
        let mut rom = vec![0u8; 16];
        rom.extend_from_slice(b"FLASH1M_V100");
        assert_eq!(GamePack::detect_backup_type(&rom), BackupType::Flash128K);
    }
}
