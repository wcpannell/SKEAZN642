#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
    pub plasc: crate::Reg<plasc::PLASC_SPEC>,
    #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
    pub plamc: crate::Reg<plamc::PLAMC_SPEC>,
    #[doc = "0x0c - Platform Control Register"]
    pub placr: crate::Reg<placr::PLACR_SPEC>,
}
#[doc = "PLASC register accessor: an alias for `Reg<PLASC_SPEC>`"]
pub type PLASC = crate::Reg<plasc::PLASC_SPEC>;
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "PLAMC register accessor: an alias for `Reg<PLAMC_SPEC>`"]
pub type PLAMC = crate::Reg<plamc::PLAMC_SPEC>;
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "PLACR register accessor: an alias for `Reg<PLACR_SPEC>`"]
pub type PLACR = crate::Reg<placr::PLACR_SPEC>;
#[doc = "Platform Control Register"]
pub mod placr;
