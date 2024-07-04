use emulator::*;
use std::{fs::File, io::Write};

pub fn init_debug_table(file: &str) {
    let mut file = std::fs::File::create(file).unwrap();
    writeln!(
        file,
        "step,cycle,instruction,opcode,pc,sp,ime,af,bc,de,hl,0x8800 mem"
    )
    .unwrap();
}

pub fn debug_next_instruction(emulator: &mut Emulator, file: &mut File, step: usize) {
    let mut row = String::new();

    // opcode and pc are updated at the end of the previous instruction, so we
    // need to read them before executing the next instruction
    let opcode = emulator.instruction_register;
    let pc = emulator.program_counter.0;

    let instruction = emulator.handle_next_instruction();

    row += &format!("{step},");
    row += &format!("{},", emulator.cycles);
    row += &format!("${:04X},", pc);
    row += &format!("${:04X},", emulator.stack_pointer.0);
    row += &format!("{},", emulator.ime_flag);
    row += &format!("${:04X},", emulator.accumulator_and_flags.as_u16());
    row += &format!("${:04X},", emulator.register_bc.as_u16());
    row += &format!("${:04X},", emulator.register_de.as_u16());
    row += &format!("${:04X},", emulator.register_hl.as_u16());
    row += &format!("${:02X},", opcode);
    row += &format!("\"{:?}\"", instruction);
    let mem = emulator.get(0x8800);
    row += &format!(",${:02X}", mem);

    writeln!(file, "{}", row).unwrap();
}
