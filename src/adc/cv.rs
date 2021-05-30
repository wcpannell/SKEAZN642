#[doc = "Register `CV` reader"]
pub struct R(crate::R<CV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CV_SPEC>> for R {
    fn from(reader: crate::R<CV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CV` writer"]
pub struct W(crate::W<CV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CV_SPEC>;
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
impl core::convert::From<crate::W<CV_SPEC>> for W {
    fn from(writer: crate::W<CV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CV` reader - Conversion Result\\[11:0\\]"]
pub struct CV_R(crate::FieldReader<u16, u16>);
impl CV_R {
    pub(crate) fn new(bits: u16) -> Self {
        CV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CV` writer - Conversion Result\\[11:0\\]"]
pub struct CV_W<'a> {
    w: &'a mut W,
}
impl<'a> CV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Conversion Result\\[11:0\\]"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Conversion Result\\[11:0\\]"]
    #[inline(always)]
    pub fn cv(&mut self) -> CV_W {
        CV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](index.html) module"]
pub struct CV_SPEC;
impl crate::RegisterSpec for CV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cv::R](R) reader structure"]
impl crate::Readable for CV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cv::W](W) writer structure"]
impl crate::Writable for CV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CV to value 0"]
impl crate::Resettable for CV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
