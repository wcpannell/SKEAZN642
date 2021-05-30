#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Filter Register"]
    pub ioflt: crate::Reg<ioflt::IOFLT_SPEC>,
    #[doc = "0x04 - Port Pullup Enable Low Register"]
    pub puel: crate::Reg<puel::PUEL_SPEC>,
    #[doc = "0x08 - Port Pullup Enable High Register"]
    pub pueh: crate::Reg<pueh::PUEH_SPEC>,
    #[doc = "0x0c - Port High Drive Enable Register"]
    pub hdrve: crate::Reg<hdrve::HDRVE_SPEC>,
}
#[doc = "IOFLT register accessor: an alias for `Reg<IOFLT_SPEC>`"]
pub type IOFLT = crate::Reg<ioflt::IOFLT_SPEC>;
#[doc = "Port Filter Register"]
pub mod ioflt;
#[doc = "PUEL register accessor: an alias for `Reg<PUEL_SPEC>`"]
pub type PUEL = crate::Reg<puel::PUEL_SPEC>;
#[doc = "Port Pullup Enable Low Register"]
pub mod puel;
#[doc = "PUEH register accessor: an alias for `Reg<PUEH_SPEC>`"]
pub type PUEH = crate::Reg<pueh::PUEH_SPEC>;
#[doc = "Port Pullup Enable High Register"]
pub mod pueh;
#[doc = "HDRVE register accessor: an alias for `Reg<HDRVE_SPEC>`"]
pub type HDRVE = crate::Reg<hdrve::HDRVE_SPEC>;
#[doc = "Port High Drive Enable Register"]
pub mod hdrve;
