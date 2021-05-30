#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_datahu: [u8; 4usize],
    _reserved_1_gpolyhu: [u8; 4usize],
    _reserved_2_ctrl: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x00 - CRC_DATAL register."]
    #[inline(always)]
    pub fn crc_datal(&self) -> &crate::Reg<crc_datal::CRC_DATAL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<crc_datal::CRC_DATAL_SPEC>)
        }
    }
    #[doc = "0x00 - CRC Data register"]
    #[inline(always)]
    pub fn crc_data(&self) -> &crate::Reg<crc_data::CRC_DATA_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<crc_data::CRC_DATA_SPEC>)
        }
    }
    #[doc = "0x00 - CRC_DATALL register."]
    #[inline(always)]
    pub fn crc_datall(&self) -> &crate::Reg<crc_datall::CRC_DATALL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<crc_datall::CRC_DATALL_SPEC>)
        }
    }
    #[doc = "0x01 - CRC_DATALU register."]
    #[inline(always)]
    pub fn datalu(&self) -> &crate::Reg<datalu::DATALU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1usize)
                as *const crate::Reg<datalu::DATALU_SPEC>)
        }
    }
    #[doc = "0x02 - CRC_DATAH register."]
    #[inline(always)]
    pub fn crc_datah(&self) -> &crate::Reg<crc_datah::CRC_DATAH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2usize)
                as *const crate::Reg<crc_datah::CRC_DATAH_SPEC>)
        }
    }
    #[doc = "0x02 - CRC_DATAHL register."]
    #[inline(always)]
    pub fn crc_datahl(&self) -> &crate::Reg<crc_datahl::CRC_DATAHL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2usize)
                as *const crate::Reg<crc_datahl::CRC_DATAHL_SPEC>)
        }
    }
    #[doc = "0x03 - CRC_DATAHU register."]
    #[inline(always)]
    pub fn datahu(&self) -> &crate::Reg<datahu::DATAHU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(3usize)
                as *const crate::Reg<datahu::DATAHU_SPEC>)
        }
    }
    #[doc = "0x04 - CRC_GPOLYL register."]
    #[inline(always)]
    pub fn crc_gpolyl(&self) -> &crate::Reg<crc_gpolyl::CRC_GPOLYL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<crc_gpolyl::CRC_GPOLYL_SPEC>)
        }
    }
    #[doc = "0x04 - CRC_GPOLYLL register."]
    #[inline(always)]
    pub fn crc_gpolyll(&self) -> &crate::Reg<crc_gpolyll::CRC_GPOLYLL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<crc_gpolyll::CRC_GPOLYLL_SPEC>)
        }
    }
    #[doc = "0x04 - CRC Polynomial register"]
    #[inline(always)]
    pub fn crc_gpoly(&self) -> &crate::Reg<crc_gpoly::CRC_GPOLY_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<crc_gpoly::CRC_GPOLY_SPEC>)
        }
    }
    #[doc = "0x05 - CRC_GPOLYLU register."]
    #[inline(always)]
    pub fn gpolylu(&self) -> &crate::Reg<gpolylu::GPOLYLU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5usize)
                as *const crate::Reg<gpolylu::GPOLYLU_SPEC>)
        }
    }
    #[doc = "0x06 - CRC_GPOLYHL register."]
    #[inline(always)]
    pub fn crc_gpolyhl(&self) -> &crate::Reg<crc_gpolyhl::CRC_GPOLYHL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(6usize)
                as *const crate::Reg<crc_gpolyhl::CRC_GPOLYHL_SPEC>)
        }
    }
    #[doc = "0x06 - CRC_GPOLYH register."]
    #[inline(always)]
    pub fn crc_gpolyh(&self) -> &crate::Reg<crc_gpolyh::CRC_GPOLYH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(6usize)
                as *const crate::Reg<crc_gpolyh::CRC_GPOLYH_SPEC>)
        }
    }
    #[doc = "0x07 - CRC_GPOLYHU register."]
    #[inline(always)]
    pub fn gpolyhu(&self) -> &crate::Reg<gpolyhu::GPOLYHU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(7usize)
                as *const crate::Reg<gpolyhu::GPOLYHU_SPEC>)
        }
    }
    #[doc = "0x08 - CRC Control register"]
    #[inline(always)]
    pub fn ctrl(&self) -> &crate::Reg<ctrl::CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<ctrl::CTRL_SPEC>)
        }
    }
    #[doc = "0x0b - CRC_CTRLHU register."]
    #[inline(always)]
    pub fn ctrlhu(&self) -> &crate::Reg<ctrlhu::CTRLHU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(11usize)
                as *const crate::Reg<ctrlhu::CTRLHU_SPEC>)
        }
    }
}
#[doc = "CRC_DATALL register accessor: an alias for `Reg<CRC_DATALL_SPEC>`"]
pub type CRC_DATALL = crate::Reg<crc_datall::CRC_DATALL_SPEC>;
#[doc = "CRC_DATALL register."]
pub mod crc_datall;
#[doc = "CRC_DATA register accessor: an alias for `Reg<CRC_DATA_SPEC>`"]
pub type CRC_DATA = crate::Reg<crc_data::CRC_DATA_SPEC>;
#[doc = "CRC Data register"]
pub mod crc_data;
#[doc = "CRC_DATAL register accessor: an alias for `Reg<CRC_DATAL_SPEC>`"]
pub type CRC_DATAL = crate::Reg<crc_datal::CRC_DATAL_SPEC>;
#[doc = "CRC_DATAL register."]
pub mod crc_datal;
#[doc = "DATALU register accessor: an alias for `Reg<DATALU_SPEC>`"]
pub type DATALU = crate::Reg<datalu::DATALU_SPEC>;
#[doc = "CRC_DATALU register."]
pub mod datalu;
#[doc = "CRC_DATAHL register accessor: an alias for `Reg<CRC_DATAHL_SPEC>`"]
pub type CRC_DATAHL = crate::Reg<crc_datahl::CRC_DATAHL_SPEC>;
#[doc = "CRC_DATAHL register."]
pub mod crc_datahl;
#[doc = "CRC_DATAH register accessor: an alias for `Reg<CRC_DATAH_SPEC>`"]
pub type CRC_DATAH = crate::Reg<crc_datah::CRC_DATAH_SPEC>;
#[doc = "CRC_DATAH register."]
pub mod crc_datah;
#[doc = "DATAHU register accessor: an alias for `Reg<DATAHU_SPEC>`"]
pub type DATAHU = crate::Reg<datahu::DATAHU_SPEC>;
#[doc = "CRC_DATAHU register."]
pub mod datahu;
#[doc = "CRC_GPOLY register accessor: an alias for `Reg<CRC_GPOLY_SPEC>`"]
pub type CRC_GPOLY = crate::Reg<crc_gpoly::CRC_GPOLY_SPEC>;
#[doc = "CRC Polynomial register"]
pub mod crc_gpoly;
#[doc = "CRC_GPOLYLL register accessor: an alias for `Reg<CRC_GPOLYLL_SPEC>`"]
pub type CRC_GPOLYLL = crate::Reg<crc_gpolyll::CRC_GPOLYLL_SPEC>;
#[doc = "CRC_GPOLYLL register."]
pub mod crc_gpolyll;
#[doc = "CRC_GPOLYL register accessor: an alias for `Reg<CRC_GPOLYL_SPEC>`"]
pub type CRC_GPOLYL = crate::Reg<crc_gpolyl::CRC_GPOLYL_SPEC>;
#[doc = "CRC_GPOLYL register."]
pub mod crc_gpolyl;
#[doc = "GPOLYLU register accessor: an alias for `Reg<GPOLYLU_SPEC>`"]
pub type GPOLYLU = crate::Reg<gpolylu::GPOLYLU_SPEC>;
#[doc = "CRC_GPOLYLU register."]
pub mod gpolylu;
#[doc = "CRC_GPOLYH register accessor: an alias for `Reg<CRC_GPOLYH_SPEC>`"]
pub type CRC_GPOLYH = crate::Reg<crc_gpolyh::CRC_GPOLYH_SPEC>;
#[doc = "CRC_GPOLYH register."]
pub mod crc_gpolyh;
#[doc = "CRC_GPOLYHL register accessor: an alias for `Reg<CRC_GPOLYHL_SPEC>`"]
pub type CRC_GPOLYHL = crate::Reg<crc_gpolyhl::CRC_GPOLYHL_SPEC>;
#[doc = "CRC_GPOLYHL register."]
pub mod crc_gpolyhl;
#[doc = "GPOLYHU register accessor: an alias for `Reg<GPOLYHU_SPEC>`"]
pub type GPOLYHU = crate::Reg<gpolyhu::GPOLYHU_SPEC>;
#[doc = "CRC_GPOLYHU register."]
pub mod gpolyhu;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CRC Control register"]
pub mod ctrl;
#[doc = "CTRLHU register accessor: an alias for `Reg<CTRLHU_SPEC>`"]
pub type CTRLHU = crate::Reg<ctrlhu::CTRLHU_SPEC>;
#[doc = "CRC_CTRLHU register."]
pub mod ctrlhu;
