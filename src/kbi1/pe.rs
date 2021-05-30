#[doc = "Register `PE` reader"]
pub struct R(crate::R<PE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PE_SPEC>> for R {
    fn from(reader: crate::R<PE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE` writer"]
pub struct W(crate::W<PE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE_SPEC>;
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
impl core::convert::From<crate::W<PE_SPEC>> for W {
    fn from(writer: crate::W<PE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "KBI Pin Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KBIPE_A {
    #[doc = "0: Pin is not enabled as KBI interrupt."]
    _0 = 0,
    #[doc = "1: Pin is enabled as KBI interrupt."]
    _1 = 1,
}
impl From<KBIPE_A> for u8 {
    #[inline(always)]
    fn from(variant: KBIPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KBIPE` reader - KBI Pin Enables"]
pub struct KBIPE_R(crate::FieldReader<u8, KBIPE_A>);
impl KBIPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        KBIPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KBIPE_A> {
        match self.bits {
            0 => Some(KBIPE_A::_0),
            1 => Some(KBIPE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == KBIPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == KBIPE_A::_1
    }
}
impl core::ops::Deref for KBIPE_R {
    type Target = crate::FieldReader<u8, KBIPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBIPE` writer - KBI Pin Enables"]
pub struct KBIPE_W<'a> {
    w: &'a mut W,
}
impl<'a> KBIPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBIPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pin is not enabled as KBI interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBIPE_A::_0)
    }
    #[doc = "Pin is enabled as KBI interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBIPE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - KBI Pin Enables"]
    #[inline(always)]
    pub fn kbipe(&self) -> KBIPE_R {
        KBIPE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - KBI Pin Enables"]
    #[inline(always)]
    pub fn kbipe(&mut self) -> KBIPE_W {
        KBIPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KBIx Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe](index.html) module"]
pub struct PE_SPEC;
impl crate::RegisterSpec for PE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pe::R](R) reader structure"]
impl crate::Readable for PE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe::W](W) writer structure"]
impl crate::Writable for PE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PE to value 0"]
impl crate::Resettable for PE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
