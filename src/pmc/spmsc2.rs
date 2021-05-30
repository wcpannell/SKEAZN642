#[doc = "Register `SPMSC2` reader"]
pub struct R(crate::R<SPMSC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPMSC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPMSC2_SPEC>> for R {
    fn from(reader: crate::R<SPMSC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPMSC2` writer"]
pub struct W(crate::W<SPMSC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPMSC2_SPEC>;
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
impl core::convert::From<crate::W<SPMSC2_SPEC>> for W {
    fn from(writer: crate::W<SPMSC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low-Voltage Warning Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVWV_A {
    #[doc = "0: Low trip point is selected (VLVW = VLVW1)."]
    _00 = 0,
    #[doc = "1: Middle 1 trip point is selected (VLVW = VLVW2)."]
    _01 = 1,
    #[doc = "2: Middle 2 trip point is selected (VLVW = VLVW3)."]
    _10 = 2,
    #[doc = "3: High trip point is selected (VLVW = VLVW4)."]
    _11 = 3,
}
impl From<LVWV_A> for u8 {
    #[inline(always)]
    fn from(variant: LVWV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LVWV` reader - Low-Voltage Warning Voltage Select"]
pub struct LVWV_R(crate::FieldReader<u8, LVWV_A>);
impl LVWV_R {
    pub(crate) fn new(bits: u8) -> Self {
        LVWV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWV_A {
        match self.bits {
            0 => LVWV_A::_00,
            1 => LVWV_A::_01,
            2 => LVWV_A::_10,
            3 => LVWV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == LVWV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == LVWV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == LVWV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == LVWV_A::_11
    }
}
impl core::ops::Deref for LVWV_R {
    type Target = crate::FieldReader<u8, LVWV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVWV` writer - Low-Voltage Warning Voltage Select"]
pub struct LVWV_W<'a> {
    w: &'a mut W,
}
impl<'a> LVWV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVWV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low trip point is selected (VLVW = VLVW1)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LVWV_A::_00)
    }
    #[doc = "Middle 1 trip point is selected (VLVW = VLVW2)."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LVWV_A::_01)
    }
    #[doc = "Middle 2 trip point is selected (VLVW = VLVW3)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LVWV_A::_10)
    }
    #[doc = "High trip point is selected (VLVW = VLVW4)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LVWV_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
#[doc = "Low-Voltage Detect Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDV_A {
    #[doc = "0: Low trip point is selected (VLVD = VLVDL)."]
    _0 = 0,
    #[doc = "1: High trip point is selected (VLVD = VLVDH)."]
    _1 = 1,
}
impl From<LVDV_A> for bool {
    #[inline(always)]
    fn from(variant: LVDV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDV` reader - Low-Voltage Detect Voltage Select"]
pub struct LVDV_R(crate::FieldReader<bool, LVDV_A>);
impl LVDV_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVDV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDV_A {
        match self.bits {
            false => LVDV_A::_0,
            true => LVDV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVDV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVDV_A::_1
    }
}
impl core::ops::Deref for LVDV_R {
    type Target = crate::FieldReader<bool, LVDV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVDV` writer - Low-Voltage Detect Voltage Select"]
pub struct LVDV_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low trip point is selected (VLVD = VLVDL)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDV_A::_0)
    }
    #[doc = "High trip point is selected (VLVD = VLVDH)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - Low-Voltage Warning Voltage Select"]
    #[inline(always)]
    pub fn lvwv(&self) -> LVWV_R {
        LVWV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Low-Voltage Detect Voltage Select"]
    #[inline(always)]
    pub fn lvdv(&self) -> LVDV_R {
        LVDV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - Low-Voltage Warning Voltage Select"]
    #[inline(always)]
    pub fn lvwv(&mut self) -> LVWV_W {
        LVWV_W { w: self }
    }
    #[doc = "Bit 6 - Low-Voltage Detect Voltage Select"]
    #[inline(always)]
    pub fn lvdv(&mut self) -> LVDV_W {
        LVDV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Power Management Status and Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spmsc2](index.html) module"]
pub struct SPMSC2_SPEC;
impl crate::RegisterSpec for SPMSC2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spmsc2::R](R) reader structure"]
impl crate::Readable for SPMSC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spmsc2::W](W) writer structure"]
impl crate::Writable for SPMSC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPMSC2 to value 0"]
impl crate::Resettable for SPMSC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
