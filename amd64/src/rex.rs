/// Represents the REX prefix.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub struct REX(u8);
deref!(REX, u8);

impl REX {
    pub const fn new() -> Self {
        REX(0b0100_0000)
    }

    /// Set REX.W bit.
    /// Unset means Operand size determined by CS.D.
    /// Set means 64 bit Operand size.
    pub const fn w(self) -> Self {
        REX(self.0 | 0b1000)
    }

    /// Set REX.R bit.
    /// Extension of the ModR/M reg field.
    pub const fn r(self) -> Self {
        REX(self.0 | 0b0100)
    }

    pub fn set_r(&mut self) {
        *self = REX(self.0 | 0b0100);
    }

    /// Set REX.X bit.
    /// Extension of the SIB index field.
    pub const fn x(self) -> Self {
        REX(self.0 | 0b0010)
    }

    /// Set REX.B bit.
    /// Extension of the ModR/M r/m field, SIB base field, or Opcode reg field.
    pub const fn b(self) -> Self {
        REX(self.0 | 0b0001)
    }

    pub fn set_b(&mut self) {
        *self = REX(self.0 | 0b0001);
    }
}
