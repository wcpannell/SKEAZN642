#[doc = "Register `TOVAL` reader"]
pub struct R(crate::R<WDOG_TOVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_TOVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDOG_TOVAL_SPEC>> for R {
    fn from(reader: crate::R<WDOG_TOVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOVAL` writer"]
pub struct W(crate::W<WDOG_TOVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_TOVAL_SPEC>;
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
impl core::convert::From<crate::W<WDOG_TOVAL_SPEC>> for W {
    fn from(writer: crate::W<WDOG_TOVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOVAL` reader - Watchdog Timeout Value"]
pub struct TOVAL_R(crate::FieldReader<u16, u16>);
impl TOVAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOVAL` writer - Watchdog Timeout Value"]
pub struct TOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Watchdog Timeout Value"]
    #[inline(always)]
    pub fn toval(&self) -> TOVAL_R {
        TOVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog Timeout Value"]
    #[inline(always)]
    pub fn toval(&mut self) -> TOVAL_W {
        TOVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDOG_TOVAL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_toval](index.html) module"]
pub struct WDOG_TOVAL_SPEC;
impl crate::RegisterSpec for WDOG_TOVAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wdog_toval::R](R) reader structure"]
impl crate::Readable for WDOG_TOVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_toval::W](W) writer structure"]
impl crate::Writable for WDOG_TOVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOVAL to value 0"]
impl crate::Resettable for WDOG_TOVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
