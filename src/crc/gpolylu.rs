#[doc = "Register `GPOLYLU` reader"]
pub struct R(crate::R<GPOLYLU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPOLYLU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPOLYLU_SPEC>> for R {
    fn from(reader: crate::R<GPOLYLU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPOLYLU` writer"]
pub struct W(crate::W<GPOLYLU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPOLYLU_SPEC>;
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
impl core::convert::From<crate::W<GPOLYLU_SPEC>> for W {
    fn from(writer: crate::W<GPOLYLU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPOLYLU` reader - POLYLL stores the second 8 bits of the 32 bit CRC"]
pub struct GPOLYLU_R(crate::FieldReader<u8, u8>);
impl GPOLYLU_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPOLYLU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPOLYLU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPOLYLU` writer - POLYLL stores the second 8 bits of the 32 bit CRC"]
pub struct GPOLYLU_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOLYLU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - POLYLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolylu(&self) -> GPOLYLU_R {
        GPOLYLU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolylu(&mut self) -> GPOLYLU_W {
        GPOLYLU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_GPOLYLU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpolylu](index.html) module"]
pub struct GPOLYLU_SPEC;
impl crate::RegisterSpec for GPOLYLU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gpolylu::R](R) reader structure"]
impl crate::Readable for GPOLYLU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpolylu::W](W) writer structure"]
impl crate::Writable for GPOLYLU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPOLYLU to value 0xff"]
impl crate::Resettable for GPOLYLU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
