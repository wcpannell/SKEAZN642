#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status and ID Register"]
    pub srsid: crate::Reg<srsid::SRSID_SPEC>,
    #[doc = "0x04 - System Options Register"]
    pub sopt: crate::Reg<sopt::SOPT_SPEC>,
    #[doc = "0x08 - Pin Selection Register"]
    pub pinsel: crate::Reg<pinsel::PINSEL_SPEC>,
    #[doc = "0x0c - System Clock Gating Control Register"]
    pub scgc: crate::Reg<scgc::SCGC_SPEC>,
    #[doc = "0x10 - Universally Unique Identifier Low Register"]
    pub uuidl: crate::Reg<uuidl::UUIDL_SPEC>,
    #[doc = "0x14 - Universally Unique Identifier High Register"]
    pub uuidh: crate::Reg<uuidh::UUIDH_SPEC>,
    #[doc = "0x18 - BUS Clock Divider Register"]
    pub busdiv: crate::Reg<busdiv::BUSDIV_SPEC>,
}
#[doc = "SRSID register accessor: an alias for `Reg<SRSID_SPEC>`"]
pub type SRSID = crate::Reg<srsid::SRSID_SPEC>;
#[doc = "System Reset Status and ID Register"]
pub mod srsid;
#[doc = "SOPT register accessor: an alias for `Reg<SOPT_SPEC>`"]
pub type SOPT = crate::Reg<sopt::SOPT_SPEC>;
#[doc = "System Options Register"]
pub mod sopt;
#[doc = "PINSEL register accessor: an alias for `Reg<PINSEL_SPEC>`"]
pub type PINSEL = crate::Reg<pinsel::PINSEL_SPEC>;
#[doc = "Pin Selection Register"]
pub mod pinsel;
#[doc = "SCGC register accessor: an alias for `Reg<SCGC_SPEC>`"]
pub type SCGC = crate::Reg<scgc::SCGC_SPEC>;
#[doc = "System Clock Gating Control Register"]
pub mod scgc;
#[doc = "UUIDL register accessor: an alias for `Reg<UUIDL_SPEC>`"]
pub type UUIDL = crate::Reg<uuidl::UUIDL_SPEC>;
#[doc = "Universally Unique Identifier Low Register"]
pub mod uuidl;
#[doc = "UUIDH register accessor: an alias for `Reg<UUIDH_SPEC>`"]
pub type UUIDH = crate::Reg<uuidh::UUIDH_SPEC>;
#[doc = "Universally Unique Identifier High Register"]
pub mod uuidh;
#[doc = "BUSDIV register accessor: an alias for `Reg<BUSDIV_SPEC>`"]
pub type BUSDIV = crate::Reg<busdiv::BUSDIV_SPEC>;
#[doc = "BUS Clock Divider Register"]
pub mod busdiv;
