#[doc = "Register `WINH` reader"]
pub struct R(crate::R<WDOG_WINH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_WINH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDOG_WINH_SPEC>> for R {
    fn from(reader: crate::R<WDOG_WINH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINH` writer"]
pub struct W(crate::W<WDOG_WINH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_WINH_SPEC>;
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
impl core::convert::From<crate::W<WDOG_WINH_SPEC>> for W {
    fn from(writer: crate::W<WDOG_WINH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINHIGH` reader - High byte of Watchdog Window"]
pub struct WINHIGH_R(crate::FieldReader<u8, u8>);
impl WINHIGH_R {
    pub(crate) fn new(bits: u8) -> Self {
        WINHIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINHIGH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINHIGH` writer - High byte of Watchdog Window"]
pub struct WINHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> WINHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - High byte of Watchdog Window"]
    #[inline(always)]
    pub fn winhigh(&self) -> WINHIGH_R {
        WINHIGH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High byte of Watchdog Window"]
    #[inline(always)]
    pub fn winhigh(&mut self) -> WINHIGH_W {
        WINHIGH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Window Register: High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_winh](index.html) module"]
pub struct WDOG_WINH_SPEC;
impl crate::RegisterSpec for WDOG_WINH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wdog_winh::R](R) reader structure"]
impl crate::Readable for WDOG_WINH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_winh::W](W) writer structure"]
impl crate::Writable for WDOG_WINH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WINH to value 0"]
impl crate::Resettable for WDOG_WINH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
