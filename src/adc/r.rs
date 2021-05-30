#[doc = "Register `R` reader"]
pub struct R(crate::R<R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<R_SPEC>> for R {
    fn from(reader: crate::R<R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADR` reader - Conversion Result"]
pub struct ADR_R(crate::FieldReader<u16, u16>);
impl ADR_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Conversion Result"]
    #[inline(always)]
    pub fn adr(&self) -> ADR_R {
        ADR_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Conversion Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r](index.html) module"]
pub struct R_SPEC;
impl crate::RegisterSpec for R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r::R](R) reader structure"]
impl crate::Readable for R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R to value 0"]
impl crate::Resettable for R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
