#[doc = "Register `CS` reader"]
pub struct R(crate::R<CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CS_SPEC>> for R {
    fn from(reader: crate::R<CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS` writer"]
pub struct W(crate::W<CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS_SPEC>;
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
impl core::convert::From<crate::W<CS_SPEC>> for W {
    fn from(writer: crate::W<CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ACMP MOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACMOD_A {
    #[doc = "0: ACMP interrupt on output falling edge."]
    _00 = 0,
    #[doc = "1: ACMP interrupt on output rising edge."]
    _01 = 1,
    #[doc = "2: ACMP interrupt on output falling edge."]
    _10 = 2,
    #[doc = "3: ACMP interrupt on output falling or rising edge."]
    _11 = 3,
}
impl From<ACMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACMOD` reader - ACMP MOD"]
pub struct ACMOD_R(crate::FieldReader<u8, ACMOD_A>);
impl ACMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMOD_A {
        match self.bits {
            0 => ACMOD_A::_00,
            1 => ACMOD_A::_01,
            2 => ACMOD_A::_10,
            3 => ACMOD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == ACMOD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == ACMOD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == ACMOD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == ACMOD_A::_11
    }
}
impl core::ops::Deref for ACMOD_R {
    type Target = crate::FieldReader<u8, ACMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMOD` writer - ACMP MOD"]
pub struct ACMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ACMP interrupt on output falling edge."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ACMOD_A::_00)
    }
    #[doc = "ACMP interrupt on output rising edge."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ACMOD_A::_01)
    }
    #[doc = "ACMP interrupt on output falling edge."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ACMOD_A::_10)
    }
    #[doc = "ACMP interrupt on output falling or rising edge."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ACMOD_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "ACMP Output Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACOPE_A {
    #[doc = "0: ACMP output cannot be placed onto external pin."]
    _0 = 0,
    #[doc = "1: ACMP output can be placed onto external pin."]
    _1 = 1,
}
impl From<ACOPE_A> for bool {
    #[inline(always)]
    fn from(variant: ACOPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACOPE` reader - ACMP Output Pin Enable"]
pub struct ACOPE_R(crate::FieldReader<bool, ACOPE_A>);
impl ACOPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACOPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACOPE_A {
        match self.bits {
            false => ACOPE_A::_0,
            true => ACOPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACOPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACOPE_A::_1
    }
}
impl core::ops::Deref for ACOPE_R {
    type Target = crate::FieldReader<bool, ACOPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACOPE` writer - ACMP Output Pin Enable"]
pub struct ACOPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACOPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ACMP output cannot be placed onto external pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACOPE_A::_0)
    }
    #[doc = "ACMP output can be placed onto external pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACOPE_A::_1)
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
#[doc = "Field `ACO` reader - ACMP Output"]
pub struct ACO_R(crate::FieldReader<bool, bool>);
impl ACO_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ACMP Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACIE_A {
    #[doc = "0: Disable the ACMP Interrupt."]
    _0 = 0,
    #[doc = "1: Enable the ACMP Interrupt."]
    _1 = 1,
}
impl From<ACIE_A> for bool {
    #[inline(always)]
    fn from(variant: ACIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACIE` reader - ACMP Interrupt Enable"]
pub struct ACIE_R(crate::FieldReader<bool, ACIE_A>);
impl ACIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACIE_A {
        match self.bits {
            false => ACIE_A::_0,
            true => ACIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACIE_A::_1
    }
}
impl core::ops::Deref for ACIE_R {
    type Target = crate::FieldReader<bool, ACIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIE` writer - ACMP Interrupt Enable"]
pub struct ACIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the ACMP Interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACIE_A::_0)
    }
    #[doc = "Enable the ACMP Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACIE_A::_1)
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
#[doc = "Field `ACF` reader - ACMP Interrupt Flag Bit"]
pub struct ACF_R(crate::FieldReader<bool, bool>);
impl ACF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACF` writer - ACMP Interrupt Flag Bit"]
pub struct ACF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACF_W<'a> {
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
#[doc = "Analog Comparator Hysterisis Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYST_A {
    #[doc = "0: 20 mV."]
    _0 = 0,
    #[doc = "1: 30 mV."]
    _1 = 1,
}
impl From<HYST_A> for bool {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYST` reader - Analog Comparator Hysterisis Selection"]
pub struct HYST_R(crate::FieldReader<bool, HYST_A>);
impl HYST_R {
    pub(crate) fn new(bits: bool) -> Self {
        HYST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            false => HYST_A::_0,
            true => HYST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HYST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HYST_A::_1
    }
}
impl core::ops::Deref for HYST_R {
    type Target = crate::FieldReader<bool, HYST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST` writer - Analog Comparator Hysterisis Selection"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "20 mV."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HYST_A::_0)
    }
    #[doc = "30 mV."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HYST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Analog Comparator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACE_A {
    #[doc = "0: The ACMP is disabled."]
    _0 = 0,
    #[doc = "1: The ACMP is enabled."]
    _1 = 1,
}
impl From<ACE_A> for bool {
    #[inline(always)]
    fn from(variant: ACE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACE` reader - Analog Comparator Enable"]
pub struct ACE_R(crate::FieldReader<bool, ACE_A>);
impl ACE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACE_A {
        match self.bits {
            false => ACE_A::_0,
            true => ACE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACE_A::_1
    }
}
impl core::ops::Deref for ACE_R {
    type Target = crate::FieldReader<bool, ACE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACE` writer - Analog Comparator Enable"]
pub struct ACE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The ACMP is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACE_A::_0)
    }
    #[doc = "The ACMP is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACE_A::_1)
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
    #[doc = "Bits 0:1 - ACMP MOD"]
    #[inline(always)]
    pub fn acmod(&self) -> ACMOD_R {
        ACMOD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - ACMP Output Pin Enable"]
    #[inline(always)]
    pub fn acope(&self) -> ACOPE_R {
        ACOPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ACMP Output"]
    #[inline(always)]
    pub fn aco(&self) -> ACO_R {
        ACO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ACMP Interrupt Enable"]
    #[inline(always)]
    pub fn acie(&self) -> ACIE_R {
        ACIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACMP Interrupt Flag Bit"]
    #[inline(always)]
    pub fn acf(&self) -> ACF_R {
        ACF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator Hysterisis Selection"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn ace(&self) -> ACE_R {
        ACE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ACMP MOD"]
    #[inline(always)]
    pub fn acmod(&mut self) -> ACMOD_W {
        ACMOD_W { w: self }
    }
    #[doc = "Bit 2 - ACMP Output Pin Enable"]
    #[inline(always)]
    pub fn acope(&mut self) -> ACOPE_W {
        ACOPE_W { w: self }
    }
    #[doc = "Bit 4 - ACMP Interrupt Enable"]
    #[inline(always)]
    pub fn acie(&mut self) -> ACIE_W {
        ACIE_W { w: self }
    }
    #[doc = "Bit 5 - ACMP Interrupt Flag Bit"]
    #[inline(always)]
    pub fn acf(&mut self) -> ACF_W {
        ACF_W { w: self }
    }
    #[doc = "Bit 6 - Analog Comparator Hysterisis Selection"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Bit 7 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn ace(&mut self) -> ACE_W {
        ACE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ACMP Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](index.html) module"]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cs::R](R) reader structure"]
impl crate::Readable for CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs::W](W) writer structure"]
impl crate::Writable for CS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
