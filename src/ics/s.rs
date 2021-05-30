#[doc = "Register `S` reader"]
pub struct R(crate::R<S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<S_SPEC>> for R {
    fn from(reader: crate::R<S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S` writer"]
pub struct W(crate::W<S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_SPEC>;
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
impl core::convert::From<crate::W<S_SPEC>> for W {
    fn from(writer: crate::W<S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKST_A {
    #[doc = "0: Output of FLL is selected."]
    _00 = 0,
    #[doc = "1: FLL Bypassed, internal reference clock is selected."]
    _01 = 1,
    #[doc = "2: FLL Bypassed, external reference clock is selected."]
    _10 = 2,
    #[doc = "3: Reserved."]
    _11 = 3,
}
impl From<CLKST_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKST` reader - Clock Mode Status"]
pub struct CLKST_R(crate::FieldReader<u8, CLKST_A>);
impl CLKST_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKST_A {
        match self.bits {
            0 => CLKST_A::_00,
            1 => CLKST_A::_01,
            2 => CLKST_A::_10,
            3 => CLKST_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == CLKST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == CLKST_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == CLKST_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == CLKST_A::_11
    }
}
impl core::ops::Deref for CLKST_R {
    type Target = crate::FieldReader<u8, CLKST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal Reference Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFST_A {
    #[doc = "0: Source of reference clock is external clock."]
    _0 = 0,
    #[doc = "1: Source of reference clock is internal clock."]
    _1 = 1,
}
impl From<IREFST_A> for bool {
    #[inline(always)]
    fn from(variant: IREFST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFST` reader - Internal Reference Status"]
pub struct IREFST_R(crate::FieldReader<bool, IREFST_A>);
impl IREFST_R {
    pub(crate) fn new(bits: bool) -> Self {
        IREFST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFST_A {
        match self.bits {
            false => IREFST_A::_0,
            true => IREFST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IREFST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IREFST_A::_1
    }
}
impl core::ops::Deref for IREFST_R {
    type Target = crate::FieldReader<bool, IREFST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: FLL is currently unlocked."]
    _0 = 0,
    #[doc = "1: FLL is currently locked."]
    _1 = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Lock Status"]
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::_0,
            true => LOCK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOCK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOCK_A::_1
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Loss of Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLS_A {
    #[doc = "0: FLL has not lost lock since LOLS was last cleared."]
    _0 = 0,
    #[doc = "1: FLL has lost lock since LOLS was last cleared."]
    _1 = 1,
}
impl From<LOLS_A> for bool {
    #[inline(always)]
    fn from(variant: LOLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOLS` reader - Loss of Lock Status"]
pub struct LOLS_R(crate::FieldReader<bool, LOLS_A>);
impl LOLS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOLS_A {
        match self.bits {
            false => LOLS_A::_0,
            true => LOLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOLS_A::_1
    }
}
impl core::ops::Deref for LOLS_R {
    type Target = crate::FieldReader<bool, LOLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOLS` writer - Loss of Lock Status"]
pub struct LOLS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOLS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FLL has not lost lock since LOLS was last cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLS_A::_0)
    }
    #[doc = "FLL has lost lock since LOLS was last cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLS_A::_1)
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
    #[doc = "Bits 2:3 - Clock Mode Status"]
    #[inline(always)]
    pub fn clkst(&self) -> CLKST_R {
        CLKST_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Internal Reference Status"]
    #[inline(always)]
    pub fn irefst(&self) -> IREFST_R {
        IREFST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loss of Lock Status"]
    #[inline(always)]
    pub fn lols(&self) -> LOLS_R {
        LOLS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Loss of Lock Status"]
    #[inline(always)]
    pub fn lols(&mut self) -> LOLS_W {
        LOLS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICS Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s](index.html) module"]
pub struct S_SPEC;
impl crate::RegisterSpec for S_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [s::R](R) reader structure"]
impl crate::Readable for S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s::W](W) writer structure"]
impl crate::Writable for S_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S to value 0x10"]
impl crate::Resettable for S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
