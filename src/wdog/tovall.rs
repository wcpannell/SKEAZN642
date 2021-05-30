#[doc = "Register `TOVALL` reader"]
pub struct R(crate::R<TOVALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOVALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TOVALL_SPEC>> for R {
    fn from(reader: crate::R<TOVALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOVALL` writer"]
pub struct W(crate::W<TOVALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOVALL_SPEC>;
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
impl core::convert::From<crate::W<TOVALL_SPEC>> for W {
    fn from(writer: crate::W<TOVALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOVALLOW` reader - Low byte of the timeout value"]
pub struct TOVALLOW_R(crate::FieldReader<u8, u8>);
impl TOVALLOW_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOVALLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOVALLOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOVALLOW` writer - Low byte of the timeout value"]
pub struct TOVALLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVALLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low byte of the timeout value"]
    #[inline(always)]
    pub fn tovallow(&self) -> TOVALLOW_R {
        TOVALLOW_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of the timeout value"]
    #[inline(always)]
    pub fn tovallow(&mut self) -> TOVALLOW_W {
        TOVALLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timeout Value Register: Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tovall](index.html) module"]
pub struct TOVALL_SPEC;
impl crate::RegisterSpec for TOVALL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tovall::R](R) reader structure"]
impl crate::Readable for TOVALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tovall::W](W) writer structure"]
impl crate::Writable for TOVALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOVALL to value 0x04"]
impl crate::Resettable for TOVALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
