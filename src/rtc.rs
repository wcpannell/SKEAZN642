#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Status and Control Register"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x04 - RTC Modulo Register"]
    pub mod_: crate::Reg<mod_::MOD_SPEC>,
    #[doc = "0x08 - RTC Counter Register"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
}
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "RTC Status and Control Register"]
pub mod sc;
#[doc = "MOD register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "RTC Modulo Register"]
pub mod mod_;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "RTC Counter Register"]
pub mod cnt;
