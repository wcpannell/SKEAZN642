#[doc = "Register `C1` reader"]
pub struct R(crate::R<C1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<C1_SPEC>> for R {
    fn from(reader: crate::R<C1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1` writer"]
pub struct W(crate::W<C1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_SPEC>;
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
impl core::convert::From<crate::W<C1_SPEC>> for W {
    fn from(writer: crate::W<C1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACVAL` reader - DAC Output Level Selection"]
pub struct DACVAL_R(crate::FieldReader<u8, u8>);
impl DACVAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DACVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACVAL` writer - DAC Output Level Selection"]
pub struct DACVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DACVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u8 & 0x3f);
        self.w
    }
}
#[doc = "DAC Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACREF_A {
    #[doc = "0: The DAC selects Bandgap as the reference."]
    _0 = 0,
    #[doc = "1: The DAC selects VDDA as the reference."]
    _1 = 1,
}
impl From<DACREF_A> for bool {
    #[inline(always)]
    fn from(variant: DACREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACREF` reader - DAC Reference Select"]
pub struct DACREF_R(crate::FieldReader<bool, DACREF_A>);
impl DACREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACREF_A {
        match self.bits {
            false => DACREF_A::_0,
            true => DACREF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACREF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACREF_A::_1
    }
}
impl core::ops::Deref for DACREF_R {
    type Target = crate::FieldReader<bool, DACREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACREF` writer - DAC Reference Select"]
pub struct DACREF_W<'a> {
    w: &'a mut W,
}
impl<'a> DACREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACREF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DAC selects Bandgap as the reference."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACREF_A::_0)
    }
    #[doc = "The DAC selects VDDA as the reference."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACREF_A::_1)
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
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACEN_A {
    #[doc = "0: The DAC is disabled."]
    _0 = 0,
    #[doc = "1: The DAC is enabled."]
    _1 = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACEN` reader - DAC Enable"]
pub struct DACEN_R(crate::FieldReader<bool, DACEN_A>);
impl DACEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::_0,
            true => DACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DACEN_A::_1
    }
}
impl core::ops::Deref for DACEN_R {
    type Target = crate::FieldReader<bool, DACEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACEN` writer - DAC Enable"]
pub struct DACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DAC is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACEN_A::_0)
    }
    #[doc = "The DAC is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACEN_A::_1)
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
    #[doc = "Bits 0:5 - DAC Output Level Selection"]
    #[inline(always)]
    pub fn dacval(&self) -> DACVAL_R {
        DACVAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline(always)]
    pub fn dacref(&self) -> DACREF_R {
        DACREF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DAC Output Level Selection"]
    #[inline(always)]
    pub fn dacval(&mut self) -> DACVAL_W {
        DACVAL_W { w: self }
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline(always)]
    pub fn dacref(&mut self) -> DACREF_W {
        DACREF_W { w: self }
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ACMP Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](index.html) module"]
pub struct C1_SPEC;
impl crate::RegisterSpec for C1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c1::R](R) reader structure"]
impl crate::Readable for C1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1::W](W) writer structure"]
impl crate::Writable for C1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
