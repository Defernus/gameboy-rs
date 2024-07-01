pub const ROM_RANGE_ENTRYPOINT: std::ops::Range<usize> = 0x0100..0x0104;
pub const ROM_RANGE_LOGO: std::ops::Range<usize> = 0x0104..0x0134;
pub const ROM_RANGE_TITLE: std::ops::Range<usize> = 0x0134..0x13E;
pub const ROM_RANGE_MANUFACTURER_CODE: std::ops::Range<usize> = 0x13F..0x143;
pub const ROM_ADDRESS_CGB_FLAG: usize = 0x143;
pub const ROM_RANGE_NEW_LICENSEE_CODE: std::ops::Range<usize> = 0x144..0x146;
pub const ROM_ADDRESS_SGB_FLAG: usize = 0x146;
pub const ROM_ADDRESS_CARTRIDGE_TYPE: usize = 0x147;
pub const ROM_ADDRESS_ROM_SIZE: usize = 0x148;
pub const ROM_ADDRESS_RAM_SIZE: usize = 0x149;
pub const ROM_ADDRESS_DESTINATION_CODE: usize = 0x14A;
pub const ROM_ADDRESS_OLD_LICENSEE_CODE: usize = 0x14B;
pub const ROM_ADDRESS_MASK_ROM_VERSION_NUMBER: usize = 0x14C;
pub const ROM_ADDRESS_HEADER_CHECKSUM: usize = 0x14D;
pub const ROM_RANGE_GLOBAL_CHECKSUM: std::ops::Range<usize> = 0x14E..0x150;

// TODO add mappers
#[derive(Default)]
pub struct Rom {
    pub data: Vec<u8>,
}

impl Rom {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn read_range(&mut self, range: std::ops::Range<usize>) -> &[u8] {
        &self.data[range]
    }
}

impl<T> From<T> for Rom
where
    T: Into<Vec<u8>>,
{
    fn from(data: T) -> Self {
        Self::from_bytes(data.into())
    }
}
