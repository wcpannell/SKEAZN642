#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - KBI Status and Control Register"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x01 - KBIx Pin Enable Register"]
    pub pe: crate::Reg<pe::PE_SPEC>,
    #[doc = "0x02 - KBIx Edge Select Register"]
    pub es: crate::Reg<es::ES_SPEC>,
}
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "KBI Status and Control Register"]
pub mod sc;
#[doc = "PE register accessor: an alias for `Reg<PE_SPEC>`"]
pub type PE = crate::Reg<pe::PE_SPEC>;
#[doc = "KBIx Pin Enable Register"]
pub mod pe;
#[doc = "ES register accessor: an alias for `Reg<ES_SPEC>`"]
pub type ES = crate::Reg<es::ES_SPEC>;
#[doc = "KBIx Edge Select Register"]
pub mod es;
