#[doc = "Reader of register BUSDIV"]
pub type R = crate::R<u32, super::BUSDIV>;
#[doc = "Writer for register BUSDIV"]
pub type W = crate::W<u32, super::BUSDIV>;
#[doc = "Register BUSDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::BUSDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "BUS Clock Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSDIV_A {
    #[doc = "0: Bus clock is same as ICSOUTCLK."]
    _0 = 0,
    #[doc = "1: Bus clock is ICSOUTCLK divided by 2."]
    _1 = 1,
}
impl From<BUSDIV_A> for bool {
    #[inline(always)]
    fn from(variant: BUSDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSDIV`"]
pub type BUSDIV_R = crate::R<bool, BUSDIV_A>;
impl BUSDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSDIV_A {
        match self.bits {
            false => BUSDIV_A::_0,
            true => BUSDIV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSDIV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSDIV_A::_1
    }
}
#[doc = "Write proxy for field `BUSDIV`"]
pub struct BUSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSDIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus clock is same as ICSOUTCLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSDIV_A::_0)
    }
    #[doc = "Bus clock is ICSOUTCLK divided by 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSDIV_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BUS Clock Divider"]
    #[inline(always)]
    pub fn busdiv(&self) -> BUSDIV_R {
        BUSDIV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BUS Clock Divider"]
    #[inline(always)]
    pub fn busdiv(&mut self) -> BUSDIV_W {
        BUSDIV_W { w: self }
    }
}
