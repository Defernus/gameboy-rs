use crate::*;

pub const MEMORY_SIZE: usize = 0x10000;

/// 16 KiB ROM bank 00
///
/// From cartridge, usually a fixed bank
pub const MEMORY_RANGE_ROM_BANK_00: std::ops::Range<usize> = 0x0000..0x4000;
pub const MEMORY_SIZE_ROM_BANK_00: usize =
    MEMORY_RANGE_ROM_BANK_00.end - MEMORY_RANGE_ROM_BANK_00.start;

/// 16 KiB ROM Bank 01–NN
///
/// From cartridge, switchable bank via mapper (if any)
pub const MEMORY_RANGE_ROM_BANK_01: std::ops::Range<usize> = 0x4000..0x8000;
pub const MEMORY_SIZE_ROM_BANK_01: usize =
    MEMORY_RANGE_ROM_BANK_01.end - MEMORY_RANGE_ROM_BANK_01.start;

/// 8 KiB Video RAM (VRAM)
///
/// In CGB mode, switchable bank 0/1
pub const MEMORY_RANGE_VRAM: std::ops::Range<usize> = 0x8000..0xA000;
pub const MEMORY_SIZE_VRAM: usize = MEMORY_RANGE_VRAM.end - MEMORY_RANGE_VRAM.start;

/// 8 KiB External RAM
///
/// From cartridge, switchable bank if any
pub const MEMORY_RANGE_EXTERNAL_RAM: std::ops::Range<usize> = 0xA000..0xC000;
pub const MEMORY_SIZE_EXTERNAL_RAM: usize =
    MEMORY_RANGE_EXTERNAL_RAM.end - MEMORY_RANGE_EXTERNAL_RAM.start;

/// 4 KiB Work RAM (WRAM)
pub const MEMORY_RANGE_WORK_RAM_0: std::ops::Range<usize> = 0xC000..0xD000;
pub const MEMORY_SIZE_WORK_RAM_0: usize =
    MEMORY_RANGE_WORK_RAM_0.end - MEMORY_RANGE_WORK_RAM_0.start;

/// 4 KiB Work RAM (WRAM) In CGB mode, switchable bank 1–7
pub const MEMORY_RANGE_WORK_RAM_1: std::ops::Range<usize> = 0xD000..0xE000;
pub const MEMORY_SIZE_WORK_RAM_1: usize =
    MEMORY_RANGE_WORK_RAM_1.end - MEMORY_RANGE_WORK_RAM_1.start;

/// Echo RAM (mirror of C000–DDFF)
///
/// Nintendo says use of this area is prohibited.
pub const MEMORY_RANGE_ECHO_RAM: std::ops::Range<usize> = 0xE000..0xFE00;
pub const MEMORY_SIZE_ECHO_RAM: usize = MEMORY_RANGE_ECHO_RAM.end - MEMORY_RANGE_ECHO_RAM.start;

/// Object attribute memory (OAM)
pub const MEMORY_RANGE_OAM: std::ops::Range<usize> = 0xFE00..0xFEA0;
pub const MEMORY_SIZE_OAM: usize = MEMORY_RANGE_OAM.end - MEMORY_RANGE_OAM.start;

/// Not Usable.
///
/// Nintendo says use of this area is prohibited.
pub const MEMORY_RANGE_NOT_USABLE: std::ops::Range<usize> = 0xFEA0..0xFF00;
pub const MEMORY_SIZE_NOT_USABLE: usize =
    MEMORY_RANGE_NOT_USABLE.end - MEMORY_RANGE_NOT_USABLE.start;

/// I/O Registers
pub const MEMORY_RANGE_IO_REGISTERS: std::ops::Range<usize> = 0xFF00..0xFF80;
pub const MEMORY_SIZE_IO_REGISTERS: usize =
    MEMORY_RANGE_IO_REGISTERS.end - MEMORY_RANGE_IO_REGISTERS.start;

/// High RAM (HRAM)
pub const MEMORY_RANGE_HIGH_RAM: std::ops::Range<usize> = 0xFF80..0xFFFF;
pub const MEMORY_SIZE_HIGH_RAM: usize = MEMORY_RANGE_HIGH_RAM.end - MEMORY_RANGE_HIGH_RAM.start;

/// Interrupt Enable register (IE)
pub const MEMORY_ADDRESS_INTERRUPT_ENABLE_REGISTER: usize = 0xFFFF;

pub const MEMORY_RANGE_TILE_INDICES_BANK0: std::ops::Range<usize> = 0x9800..0x9BFF;
pub const MEMORY_RANGE_TILE_INDICES_BANK1: std::ops::Range<usize> = 0x9C00..0x9FFF;

/// Background and Window tiles block 0 (first half of object tiles)
pub const MEMORY_RANGE_TILES_BLOCK0: std::ops::Range<usize> = 0x8000..0x87FF;
/// Background and Window tiles block 1 (second half of object tiles)
pub const MEMORY_RANGE_TILES_BLOCK1: std::ops::Range<usize> = 0x8800..0x8FFF;
/// Background and Window tiles block 2 (can't be used for object tiles)
pub const MEMORY_RANGE_TILES_BLOCK2: std::ops::Range<usize> = 0x9000..0x97FF;

/// Object Tiles
pub const MEMORY_RANGE_OBJECT_TILES: std::ops::Range<usize> = 0x8000..0x8FFF;

// TODO add I/O range

impl Emulator {
    #[inline(always)]
    pub fn get(&self, address: u16) -> u8 {
        let index = address as usize;
        if MEMORY_RANGE_ROM_BANK_00.contains(&index) {
            return self.rom_bank_00[index];
        }

        if MEMORY_RANGE_ROM_BANK_01.contains(&index) {
            return self.rom_bank_01[index - MEMORY_RANGE_ROM_BANK_01.start];
        }

        if MEMORY_RANGE_VRAM.contains(&index) {
            return self.vram[index - MEMORY_RANGE_VRAM.start];
        }

        if MEMORY_RANGE_EXTERNAL_RAM.contains(&index) {
            return self.external_ram[index - MEMORY_RANGE_EXTERNAL_RAM.start];
        }

        if MEMORY_RANGE_WORK_RAM_0.contains(&index) {
            return self.work_ram_0[index - MEMORY_RANGE_WORK_RAM_0.start];
        }

        if MEMORY_RANGE_WORK_RAM_1.contains(&index) {
            return self.work_ram_1[index - MEMORY_RANGE_WORK_RAM_1.start];
        }

        if MEMORY_RANGE_ECHO_RAM.contains(&index) {
            let index = index - MEMORY_RANGE_ECHO_RAM.start;
            if index < MEMORY_SIZE_WORK_RAM_0 {
                return self.work_ram_0[index];
            }
            return self.work_ram_1[index - MEMORY_SIZE_WORK_RAM_0];
        }

        if MEMORY_RANGE_OAM.contains(&index) {
            return self.oam[index - MEMORY_RANGE_OAM.start];
        }

        if MEMORY_RANGE_NOT_USABLE.contains(&index) {
            return self.not_usable[index - MEMORY_RANGE_NOT_USABLE.start];
        }

        if MEMORY_RANGE_IO_REGISTERS.contains(&index) {
            return self.io_registers[index - MEMORY_RANGE_IO_REGISTERS.start];
        }

        if MEMORY_RANGE_HIGH_RAM.contains(&index) {
            return self.high_ram[index - MEMORY_RANGE_HIGH_RAM.start];
        }

        if MEMORY_ADDRESS_INTERRUPT_ENABLE_REGISTER == index {
            return self.interrupt_enable_register;
        }

        panic!("Invalid memory address: {address:#04X}");
    }

    pub fn get_i8(&self, address: u16) -> i8 {
        // Safety: u8 and i8 have the same size and alignment
        unsafe { std::mem::transmute(self.get(address)) }
    }

    pub fn get_u16(&self, address: u16) -> u16 {
        let byte0 = self.get(address);
        let byte1 = self.get(address + 1);

        u16::from_le_bytes([byte0, byte1])
    }

    /// Read a u8 from the memory at the current program counter and advance the program counter by 1.
    pub fn read_u8_at_pc(&mut self) -> u8 {
        let address = self.program_counter.post_increment(1);
        self.get(address)
    }

    /// Read an i8 from the memory at the current program counter and advance the program counter by 1.
    pub fn read_i8_at_pc(&mut self) -> i8 {
        let address = self.program_counter.post_increment(1);
        self.get_i8(address)
    }

    /// Read a u16 from the memory at the current program counter and advance the program counter by 2.
    pub fn read_u16_at_pc(&mut self) -> u16 {
        let address = self.program_counter.post_increment(2);
        self.get_u16(address)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, address: u16) -> &mut u8 {
        let index = address as usize;
        if MEMORY_RANGE_ROM_BANK_00.contains(&index) {
            return &mut self.rom_bank_00[index];
        }

        if MEMORY_RANGE_ROM_BANK_01.contains(&index) {
            return &mut self.rom_bank_01[index - MEMORY_RANGE_ROM_BANK_01.start];
        }

        if MEMORY_RANGE_VRAM.contains(&index) {
            return &mut self.vram[index - MEMORY_RANGE_VRAM.start];
        }

        if MEMORY_RANGE_EXTERNAL_RAM.contains(&index) {
            return &mut self.external_ram[index - MEMORY_RANGE_EXTERNAL_RAM.start];
        }

        if MEMORY_RANGE_WORK_RAM_0.contains(&index) {
            return &mut self.work_ram_0[index - MEMORY_RANGE_WORK_RAM_0.start];
        }

        if MEMORY_RANGE_WORK_RAM_1.contains(&index) {
            return &mut self.work_ram_1[index - MEMORY_RANGE_WORK_RAM_1.start];
        }

        if MEMORY_RANGE_ECHO_RAM.contains(&index) {
            let index = index - MEMORY_RANGE_ECHO_RAM.start;
            if index < MEMORY_SIZE_WORK_RAM_0 {
                return &mut self.work_ram_0[index];
            }
            return &mut self.work_ram_1[index - MEMORY_SIZE_WORK_RAM_0];
        }

        if MEMORY_RANGE_OAM.contains(&index) {
            return &mut self.oam[index - MEMORY_RANGE_OAM.start];
        }

        if MEMORY_RANGE_NOT_USABLE.contains(&index) {
            return &mut self.not_usable[index - MEMORY_RANGE_NOT_USABLE.start];
        }

        if MEMORY_RANGE_IO_REGISTERS.contains(&index) {
            return &mut self.io_registers[index - MEMORY_RANGE_IO_REGISTERS.start];
        }

        if MEMORY_RANGE_HIGH_RAM.contains(&index) {
            return &mut self.high_ram[index - MEMORY_RANGE_HIGH_RAM.start];
        }

        if MEMORY_ADDRESS_INTERRUPT_ENABLE_REGISTER == index {
            return &mut self.interrupt_enable_register;
        }

        panic!("Invalid memory address: {address:#04X}");
    }

    #[inline(always)]
    pub fn set(&mut self, address: u16, value: u8) {
        *self.get_mut(address) = value;
    }
}

pub trait MemoryAccess {
    fn at(&self, emulator: &Emulator) -> u8;
    fn at_mut<'a>(&self, emulator: &'a mut Emulator) -> &'a mut u8;
}

#[test]
fn test_memory_access_at_region_edges() {
    let emulator = Emulator::default();

    const ADDRESS: usize = MEMORY_RANGE_ECHO_RAM.start + 4096;

    emulator.get(ADDRESS as u16);
}
