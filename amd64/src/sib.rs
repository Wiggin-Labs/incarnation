use Register;

/// Represents the SIB byte.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub struct SIB(u8);
deref!(SIB, u8);

impl SIB {
    pub const fn new() -> Self {
        // We default to an index of `none`
        SIB(0b00_100_000)
    }

    /// Set base bits.
    pub fn base(self, value: u8) -> Self {
        assert!(value < 8);
        SIB(self.0 | value)
    }

    pub fn base_reg(self, register: Register) -> Self {
        self.base(register.value())
    }


    /// Set index bits.
    pub fn index(self, value: u8) -> Self {
        assert!(value < 8);
        SIB(self.0 | value << 3)
    }

    /// Set scale bits
    pub fn scale(self, value: u8) -> Self {
        assert!(value < 4);
        SIB(self.0 | value << 6)
    }
}
