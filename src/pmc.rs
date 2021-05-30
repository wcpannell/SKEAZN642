#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Power Management Status and Control 1 Register"]
    pub spmsc1: crate::Reg<spmsc1::SPMSC1_SPEC>,
    #[doc = "0x01 - System Power Management Status and Control 2 Register"]
    pub spmsc2: crate::Reg<spmsc2::SPMSC2_SPEC>,
}
#[doc = "SPMSC1 register accessor: an alias for `Reg<SPMSC1_SPEC>`"]
pub type SPMSC1 = crate::Reg<spmsc1::SPMSC1_SPEC>;
#[doc = "System Power Management Status and Control 1 Register"]
pub mod spmsc1;
#[doc = "SPMSC2 register accessor: an alias for `Reg<SPMSC2_SPEC>`"]
pub type SPMSC2 = crate::Reg<spmsc2::SPMSC2_SPEC>;
#[doc = "System Power Management Status and Control 2 Register"]
pub mod spmsc2;
