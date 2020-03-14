//! Address-Size Override: changes the default address size of a memory or register operand.
//!
//! As far as I know, this is always 0x67 and is used with 32bit addresses.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub struct ASO(u8);
deref!(ASO, u8);

impl ASO {
    pub fn new() -> Self {
        ASO(0x67)
    }
}
