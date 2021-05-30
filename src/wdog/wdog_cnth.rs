#[doc = "Register `CNTH` reader"]
pub struct R(crate::R<WDOG_CNTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_CNTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDOG_CNTH_SPEC>> for R {
    fn from(reader: crate::R<WDOG_CNTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNTHIGH` reader - High byte of the Watchdog Counter"]
pub struct CNTHIGH_R(crate::FieldReader<u8, u8>);
impl CNTHIGH_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTHIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTHIGH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - High byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cnthigh(&self) -> CNTHIGH_R {
        CNTHIGH_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Watchdog Counter Register: High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_cnth](index.html) module"]
pub struct WDOG_CNTH_SPEC;
impl crate::RegisterSpec for WDOG_CNTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wdog_cnth::R](R) reader structure"]
impl crate::Readable for WDOG_CNTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CNTH to value 0"]
impl crate::Resettable for WDOG_CNTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
