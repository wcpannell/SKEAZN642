#[doc = "Register `FCCOBIX` reader"]
pub struct R(crate::R<FCCOBIX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCOBIX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FCCOBIX_SPEC>> for R {
    fn from(reader: crate::R<FCCOBIX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCOBIX` writer"]
pub struct W(crate::W<FCCOBIX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCOBIX_SPEC>;
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
impl core::convert::From<crate::W<FCCOBIX_SPEC>> for W {
    fn from(writer: crate::W<FCCOBIX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCOBIX` reader - Common Command Register Index"]
pub struct CCOBIX_R(crate::FieldReader<u8, u8>);
impl CCOBIX_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCOBIX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCOBIX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCOBIX` writer - Common Command Register Index"]
pub struct CCOBIX_W<'a> {
    w: &'a mut W,
}
impl<'a> CCOBIX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Common Command Register Index"]
    #[inline(always)]
    pub fn ccobix(&self) -> CCOBIX_R {
        CCOBIX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Common Command Register Index"]
    #[inline(always)]
    pub fn ccobix(&mut self) -> CCOBIX_W {
        CCOBIX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash CCOB Index Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccobix](index.html) module"]
pub struct FCCOBIX_SPEC;
impl crate::RegisterSpec for FCCOBIX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fccobix::R](R) reader structure"]
impl crate::Readable for FCCOBIX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fccobix::W](W) writer structure"]
impl crate::Writable for FCCOBIX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCCOBIX to value 0"]
impl crate::Resettable for FCCOBIX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
