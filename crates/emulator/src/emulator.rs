use crate::*;

pub struct Emulator {
    /// **AF** register
    pub accumulator_and_flags: Register,
    /// **BC** register
    pub register_bc: Register,
    /// **DE** register
    pub register_de: Register,
    /// **HL** register
    pub register_hl: Register,
    /// **SP** register
    pub stack_pointer: StackPointer,
    /// **PC** register
    pub program_counter: ProgramCounter,

    pub rom: Option<Rom>,

    /// Indicate that IME flag should be set after the next instruction
    pub delayed_ime_set: bool,
    /// IME: Interrupt master enable flag
    pub ime_flag: bool,

    /// Number of cycles that have passed since the CPU was started
    pub cycles: usize,

    pub is_in_low_power_mode: bool,

    /// **IR** register. Internal cpu register used to store the opcode of the
    /// next instruction.
    pub instruction_register: u8,

    pub rom_bank_00: Box<[u8; MEMORY_SIZE_ROM_BANK_00]>,
    pub rom_bank_01: Box<[u8; MEMORY_SIZE_ROM_BANK_01]>,
    pub vram: Box<[u8; MEMORY_SIZE_VRAM]>,
    pub external_ram: Box<[u8; MEMORY_SIZE_EXTERNAL_RAM]>,
    pub work_ram_0: Box<[u8; MEMORY_SIZE_WORK_RAM_0]>,
    pub work_ram_1: Box<[u8; MEMORY_SIZE_WORK_RAM_1]>,
    pub oam: Box<[u8; MEMORY_SIZE_OAM]>,
    pub not_usable: Box<[u8; MEMORY_SIZE_NOT_USABLE]>,
    pub io_registers: Box<[u8; MEMORY_SIZE_IO_REGISTERS]>,
    pub high_ram: Box<[u8; MEMORY_SIZE_HIGH_RAM]>,
    pub interrupt_enable_register: u8,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            accumulator_and_flags: Register::default(),
            register_bc: Register::default(),
            register_de: Register::default(),
            register_hl: Register::default(),
            stack_pointer: StackPointer::default(),
            program_counter: ProgramCounter::default(),
            delayed_ime_set: false,
            ime_flag: false,
            rom: None,
            cycles: 0,
            is_in_low_power_mode: false,

            instruction_register: 0,

            rom_bank_00: Box::new([0; MEMORY_SIZE_ROM_BANK_00]),
            rom_bank_01: Box::new([0; MEMORY_SIZE_ROM_BANK_01]),
            vram: Box::new([0; MEMORY_SIZE_VRAM]),
            external_ram: Box::new([0; MEMORY_SIZE_EXTERNAL_RAM]),
            work_ram_0: Box::new([0; MEMORY_SIZE_WORK_RAM_0]),
            work_ram_1: Box::new([0; MEMORY_SIZE_WORK_RAM_1]),
            oam: Box::new([0; MEMORY_SIZE_OAM]),
            not_usable: Box::new([0; MEMORY_SIZE_NOT_USABLE]),
            io_registers: Box::new([0; MEMORY_SIZE_IO_REGISTERS]),
            high_ram: Box::new([0; MEMORY_SIZE_HIGH_RAM]),
            interrupt_enable_register: 0,
        }
    }

    pub fn from_rom(rom: impl Into<Rom>) -> Self {
        let mut rom: Rom = rom.into();
        let mut emulator = Self::new();

        emulator.rom_bank_00 = Box::new(
            rom.read_range(MEMORY_RANGE_ROM_BANK_00)
                .try_into()
                .expect("Invalid ROM bank 00 size"),
        );

        emulator.rom_bank_01 = Box::new(
            rom.read_range(MEMORY_RANGE_ROM_BANK_01)
                .try_into()
                .expect("Invalid ROM bank 01 size"),
        );

        emulator.rom = Some(rom);

        emulator
    }

    /// Handle given instruction without fetch and pc increment
    pub fn handle_instruction(&mut self, instruction: Instruction) {
        let set_ime = self.delayed_ime_set;

        let cycles = instruction.execute(self);

        if cycles == 0 {
            panic!("Instruction {:?} failed", instruction);
        }

        self.cycles += cycles as usize;

        if set_ime {
            self.ime_flag = true;
            self.delayed_ime_set = false;
        }
    }

    /// Handle instruction from instruction register(IR) without fetch and pc
    /// increment
    pub fn handle_next_instruction_pre_fetch(&mut self) -> Instruction {
        let instruction = Instruction::read(self).unwrap_or_else(|| {
            panic!(
                "Failed to read next instruction, opcode: {:02X}",
                self.instruction_register
            )
        });

        self.handle_instruction(instruction);

        instruction
    }

    /// Handle instruction from instruction register(IR) and fetch next
    /// instruction opcode
    ///
    /// Returns the instruction that was executed
    pub fn handle_next_instruction(&mut self) -> Instruction {
        let instruction = self.handle_next_instruction_pre_fetch();

        // fetch next instruction opcode
        self.fetch_opcode();

        instruction
    }

    /// Fetch next instruction opcode, store it in the IR register and increment the PC
    pub fn fetch_opcode(&mut self) {
        self.instruction_register = self.read_u8_at_pc();
    }

    /// Get control register
    pub fn reg<T: ControlRegister>(&self) -> T {
        T::from(self.get(T::ADDRESS))
    }

    /// Get mutable reference to control register
    pub fn reg_mut<T: ControlRegister>(&mut self) -> &mut T {
        T::from_memory_mut(self)
    }
}
