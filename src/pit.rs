#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIT Module Control Register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    _reserved1: [u8; 252usize],
    #[doc = "0x100 - Timer Load Value Register"]
    pub ldval0: crate::Reg<ldval::LDVAL_SPEC>,
    #[doc = "0x104 - Current Timer Value Register"]
    pub cval0: crate::Reg<cval::CVAL_SPEC>,
    #[doc = "0x108 - Timer Control Register"]
    pub tctrl0: crate::Reg<tctrl::TCTRL_SPEC>,
    #[doc = "0x10c - Timer Flag Register"]
    pub tflg0: crate::Reg<tflg::TFLG_SPEC>,
    #[doc = "0x110 - Timer Load Value Register"]
    pub ldval1: crate::Reg<ldval::LDVAL_SPEC>,
    #[doc = "0x114 - Current Timer Value Register"]
    pub cval1: crate::Reg<cval::CVAL_SPEC>,
    #[doc = "0x118 - Timer Control Register"]
    pub tctrl1: crate::Reg<tctrl::TCTRL_SPEC>,
    #[doc = "0x11c - Timer Flag Register"]
    pub tflg1: crate::Reg<tflg::TFLG_SPEC>,
}
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "PIT Module Control Register"]
pub mod mcr;
#[doc = "LDVAL register accessor: an alias for `Reg<LDVAL_SPEC>`"]
pub type LDVAL = crate::Reg<ldval::LDVAL_SPEC>;
#[doc = "Timer Load Value Register"]
pub mod ldval;
#[doc = "CVAL register accessor: an alias for `Reg<CVAL_SPEC>`"]
pub type CVAL = crate::Reg<cval::CVAL_SPEC>;
#[doc = "Current Timer Value Register"]
pub mod cval;
#[doc = "TCTRL register accessor: an alias for `Reg<TCTRL_SPEC>`"]
pub type TCTRL = crate::Reg<tctrl::TCTRL_SPEC>;
#[doc = "Timer Control Register"]
pub mod tctrl;
#[doc = "TFLG register accessor: an alias for `Reg<TFLG_SPEC>`"]
pub type TFLG = crate::Reg<tflg::TFLG_SPEC>;
#[doc = "Timer Flag Register"]
pub mod tflg;
