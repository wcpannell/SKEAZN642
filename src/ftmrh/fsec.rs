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
#[doc = "Flash Security Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_A {
    #[doc = "0: Secured"]
    _00 = 0,
    #[doc = "1: Secured"]
    _01 = 1,
    #[doc = "2: Unsecured"]
    _10 = 2,
    #[doc = "3: Secured"]
    _11 = 3,
}
impl From<SEC_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEC` reader - Flash Security Bits"]
pub struct SEC_R(crate::FieldReader<u8, SEC_A>);
impl SEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_A {
        match self.bits {
            0 => SEC_A::_00,
            1 => SEC_A::_01,
            2 => SEC_A::_10,
            3 => SEC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == SEC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == SEC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == SEC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == SEC_A::_11
    }
}
impl core::ops::Deref for SEC_R {
    type Target = crate::FieldReader<u8, SEC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Backdoor Key Security Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEYEN_A {
    #[doc = "0: Disabled"]
    _00 = 0,
    #[doc = "1: Disabled"]
    _01 = 1,
    #[doc = "2: Enabled"]
    _10 = 2,
    #[doc = "3: Disabled"]
    _11 = 3,
}
impl From<KEYEN_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEYEN` reader - Backdoor Key Security Enable Bits"]
pub struct KEYEN_R(crate::FieldReader<u8, KEYEN_A>);
impl KEYEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYEN_A {
        match self.bits {
            0 => KEYEN_A::_00,
            1 => KEYEN_A::_01,
            2 => KEYEN_A::_10,
            3 => KEYEN_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == KEYEN_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == KEYEN_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == KEYEN_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == KEYEN_A::_11
    }
}
impl core::ops::Deref for KEYEN_R {
    type Target = crate::FieldReader<u8, KEYEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash Security Bits"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Backdoor Key Security Enable Bits"]
    #[inline(always)]
    pub fn keyen(&self) -> KEYEN_R {
        KEYEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
#[doc = "Flash Security Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsec](index.html) module"]
pub struct FSEC_SPEC;
impl crate::RegisterSpec for FSEC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fsec::R](R) reader structure"]
impl crate::Readable for FSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSEC to value 0"]
impl crate::Resettable for FSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
