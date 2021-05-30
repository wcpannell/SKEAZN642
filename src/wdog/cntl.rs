#[doc = "Register `CNTL` reader"]
pub struct R(crate::R<CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CNTL_SPEC>> for R {
    fn from(reader: crate::R<CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNTLOW` reader - Low byte of the Watchdog Counter"]
pub struct CNTLOW_R(crate::FieldReader<u8, u8>);
impl CNTLOW_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTLOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cntlow(&self) -> CNTLOW_R {
        CNTLOW_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Watchdog Counter Register: Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl](index.html) module"]
pub struct CNTL_SPEC;
impl crate::RegisterSpec for CNTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cntl::R](R) reader structure"]
impl crate::Readable for CNTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CNTL to value 0"]
impl crate::Resettable for CNTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
