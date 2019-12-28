//! Operand-Size Override: changes the default operand size of a memory or register operand.
//!
//! As far as I know, this is always 0x66 and is used with 16bit registers.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub struct OSO(u8);
deref!(OSO, u8);

impl OSO {
    pub fn new() -> Self {
        OSO(0x66)
    }
}
