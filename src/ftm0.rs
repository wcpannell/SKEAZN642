#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status And Control"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x04 - Counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x08 - Modulo"]
    pub mod_: crate::Reg<mod_::MOD_SPEC>,
    #[doc = "0x0c - Channel (n) Status And Control"]
    pub c0sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x10 - Channel (n) Value"]
    pub c0v: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x14 - Channel (n) Status And Control"]
    pub c1sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x18 - Channel (n) Value"]
    pub c1v: crate::Reg<cv::CV_SPEC>,
}
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "Status And Control"]
pub mod sc;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "MOD register accessor: an alias for `Reg<MOD_SPEC>`"]
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
#[doc = "Modulo"]
pub mod mod_;
#[doc = "CSC register accessor: an alias for `Reg<CSC_SPEC>`"]
pub type CSC = crate::Reg<csc::CSC_SPEC>;
#[doc = "Channel (n) Status And Control"]
pub mod csc;
#[doc = "CV register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Channel (n) Value"]
pub mod cv;
