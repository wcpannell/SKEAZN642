#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Pin Request Status and Control Register"]
    pub sc: crate::Reg<sc::SC_SPEC>,
}
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "Interrupt Pin Request Status and Control Register"]
pub mod sc;
