#[doc = "Register `EEPROT` reader"]
pub struct R(crate::R<EEPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EEPROT_SPEC>> for R {
    fn from(reader: crate::R<EEPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DPS` reader - no description available"]
pub struct DPS_R(crate::FieldReader<u8, u8>);
impl DPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPOPEN` reader - no description available"]
pub struct DPOPEN_R(crate::FieldReader<bool, bool>);
impl DPOPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPOPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPOPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - no description available"]
    #[inline(always)]
    pub fn dps(&self) -> DPS_R {
        DPS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn dpopen(&self) -> DPOPEN_R {
        DPOPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Non-volatile E-Flash Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeprot](index.html) module"]
pub struct EEPROT_SPEC;
impl crate::RegisterSpec for EEPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eeprot::R](R) reader structure"]
impl crate::Readable for EEPROT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EEPROT to value 0x87"]
impl crate::Resettable for EEPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x87
    }
}
