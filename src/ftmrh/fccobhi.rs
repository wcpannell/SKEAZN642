#[doc = "Register `FCCOBHI` reader"]
pub struct R(crate::R<FCCOBHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCOBHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FCCOBHI_SPEC>> for R {
    fn from(reader: crate::R<FCCOBHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCOBHI` writer"]
pub struct W(crate::W<FCCOBHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCOBHI_SPEC>;
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
impl core::convert::From<crate::W<FCCOBHI_SPEC>> for W {
    fn from(writer: crate::W<FCCOBHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCOB` reader - Common Command Object Bit 15:8"]
pub struct CCOB_R(crate::FieldReader<u8, u8>);
impl CCOB_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCOB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCOB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCOB` writer - Common Command Object Bit 15:8"]
pub struct CCOB_W<'a> {
    w: &'a mut W,
}
impl<'a> CCOB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Common Command Object Bit 15:8"]
    #[inline(always)]
    pub fn ccob(&self) -> CCOB_R {
        CCOB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Common Command Object Bit 15:8"]
    #[inline(always)]
    pub fn ccob(&mut self) -> CCOB_W {
        CCOB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Common Command Object Register:High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccobhi](index.html) module"]
pub struct FCCOBHI_SPEC;
impl crate::RegisterSpec for FCCOBHI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fccobhi::R](R) reader structure"]
impl crate::Readable for FCCOBHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fccobhi::W](W) writer structure"]
impl crate::Writable for FCCOBHI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCCOBHI to value 0"]
impl crate::Resettable for FCCOBHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
