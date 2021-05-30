#[doc = "Register `FERCNFG` reader"]
pub struct R(crate::R<FERCNFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FERCNFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FERCNFG_SPEC>> for R {
    fn from(reader: crate::R<FERCNFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FERCNFG` writer"]
pub struct W(crate::W<FERCNFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FERCNFG_SPEC>;
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
impl core::convert::From<crate::W<FERCNFG_SPEC>> for W {
    fn from(writer: crate::W<FERCNFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Single Bit Fault Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFDIE_A {
    #[doc = "0: SFDIF interrupt is disabled whenever the SFDIF flag is set."]
    _0 = 0,
    #[doc = "1: An interrupt will be requested whenever the SFDIF flag is set."]
    _1 = 1,
}
impl From<SFDIE_A> for bool {
    #[inline(always)]
    fn from(variant: SFDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFDIE` reader - Single Bit Fault Detect Interrupt Enable"]
pub struct SFDIE_R(crate::FieldReader<bool, SFDIE_A>);
impl SFDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFDIE_A {
        match self.bits {
            false => SFDIE_A::_0,
            true => SFDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SFDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SFDIE_A::_1
    }
}
impl core::ops::Deref for SFDIE_R {
    type Target = crate::FieldReader<bool, SFDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFDIE` writer - Single Bit Fault Detect Interrupt Enable"]
pub struct SFDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SFDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SFDIF interrupt is disabled whenever the SFDIF flag is set."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFDIE_A::_0)
    }
    #[doc = "An interrupt will be requested whenever the SFDIF flag is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFDIE_A::_1)
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
#[doc = "Double Bit Fault Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFDIE_A {
    #[doc = "0: DFDIF interrupt is disabled."]
    _0 = 0,
    #[doc = "1: An interrupt will be requested whenever the DFDIF flag is set."]
    _1 = 1,
}
impl From<DFDIE_A> for bool {
    #[inline(always)]
    fn from(variant: DFDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFDIE` reader - Double Bit Fault Detect Interrupt Enable"]
pub struct DFDIE_R(crate::FieldReader<bool, DFDIE_A>);
impl DFDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFDIE_A {
        match self.bits {
            false => DFDIE_A::_0,
            true => DFDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DFDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DFDIE_A::_1
    }
}
impl core::ops::Deref for DFDIE_R {
    type Target = crate::FieldReader<bool, DFDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFDIE` writer - Double Bit Fault Detect Interrupt Enable"]
pub struct DFDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DFDIF interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFDIE_A::_0)
    }
    #[doc = "An interrupt will be requested whenever the DFDIF flag is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFDIE_A::_1)
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
    #[doc = "Bit 0 - Single Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sfdie(&self) -> SFDIE_R {
        SFDIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dfdie(&self) -> DFDIE_R {
        DFDIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sfdie(&mut self) -> SFDIE_W {
        SFDIE_W { w: self }
    }
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dfdie(&mut self) -> DFDIE_W {
        DFDIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Error Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fercnfg](index.html) module"]
pub struct FERCNFG_SPEC;
impl crate::RegisterSpec for FERCNFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fercnfg::R](R) reader structure"]
impl crate::Readable for FERCNFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fercnfg::W](W) writer structure"]
impl crate::Writable for FERCNFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FERCNFG to value 0"]
impl crate::Resettable for FERCNFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
