#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control Register 1"]
    pub sc1: crate::Reg<sc1::SC1_SPEC>,
    #[doc = "0x04 - Status and Control Register 2"]
    pub sc2: crate::Reg<sc2::SC2_SPEC>,
    #[doc = "0x08 - Status and Control Register 3"]
    pub sc3: crate::Reg<sc3::SC3_SPEC>,
    #[doc = "0x0c - Status and Control Register 4"]
    pub sc4: crate::Reg<sc4::SC4_SPEC>,
    #[doc = "0x10 - Conversion Result Register"]
    pub r: crate::Reg<r::R_SPEC>,
    #[doc = "0x14 - Compare Value Register"]
    pub cv: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x18 - Pin Control 1 Register"]
    pub apctl1: crate::Reg<apctl1::APCTL1_SPEC>,
}
#[doc = "SC1 register accessor: an alias for `Reg<SC1_SPEC>`"]
pub type SC1 = crate::Reg<sc1::SC1_SPEC>;
#[doc = "Status and Control Register 1"]
pub mod sc1;
#[doc = "SC2 register accessor: an alias for `Reg<SC2_SPEC>`"]
pub type SC2 = crate::Reg<sc2::SC2_SPEC>;
#[doc = "Status and Control Register 2"]
pub mod sc2;
#[doc = "SC3 register accessor: an alias for `Reg<SC3_SPEC>`"]
pub type SC3 = crate::Reg<sc3::SC3_SPEC>;
#[doc = "Status and Control Register 3"]
pub mod sc3;
#[doc = "SC4 register accessor: an alias for `Reg<SC4_SPEC>`"]
pub type SC4 = crate::Reg<sc4::SC4_SPEC>;
#[doc = "Status and Control Register 4"]
pub mod sc4;
#[doc = "R register accessor: an alias for `Reg<R_SPEC>`"]
pub type R = crate::Reg<r::R_SPEC>;
#[doc = "Conversion Result Register"]
pub mod r;
#[doc = "CV register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Compare Value Register"]
pub mod cv;
#[doc = "APCTL1 register accessor: an alias for `Reg<APCTL1_SPEC>`"]
pub type APCTL1 = crate::Reg<apctl1::APCTL1_SPEC>;
#[doc = "Pin Control 1 Register"]
pub mod apctl1;
