use crate::*;

pub const V_BLANK_INTERRUPT: u16 = 0x0040;
pub const LCD_STAT_INTERRUPT: u16 = 0x0048;
pub const TIMER_INTERRUPT: u16 = 0x0050;
pub const SERIAL_INTERRUPT: u16 = 0x0058;
pub const JOYPAD_INTERRUPT: u16 = 0x0060;

impl Emulator {
    pub fn process_interrupt(&mut self) {
        if !self.ime_flag {
            return;
        }

        let reg_if = self.reg::<RegisterIF>().clone();
        let reg_ie = self.reg_mut::<RegisterIE>();

        let interrupt_handler = if reg_ie.get_v_blank() && reg_if.get_v_blank() {
            reg_ie.set_v_blank(false);
            V_BLANK_INTERRUPT
        } else if reg_ie.get_lcd() && reg_if.get_lcd() {
            reg_ie.set_lcd(false);
            LCD_STAT_INTERRUPT
        } else if reg_ie.get_timer() && reg_if.get_timer() {
            reg_ie.set_timer(false);
            TIMER_INTERRUPT
        } else if reg_ie.get_serial() && reg_if.get_serial() {
            reg_ie.set_serial(false);
            SERIAL_INTERRUPT
        } else if reg_ie.get_joypad() && reg_if.get_joypad() {
            reg_ie.set_joypad(false);
            JOYPAD_INTERRUPT
        } else {
            return;
        };

        self.ime_flag = false;
        self.push_to_stack(self.program_counter.0);
        self.program_counter.0 = interrupt_handler;
    }
}
