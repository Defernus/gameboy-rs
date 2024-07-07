/// Helper struct to work with values with bitness less than 8.
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct BitsValue<const N: usize>(u8);

pub type U2 = BitsValue<2>;
pub type U3 = BitsValue<3>;
pub type U4 = BitsValue<4>;
pub type U5 = BitsValue<5>;
pub type U6 = BitsValue<6>;
pub type U7 = BitsValue<7>;

impl<const N: usize> From<u8> for BitsValue<N> {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<BitsValue<N>> for u8 {
    fn from(value: BitsValue<N>) -> Self {
        value.0 & !(!0 << N)
    }
}

impl<const N: usize> BitsValue<N> {
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn from_bits(bits: [bool; N]) -> Self {
        let mut value = 0;
        for (i, bit) in bits.iter().enumerate() {
            if *bit {
                value |= 1 << i;
            }
        }
        Self(value)
    }

    pub fn to_bits(self) -> [bool; N] {
        let mut bits = [false; N];
        for i in 0..N {
            bits[i] = self.0 & (1 << i) != 0;
        }
        bits
    }
}
