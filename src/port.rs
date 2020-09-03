#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Filter Register"]
    pub ioflt: IOFLT,
    #[doc = "0x04 - Port Pullup Enable Low Register"]
    pub puel: PUEL,
    #[doc = "0x08 - Port Pullup Enable High Register"]
    pub pueh: PUEH,
    #[doc = "0x0c - Port High Drive Enable Register"]
    pub hdrve: HDRVE,
}
#[doc = "Port Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioflt](ioflt) module"]
pub type IOFLT = crate::Reg<u32, _IOFLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOFLT;
#[doc = "`read()` method returns [ioflt::R](ioflt::R) reader structure"]
impl crate::Readable for IOFLT {}
#[doc = "`write(|w| ..)` method takes [ioflt::W](ioflt::W) writer structure"]
impl crate::Writable for IOFLT {}
#[doc = "Port Filter Register"]
pub mod ioflt;
#[doc = "Port Pullup Enable Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [puel](puel) module"]
pub type PUEL = crate::Reg<u32, _PUEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUEL;
#[doc = "`read()` method returns [puel::R](puel::R) reader structure"]
impl crate::Readable for PUEL {}
#[doc = "`write(|w| ..)` method takes [puel::W](puel::W) writer structure"]
impl crate::Writable for PUEL {}
#[doc = "Port Pullup Enable Low Register"]
pub mod puel;
#[doc = "Port Pullup Enable High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pueh](pueh) module"]
pub type PUEH = crate::Reg<u32, _PUEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUEH;
#[doc = "`read()` method returns [pueh::R](pueh::R) reader structure"]
impl crate::Readable for PUEH {}
#[doc = "`write(|w| ..)` method takes [pueh::W](pueh::W) writer structure"]
impl crate::Writable for PUEH {}
#[doc = "Port Pullup Enable High Register"]
pub mod pueh;
#[doc = "Port High Drive Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdrve](hdrve) module"]
pub type HDRVE = crate::Reg<u32, _HDRVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDRVE;
#[doc = "`read()` method returns [hdrve::R](hdrve::R) reader structure"]
impl crate::Readable for HDRVE {}
#[doc = "`write(|w| ..)` method takes [hdrve::W](hdrve::W) writer structure"]
impl crate::Writable for HDRVE {}
#[doc = "Port High Drive Enable Register"]
pub mod hdrve;
