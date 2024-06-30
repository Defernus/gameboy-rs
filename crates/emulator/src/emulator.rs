use crate::*;

#[derive(Default)]
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

    pub memory: Memory,

    pub rom: Vec<u8>,

    /// Indicate that IME flag should be set after the next instruction
    pub delayed_ime_set: bool,
    /// IME: Interrupt master enable flag
    pub ime_flag: bool,

    /// Number of cycles that have passed since the CPU was started
    pub cycles: u64,

    pub is_in_low_power_mode: bool,
}

impl Emulator {
    pub fn from_rom(rom: Vec<u8>) -> Self {
        let mut rom = Rom::from_bytes(rom);
        let mut memory = Memory::default();
        memory.load_rom(&mut rom);

        Self {
            memory,
            ..Default::default()
        }
    }

    pub fn read_next_instruction(&mut self) -> Option<Instruction> {
        let (instruction, size) = Instruction::read(&self.memory, self.program_counter)?;

        self.program_counter = self.program_counter.0.wrapping_add(size).into();

        Some(instruction)
    }

    pub fn handle_instruction(&mut self, instruction: Instruction) {
        let set_ime = self.delayed_ime_set;

        let cycles = instruction.execute(self);

        if cycles == 0 {
            panic!("Instruction {:?} failed", instruction);
        }

        self.cycles += cycles as u64;

        if set_ime {
            self.ime_flag = true;
            self.delayed_ime_set = false;
        }
    }

    pub fn handle_next_instruction(&mut self) {
        let instruction = self.read_next_instruction().unwrap_or_else(|| {
            panic!(
                "Failed to read next instruction, opcode: {:02X}",
                self.memory.get(self.program_counter.into())
            )
        });

        self.handle_instruction(instruction);
    }
}
