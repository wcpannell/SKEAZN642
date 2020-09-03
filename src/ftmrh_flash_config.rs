#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Backdoor Comparison Key 0"]
    pub backkey0: BACKKEY0,
    #[doc = "0x01 - Backdoor Comparison Key 1"]
    pub backkey1: BACKKEY1,
    #[doc = "0x02 - Backdoor Comparison Key 2"]
    pub backkey2: BACKKEY2,
    #[doc = "0x03 - Backdoor Comparison Key 3"]
    pub backkey3: BACKKEY3,
    #[doc = "0x04 - Backdoor Comparison Key 4"]
    pub backkey4: BACKKEY4,
    #[doc = "0x05 - Backdoor Comparison Key 5"]
    pub backkey5: BACKKEY5,
    #[doc = "0x06 - Backdoor Comparison Key 6"]
    pub backkey6: BACKKEY6,
    #[doc = "0x07 - Backdoor Comparison Key 7"]
    pub backkey7: BACKKEY7,
    _reserved8: [u8; 4usize],
    #[doc = "0x0c - Non-volatile E-Flash Protection Register"]
    pub eeprot: EEPROT,
    #[doc = "0x0d - Non-volatile P-Flash Protection Register"]
    pub fprot: FPROT,
    #[doc = "0x0e - Non-volatile Flash Security Register"]
    pub fsec: FSEC,
    #[doc = "0x0f - Non-volatile Flash Option Register"]
    pub fopt: FOPT,
}
#[doc = "Backdoor Comparison Key 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backkey0](backkey0) module"]
pub type BACKKEY0 = crate::Reg<u8, _BACKKEY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BACKKEY0;
#[doc = "`read()` method returns [backkey0::R](backkey0::R) reader structure"]
impl crate::Readable for BACKKEY0 {}
#[doc = "Backdoor Comparison Key 0"]
pub mod backkey0;
#[doc = "Backdoor Comparison Key 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backkey1](backkey1) module"]
pub type BACKKEY1 = crate::Reg<u8, _BACKKEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BACKKEY1;
#[doc = "`read()` method returns [backkey1::R](backkey1::R) reader structure"]
impl crate::Readable for BACKKEY1 {}
#[doc = "Backdoor Comparison Key 1"]
pub mod backkey1;
#[doc = "Backdoor Comparison Key 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backkey2](backkey2) module"]
pub type BACKKEY2 = crate::Reg<u8, _BACKKEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BACKKEY2;
#[doc = "`read()` method returns [backkey2::R](backkey2::R) reader structure"]
impl crate::Readable for BACKKEY2 {}
#[doc = "Backdoor Comparison Key 2"]
pub mod backkey2;
#[doc = "Backdoor Comparison Key 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backkey3](backkey3) module"]
pub type BACKKEY3 = crate::Reg<u8, _BACKKEY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BACKKEY3;
#[doc = "`read()` method returns [backkey3::R](backkey3::R) reader structure"]
impl crate::Readable for BACKKEY3 {}
#[doc = "Backdoor Comparison Key 3"]
pub mod backkey3;
#[doc = "Backdoor Comparison Key 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backkey4](backkey4) module"]
pub type BACKKEY4 = crate::Reg<u8, _BACKKEY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BACKKEY4;
#[doc = "`read()` method returns [backkey4::R](backkey4::R) reader structure"]
impl crate::Readable for BACKKEY4 {}
#[doc = "Backdoor Comparison Key 4"]
pub mod backkey4;
#[doc = "Backdoor Comparison Key 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backkey5](backkey5) module"]
pub type BACKKEY5 = crate::Reg<u8, _BACKKEY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BACKKEY5;
#[doc = "`read()` method returns [backkey5::R](backkey5::R) reader structure"]
impl crate::Readable for BACKKEY5 {}
#[doc = "Backdoor Comparison Key 5"]
pub mod backkey5;
#[doc = "Backdoor Comparison Key 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backkey6](backkey6) module"]
pub type BACKKEY6 = crate::Reg<u8, _BACKKEY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BACKKEY6;
#[doc = "`read()` method returns [backkey6::R](backkey6::R) reader structure"]
impl crate::Readable for BACKKEY6 {}
#[doc = "Backdoor Comparison Key 6"]
pub mod backkey6;
#[doc = "Backdoor Comparison Key 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backkey7](backkey7) module"]
pub type BACKKEY7 = crate::Reg<u8, _BACKKEY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BACKKEY7;
#[doc = "`read()` method returns [backkey7::R](backkey7::R) reader structure"]
impl crate::Readable for BACKKEY7 {}
#[doc = "Backdoor Comparison Key 7"]
pub mod backkey7;
#[doc = "Non-volatile E-Flash Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeprot](eeprot) module"]
pub type EEPROT = crate::Reg<u8, _EEPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEPROT;
#[doc = "`read()` method returns [eeprot::R](eeprot::R) reader structure"]
impl crate::Readable for EEPROT {}
#[doc = "Non-volatile E-Flash Protection Register"]
pub mod eeprot;
#[doc = "Non-volatile P-Flash Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot](fprot) module"]
pub type FPROT = crate::Reg<u8, _FPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPROT;
#[doc = "`read()` method returns [fprot::R](fprot::R) reader structure"]
impl crate::Readable for FPROT {}
#[doc = "Non-volatile P-Flash Protection Register"]
pub mod fprot;
#[doc = "Non-volatile Flash Security Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsec](fsec) module"]
pub type FSEC = crate::Reg<u8, _FSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSEC;
#[doc = "`read()` method returns [fsec::R](fsec::R) reader structure"]
impl crate::Readable for FSEC {}
#[doc = "Non-volatile Flash Security Register"]
pub mod fsec;
#[doc = "Non-volatile Flash Option Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fopt](fopt) module"]
pub type FOPT = crate::Reg<u8, _FOPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FOPT;
#[doc = "`read()` method returns [fopt::R](fopt::R) reader structure"]
impl crate::Readable for FOPT {}
#[doc = "Non-volatile Flash Option Register"]
pub mod fopt;
