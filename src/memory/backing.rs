use serde::{Deserialize, Deserializer};
use std::{cell::Cell, ops::Index};

const REGIONS: [(usize, usize); 9] = [
    (0, 0x4000),
    (0x02000000, 0x40000),
    (0x03000000, 0x8000),
    (0x04000000, 0x410),
    (0x05000000, 0x400),
    (0x06000000, 0x18000),
    (0x07000000, 0x400),
    (0x0E000000, 0x20000),
    (0x10000000, 0xF0),
];
const MAGIC: &[u8; 8] = b"GBAMEM01";
const LEGACY_SIZE: usize = 0x100000F0;

/// Physical backing only. Bus mirroring and side effects remain in MemoryMap.
pub struct GbaMem {
    data: Box<[Cell<u8>]>,
    pages: Box<[usize]>,
    rom_start: usize,
    rom_len: usize,
    unmapped: Cell<u8>,
}

impl GbaMem {
    pub fn new(rom_len: usize) -> Self {
        assert!(rom_len <= 0x2000000, "ROM exceeds 32 MiB");
        // 64 KiB translation pages avoid a region match on every GPU/register byte.
        let mut pages = vec![usize::MAX; 0x1001];
        let mut length = 0;
        for &(start, len) in &REGIONS {
            for page in 0..len.div_ceil(0x10000) {
                pages[(start >> 16) + page] = length;
                length += 0x10000;
            }
        }
        let rom_start = length;
        for page in 0..rom_len.div_ceil(0x10000) {
            for base in [0x800, 0xA00, 0xC00] {
                pages[base + page] = length;
            }
            length += 0x10000;
        }
        pages[0xF00] = pages[0xE00];
        pages[0xF01] = pages[0xE01];
        let result = Self {
            data: vec![Cell::new(0); length].into_boxed_slice(),
            pages: pages.into_boxed_slice(),
            rom_start,
            rom_len,
            unmapped: Cell::new(0),
        };
        for i in 0..0x20000 {
            result[0x0E000000 + i].set(255);
        }
        result
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn rom_capacity(&self) -> usize {
        self.rom_len
    }

    /// Resolve a register binding once, instead of translating each register read.
    pub fn resolve(&self, address: usize) -> usize {
        let page = self.pages[address >> 16];
        assert_ne!(page, usize::MAX, "unmapped register binding");
        page + (address & 0xFFFF)
    }

    #[inline(always)]
    pub fn cell_at(&self, index: usize) -> &Cell<u8> {
        &self.data[index]
    }

    pub fn write_block(&self, mut address: usize, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            let count = bytes.len().min(0x10000 - (address & 0xFFFF));
            let start = self.resolve(address);
            for (dst, src) in self.data[start..start + count].iter().zip(&bytes[..count]) {
                dst.set(*src);
            }
            address += count;
            bytes = &bytes[count..];
        }
    }

    #[inline]
    pub fn rom_word(&self, offset: usize) -> Option<u32> {
        if offset + 4 > self.rom_len {
            return None;
        }
        let bytes = &self.data[self.rom_start + offset..self.rom_start + offset + 4];
        Some(u32::from_le_bytes([
            bytes[0].get(),
            bytes[1].get(),
            bytes[2].get(),
            bytes[3].get(),
        ]))
    }

    #[inline]
    pub fn rom_halfword(&self, offset: usize) -> Option<u16> {
        if offset + 2 > self.rom_len {
            return None;
        }
        let bytes = &self.data[self.rom_start + offset..self.rom_start + offset + 2];
        Some(u16::from_le_bytes([bytes[0].get(), bytes[1].get()]))
    }

    pub fn resized_rom(&self, rom_len: usize) -> Self {
        let result = Self::new(rom_len);
        for &(start, len) in &REGIONS {
            for i in 0..len {
                result[start + i].set(self[start + i].get());
            }
        }
        result
    }

    /// Versioned mutable-memory payload; ROM and BIOS are reattached by the frontend.
    pub fn snapshot(&self, rom_size: u32, mirror: Option<u32>, wave: &[[u8; 16]; 2]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(540000);
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&rom_size.to_le_bytes());
        bytes.extend_from_slice(&mirror.unwrap_or(u32::MAX).to_le_bytes());
        bytes.extend_from_slice(&wave[0]);
        bytes.extend_from_slice(&wave[1]);
        for &(start, len) in &REGIONS[1..] {
            bytes.extend((start..start + len).map(|i| self[i].get()));
        }
        bytes
    }
}

impl Index<usize> for GbaMem {
    type Output = Cell<u8>;

    #[inline(always)]
    fn index(&self, address: usize) -> &Self::Output {
        let page = self.pages.get(address >> 16).copied().unwrap_or(usize::MAX);
        if page == usize::MAX {
            self.unmapped.set(0);
            return &self.unmapped;
        }
        &self.data[page + (address & 0xFFFF)]
    }
}

pub(super) struct MemorySnapshot {
    pub memory: GbaMem,
    pub rom_size: u32,
    pub mirror: Option<u32>,
    pub wave: [[u8; 16]; 2],
}

impl<'de> Deserialize<'de> for MemorySnapshot {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        if bytes.len() == LEGACY_SIZE {
            let memory = GbaMem::new(0x2000000);
            for &(start, len) in &REGIONS {
                for i in 0..len {
                    memory[start + i].set(bytes[start + i]);
                }
            }
            for i in 0..0x2000000 {
                memory[0x08000000 + i].set(bytes[0x08000000 + i]);
            }
            return Ok(Self {
                memory,
                rom_size: 0x2000000,
                mirror: None,
                wave: [[0; 16]; 2],
            });
        }
        let expected = 48 + REGIONS[1..].iter().map(|r| r.1).sum::<usize>();
        if bytes.len() != expected || &bytes[..8] != MAGIC {
            return Err(serde::de::Error::custom(
                "unsupported memory snapshot version or length",
            ));
        }
        let word = |offset| {
            u32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ])
        };
        let rom_size = word(8);
        if rom_size > 0x2000000 {
            return Err(serde::de::Error::custom("invalid ROM size in snapshot"));
        }
        let mask = word(12);
        let mut result = Self {
            memory: GbaMem::new(rom_size as usize),
            rom_size,
            mirror: (mask != u32::MAX).then_some(mask),
            wave: [[0; 16]; 2],
        };
        result.wave[0].copy_from_slice(&bytes[16..32]);
        result.wave[1].copy_from_slice(&bytes[32..48]);
        let mut offset = 48;
        for &(start, len) in &REGIONS[1..] {
            for i in 0..len {
                result.memory[start + i].set(bytes[offset]);
                offset += 1;
            }
        }
        Ok(result)
    }
}
