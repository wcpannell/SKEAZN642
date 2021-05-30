#[doc = "Register `ES` reader"]
pub struct R(crate::R<ES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ES_SPEC>> for R {
    fn from(reader: crate::R<ES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ES` writer"]
pub struct W(crate::W<ES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ES_SPEC>;
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
impl core::convert::From<crate::W<ES_SPEC>> for W {
    fn from(writer: crate::W<ES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "KBI Edge Selects\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KBEDG_A {
    #[doc = "0: Falling edge/low level."]
    _0 = 0,
    #[doc = "1: Rising edge/high level."]
    _1 = 1,
}
impl From<KBEDG_A> for u8 {
    #[inline(always)]
    fn from(variant: KBEDG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KBEDG` reader - KBI Edge Selects"]
pub struct KBEDG_R(crate::FieldReader<u8, KBEDG_A>);
impl KBEDG_R {
    pub(crate) fn new(bits: u8) -> Self {
        KBEDG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KBEDG_A> {
        match self.bits {
            0 => Some(KBEDG_A::_0),
            1 => Some(KBEDG_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == KBEDG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == KBEDG_A::_1
    }
}
impl core::ops::Deref for KBEDG_R {
    type Target = crate::FieldReader<u8, KBEDG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBEDG` writer - KBI Edge Selects"]
pub struct KBEDG_W<'a> {
    w: &'a mut W,
}
impl<'a> KBEDG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBEDG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Falling edge/low level."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBEDG_A::_0)
    }
    #[doc = "Rising edge/high level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBEDG_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - KBI Edge Selects"]
    #[inline(always)]
    pub fn kbedg(&self) -> KBEDG_R {
        KBEDG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - KBI Edge Selects"]
    #[inline(always)]
    pub fn kbedg(&mut self) -> KBEDG_W {
        KBEDG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KBIx Edge Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [es](index.html) module"]
pub struct ES_SPEC;
impl crate::RegisterSpec for ES_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [es::R](R) reader structure"]
impl crate::Readable for ES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [es::W](W) writer structure"]
impl crate::Writable for ES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ES to value 0"]
impl crate::Resettable for ES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
