#[doc = "Register `PIDR` reader"]
pub struct R(crate::R<PIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PIDR_SPEC>> for R {
    fn from(reader: crate::R<PIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIDR` writer"]
pub struct W(crate::W<PIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIDR_SPEC>;
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
impl core::convert::From<crate::W<PIDR_SPEC>> for W {
    fn from(writer: crate::W<PIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Input Disable\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PID_A {
    #[doc = "0: Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    _0 = 0,
    #[doc = "1: Pin is not configured as General Purpose Input.Corresponding Pin Data Input Register bit will read zero."]
    _1 = 1,
}
impl From<PID_A> for u32 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PID` reader - Port Input Disable"]
pub struct PID_R(crate::FieldReader<u32, PID_A>);
impl PID_R {
    pub(crate) fn new(bits: u32) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PID_A> {
        match self.bits {
            0 => Some(PID_A::_0),
            1 => Some(PID_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PID_A::_1
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<u32, PID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID` writer - Port Input Disable"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pin is configured for General Purpose Input, provided the pin is configured for any digital function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PID_A::_0)
    }
    #[doc = "Pin is not configured as General Purpose Input.Corresponding Pin Data Input Register bit will read zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PID_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Input Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr](index.html) module"]
pub struct PIDR_SPEC;
impl crate::RegisterSpec for PIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr::R](R) reader structure"]
impl crate::Readable for PIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pidr::W](W) writer structure"]
impl crate::Writable for PIDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIDR to value 0xffff_ffff"]
impl crate::Resettable for PIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
