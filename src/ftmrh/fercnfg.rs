#[doc = "Reader of register FERCNFG"]
pub type R = crate::R<u8, super::FERCNFG>;
#[doc = "Writer for register FERCNFG"]
pub type W = crate::W<u8, super::FERCNFG>;
#[doc = "Register FERCNFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FERCNFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Single Bit Fault Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFDIE_A {
    #[doc = "0: SFDIF interrupt is disabled whenever the SFDIF flag is set."]
    _0 = 0,
    #[doc = "1: An interrupt will be requested whenever the SFDIF flag is set."]
    _1 = 1,
}
impl From<SFDIE_A> for bool {
    #[inline(always)]
    fn from(variant: SFDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SFDIE`"]
pub type SFDIE_R = crate::R<bool, SFDIE_A>;
impl SFDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFDIE_A {
        match self.bits {
            false => SFDIE_A::_0,
            true => SFDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFDIE_A::_1
    }
}
#[doc = "Write proxy for field `SFDIE`"]
pub struct SFDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SFDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SFDIF interrupt is disabled whenever the SFDIF flag is set."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFDIE_A::_0)
    }
    #[doc = "An interrupt will be requested whenever the SFDIF flag is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFDIE_A::_1)
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
#[doc = "Double Bit Fault Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFDIE_A {
    #[doc = "0: DFDIF interrupt is disabled."]
    _0 = 0,
    #[doc = "1: An interrupt will be requested whenever the DFDIF flag is set."]
    _1 = 1,
}
impl From<DFDIE_A> for bool {
    #[inline(always)]
    fn from(variant: DFDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFDIE`"]
pub type DFDIE_R = crate::R<bool, DFDIE_A>;
impl DFDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFDIE_A {
        match self.bits {
            false => DFDIE_A::_0,
            true => DFDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFDIE_A::_1
    }
}
#[doc = "Write proxy for field `DFDIE`"]
pub struct DFDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DFDIF interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFDIE_A::_0)
    }
    #[doc = "An interrupt will be requested whenever the DFDIF flag is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFDIE_A::_1)
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
    #[doc = "Bit 0 - Single Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sfdie(&self) -> SFDIE_R {
        SFDIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dfdie(&self) -> DFDIE_R {
        DFDIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sfdie(&mut self) -> SFDIE_W {
        SFDIE_W { w: self }
    }
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dfdie(&mut self) -> DFDIE_W {
        DFDIE_W { w: self }
    }
}
