#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ACMP Control and Status Register"]
    pub cs: crate::Reg<cs::CS_SPEC>,
    #[doc = "0x01 - ACMP Control Register 0"]
    pub c0: crate::Reg<c0::C0_SPEC>,
    #[doc = "0x02 - ACMP Control Register 1"]
    pub c1: crate::Reg<c1::C1_SPEC>,
    #[doc = "0x03 - ACMP Control Register 2"]
    pub c2: crate::Reg<c2::C2_SPEC>,
}
#[doc = "CS register accessor: an alias for `Reg<CS_SPEC>`"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "ACMP Control and Status Register"]
pub mod cs;
#[doc = "C0 register accessor: an alias for `Reg<C0_SPEC>`"]
pub type C0 = crate::Reg<c0::C0_SPEC>;
#[doc = "ACMP Control Register 0"]
pub mod c0;
#[doc = "C1 register accessor: an alias for `Reg<C1_SPEC>`"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "ACMP Control Register 1"]
pub mod c1;
#[doc = "C2 register accessor: an alias for `Reg<C2_SPEC>`"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "ACMP Control Register 2"]
pub mod c2;
