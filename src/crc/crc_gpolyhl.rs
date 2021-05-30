#[doc = "Register `GPOLYHL` reader"]
pub struct R(crate::R<CRC_GPOLYHL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_GPOLYHL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CRC_GPOLYHL_SPEC>> for R {
    fn from(reader: crate::R<CRC_GPOLYHL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPOLYHL` writer"]
pub struct W(crate::W<CRC_GPOLYHL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_GPOLYHL_SPEC>;
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
impl core::convert::From<crate::W<CRC_GPOLYHL_SPEC>> for W {
    fn from(writer: crate::W<CRC_GPOLYHL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPOLYHL` reader - POLYHL stores the third 8 bits of the 32 bit CRC"]
pub struct GPOLYHL_R(crate::FieldReader<u8, u8>);
impl GPOLYHL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPOLYHL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPOLYHL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPOLYHL` writer - POLYHL stores the third 8 bits of the 32 bit CRC"]
pub struct GPOLYHL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOLYHL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - POLYHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyhl(&self) -> GPOLYHL_R {
        GPOLYHL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyhl(&mut self) -> GPOLYHL_W {
        GPOLYHL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_GPOLYHL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_gpolyhl](index.html) module"]
pub struct CRC_GPOLYHL_SPEC;
impl crate::RegisterSpec for CRC_GPOLYHL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crc_gpolyhl::R](R) reader structure"]
impl crate::Readable for CRC_GPOLYHL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_gpolyhl::W](W) writer structure"]
impl crate::Writable for CRC_GPOLYHL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPOLYHL to value 0xff"]
impl crate::Resettable for CRC_GPOLYHL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
