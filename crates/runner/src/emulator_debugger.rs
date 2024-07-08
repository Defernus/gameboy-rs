use emulator::*;
use sqlite::Connection;

const CREATE_TABLE_QUERY: &str = r#"
    CREATE TABLE instructions (
        id INTEGER PRIMARY KEY,
        machine_cycle INTEGER NOT NULL,

        opcode INTEGER NOT NULL,
        decoded_instruction TEXT NOT NULL,

        ime_flag INTEGER NOT NULL,

        ie_v_blank INTEGER NOT NULL,
        ie_lcd INTEGER NOT NULL,
        ie_timer INTEGER NOT NULL,
        ie_serial INTEGER NOT NULL,

        if_v_blank INTEGER NOT NULL,
        if_lcd INTEGER NOT NULL,
        if_timer INTEGER NOT NULL,
        if_serial INTEGER NOT NULL,

        a INTEGER NOT NULL,
        bc TEXT NOT NULL,
        de TEXT NOT NULL,
        hl TEXT NOT NULL,

        flag_zero INTEGER NOT NULL,
        flag_subtract INTEGER NOT NULL,
        flag_half_carry INTEGER NOT NULL,
        flag_carry INTEGER NOT NULL
    )
"#;

pub struct SqliteDebugger {
    pub db: Connection,
}

impl SqliteDebugger {
    pub fn new(path: &str) -> Result<Self, DebuggerNewError> {
        // remove old db if file exists
        std::fs::remove_file(path).ok();

        let db = Connection::open(path).map_err(DebuggerNewError::SqliteError)?;

        db.execute(CREATE_TABLE_QUERY)
            .map_err(DebuggerNewError::CreateTableError)?;

        Ok(Self { db })
    }
}

impl EmulatorDebugger for SqliteDebugger {
    fn on_after_instruction(
        &mut self,
        emulator: &emulator::Emulator,
        opcode: u8,
        instruction: emulator::Instruction,
    ) {
        let machine_cycle = emulator.cycles;
        let ime_flag = emulator.ime_flag as i32;

        let reg_ie = emulator.reg::<RegisterIE>();
        let ie_v_blank = reg_ie.get_v_blank() as i32;
        let ie_lcd = reg_ie.get_lcd() as i32;
        let ie_timer = reg_ie.get_timer() as i32;
        let ie_serial = reg_ie.get_serial() as i32;

        let reg_if = emulator.reg::<emulator::RegisterIF>();
        let if_v_blank = reg_if.get_v_blank() as i32;
        let if_lcd = reg_if.get_lcd() as i32;
        let if_timer = reg_if.get_timer() as i32;
        let if_serial = reg_if.get_serial() as i32;

        let a = emulator.accumulator_and_flags.high();
        let f = emulator.accumulator_and_flags.low();
        let bc = format!("{:04X}", emulator.register_bc.as_u16());
        let de = format!("{:04X}", emulator.register_de.as_u16());
        let hl = format!("{:04X}", emulator.register_hl.as_u16());

        let flag_zero = f & FLAG_ZERO;
        let flag_subtract = f & FLAG_SUBTRACT;
        let flag_half_carry = f & FLAG_HALF_CARRY;
        let flag_carry = f & FLAG_CARRY;

        self.db
            .execute(format!(
                r#"
                    INSERT INTO instructions (
                        machine_cycle,
                        opcode,
                        decoded_instruction,
                        ime_flag,
                        ie_v_blank,
                        ie_lcd,
                        ie_timer,
                        ie_serial,
                        if_v_blank,
                        if_lcd,
                        if_timer,
                        if_serial,
                        a,
                        bc,
                        de,
                        hl,
                        flag_zero,
                        flag_subtract,
                        flag_half_carry,
                        flag_carry
                    ) VALUES (
                        {machine_cycle},
                        {opcode},
                        '{instruction:?}',
                        {ime_flag},
                        {ie_v_blank},
                        {ie_lcd},
                        {ie_timer},
                        {ie_serial},
                        {if_v_blank},
                        {if_lcd},
                        {if_timer},
                        {if_serial},
                        {a},
                        '{bc}',
                        '{de}',
                        '{hl}',
                        {flag_zero},
                        {flag_subtract},
                        {flag_half_carry},
                        {flag_carry}
                    )
                "#,
            ))
            .unwrap();
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DebuggerNewError {
    #[error("Sqlite error: {0}")]
    SqliteError(sqlite::Error),
    #[error("Failed to create table: {0}")]
    CreateTableError(sqlite::Error),
}
