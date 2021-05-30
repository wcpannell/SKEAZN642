#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Clock Divider Register"]
    pub fclkdiv: crate::Reg<fclkdiv::FCLKDIV_SPEC>,
    #[doc = "0x01 - Flash Security Register"]
    pub fsec: crate::Reg<fsec::FSEC_SPEC>,
    #[doc = "0x02 - Flash CCOB Index Register"]
    pub fccobix: crate::Reg<fccobix::FCCOBIX_SPEC>,
    _reserved3: [u8; 1usize],
    #[doc = "0x04 - Flash Configuration Register"]
    pub fcnfg: crate::Reg<fcnfg::FCNFG_SPEC>,
    #[doc = "0x05 - Flash Error Configuration Register"]
    pub fercnfg: crate::Reg<fercnfg::FERCNFG_SPEC>,
    #[doc = "0x06 - Flash Status Register"]
    pub fstat: crate::Reg<fstat::FSTAT_SPEC>,
    #[doc = "0x07 - Flash Error Status Register"]
    pub ferstat: crate::Reg<ferstat::FERSTAT_SPEC>,
    #[doc = "0x08 - Flash Protection Register"]
    pub fprot: crate::Reg<fprot::FPROT_SPEC>,
    #[doc = "0x09 - EEPROM Protection Register"]
    pub eeprot: crate::Reg<eeprot::EEPROT_SPEC>,
    #[doc = "0x0a - Flash Common Command Object Register:High"]
    pub fccobhi: crate::Reg<fccobhi::FCCOBHI_SPEC>,
    #[doc = "0x0b - Flash Common Command Object Register: Low"]
    pub fccoblo: crate::Reg<fccoblo::FCCOBLO_SPEC>,
    #[doc = "0x0c - Flash Option Register"]
    pub fopt: crate::Reg<fopt::FOPT_SPEC>,
}
#[doc = "FCLKDIV register accessor: an alias for `Reg<FCLKDIV_SPEC>`"]
pub type FCLKDIV = crate::Reg<fclkdiv::FCLKDIV_SPEC>;
#[doc = "Flash Clock Divider Register"]
pub mod fclkdiv;
#[doc = "FSEC register accessor: an alias for `Reg<FSEC_SPEC>`"]
pub type FSEC = crate::Reg<fsec::FSEC_SPEC>;
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "FCCOBIX register accessor: an alias for `Reg<FCCOBIX_SPEC>`"]
pub type FCCOBIX = crate::Reg<fccobix::FCCOBIX_SPEC>;
#[doc = "Flash CCOB Index Register"]
pub mod fccobix;
#[doc = "FCNFG register accessor: an alias for `Reg<FCNFG_SPEC>`"]
pub type FCNFG = crate::Reg<fcnfg::FCNFG_SPEC>;
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "FERCNFG register accessor: an alias for `Reg<FERCNFG_SPEC>`"]
pub type FERCNFG = crate::Reg<fercnfg::FERCNFG_SPEC>;
#[doc = "Flash Error Configuration Register"]
pub mod fercnfg;
#[doc = "FSTAT register accessor: an alias for `Reg<FSTAT_SPEC>`"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "FERSTAT register accessor: an alias for `Reg<FERSTAT_SPEC>`"]
pub type FERSTAT = crate::Reg<ferstat::FERSTAT_SPEC>;
#[doc = "Flash Error Status Register"]
pub mod ferstat;
#[doc = "FPROT register accessor: an alias for `Reg<FPROT_SPEC>`"]
pub type FPROT = crate::Reg<fprot::FPROT_SPEC>;
#[doc = "Flash Protection Register"]
pub mod fprot;
#[doc = "EEPROT register accessor: an alias for `Reg<EEPROT_SPEC>`"]
pub type EEPROT = crate::Reg<eeprot::EEPROT_SPEC>;
#[doc = "EEPROM Protection Register"]
pub mod eeprot;
#[doc = "FCCOBHI register accessor: an alias for `Reg<FCCOBHI_SPEC>`"]
pub type FCCOBHI = crate::Reg<fccobhi::FCCOBHI_SPEC>;
#[doc = "Flash Common Command Object Register:High"]
pub mod fccobhi;
#[doc = "FCCOBLO register accessor: an alias for `Reg<FCCOBLO_SPEC>`"]
pub type FCCOBLO = crate::Reg<fccoblo::FCCOBLO_SPEC>;
#[doc = "Flash Common Command Object Register: Low"]
pub mod fccoblo;
#[doc = "FOPT register accessor: an alias for `Reg<FOPT_SPEC>`"]
pub type FOPT = crate::Reg<fopt::FOPT_SPEC>;
#[doc = "Flash Option Register"]
pub mod fopt;
