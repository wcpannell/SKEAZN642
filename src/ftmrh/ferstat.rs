#[doc = "Register `FERSTAT` reader"]
pub struct R(crate::R<FERSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FERSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FERSTAT_SPEC>> for R {
    fn from(reader: crate::R<FERSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FERSTAT` writer"]
pub struct W(crate::W<FERSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FERSTAT_SPEC>;
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
impl core::convert::From<crate::W<FERSTAT_SPEC>> for W {
    fn from(writer: crate::W<FERSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Single Bit Fault Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFDIF_A {
    #[doc = "0: No single bit fault detected."]
    _0 = 0,
    #[doc = "1: Single bit fault detected and corrected or a flash array read operation returning invalid data was attempted while command running."]
    _1 = 1,
}
impl From<SFDIF_A> for bool {
    #[inline(always)]
    fn from(variant: SFDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFDIF` reader - Single Bit Fault Detect Interrupt Flag"]
pub struct SFDIF_R(crate::FieldReader<bool, SFDIF_A>);
impl SFDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFDIF_A {
        match self.bits {
            false => SFDIF_A::_0,
            true => SFDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SFDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SFDIF_A::_1
    }
}
impl core::ops::Deref for SFDIF_R {
    type Target = crate::FieldReader<bool, SFDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFDIF` writer - Single Bit Fault Detect Interrupt Flag"]
pub struct SFDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SFDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No single bit fault detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFDIF_A::_0)
    }
    #[doc = "Single bit fault detected and corrected or a flash array read operation returning invalid data was attempted while command running."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFDIF_A::_1)
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
#[doc = "Double Bit Fault Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFDIF_A {
    #[doc = "0: No double bit fault detected."]
    _0 = 0,
    #[doc = "1: Double bit fault detected or a flash array read operation returning invalid data was attempted while command running."]
    _1 = 1,
}
impl From<DFDIF_A> for bool {
    #[inline(always)]
    fn from(variant: DFDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFDIF` reader - Double Bit Fault Detect Interrupt Flag"]
pub struct DFDIF_R(crate::FieldReader<bool, DFDIF_A>);
impl DFDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFDIF_A {
        match self.bits {
            false => DFDIF_A::_0,
            true => DFDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DFDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DFDIF_A::_1
    }
}
impl core::ops::Deref for DFDIF_R {
    type Target = crate::FieldReader<bool, DFDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFDIF` writer - Double Bit Fault Detect Interrupt Flag"]
pub struct DFDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No double bit fault detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFDIF_A::_0)
    }
    #[doc = "Double bit fault detected or a flash array read operation returning invalid data was attempted while command running."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFDIF_A::_1)
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
impl R {
    #[doc = "Bit 0 - Single Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn sfdif(&self) -> SFDIF_R {
        SFDIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dfdif(&self) -> DFDIF_R {
        DFDIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn sfdif(&mut self) -> SFDIF_W {
        SFDIF_W { w: self }
    }
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dfdif(&mut self) -> DFDIF_W {
        DFDIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ferstat](index.html) module"]
pub struct FERSTAT_SPEC;
impl crate::RegisterSpec for FERSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ferstat::R](R) reader structure"]
impl crate::Readable for FERSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ferstat::W](W) writer structure"]
impl crate::Writable for FERSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FERSTAT to value 0"]
impl crate::Resettable for FERSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
