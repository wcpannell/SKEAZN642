#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Baud Rate Register: High"]
    pub bdh: crate::Reg<bdh::BDH_SPEC>,
    #[doc = "0x01 - UART Baud Rate Register: Low"]
    pub bdl: crate::Reg<bdl::BDL_SPEC>,
    #[doc = "0x02 - UART Control Register 1"]
    pub c1: crate::Reg<c1::C1_SPEC>,
    #[doc = "0x03 - UART Control Register 2"]
    pub c2: crate::Reg<c2::C2_SPEC>,
    #[doc = "0x04 - UART Status Register 1"]
    pub s1: crate::Reg<s1::S1_SPEC>,
    #[doc = "0x05 - UART Status Register 2"]
    pub s2: crate::Reg<s2::S2_SPEC>,
    #[doc = "0x06 - UART Control Register 3"]
    pub c3: crate::Reg<c3::C3_SPEC>,
    #[doc = "0x07 - UART Data Register"]
    pub d: crate::Reg<d::D_SPEC>,
}
#[doc = "BDH register accessor: an alias for `Reg<BDH_SPEC>`"]
pub type BDH = crate::Reg<bdh::BDH_SPEC>;
#[doc = "UART Baud Rate Register: High"]
pub mod bdh;
#[doc = "BDL register accessor: an alias for `Reg<BDL_SPEC>`"]
pub type BDL = crate::Reg<bdl::BDL_SPEC>;
#[doc = "UART Baud Rate Register: Low"]
pub mod bdl;
#[doc = "C1 register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "UART Control Register 1"]
pub mod c1;
#[doc = "C2 register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "UART Control Register 2"]
pub mod c2;
#[doc = "S1 register accessor: an alias for `Reg<S1_SPEC>`"]
pub type S1 = crate::Reg<s1::S1_SPEC>;
#[doc = "UART Status Register 1"]
pub mod s1;
#[doc = "S2 register accessor: an alias for `Reg<S2_SPEC>`"]
pub type S2 = crate::Reg<s2::S2_SPEC>;
#[doc = "UART Status Register 2"]
pub mod s2;
#[doc = "C3 register accessor: an alias for `Reg<C3_SPEC>`"]
pub type C3 = crate::Reg<c3::C3_SPEC>;
#[doc = "UART Control Register 3"]
pub mod c3;
#[doc = "D register accessor: an alias for `Reg<D_SPEC>`"]
pub type D = crate::Reg<d::D_SPEC>;
#[doc = "UART Data Register"]
pub mod d;
