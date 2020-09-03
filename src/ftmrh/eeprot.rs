#[doc = "Reader of register EEPROT"]
pub type R = crate::R<u8, super::EEPROT>;
#[doc = "Writer for register EEPROT"]
pub type W = crate::W<u8, super::EEPROT>;
#[doc = "Register EEPROT `reset()`'s with value 0"]
impl crate::ResetValue for super::EEPROT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPS`"]
pub type DPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPS`"]
pub struct DPS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
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
#[doc = "Reader of field `DPOPEN`"]
pub type DPOPEN_R = crate::R<bool, DPOPEN_A>;
impl DPOPEN_R {
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
        *self == DPOPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPOPEN_A::_1
    }
}
#[doc = "Write proxy for field `DPOPEN`"]
pub struct DPOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPOPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPOPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
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
}
