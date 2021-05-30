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
#[doc = "Register `EEPROT` writer"]
pub struct W(crate::W<EEPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEPROT_SPEC>;
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
impl core::convert::From<crate::W<EEPROT_SPEC>> for W {
    fn from(writer: crate::W<EEPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPS` reader - EEPROM Protection Size"]
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
#[doc = "Field `DPS` writer - EEPROM Protection Size"]
pub struct DPS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
#[doc = "EEPROM Protection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPOPEN_A {
    #[doc = "0: Enables EEPROM memory protection from program and erase with protected address range defined by DPS bits."]
    _0 = 0,
    #[doc = "1: Disables EEPROM memory protection from program and erase."]
    _1 = 1,
}
impl From<DPOPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DPOPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPOPEN` reader - EEPROM Protection Control"]
pub struct DPOPEN_R(crate::FieldReader<bool, DPOPEN_A>);
impl DPOPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPOPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPOPEN_A {
        match self.bits {
            false => DPOPEN_A::_0,
            true => DPOPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DPOPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DPOPEN_A::_1
    }
}
impl core::ops::Deref for DPOPEN_R {
    type Target = crate::FieldReader<bool, DPOPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPOPEN` writer - EEPROM Protection Control"]
pub struct DPOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPOPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPOPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables EEPROM memory protection from program and erase with protected address range defined by DPS bits."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPOPEN_A::_0)
    }
    #[doc = "Disables EEPROM memory protection from program and erase."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPOPEN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - EEPROM Protection Size"]
    #[inline(always)]
    pub fn dps(&self) -> DPS_R {
        DPS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - EEPROM Protection Control"]
    #[inline(always)]
    pub fn dpopen(&self) -> DPOPEN_R {
        DPOPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - EEPROM Protection Size"]
    #[inline(always)]
    pub fn dps(&mut self) -> DPS_W {
        DPS_W { w: self }
    }
    #[doc = "Bit 7 - EEPROM Protection Control"]
    #[inline(always)]
    pub fn dpopen(&mut self) -> DPOPEN_W {
        DPOPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeprot](index.html) module"]
pub struct EEPROT_SPEC;
impl crate::RegisterSpec for EEPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [eeprot::R](R) reader structure"]
impl crate::Readable for EEPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eeprot::W](W) writer structure"]
impl crate::Writable for EEPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEPROT to value 0"]
impl crate::Resettable for EEPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
