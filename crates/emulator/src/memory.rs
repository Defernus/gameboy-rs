use crate::*;

pub const MEMORY_SIZE: usize = 0x10000;

/// 16 KiB ROM bank 00
///
/// From cartridge, usually a fixed bank
pub const ROM_BANK_00_RANGE: std::ops::Range<usize> = 0x0000..0x4000;
pub const ROM_BANK_00_SIZE: usize = 0x4000;

/// 16 KiB ROM Bank 01–NN
///
/// From cartridge, switchable bank via mapper (if any)
pub const ROM_BANK_01_RANGE: std::ops::Range<usize> = 0x4000..0x8000;
pub const ROM_BANK_01_SIZE: usize = 0x8000 - 0x4000;

/// 8 KiB Video RAM (VRAM)
///
/// In CGB mode, switchable bank 0/1
pub const VRAM_RANGE: std::ops::Range<usize> = 0x8000..0xA000;
pub const VRAM_SIZE: usize = 0xA000 - 0x8000;

/// 8 KiB External RAM
///
/// From cartridge, switchable bank if any
pub const EXTERNAL_RAM_RANGE: std::ops::Range<usize> = 0xA000..0xC000;
pub const EXTERNAL_RAM_SIZE: usize = 0xC000 - 0xA000;

/// 4 KiB Work RAM (WRAM)
pub const WORK_RAM_0_RANGE: std::ops::Range<usize> = 0xC000..0xD000;
pub const WORK_RAM_0_SIZE: usize = 0xD000 - 0xC000;

/// 4 KiB Work RAM (WRAM) In CGB mode, switchable bank 1–7
pub const WORK_RAM_1_RANGE: std::ops::Range<usize> = 0xD000..0xE000;
pub const WORK_RAM_1_SIZE: usize = 0xE000 - 0xD000;

/// Echo RAM (mirror of C000–DDFF)
///
/// Nintendo says use of this area is prohibited.
pub const ECHO_RAM_RANGE: std::ops::Range<usize> = 0xE000..0xFE00;
pub const ECHO_RAM_SIZE: usize = 0xFE00 - 0xE000;

/// Object attribute memory (OAM)
pub const OAM_RANGE: std::ops::Range<usize> = 0xFE00..0xFEA0;
pub const OAM_SIZE: usize = 0xFEA0 - 0xFE00;

/// Not Usable.
///
/// Nintendo says use of this area is prohibited.
pub const NOT_USABLE_RANGE: std::ops::Range<usize> = 0xFEA0..0xFF00;
pub const NOT_USABLE_SIZE: usize = 0xFF00 - 0xFEA0;

/// I/O Registers
pub const IO_REGISTERS_RANGE: std::ops::Range<usize> = 0xFF00..0xFF80;
pub const IO_REGISTERS_SIZE: usize = 0xFF80 - 0xFF00;

/// High RAM (HRAM)
pub const HIGH_RAM_RANGE: std::ops::Range<usize> = 0xFF80..0xFFFF;
pub const HIGH_RAM_SIZE: usize = 0xFFFF - 0xFF80;

/// Interrupt Enable register (IE)
pub const INTERRUPT_ENABLE_REGISTER_RANGE: std::ops::Range<usize> = 0xFFFF..0x10000;
pub const INTERRUPT_ENABLE_REGISTER_SIZE: usize = 1;

// TODO add I/O range

pub struct Memory {
    pub rom_bank_00: Box<[u8; ROM_BANK_00_SIZE]>,
    pub rom_bank_01: Box<[u8; ROM_BANK_01_SIZE]>,
    pub vram: Box<[u8; VRAM_SIZE]>,
    pub external_ram: Box<[u8; EXTERNAL_RAM_SIZE]>,
    pub work_ram_0: Box<[u8; WORK_RAM_0_SIZE]>,
    pub work_ram_1: Box<[u8; WORK_RAM_1_SIZE]>,
    pub oam: Box<[u8; OAM_SIZE]>,
    pub not_usable: Box<[u8; NOT_USABLE_SIZE]>,
    pub io_registers: Box<[u8; IO_REGISTERS_SIZE]>,
    pub high_ram: Box<[u8; HIGH_RAM_SIZE]>,
    pub interrupt_enable_register: u8,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            rom_bank_00: Box::new([0; ROM_BANK_00_SIZE]),
            rom_bank_01: Box::new([0; ROM_BANK_01_SIZE]),
            vram: Box::new([0; VRAM_SIZE]),
            external_ram: Box::new([0; EXTERNAL_RAM_SIZE]),
            work_ram_0: Box::new([0; WORK_RAM_0_SIZE]),
            work_ram_1: Box::new([0; WORK_RAM_1_SIZE]),
            oam: Box::new([0; OAM_SIZE]),
            not_usable: Box::new([0; NOT_USABLE_SIZE]),
            io_registers: Box::new([0; IO_REGISTERS_SIZE]),
            high_ram: Box::new([0; HIGH_RAM_SIZE]),
            interrupt_enable_register: 0,
        }
    }

    pub fn load_rom(&mut self, rom: &mut Rom) {
        self.rom_bank_00 = Box::new(
            rom.read_range(ROM_BANK_00_RANGE)
                .try_into()
                .expect("Invalid ROM bank 00 size"),
        );

        self.rom_bank_01 = Box::new(
            rom.read_range(ROM_BANK_01_RANGE)
                .try_into()
                .expect("Invalid ROM bank 01 size"),
        );
    }

    #[inline(always)]
    pub fn get(&self, address: u16) -> u8 {
        let index = address as usize;
        if ROM_BANK_00_RANGE.contains(&(index)) {
            return self.rom_bank_00[index];
        }

        if ROM_BANK_01_RANGE.contains(&(index)) {
            return self.rom_bank_01[index - ROM_BANK_01_RANGE.start];
        }

        if VRAM_RANGE.contains(&(index)) {
            return self.vram[index - VRAM_RANGE.start];
        }

        if EXTERNAL_RAM_RANGE.contains(&(index)) {
            return self.external_ram[index - EXTERNAL_RAM_RANGE.start];
        }

        if WORK_RAM_0_RANGE.contains(&(index)) {
            return self.work_ram_0[index - WORK_RAM_0_RANGE.start];
        }

        if WORK_RAM_1_RANGE.contains(&(index)) {
            return self.work_ram_1[index - WORK_RAM_1_RANGE.start];
        }

        if ECHO_RAM_RANGE.contains(&(index)) {
            return self.work_ram_0[index - ECHO_RAM_RANGE.start];
        }

        if OAM_RANGE.contains(&(index)) {
            return self.oam[index - OAM_RANGE.start];
        }

        if NOT_USABLE_RANGE.contains(&(index)) {
            return self.not_usable[index - NOT_USABLE_RANGE.start];
        }

        if IO_REGISTERS_RANGE.contains(&(index)) {
            return self.io_registers[index - IO_REGISTERS_RANGE.start];
        }

        if HIGH_RAM_RANGE.contains(&(index)) {
            return self.high_ram[index - HIGH_RAM_RANGE.start];
        }

        if INTERRUPT_ENABLE_REGISTER_RANGE.contains(&(index)) {
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

    #[inline(always)]
    pub fn get_mut(&mut self, address: u16) -> &mut u8 {
        let index = address as usize;
        if ROM_BANK_00_RANGE.contains(&(index)) {
            return &mut self.rom_bank_00[index];
        }

        if ROM_BANK_01_RANGE.contains(&(index)) {
            return &mut self.rom_bank_01[index - ROM_BANK_01_RANGE.start];
        }

        if VRAM_RANGE.contains(&(index)) {
            return &mut self.vram[index - VRAM_RANGE.start];
        }

        if EXTERNAL_RAM_RANGE.contains(&(index)) {
            return &mut self.external_ram[index - EXTERNAL_RAM_RANGE.start];
        }

        if WORK_RAM_0_RANGE.contains(&(index)) {
            return &mut self.work_ram_0[index - WORK_RAM_0_RANGE.start];
        }

        if WORK_RAM_1_RANGE.contains(&(index)) {
            return &mut self.work_ram_1[index - WORK_RAM_1_RANGE.start];
        }

        if ECHO_RAM_RANGE.contains(&(index)) {
            return &mut self.work_ram_0[index - ECHO_RAM_RANGE.start];
        }

        if OAM_RANGE.contains(&(index)) {
            return &mut self.oam[index - OAM_RANGE.start];
        }

        if NOT_USABLE_RANGE.contains(&(index)) {
            return &mut self.not_usable[index - NOT_USABLE_RANGE.start];
        }

        if IO_REGISTERS_RANGE.contains(&(index)) {
            return &mut self.io_registers[index - IO_REGISTERS_RANGE.start];
        }

        if HIGH_RAM_RANGE.contains(&(index)) {
            return &mut self.high_ram[index - HIGH_RAM_RANGE.start];
        }

        if INTERRUPT_ENABLE_REGISTER_RANGE.contains(&(index)) {
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
