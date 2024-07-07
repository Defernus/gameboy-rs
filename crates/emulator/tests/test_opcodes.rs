use emulator::*;
use serde::Deserialize;
use std::fs::read_to_string;

const TEST_FOLDER: &str = "../../cpu-test-data";

#[test]
#[ignore = "integration test"]
fn test_opcodes() {
    let mut missing_tests = vec![];

    for i in u8::MIN..u8::MAX {
        if execute_tests_for_opcode(i).is_err() {
            missing_tests.push(i);
        }
    }

    assert_eq!(
        missing_tests.len(),
        15,
        "expected only 15 missing opcodes: {:?}",
        missing_tests
    );
}

fn execute_tests_for_opcode(opcode: u8) -> std::io::Result<()> {
    let test_file_name = format!("{opcode:02X}.json").to_lowercase();
    let test_path = format!("{TEST_FOLDER}/{test_file_name}");

    let file = read_to_string(&test_path)?;

    let tests: Vec<TestData> = serde_json::from_str(&file)
        .unwrap_or_else(|err| panic!("failed to parse {test_path}: {err:?}"));

    for (index, test) in tests.into_iter().enumerate() {
        test.execute_test(index, opcode, &test_path);
    }

    Ok(())
}

#[derive(Deserialize)]
struct TestData {
    #[allow(dead_code)]
    name: String,

    #[serde(rename = "initial")]
    initial_state: EmulatorTestState,

    #[serde(rename = "final")]
    final_state: EmulatorTestState,

    cycles: Vec<Option<(u16, u8, Option<CycleAction>)>>,
}

impl TestData {
    fn execute_test(self, index: usize, test_opcode: u8, test_path: &str) {
        let mut emulator = Emulator::default();
        emulator.instruction_register = test_opcode;
        emulator.disable_registers_update = true;

        self.initial_state.set_to(&mut emulator);

        let location = format!("{} [{}]", test_path, index);

        let mut last_cycle = 0;
        let m_cycles_count = self.cycles.len();
        while emulator.cycles < m_cycles_count {
            let opcode = emulator.instruction_register;
            let instruction = emulator.handle_next_instruction();

            let hl = emulator.register_hl.as_u16();
            println!("{instruction:?} ({opcode:02X}) HL: {hl} ({hl:04X})");

            if emulator.cycles > m_cycles_count {
                panic!(
                "unexpected end of test at {}.cycles: M-Cycles expected {}, but CPU executed {}",
                location, m_cycles_count, emulator.cycles
            );
            }
            for _cycle in &self.cycles[last_cycle..emulator.cycles] {
                // TODO check bus activity
            }

            last_cycle = emulator.cycles;
        }

        assert_eq!(
            emulator.cycles, m_cycles_count,
            "expected {} cycles, but CPU executed {}",
            m_cycles_count, emulator.cycles
        );

        self.final_state.check(&emulator, &location);

        println!("{} passed", location);
    }
}

#[derive(Deserialize)]
enum CycleAction {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

#[derive(Deserialize)]
struct EmulatorTestState {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
    ram: Vec<(u16, u8)>,
}

impl EmulatorTestState {
    fn set_to(self, emulator: &mut Emulator) {
        emulator.accumulator_and_flags.set_high(self.a);
        emulator.accumulator_and_flags.set_low(self.f);
        emulator.register_bc.set_high(self.b);
        emulator.register_bc.set_low(self.c);
        emulator.register_de.set_high(self.d);
        emulator.register_de.set_low(self.e);
        emulator.register_hl.set_high(self.h);
        emulator.register_hl.set_low(self.l);
        emulator.program_counter = self.pc.into();
        emulator.stack_pointer = self.sp.into();

        for (address, value) in self.ram {
            emulator.set(address, value);
        }
    }

    fn check(self, emulator: &Emulator, location: &str) {
        assert_eq!(
            emulator.accumulator_and_flags.high(),
            self.a,
            "mismatch value at {location}.final.a",
        );
        assert_eq!(
            emulator.accumulator_and_flags.low(),
            self.f,
            "mismatch value at {location}.final.f",
        );
        assert_eq!(
            emulator.register_bc.high(),
            self.b,
            "mismatch value at {location}.final.b",
        );
        assert_eq!(
            emulator.register_bc.low(),
            self.c,
            "mismatch value at {location}.final.c",
        );
        assert_eq!(
            emulator.register_de.high(),
            self.d,
            "mismatch value at {location}.final.d",
        );
        assert_eq!(
            emulator.register_de.low(),
            self.e,
            "mismatch value at {location}.final.e",
        );
        assert_eq!(
            emulator.register_hl.high(),
            self.h,
            "mismatch value at {location}.final.h",
        );
        assert_eq!(
            emulator.register_hl.low(),
            self.l,
            "mismatch value at {location}.final.l",
        );
        assert_eq!(
            emulator.program_counter.0, self.pc,
            "mismatch value at {location}.final.pc",
        );
        assert_eq!(
            emulator.stack_pointer.0, self.sp,
            "mismatch value at {location}.final.sp",
        );

        for (i, (address, value)) in self.ram.into_iter().enumerate() {
            assert_eq!(
                emulator.get(address),
                value,
                "memory at address {address}({address:04X}) mismatch {location}.final.ram[{i}]"
            );
        }
    }
}
