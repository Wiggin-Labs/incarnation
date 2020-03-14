use Register;

/// Represents the ModR/M byte.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub struct ModRM(u8);
deref!(ModRM, u8);

impl ModRM {
    pub const fn new() -> Self {
        ModRM(0)
    }

    // TODO: make const when if is allowed
    pub fn mod_(self, value: u8) -> Self {
        assert!(value < 4);
        ModRM(self.0 | value << 6)
    }

    pub fn mod_direct(self) -> Self {
        self.mod_(0b11)
    }

    // TODO: can this be 0b10 as well?
    pub fn mod_indirect(self) -> Self {
        self.mod_(0b01)
    }

    pub fn set_mod_indirect(&mut self) {
        *self = self.mod_indirect();
    }

    // TODO: make const when if is allowed
    pub fn rm(self, value: u8) -> Self {
        assert!(value < 8);
        ModRM(self.0 | value)
    }

    pub fn rm_reg(self, register: Register) -> Self {
        self.rm(register.value())
    }

    pub fn rm_addr(self, register: Register) -> Self {
        self.rm(register.value())
    }

    // TODO: make const when if is allowed
    pub fn reg(self, value: u8) -> Self {
        assert!(value < 8);
        ModRM(self.0 | value << 3)
    }

    pub fn reg_reg(self, register: Register) -> Self {
        self.reg(register.value())
    }

    pub fn reg_addr(self, register: Register) -> Self {
        self.reg(register.value())
    }

    pub fn sibp(&self) -> bool {
        (self.0 & (0b11 << 6)) != (0b11 << 6) &&
        (self.0 & 0b100) == 0b100
    }
}
