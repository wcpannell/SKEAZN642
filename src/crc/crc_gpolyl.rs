#[doc = "Register `GPOLYL` reader"]
pub struct R(crate::R<CRC_GPOLYL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_GPOLYL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CRC_GPOLYL_SPEC>> for R {
    fn from(reader: crate::R<CRC_GPOLYL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPOLYL` writer"]
pub struct W(crate::W<CRC_GPOLYL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_GPOLYL_SPEC>;
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
impl core::convert::From<crate::W<CRC_GPOLYL_SPEC>> for W {
    fn from(writer: crate::W<CRC_GPOLYL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPOLYL` reader - POLYL stores the lower 16 bits of the 16/32 bit CRC polynomial value"]
pub struct GPOLYL_R(crate::FieldReader<u16, u16>);
impl GPOLYL_R {
    pub(crate) fn new(bits: u16) -> Self {
        GPOLYL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPOLYL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPOLYL` writer - POLYL stores the lower 16 bits of the 16/32 bit CRC polynomial value"]
pub struct GPOLYL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOLYL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - POLYL stores the lower 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline(always)]
    pub fn gpolyl(&self) -> GPOLYL_R {
        GPOLYL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - POLYL stores the lower 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline(always)]
    pub fn gpolyl(&mut self) -> GPOLYL_W {
        GPOLYL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_GPOLYL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_gpolyl](index.html) module"]
pub struct CRC_GPOLYL_SPEC;
impl crate::RegisterSpec for CRC_GPOLYL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc_gpolyl::R](R) reader structure"]
impl crate::Readable for CRC_GPOLYL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_gpolyl::W](W) writer structure"]
impl crate::Writable for CRC_GPOLYL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPOLYL to value 0xffff"]
impl crate::Resettable for CRC_GPOLYL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
