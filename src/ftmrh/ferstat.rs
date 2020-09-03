#[doc = "Reader of register FERSTAT"]
pub type R = crate::R<u8, super::FERSTAT>;
#[doc = "Writer for register FERSTAT"]
pub type W = crate::W<u8, super::FERSTAT>;
#[doc = "Register FERSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::FERSTAT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Single Bit Fault Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFDIF_A {
    #[doc = "0: No single bit fault detected."]
    _0 = 0,
    #[doc = "1: Single bit fault detected and corrected or a flash array read operation returning invalid data was attempted while command running."]
    _1 = 1,
}
impl From<SFDIF_A> for bool {
    #[inline(always)]
    fn from(variant: SFDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SFDIF`"]
pub type SFDIF_R = crate::R<bool, SFDIF_A>;
impl SFDIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFDIF_A {
        match self.bits {
            false => SFDIF_A::_0,
            true => SFDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFDIF_A::_1
    }
}
#[doc = "Write proxy for field `SFDIF`"]
pub struct SFDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SFDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFDIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No single bit fault detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFDIF_A::_0)
    }
    #[doc = "Single bit fault detected and corrected or a flash array read operation returning invalid data was attempted while command running."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFDIF_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Double Bit Fault Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFDIF_A {
    #[doc = "0: No double bit fault detected."]
    _0 = 0,
    #[doc = "1: Double bit fault detected or a flash array read operation returning invalid data was attempted while command running."]
    _1 = 1,
}
impl From<DFDIF_A> for bool {
    #[inline(always)]
    fn from(variant: DFDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFDIF`"]
pub type DFDIF_R = crate::R<bool, DFDIF_A>;
impl DFDIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFDIF_A {
        match self.bits {
            false => DFDIF_A::_0,
            true => DFDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFDIF_A::_1
    }
}
#[doc = "Write proxy for field `DFDIF`"]
pub struct DFDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFDIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No double bit fault detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFDIF_A::_0)
    }
    #[doc = "Double bit fault detected or a flash array read operation returning invalid data was attempted while command running."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFDIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Single Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn sfdif(&self) -> SFDIF_R {
        SFDIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dfdif(&self) -> DFDIF_R {
        DFDIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn sfdif(&mut self) -> SFDIF_W {
        SFDIF_W { w: self }
    }
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dfdif(&mut self) -> DFDIF_W {
        DFDIF_W { w: self }
    }
}
