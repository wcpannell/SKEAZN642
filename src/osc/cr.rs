#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CR_SPEC>> for R {
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl core::convert::From<crate::W<CR_SPEC>> for W {
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "OSC Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCINIT_A {
    #[doc = "0: Oscillator initialization is not complete."]
    _0 = 0,
    #[doc = "1: Oscillator initialization is completed."]
    _1 = 1,
}
impl From<OSCINIT_A> for bool {
    #[inline(always)]
    fn from(variant: OSCINIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCINIT` reader - OSC Initialization"]
pub struct OSCINIT_R(crate::FieldReader<bool, OSCINIT_A>);
impl OSCINIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCINIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCINIT_A {
        match self.bits {
            false => OSCINIT_A::_0,
            true => OSCINIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OSCINIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OSCINIT_A::_1
    }
}
impl core::ops::Deref for OSCINIT_R {
    type Target = crate::FieldReader<bool, OSCINIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "High Gain Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HGO_A {
    #[doc = "0: Low-power mode"]
    _0 = 0,
    #[doc = "1: High-gain mode"]
    _1 = 1,
}
impl From<HGO_A> for bool {
    #[inline(always)]
    fn from(variant: HGO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HGO` reader - High Gain Oscillator Select"]
pub struct HGO_R(crate::FieldReader<bool, HGO_A>);
impl HGO_R {
    pub(crate) fn new(bits: bool) -> Self {
        HGO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HGO_A {
        match self.bits {
            false => HGO_A::_0,
            true => HGO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HGO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HGO_A::_1
    }
}
impl core::ops::Deref for HGO_R {
    type Target = crate::FieldReader<bool, HGO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HGO` writer - High Gain Oscillator Select"]
pub struct HGO_W<'a> {
    w: &'a mut W,
}
impl<'a> HGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HGO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HGO_A::_0)
    }
    #[doc = "High-gain mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HGO_A::_1)
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
#[doc = "Frequency Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGE_A {
    #[doc = "0: Low frequency range of 32 kHz."]
    _0 = 0,
    #[doc = "1: High frequency range of 4-20 MHz."]
    _1 = 1,
}
impl From<RANGE_A> for bool {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE` reader - Frequency Range Select"]
pub struct RANGE_R(crate::FieldReader<bool, RANGE_A>);
impl RANGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RANGE_A {
        match self.bits {
            false => RANGE_A::_0,
            true => RANGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RANGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RANGE_A::_1
    }
}
impl core::ops::Deref for RANGE_R {
    type Target = crate::FieldReader<bool, RANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANGE` writer - Frequency Range Select"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low frequency range of 32 kHz."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RANGE_A::_0)
    }
    #[doc = "High frequency range of 4-20 MHz."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RANGE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "OSC Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCOS_A {
    #[doc = "0: External clock source from EXTAL pin is selected."]
    _0 = 0,
    #[doc = "1: Oscillator clock source is selected."]
    _1 = 1,
}
impl From<OSCOS_A> for bool {
    #[inline(always)]
    fn from(variant: OSCOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCOS` reader - OSC Output Select"]
pub struct OSCOS_R(crate::FieldReader<bool, OSCOS_A>);
impl OSCOS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCOS_A {
        match self.bits {
            false => OSCOS_A::_0,
            true => OSCOS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OSCOS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OSCOS_A::_1
    }
}
impl core::ops::Deref for OSCOS_R {
    type Target = crate::FieldReader<bool, OSCOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCOS` writer - OSC Output Select"]
pub struct OSCOS_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCOS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External clock source from EXTAL pin is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSCOS_A::_0)
    }
    #[doc = "Oscillator clock source is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSCOS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "OSC Enable in Stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSTEN_A {
    #[doc = "0: OSC clock is disabled in Stop mode."]
    _0 = 0,
    #[doc = "1: OSC clock stays enabled in Stop mode."]
    _1 = 1,
}
impl From<OSCSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSTEN` reader - OSC Enable in Stop mode"]
pub struct OSCSTEN_R(crate::FieldReader<bool, OSCSTEN_A>);
impl OSCSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSTEN_A {
        match self.bits {
            false => OSCSTEN_A::_0,
            true => OSCSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OSCSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OSCSTEN_A::_1
    }
}
impl core::ops::Deref for OSCSTEN_R {
    type Target = crate::FieldReader<bool, OSCSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCSTEN` writer - OSC Enable in Stop mode"]
pub struct OSCSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OSC clock is disabled in Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSCSTEN_A::_0)
    }
    #[doc = "OSC clock stays enabled in Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSCSTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "OSC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCEN_A {
    #[doc = "0: OSC module is disabled."]
    _0 = 0,
    #[doc = "1: OSC module is enabled."]
    _1 = 1,
}
impl From<OSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCEN` reader - OSC Enable"]
pub struct OSCEN_R(crate::FieldReader<bool, OSCEN_A>);
impl OSCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCEN_A {
        match self.bits {
            false => OSCEN_A::_0,
            true => OSCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OSCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OSCEN_A::_1
    }
}
impl core::ops::Deref for OSCEN_R {
    type Target = crate::FieldReader<bool, OSCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCEN` writer - OSC Enable"]
pub struct OSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OSC module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSCEN_A::_0)
    }
    #[doc = "OSC module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSCEN_A::_1)
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
    #[doc = "Bit 0 - OSC Initialization"]
    #[inline(always)]
    pub fn oscinit(&self) -> OSCINIT_R {
        OSCINIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo(&self) -> HGO_R {
        HGO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Frequency Range Select"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OSC Output Select"]
    #[inline(always)]
    pub fn oscos(&self) -> OSCOS_R {
        OSCOS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OSC Enable in Stop mode"]
    #[inline(always)]
    pub fn oscsten(&self) -> OSCSTEN_R {
        OSCSTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - OSC Enable"]
    #[inline(always)]
    pub fn oscen(&self) -> OSCEN_R {
        OSCEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo(&mut self) -> HGO_W {
        HGO_W { w: self }
    }
    #[doc = "Bit 2 - Frequency Range Select"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
    #[doc = "Bit 4 - OSC Output Select"]
    #[inline(always)]
    pub fn oscos(&mut self) -> OSCOS_W {
        OSCOS_W { w: self }
    }
    #[doc = "Bit 5 - OSC Enable in Stop mode"]
    #[inline(always)]
    pub fn oscsten(&mut self) -> OSCSTEN_W {
        OSCSTEN_W { w: self }
    }
    #[doc = "Bit 7 - OSC Enable"]
    #[inline(always)]
    pub fn oscen(&mut self) -> OSCEN_W {
        OSCEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
