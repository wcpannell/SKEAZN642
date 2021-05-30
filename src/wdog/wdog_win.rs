#[doc = "Register `WIN` reader"]
pub struct R(crate::R<WDOG_WIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_WIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDOG_WIN_SPEC>> for R {
    fn from(reader: crate::R<WDOG_WIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIN` writer"]
pub struct W(crate::W<WDOG_WIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_WIN_SPEC>;
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
impl core::convert::From<crate::W<WDOG_WIN_SPEC>> for W {
    fn from(writer: crate::W<WDOG_WIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIN` reader - Watchdog Window Value"]
pub struct WIN_R(crate::FieldReader<u16, u16>);
impl WIN_R {
    pub(crate) fn new(bits: u16) -> Self {
        WIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIN` writer - Watchdog Window Value"]
pub struct WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Watchdog Window Value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog Window Value"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W {
        WIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDOG_WIN register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_win](index.html) module"]
pub struct WDOG_WIN_SPEC;
impl crate::RegisterSpec for WDOG_WIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wdog_win::R](R) reader structure"]
impl crate::Readable for WDOG_WIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_win::W](W) writer structure"]
impl crate::Writable for WDOG_WIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIN to value 0"]
impl crate::Resettable for WDOG_WIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
