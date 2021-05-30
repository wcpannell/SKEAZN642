#[doc = "Register `FSEC` reader"]
pub struct R(crate::R<FSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FSEC_SPEC>> for R {
    fn from(reader: crate::R<FSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEC` reader - Flash Security"]
pub struct SEC_R(crate::FieldReader<u8, u8>);
impl SEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYEN` reader - Backdoor Key Security Enable"]
pub struct KEYEN_R(crate::FieldReader<u8, u8>);
impl KEYEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash Security"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Backdoor Key Security Enable"]
    #[inline(always)]
    pub fn keyen(&self) -> KEYEN_R {
        KEYEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
#[doc = "Non-volatile Flash Security Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsec](index.html) module"]
pub struct FSEC_SPEC;
impl crate::RegisterSpec for FSEC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fsec::R](R) reader structure"]
impl crate::Readable for FSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSEC to value 0xff"]
impl crate::Resettable for FSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
