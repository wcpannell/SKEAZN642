#[doc = "Register `C0` reader"]
pub struct R(crate::R<C0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<C0_SPEC>> for R {
    fn from(reader: crate::R<C0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C0` writer"]
pub struct W(crate::W<C0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C0_SPEC>;
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
impl core::convert::From<crate::W<C0_SPEC>> for W {
    fn from(writer: crate::W<C0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ACMP Negative Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACNSEL_A {
    #[doc = "0: External reference 0"]
    _00 = 0,
    #[doc = "1: External reference 1"]
    _01 = 1,
    #[doc = "2: External reference 2"]
    _10 = 2,
    #[doc = "3: DAC output"]
    _11 = 3,
}
impl From<ACNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ACNSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACNSEL` reader - ACMP Negative Input Select"]
pub struct ACNSEL_R(crate::FieldReader<u8, ACNSEL_A>);
impl ACNSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACNSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACNSEL_A {
        match self.bits {
            0 => ACNSEL_A::_00,
            1 => ACNSEL_A::_01,
            2 => ACNSEL_A::_10,
            3 => ACNSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == ACNSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == ACNSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == ACNSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == ACNSEL_A::_11
    }
}
impl core::ops::Deref for ACNSEL_R {
    type Target = crate::FieldReader<u8, ACNSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACNSEL` writer - ACMP Negative Input Select"]
pub struct ACNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACNSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External reference 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ACNSEL_A::_00)
    }
    #[doc = "External reference 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ACNSEL_A::_01)
    }
    #[doc = "External reference 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ACNSEL_A::_10)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ACNSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "ACMP Positive Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACPSEL_A {
    #[doc = "0: External reference 0"]
    _00 = 0,
    #[doc = "1: External reference 1"]
    _01 = 1,
    #[doc = "2: External reference 2"]
    _10 = 2,
    #[doc = "3: DAC output"]
    _11 = 3,
}
impl From<ACPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ACPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACPSEL` reader - ACMP Positive Input Select"]
pub struct ACPSEL_R(crate::FieldReader<u8, ACPSEL_A>);
impl ACPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACPSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACPSEL_A {
        match self.bits {
            0 => ACPSEL_A::_00,
            1 => ACPSEL_A::_01,
            2 => ACPSEL_A::_10,
            3 => ACPSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == ACPSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == ACPSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == ACPSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == ACPSEL_A::_11
    }
}
impl core::ops::Deref for ACPSEL_R {
    type Target = crate::FieldReader<u8, ACPSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACPSEL` writer - ACMP Positive Input Select"]
pub struct ACPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACPSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "External reference 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ACPSEL_A::_00)
    }
    #[doc = "External reference 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ACPSEL_A::_01)
    }
    #[doc = "External reference 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ACPSEL_A::_10)
    }
    #[doc = "DAC output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ACPSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ACMP Negative Input Select"]
    #[inline(always)]
    pub fn acnsel(&self) -> ACNSEL_R {
        ACNSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - ACMP Positive Input Select"]
    #[inline(always)]
    pub fn acpsel(&self) -> ACPSEL_R {
        ACPSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ACMP Negative Input Select"]
    #[inline(always)]
    pub fn acnsel(&mut self) -> ACNSEL_W {
        ACNSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - ACMP Positive Input Select"]
    #[inline(always)]
    pub fn acpsel(&mut self) -> ACPSEL_W {
        ACPSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ACMP Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0](index.html) module"]
pub struct C0_SPEC;
impl crate::RegisterSpec for C0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c0::R](R) reader structure"]
impl crate::Readable for C0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c0::W](W) writer structure"]
impl crate::Writable for C0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C0 to value 0"]
impl crate::Resettable for C0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
