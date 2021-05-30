#[doc = "Register `GPOLYHU` reader"]
pub struct R(crate::R<GPOLYHU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPOLYHU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPOLYHU_SPEC>> for R {
    fn from(reader: crate::R<GPOLYHU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPOLYHU` writer"]
pub struct W(crate::W<GPOLYHU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPOLYHU_SPEC>;
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
impl core::convert::From<crate::W<GPOLYHU_SPEC>> for W {
    fn from(writer: crate::W<GPOLYHU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPOLYHU` reader - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
pub struct GPOLYHU_R(crate::FieldReader<u8, u8>);
impl GPOLYHU_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPOLYHU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPOLYHU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPOLYHU` writer - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
pub struct GPOLYHU_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOLYHU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyhu(&self) -> GPOLYHU_R {
        GPOLYHU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyhu(&mut self) -> GPOLYHU_W {
        GPOLYHU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_GPOLYHU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpolyhu](index.html) module"]
pub struct GPOLYHU_SPEC;
impl crate::RegisterSpec for GPOLYHU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gpolyhu::R](R) reader structure"]
impl crate::Readable for GPOLYHU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpolyhu::W](W) writer structure"]
impl crate::Writable for GPOLYHU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPOLYHU to value 0xff"]
impl crate::Resettable for GPOLYHU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
