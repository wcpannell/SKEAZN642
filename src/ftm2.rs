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
    #[doc = "0x1c - Channel (n) Status And Control"]
    pub c2sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x20 - Channel (n) Value"]
    pub c2v: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x24 - Channel (n) Status And Control"]
    pub c3sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x28 - Channel (n) Value"]
    pub c3v: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x2c - Channel (n) Status And Control"]
    pub c4sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x30 - Channel (n) Value"]
    pub c4v: crate::Reg<cv::CV_SPEC>,
    #[doc = "0x34 - Channel (n) Status And Control"]
    pub c5sc: crate::Reg<csc::CSC_SPEC>,
    #[doc = "0x38 - Channel (n) Value"]
    pub c5v: crate::Reg<cv::CV_SPEC>,
    _reserved15: [u8; 16usize],
    #[doc = "0x4c - Counter Initial Value"]
    pub cntin: crate::Reg<cntin::CNTIN_SPEC>,
    #[doc = "0x50 - Capture And Compare Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x54 - Features Mode Selection"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x58 - Synchronization"]
    pub sync: crate::Reg<sync::SYNC_SPEC>,
    #[doc = "0x5c - Initial State For Channels Output"]
    pub outinit: crate::Reg<outinit::OUTINIT_SPEC>,
    #[doc = "0x60 - Output Mask"]
    pub outmask: crate::Reg<outmask::OUTMASK_SPEC>,
    #[doc = "0x64 - Function For Linked Channels"]
    pub combine: crate::Reg<combine::COMBINE_SPEC>,
    #[doc = "0x68 - Deadtime Insertion Control"]
    pub deadtime: crate::Reg<deadtime::DEADTIME_SPEC>,
    #[doc = "0x6c - FTM External Trigger"]
    pub exttrig: crate::Reg<exttrig::EXTTRIG_SPEC>,
    #[doc = "0x70 - Channels Polarity"]
    pub pol: crate::Reg<pol::POL_SPEC>,
    #[doc = "0x74 - Fault Mode Status"]
    pub fms: crate::Reg<fms::FMS_SPEC>,
    #[doc = "0x78 - Input Capture Filter Control"]
    pub filter: crate::Reg<filter::FILTER_SPEC>,
    #[doc = "0x7c - Fault Control"]
    pub fltctrl: crate::Reg<fltctrl::FLTCTRL_SPEC>,
    _reserved28: [u8; 4usize],
    #[doc = "0x84 - Configuration"]
    pub conf: crate::Reg<conf::CONF_SPEC>,
    #[doc = "0x88 - FTM Fault Input Polarity"]
    pub fltpol: crate::Reg<fltpol::FLTPOL_SPEC>,
    #[doc = "0x8c - Synchronization Configuration"]
    pub synconf: crate::Reg<synconf::SYNCONF_SPEC>,
    #[doc = "0x90 - FTM Inverting Control"]
    pub invctrl: crate::Reg<invctrl::INVCTRL_SPEC>,
    #[doc = "0x94 - FTM Software Output Control"]
    pub swoctrl: crate::Reg<swoctrl::SWOCTRL_SPEC>,
    #[doc = "0x98 - FTM PWM Load"]
    pub pwmload: crate::Reg<pwmload::PWMLOAD_SPEC>,
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
#[doc = "CNTIN register accessor: an alias for `Reg<CNTIN_SPEC>`"]
pub type CNTIN = crate::Reg<cntin::CNTIN_SPEC>;
#[doc = "Counter Initial Value"]
pub mod cntin;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Capture And Compare Status"]
pub mod status;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Features Mode Selection"]
pub mod mode;
#[doc = "SYNC register accessor: an alias for `Reg<SYNC_SPEC>`"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "Synchronization"]
pub mod sync;
#[doc = "OUTINIT register accessor: an alias for `Reg<OUTINIT_SPEC>`"]
pub type OUTINIT = crate::Reg<outinit::OUTINIT_SPEC>;
#[doc = "Initial State For Channels Output"]
pub mod outinit;
#[doc = "OUTMASK register accessor: an alias for `Reg<OUTMASK_SPEC>`"]
pub type OUTMASK = crate::Reg<outmask::OUTMASK_SPEC>;
#[doc = "Output Mask"]
pub mod outmask;
#[doc = "COMBINE register accessor: an alias for `Reg<COMBINE_SPEC>`"]
pub type COMBINE = crate::Reg<combine::COMBINE_SPEC>;
#[doc = "Function For Linked Channels"]
pub mod combine;
#[doc = "DEADTIME register accessor: an alias for `Reg<DEADTIME_SPEC>`"]
pub type DEADTIME = crate::Reg<deadtime::DEADTIME_SPEC>;
#[doc = "Deadtime Insertion Control"]
pub mod deadtime;
#[doc = "EXTTRIG register accessor: an alias for `Reg<EXTTRIG_SPEC>`"]
pub type EXTTRIG = crate::Reg<exttrig::EXTTRIG_SPEC>;
#[doc = "FTM External Trigger"]
pub mod exttrig;
#[doc = "POL register accessor: an alias for `Reg<POL_SPEC>`"]
pub type POL = crate::Reg<pol::POL_SPEC>;
#[doc = "Channels Polarity"]
pub mod pol;
#[doc = "FMS register accessor: an alias for `Reg<FMS_SPEC>`"]
pub type FMS = crate::Reg<fms::FMS_SPEC>;
#[doc = "Fault Mode Status"]
pub mod fms;
#[doc = "FILTER register accessor: an alias for `Reg<FILTER_SPEC>`"]
pub type FILTER = crate::Reg<filter::FILTER_SPEC>;
#[doc = "Input Capture Filter Control"]
pub mod filter;
#[doc = "FLTCTRL register accessor: an alias for `Reg<FLTCTRL_SPEC>`"]
pub type FLTCTRL = crate::Reg<fltctrl::FLTCTRL_SPEC>;
#[doc = "Fault Control"]
pub mod fltctrl;
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configuration"]
pub mod conf;
#[doc = "FLTPOL register accessor: an alias for `Reg<FLTPOL_SPEC>`"]
pub type FLTPOL = crate::Reg<fltpol::FLTPOL_SPEC>;
#[doc = "FTM Fault Input Polarity"]
pub mod fltpol;
#[doc = "SYNCONF register accessor: an alias for `Reg<SYNCONF_SPEC>`"]
pub type SYNCONF = crate::Reg<synconf::SYNCONF_SPEC>;
#[doc = "Synchronization Configuration"]
pub mod synconf;
#[doc = "INVCTRL register accessor: an alias for `Reg<INVCTRL_SPEC>`"]
pub type INVCTRL = crate::Reg<invctrl::INVCTRL_SPEC>;
#[doc = "FTM Inverting Control"]
pub mod invctrl;
#[doc = "SWOCTRL register accessor: an alias for `Reg<SWOCTRL_SPEC>`"]
pub type SWOCTRL = crate::Reg<swoctrl::SWOCTRL_SPEC>;
#[doc = "FTM Software Output Control"]
pub mod swoctrl;
#[doc = "PWMLOAD register accessor: an alias for `Reg<PWMLOAD_SPEC>`"]
pub type PWMLOAD = crate::Reg<pwmload::PWMLOAD_SPEC>;
#[doc = "FTM PWM Load"]
pub mod pwmload;
