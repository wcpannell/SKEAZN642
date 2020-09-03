#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status and ID Register"]
    pub srsid: SRSID,
    #[doc = "0x04 - System Options Register"]
    pub sopt: SOPT,
    #[doc = "0x08 - Pin Selection Register"]
    pub pinsel: PINSEL,
    #[doc = "0x0c - System Clock Gating Control Register"]
    pub scgc: SCGC,
    #[doc = "0x10 - Universally Unique Identifier Low Register"]
    pub uuidl: UUIDL,
    #[doc = "0x14 - Universally Unique Identifier High Register"]
    pub uuidh: UUIDH,
    #[doc = "0x18 - BUS Clock Divider Register"]
    pub busdiv: BUSDIV,
}
#[doc = "System Reset Status and ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srsid](srsid) module"]
pub type SRSID = crate::Reg<u32, _SRSID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSID;
#[doc = "`read()` method returns [srsid::R](srsid::R) reader structure"]
impl crate::Readable for SRSID {}
#[doc = "System Reset Status and ID Register"]
pub mod srsid;
#[doc = "System Options Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt](sopt) module"]
pub type SOPT = crate::Reg<u32, _SOPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT;
#[doc = "`read()` method returns [sopt::R](sopt::R) reader structure"]
impl crate::Readable for SOPT {}
#[doc = "`write(|w| ..)` method takes [sopt::W](sopt::W) writer structure"]
impl crate::Writable for SOPT {}
#[doc = "System Options Register"]
pub mod sopt;
#[doc = "Pin Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel](pinsel) module"]
pub type PINSEL = crate::Reg<u32, _PINSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL;
#[doc = "`read()` method returns [pinsel::R](pinsel::R) reader structure"]
impl crate::Readable for PINSEL {}
#[doc = "`write(|w| ..)` method takes [pinsel::W](pinsel::W) writer structure"]
impl crate::Writable for PINSEL {}
#[doc = "Pin Selection Register"]
pub mod pinsel;
#[doc = "System Clock Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc](scgc) module"]
pub type SCGC = crate::Reg<u32, _SCGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC;
#[doc = "`read()` method returns [scgc::R](scgc::R) reader structure"]
impl crate::Readable for SCGC {}
#[doc = "`write(|w| ..)` method takes [scgc::W](scgc::W) writer structure"]
impl crate::Writable for SCGC {}
#[doc = "System Clock Gating Control Register"]
pub mod scgc;
#[doc = "Universally Unique Identifier Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuidl](uuidl) module"]
pub type UUIDL = crate::Reg<u32, _UUIDL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UUIDL;
#[doc = "`read()` method returns [uuidl::R](uuidl::R) reader structure"]
impl crate::Readable for UUIDL {}
#[doc = "Universally Unique Identifier Low Register"]
pub mod uuidl;
#[doc = "Universally Unique Identifier High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uuidh](uuidh) module"]
pub type UUIDH = crate::Reg<u32, _UUIDH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UUIDH;
#[doc = "`read()` method returns [uuidh::R](uuidh::R) reader structure"]
impl crate::Readable for UUIDH {}
#[doc = "Universally Unique Identifier High Register"]
pub mod uuidh;
#[doc = "BUS Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busdiv](busdiv) module"]
pub type BUSDIV = crate::Reg<u32, _BUSDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSDIV;
#[doc = "`read()` method returns [busdiv::R](busdiv::R) reader structure"]
impl crate::Readable for BUSDIV {}
#[doc = "`write(|w| ..)` method takes [busdiv::W](busdiv::W) writer structure"]
impl crate::Writable for BUSDIV {}
#[doc = "BUS Clock Divider Register"]
pub mod busdiv;
