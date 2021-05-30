#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_SPEC>> for R {
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl core::convert::From<crate::W<SC_SPEC>> for W {
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "KBI Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KBMOD_A {
    #[doc = "0: Keyboard detects edges only."]
    _0 = 0,
    #[doc = "1: Keyboard detects both edges and levels."]
    _1 = 1,
}
impl From<KBMOD_A> for bool {
    #[inline(always)]
    fn from(variant: KBMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KBMOD` reader - KBI Detection Mode"]
pub struct KBMOD_R(crate::FieldReader<bool, KBMOD_A>);
impl KBMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        KBMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KBMOD_A {
        match self.bits {
            false => KBMOD_A::_0,
            true => KBMOD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == KBMOD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == KBMOD_A::_1
    }
}
impl core::ops::Deref for KBMOD_R {
    type Target = crate::FieldReader<bool, KBMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBMOD` writer - KBI Detection Mode"]
pub struct KBMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> KBMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Keyboard detects edges only."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBMOD_A::_0)
    }
    #[doc = "Keyboard detects both edges and levels."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBMOD_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "KBI Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KBIE_A {
    #[doc = "0: KBI interrupt not enabled."]
    _0 = 0,
    #[doc = "1: KBI interrupt enabled."]
    _1 = 1,
}
impl From<KBIE_A> for bool {
    #[inline(always)]
    fn from(variant: KBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KBIE` reader - KBI Interrupt Enable"]
pub struct KBIE_R(crate::FieldReader<bool, KBIE_A>);
impl KBIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        KBIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KBIE_A {
        match self.bits {
            false => KBIE_A::_0,
            true => KBIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == KBIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == KBIE_A::_1
    }
}
impl core::ops::Deref for KBIE_R {
    type Target = crate::FieldReader<bool, KBIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBIE` writer - KBI Interrupt Enable"]
pub struct KBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> KBIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "KBI interrupt not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBIE_A::_0)
    }
    #[doc = "KBI interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `KBACK` writer - KBI Acknowledge"]
pub struct KBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> KBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "KBI Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KBF_A {
    #[doc = "0: KBI interrupt request not detected."]
    _0 = 0,
    #[doc = "1: KBI interrupt request detected."]
    _1 = 1,
}
impl From<KBF_A> for bool {
    #[inline(always)]
    fn from(variant: KBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KBF` reader - KBI Interrupt Flag"]
pub struct KBF_R(crate::FieldReader<bool, KBF_A>);
impl KBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        KBF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KBF_A {
        match self.bits {
            false => KBF_A::_0,
            true => KBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == KBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == KBF_A::_1
    }
}
impl core::ops::Deref for KBF_R {
    type Target = crate::FieldReader<bool, KBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - KBI Detection Mode"]
    #[inline(always)]
    pub fn kbmod(&self) -> KBMOD_R {
        KBMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - KBI Interrupt Enable"]
    #[inline(always)]
    pub fn kbie(&self) -> KBIE_R {
        KBIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - KBI Interrupt Flag"]
    #[inline(always)]
    pub fn kbf(&self) -> KBF_R {
        KBF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - KBI Detection Mode"]
    #[inline(always)]
    pub fn kbmod(&mut self) -> KBMOD_W {
        KBMOD_W { w: self }
    }
    #[doc = "Bit 1 - KBI Interrupt Enable"]
    #[inline(always)]
    pub fn kbie(&mut self) -> KBIE_W {
        KBIE_W { w: self }
    }
    #[doc = "Bit 2 - KBI Acknowledge"]
    #[inline(always)]
    pub fn kback(&mut self) -> KBACK_W {
        KBACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KBI Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
