#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control Register 1"]
    pub c1: crate::Reg<c1::C1_SPEC>,
    #[doc = "0x01 - SPI Control Register 2"]
    pub c2: crate::Reg<c2::C2_SPEC>,
    #[doc = "0x02 - SPI Baud Rate Register"]
    pub br: crate::Reg<br::BR_SPEC>,
    #[doc = "0x03 - SPI Status Register"]
    pub s: crate::Reg<s::S_SPEC>,
    _reserved4: [u8; 1usize],
    #[doc = "0x05 - SPI Data Register"]
    pub d: crate::Reg<d::D_SPEC>,
    _reserved5: [u8; 1usize],
    #[doc = "0x07 - SPI Match Register"]
    pub m: crate::Reg<m::M_SPEC>,
}
#[doc = "C1 register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "SPI Control Register 1"]
pub mod c1;
#[doc = "C2 register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "SPI Control Register 2"]
pub mod c2;
#[doc = "BR register accessor: an alias for `Reg<BR_SPEC>`"]
pub type BR = crate::Reg<br::BR_SPEC>;
#[doc = "SPI Baud Rate Register"]
pub mod br;
#[doc = "S register accessor: an alias for `Reg<S_SPEC>`"]
pub type S = crate::Reg<s::S_SPEC>;
#[doc = "SPI Status Register"]
pub mod s;
#[doc = "D register accessor: an alias for `Reg<D_SPEC>`"]
pub type D = crate::Reg<d::D_SPEC>;
#[doc = "SPI Data Register"]
pub mod d;
#[doc = "M register accessor: an alias for `Reg<M_SPEC>`"]
pub type M = crate::Reg<m::M_SPEC>;
#[doc = "SPI Match Register"]
pub mod m;
