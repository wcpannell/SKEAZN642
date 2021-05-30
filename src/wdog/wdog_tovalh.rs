#[doc = "Register `TOVALH` reader"]
pub struct R(crate::R<WDOG_TOVALH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_TOVALH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDOG_TOVALH_SPEC>> for R {
    fn from(reader: crate::R<WDOG_TOVALH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOVALH` writer"]
pub struct W(crate::W<WDOG_TOVALH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_TOVALH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<WDOG_TOVALH_SPEC>> for W {
    fn from(writer: crate::W<WDOG_TOVALH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOVALHIGH` reader - High byte of the timeout value"]
pub struct TOVALHIGH_R(crate::FieldReader<u8, u8>);
impl TOVALHIGH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOVALHIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOVALHIGH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOVALHIGH` writer - High byte of the timeout value"]
pub struct TOVALHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVALHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - High byte of the timeout value"]
    #[inline(always)]
    pub fn tovalhigh(&self) -> TOVALHIGH_R {
        TOVALHIGH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High byte of the timeout value"]
    #[inline(always)]
    pub fn tovalhigh(&mut self) -> TOVALHIGH_W {
        TOVALHIGH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timeout Value Register: High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_tovalh](index.html) module"]
pub struct WDOG_TOVALH_SPEC;
impl crate::RegisterSpec for WDOG_TOVALH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wdog_tovalh::R](R) reader structure"]
impl crate::Readable for WDOG_TOVALH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_tovalh::W](W) writer structure"]
impl crate::Writable for WDOG_TOVALH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOVALH to value 0"]
impl crate::Resettable for WDOG_TOVALH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
