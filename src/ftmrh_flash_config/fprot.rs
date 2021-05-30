#[doc = "Register `FPROT` reader"]
pub struct R(crate::R<FPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FPROT_SPEC>> for R {
    fn from(reader: crate::R<FPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FPLS` reader - no description available"]
pub struct FPLS_R(crate::FieldReader<u8, u8>);
impl FPLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FPLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPLDIS` reader - no description available"]
pub struct FPLDIS_R(crate::FieldReader<bool, bool>);
impl FPLDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPLDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPLDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPHS` reader - no description available"]
pub struct FPHS_R(crate::FieldReader<u8, u8>);
impl FPHS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FPHS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPHS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPHDIS` reader - no description available"]
pub struct FPHDIS_R(crate::FieldReader<bool, bool>);
impl FPHDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPHDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPHDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPOPEN` reader - no description available"]
pub struct FPOPEN_R(crate::FieldReader<bool, bool>);
impl FPOPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPOPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPOPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - no description available"]
    #[inline(always)]
    pub fn fpls(&self) -> FPLS_R {
        FPLS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn fpldis(&self) -> FPLDIS_R {
        FPLDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - no description available"]
    #[inline(always)]
    pub fn fphs(&self) -> FPHS_R {
        FPHS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn fphdis(&self) -> FPHDIS_R {
        FPHDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn fpopen(&self) -> FPOPEN_R {
        FPOPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Non-volatile P-Flash Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot](index.html) module"]
pub struct FPROT_SPEC;
impl crate::RegisterSpec for FPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fprot::R](R) reader structure"]
impl crate::Readable for FPROT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FPROT to value 0xff"]
impl crate::Resettable for FPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
