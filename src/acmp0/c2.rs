#[doc = "Register `C2` reader"]
pub struct R(crate::R<C2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<C2_SPEC>> for R {
    fn from(reader: crate::R<C2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2` writer"]
pub struct W(crate::W<C2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2_SPEC>;
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
impl core::convert::From<crate::W<C2_SPEC>> for W {
    fn from(writer: crate::W<C2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ACMP Input Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACIPE_A {
    #[doc = "0: The corresponding external analog input is not allowed."]
    _0 = 0,
    #[doc = "1: The corresponding external analog input is allowed."]
    _1 = 1,
}
impl From<ACIPE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACIPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACIPE` reader - ACMP Input Pin Enable"]
pub struct ACIPE_R(crate::FieldReader<u8, ACIPE_A>);
impl ACIPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACIPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACIPE_A> {
        match self.bits {
            0 => Some(ACIPE_A::_0),
            1 => Some(ACIPE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACIPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACIPE_A::_1
    }
}
impl core::ops::Deref for ACIPE_R {
    type Target = crate::FieldReader<u8, ACIPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIPE` writer - ACMP Input Pin Enable"]
pub struct ACIPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACIPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACIPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding external analog input is not allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACIPE_A::_0)
    }
    #[doc = "The corresponding external analog input is allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACIPE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - ACMP Input Pin Enable"]
    #[inline(always)]
    pub fn acipe(&self) -> ACIPE_R {
        ACIPE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ACMP Input Pin Enable"]
    #[inline(always)]
    pub fn acipe(&mut self) -> ACIPE_W {
        ACIPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ACMP Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c2::R](R) reader structure"]
impl crate::Readable for C2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2::W](W) writer structure"]
impl crate::Writable for C2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2 to value 0"]
impl crate::Resettable for C2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
