#[doc = "Register `APCTL1` reader"]
pub struct R(crate::R<APCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<APCTL1_SPEC>> for R {
    fn from(reader: crate::R<APCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APCTL1` writer"]
pub struct W(crate::W<APCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APCTL1_SPEC>;
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
impl core::convert::From<crate::W<APCTL1_SPEC>> for W {
    fn from(writer: crate::W<APCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC Pin Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ADPC_A {
    #[doc = "0: ADx pin I/O control enabled."]
    _0 = 0,
    #[doc = "1: ADx pin I/O control disabled."]
    _1 = 1,
}
impl From<ADPC_A> for u16 {
    #[inline(always)]
    fn from(variant: ADPC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADPC` reader - ADC Pin Control"]
pub struct ADPC_R(crate::FieldReader<u16, ADPC_A>);
impl ADPC_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADPC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADPC_A> {
        match self.bits {
            0 => Some(ADPC_A::_0),
            1 => Some(ADPC_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADPC_A::_1
    }
}
impl core::ops::Deref for ADPC_R {
    type Target = crate::FieldReader<u16, ADPC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPC` writer - ADC Pin Control"]
pub struct ADPC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADPC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADx pin I/O control enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADPC_A::_0)
    }
    #[doc = "ADx pin I/O control disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADPC_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ADC Pin Control"]
    #[inline(always)]
    pub fn adpc(&self) -> ADPC_R {
        ADPC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC Pin Control"]
    #[inline(always)]
    pub fn adpc(&mut self) -> ADPC_W {
        ADPC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apctl1](index.html) module"]
pub struct APCTL1_SPEC;
impl crate::RegisterSpec for APCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apctl1::R](R) reader structure"]
impl crate::Readable for APCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apctl1::W](W) writer structure"]
impl crate::Writable for APCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APCTL1 to value 0"]
impl crate::Resettable for APCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
