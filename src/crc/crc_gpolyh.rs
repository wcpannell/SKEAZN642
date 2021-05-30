#[doc = "Register `GPOLYH` reader"]
pub struct R(crate::R<CRC_GPOLYH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_GPOLYH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CRC_GPOLYH_SPEC>> for R {
    fn from(reader: crate::R<CRC_GPOLYH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPOLYH` writer"]
pub struct W(crate::W<CRC_GPOLYH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_GPOLYH_SPEC>;
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
impl core::convert::From<crate::W<CRC_GPOLYH_SPEC>> for W {
    fn from(writer: crate::W<CRC_GPOLYH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPOLYH` reader - POLYH stores the high 16 bits of the 16/32 bit CRC polynomial value"]
pub struct GPOLYH_R(crate::FieldReader<u16, u16>);
impl GPOLYH_R {
    pub(crate) fn new(bits: u16) -> Self {
        GPOLYH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPOLYH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPOLYH` writer - POLYH stores the high 16 bits of the 16/32 bit CRC polynomial value"]
pub struct GPOLYH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOLYH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - POLYH stores the high 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline(always)]
    pub fn gpolyh(&self) -> GPOLYH_R {
        GPOLYH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - POLYH stores the high 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline(always)]
    pub fn gpolyh(&mut self) -> GPOLYH_W {
        GPOLYH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_GPOLYH register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_gpolyh](index.html) module"]
pub struct CRC_GPOLYH_SPEC;
impl crate::RegisterSpec for CRC_GPOLYH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc_gpolyh::R](R) reader structure"]
impl crate::Readable for CRC_GPOLYH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_gpolyh::W](W) writer structure"]
impl crate::Writable for CRC_GPOLYH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPOLYH to value 0xffff"]
impl crate::Resettable for CRC_GPOLYH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
