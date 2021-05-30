#[doc = "Register `BUSDIV` reader"]
pub struct R(crate::R<BUSDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BUSDIV_SPEC>> for R {
    fn from(reader: crate::R<BUSDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSDIV` writer"]
pub struct W(crate::W<BUSDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSDIV_SPEC>;
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
impl core::convert::From<crate::W<BUSDIV_SPEC>> for W {
    fn from(writer: crate::W<BUSDIV_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `BUSDIV` reader - BUS Clock Divider"]
pub struct BUSDIV_R(crate::FieldReader<bool, BUSDIV_A>);
impl BUSDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSDIV_R(crate::FieldReader::new(bits))
    }
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
        **self == BUSDIV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSDIV_A::_1
    }
}
impl core::ops::Deref for BUSDIV_R {
    type Target = crate::FieldReader<bool, BUSDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSDIV` writer - BUS Clock Divider"]
pub struct BUSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSDIV_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BUS Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busdiv](index.html) module"]
pub struct BUSDIV_SPEC;
impl crate::RegisterSpec for BUSDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busdiv::R](R) reader structure"]
impl crate::Readable for BUSDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [busdiv::W](W) writer structure"]
impl crate::Writable for BUSDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUSDIV to value 0"]
impl crate::Resettable for BUSDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
