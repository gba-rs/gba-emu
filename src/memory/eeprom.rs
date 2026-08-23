use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

// Real EEPROM chips come in two sizes: a "narrow" 512-byte chip addressed
// with 6 bits, or a "wide" 8KB chip addressed with 14 bits. Neither the
// chip nor this emulator is ever told up front which size is in use — the
// game always sends the correct number of address bits for whichever chip
// its ROM was built against, and the DMA transfer that carries the request
// is exactly long enough to fit "opcode + address + optional data + stop
// bit" for that chip. So chip size is derived per-request from the halfword
// count of the enclosing DMA transfer rather than guessed from ROM size.
const DATA_BITS: u32 = 64;
const DATA_BYTES: usize = 8;
const STORAGE_SIZE: usize = 0x2000; // 8KB upper bound covers both chip sizes

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
enum EepromPhase {
    Idle,
    Command,
    Address,
    WriteData,
    WriteStop,
    AwaitingRead,
    Reading,
}

#[derive(Serialize, Deserialize)]
pub struct Eeprom {
    storage: Vec<u8>,
    phase: EepromPhase,
    pending_len: u32,
    is_write_request: bool,
    address_bits_expected: u32,
    bits_received: u32,
    command: u32,
    address: u32,
    write_buffer: u64,
    read_queue: VecDeque<bool>,
    /// Real chip size, learned the first time a request's address bit
    /// count is decoded (6 bits => 512B narrow chip, 14 bits => 8KB wide
    /// chip). Used only to size save-file import/export; the protocol
    /// logic itself doesn't need to know this ahead of time.
    detected_size: Option<usize>,
}

impl Eeprom {
    pub fn new() -> Eeprom {
        Eeprom {
            storage: vec![0xFF; STORAGE_SIZE],
            phase: EepromPhase::Idle,
            pending_len: 0,
            is_write_request: false,
            address_bits_expected: 0,
            bits_received: 0,
            command: 0,
            address: 0,
            write_buffer: 0,
            read_queue: VecDeque::new(),
            detected_size: None,
        }
    }

    /// Overwrites the backing storage with previously saved data (e.g. a
    /// .sav file loaded from disk). Shorter files are placed at the start
    /// and the remainder stays erased (0xFF); longer files are truncated.
    pub fn import_bytes(&mut self, data: &[u8]) {
        let len = data.len().min(self.storage.len());
        self.storage[..len].copy_from_slice(&data[..len]);
        if len > 0 {
            self.detected_size = Some(len);
        }
        log::info!("EEPROM: loaded {} bytes of existing save data", len);
    }

    /// Returns the backing storage, sized to whatever chip size has been
    /// observed so far (falling back to the full 8KB upper bound if no
    /// request has been made yet, e.g. on a completely fresh save).
    pub fn export_bytes(&self) -> Vec<u8> {
        let len = self.detected_size.unwrap_or(self.storage.len());
        self.storage[..len].to_vec()
    }

    /// Called by DMA right before it streams `halfword_count` values from
    /// somewhere in memory into the EEPROM address window (opcode+address,
    /// or opcode+address+data for a write request).
    pub fn prepare_write_transfer(&mut self, halfword_count: u32) {
        self.pending_len = halfword_count;
        self.phase = EepromPhase::Idle;
        self.bits_received = 0;
        self.command = 0;
    }

    /// Called by DMA right before it streams `halfword_count` values out of
    /// the EEPROM address window into memory (the data read-back following
    /// an earlier read request).
    pub fn prepare_read_transfer(&mut self, halfword_count: u32) {
        if self.phase != EepromPhase::AwaitingRead {
            return;
        }

        let dummy_bits = halfword_count.saturating_sub(DATA_BITS);
        self.read_queue.clear();
        for _ in 0..dummy_bits {
            self.read_queue.push_back(false);
        }

        let offset = (self.address as usize) * DATA_BYTES;
        let mut block_bytes = [0u8; DATA_BYTES];
        for i in 0..DATA_BYTES {
            let byte = self.storage[(offset + i) % self.storage.len()];
            block_bytes[i] = byte;
            for bit in (0..8).rev() {
                self.read_queue.push_back(((byte >> bit) & 1) != 0);
            }
        }

        // TEMP DIAGNOSTIC: the actual 8 data bytes served, not just which
        // block — Minish Cap reads each save slot's data twice, from two
        // block addresses 0x200 apart (a primary copy and a mirrored backup
        // copy), and treats the slot as corrupt if they don't match. A
        // "served fine, no protocol error" read can still hand back the
        // wrong bytes if the write path stored the two copies
        // inconsistently; this line makes that visible by dumping the raw
        // data so the primary/mirror pairs can be diffed directly.
        log::info!(
            "EEPROM: read request served for block {:#06X} ({} dummy bits, 64 data bits) data={:02X?}",
            self.address, dummy_bits, block_bytes
        );
        self.phase = EepromPhase::Reading;
    }

    /// Bit 0 of `value` is the serial bit being clocked into the chip.
    pub fn write_bit(&mut self, value: u16) {
        let bit = (value & 1) != 0;

        match self.phase {
            EepromPhase::Idle => {
                self.command = bit as u32;
                self.bits_received = 1;
                self.phase = EepromPhase::Command;
            }
            EepromPhase::Command => {
                self.command = (self.command << 1) | (bit as u32);
                self.bits_received += 1;
                if self.bits_received == 2 {
                    self.is_write_request = self.command == 0b10;
                    let remaining = self.pending_len.saturating_sub(2);
                    self.address_bits_expected = if self.is_write_request {
                        remaining.saturating_sub(DATA_BITS + 1)
                    } else {
                        remaining.saturating_sub(1)
                    };
                    if self.address_bits_expected == 6 || self.address_bits_expected == 14 {
                        let bytes = if self.address_bits_expected == 6 { 512 } else { 8192 };
                        if self.detected_size != Some(bytes) {
                            log::info!("EEPROM: detected a {}-byte chip ({}-bit addressing)", bytes, self.address_bits_expected);
                        }
                        self.detected_size = Some(bytes);
                    } else {
                        log::warn!(
                            "EEPROM: request declared {} halfwords, which decodes to {} address bits (expected 6 or 14) — DMA transfer length may be wrong",
                            self.pending_len, self.address_bits_expected
                        );
                    }
                    self.address = 0;
                    self.bits_received = 0;
                    self.phase = EepromPhase::Address;
                }
            }
            EepromPhase::Address => {
                self.address = (self.address << 1) | (bit as u32);
                self.bits_received += 1;
                if self.bits_received >= self.address_bits_expected {
                    self.bits_received = 0;
                    self.phase = if self.is_write_request {
                        self.write_buffer = 0;
                        EepromPhase::WriteData
                    } else {
                        EepromPhase::WriteStop
                    };
                }
            }
            EepromPhase::WriteData => {
                self.write_buffer = (self.write_buffer << 1) | (bit as u64);
                self.bits_received += 1;
                if self.bits_received >= DATA_BITS {
                    self.commit_write();
                    self.phase = EepromPhase::WriteStop;
                }
            }
            EepromPhase::WriteStop => {
                self.phase = if self.is_write_request {
                    EepromPhase::Idle
                } else {
                    EepromPhase::AwaitingRead
                };
            }
            EepromPhase::AwaitingRead | EepromPhase::Reading => {
                // The chip is driving the bus during a read; stray writes
                // from software during this window are not meaningful.
            }
        }
    }

    /// Bit 0 of the returned value is the serial bit the chip is driving.
    /// Outside of an active read (including status polls right after a
    /// write), the real chip has no write latency to model here, so it
    /// always reports "ready" (1) — returning 0 by default would look
    /// like a permanently busy/failed chip to any game that polls this
    /// bit after a save write before proceeding.
    pub fn read_bit(&mut self) -> u16 {
        match self.read_queue.pop_front() {
            Some(bit) => {
                if self.read_queue.is_empty() {
                    self.phase = EepromPhase::Idle;
                }
                bit as u16
            }
            None => 1,
        }
    }

    fn commit_write(&mut self) {
        let offset = (self.address as usize) * DATA_BYTES;
        for i in 0..DATA_BYTES {
            let shift = (DATA_BYTES - 1 - i) * 8;
            let byte = ((self.write_buffer >> shift) & 0xFF) as u8;
            let idx = (offset + i) % self.storage.len();
            self.storage[idx] = byte;
        }
        log::info!(
            "EEPROM: committed write to block {:#06X} data={:016X}",
            self.address, self.write_buffer
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_request(eeprom: &mut Eeprom, address: u32, address_bits: u32, data: u64) {
        let halfword_count = 2 + address_bits + DATA_BITS + 1;
        eeprom.prepare_write_transfer(halfword_count);

        eeprom.write_bit(1);
        eeprom.write_bit(0);

        for i in (0..address_bits).rev() {
            eeprom.write_bit(((address >> i) & 1) as u16);
        }

        for i in (0..DATA_BITS).rev() {
            eeprom.write_bit(((data >> i) & 1) as u16);
        }

        eeprom.write_bit(0);
    }

    fn read_request(eeprom: &mut Eeprom, address: u32, address_bits: u32) -> u64 {
        let request_len = 2 + address_bits + 1;
        eeprom.prepare_write_transfer(request_len);

        eeprom.write_bit(1);
        eeprom.write_bit(1);

        for i in (0..address_bits).rev() {
            eeprom.write_bit(((address >> i) & 1) as u16);
        }

        eeprom.write_bit(0);

        let readback_len = 4 + DATA_BITS;
        eeprom.prepare_read_transfer(readback_len);

        for _ in 0..4 {
            eeprom.read_bit();
        }

        let mut data: u64 = 0;
        for _ in 0..DATA_BITS {
            data = (data << 1) | (eeprom.read_bit() as u64);
        }
        data
    }

    #[test]
    fn round_trip_narrow_chip() {
        let mut eeprom = Eeprom::new();
        write_request(&mut eeprom, 5, 6, 0xDEADBEEF12345678);
        let data = read_request(&mut eeprom, 5, 6);
        assert_eq!(data, 0xDEADBEEF12345678);
    }

    #[test]
    fn round_trip_wide_chip() {
        let mut eeprom = Eeprom::new();
        write_request(&mut eeprom, 1000, 14, 0x1122334455667788);
        let data = read_request(&mut eeprom, 1000, 14);
        assert_eq!(data, 0x1122334455667788);
    }

    #[test]
    fn distinct_addresses_do_not_alias() {
        let mut eeprom = Eeprom::new();
        write_request(&mut eeprom, 0, 6, 0x1111111111111111);
        write_request(&mut eeprom, 1, 6, 0x2222222222222222);

        assert_eq!(read_request(&mut eeprom, 0, 6), 0x1111111111111111);
        assert_eq!(read_request(&mut eeprom, 1, 6), 0x2222222222222222);
    }

    #[test]
    fn ready_bit_defaults_to_ready_outside_an_active_read() {
        // A game polling for write-completion status right after a save
        // (or reading with no request pending at all) must see "ready"
        // (1), not a permanently "busy" chip.
        let mut eeprom = Eeprom::new();
        assert_eq!(eeprom.read_bit(), 1);

        write_request(&mut eeprom, 2, 6, 0xAAAABBBBCCCCDDDD);
        assert_eq!(eeprom.read_bit(), 1);
    }

    #[test]
    fn import_then_export_round_trips_existing_save_data() {
        let mut eeprom = Eeprom::new();
        let mut save_file = vec![0u8; 512];
        save_file[0] = 0xAB;
        save_file[511] = 0xCD;

        eeprom.import_bytes(&save_file);
        assert_eq!(eeprom.export_bytes(), save_file);

        // Imported data must actually be readable through the real
        // protocol, not just visible via export_bytes.
        let data = read_request(&mut eeprom, 0, 6);
        assert_eq!(data.to_be_bytes()[0], 0xAB);
    }
}
