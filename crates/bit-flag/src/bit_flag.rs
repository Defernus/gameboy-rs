pub trait BitFlagRegister: From<u8> + Into<u8> + Copy {
    fn set_flag(&mut self, flag: u8, value: bool);
    fn get_flag(&self, flag: u8) -> bool;
}
