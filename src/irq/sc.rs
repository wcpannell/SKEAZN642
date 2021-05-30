#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_SPEC>> for R {
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl core::convert::From<crate::W<SC_SPEC>> for W {
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IRQ Detection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQMOD_A {
    #[doc = "0: IRQ event is detected only on falling/rising edges."]
    _0 = 0,
    #[doc = "1: IRQ event is detected on falling/rising edges and low/high levels."]
    _1 = 1,
}
impl From<IRQMOD_A> for bool {
    #[inline(always)]
    fn from(variant: IRQMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQMOD` reader - IRQ Detection Mode"]
pub struct IRQMOD_R(crate::FieldReader<bool, IRQMOD_A>);
impl IRQMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQMOD_A {
        match self.bits {
            false => IRQMOD_A::_0,
            true => IRQMOD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRQMOD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRQMOD_A::_1
    }
}
impl core::ops::Deref for IRQMOD_R {
    type Target = crate::FieldReader<bool, IRQMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQMOD` writer - IRQ Detection Mode"]
pub struct IRQMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IRQ event is detected only on falling/rising edges."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQMOD_A::_0)
    }
    #[doc = "IRQ event is detected on falling/rising edges and low/high levels."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQMOD_A::_1)
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
#[doc = "IRQ Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQIE_A {
    #[doc = "0: Interrupt request when IRQF set is disabled (use polling)."]
    _0 = 0,
    #[doc = "1: Interrupt requested whenever IRQF = 1."]
    _1 = 1,
}
impl From<IRQIE_A> for bool {
    #[inline(always)]
    fn from(variant: IRQIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQIE` reader - IRQ Interrupt Enable"]
pub struct IRQIE_R(crate::FieldReader<bool, IRQIE_A>);
impl IRQIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQIE_A {
        match self.bits {
            false => IRQIE_A::_0,
            true => IRQIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRQIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRQIE_A::_1
    }
}
impl core::ops::Deref for IRQIE_R {
    type Target = crate::FieldReader<bool, IRQIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQIE` writer - IRQ Interrupt Enable"]
pub struct IRQIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request when IRQF set is disabled (use polling)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQIE_A::_0)
    }
    #[doc = "Interrupt requested whenever IRQF = 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQIE_A::_1)
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
#[doc = "Field `IRQACK` writer - IRQ Acknowledge"]
pub struct IRQACK_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQACK_W<'a> {
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
#[doc = "IRQ Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQF_A {
    #[doc = "0: No IRQ request"]
    _0 = 0,
    #[doc = "1: IRQ event is detected."]
    _1 = 1,
}
impl From<IRQF_A> for bool {
    #[inline(always)]
    fn from(variant: IRQF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQF` reader - IRQ Flag"]
pub struct IRQF_R(crate::FieldReader<bool, IRQF_A>);
impl IRQF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQF_A {
        match self.bits {
            false => IRQF_A::_0,
            true => IRQF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRQF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRQF_A::_1
    }
}
impl core::ops::Deref for IRQF_R {
    type Target = crate::FieldReader<bool, IRQF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "IRQ Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQPE_A {
    #[doc = "0: IRQ pin function is disabled."]
    _0 = 0,
    #[doc = "1: IRQ pin function is enabled."]
    _1 = 1,
}
impl From<IRQPE_A> for bool {
    #[inline(always)]
    fn from(variant: IRQPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQPE` reader - IRQ Pin Enable"]
pub struct IRQPE_R(crate::FieldReader<bool, IRQPE_A>);
impl IRQPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQPE_A {
        match self.bits {
            false => IRQPE_A::_0,
            true => IRQPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRQPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRQPE_A::_1
    }
}
impl core::ops::Deref for IRQPE_R {
    type Target = crate::FieldReader<bool, IRQPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQPE` writer - IRQ Pin Enable"]
pub struct IRQPE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IRQ pin function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQPE_A::_0)
    }
    #[doc = "IRQ pin function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQPE_A::_1)
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
#[doc = "Interrupt Request (IRQ) Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQEDG_A {
    #[doc = "0: IRQ is falling-edge or falling-edge/low-level sensitive."]
    _0 = 0,
    #[doc = "1: IRQ is rising-edge or rising-edge/high-level sensitive."]
    _1 = 1,
}
impl From<IRQEDG_A> for bool {
    #[inline(always)]
    fn from(variant: IRQEDG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQEDG` reader - Interrupt Request (IRQ) Edge Select"]
pub struct IRQEDG_R(crate::FieldReader<bool, IRQEDG_A>);
impl IRQEDG_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQEDG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQEDG_A {
        match self.bits {
            false => IRQEDG_A::_0,
            true => IRQEDG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRQEDG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRQEDG_A::_1
    }
}
impl core::ops::Deref for IRQEDG_R {
    type Target = crate::FieldReader<bool, IRQEDG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQEDG` writer - Interrupt Request (IRQ) Edge Select"]
pub struct IRQEDG_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQEDG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQEDG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IRQ is falling-edge or falling-edge/low-level sensitive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQEDG_A::_0)
    }
    #[doc = "IRQ is rising-edge or rising-edge/high-level sensitive."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQEDG_A::_1)
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
#[doc = "Interrupt Request (IRQ) Pull Device Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQPDD_A {
    #[doc = "0: IRQ pull device enabled if IRQPE = 1."]
    _0 = 0,
    #[doc = "1: IRQ pull device disabled if IRQPE = 1."]
    _1 = 1,
}
impl From<IRQPDD_A> for bool {
    #[inline(always)]
    fn from(variant: IRQPDD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQPDD` reader - Interrupt Request (IRQ) Pull Device Disable"]
pub struct IRQPDD_R(crate::FieldReader<bool, IRQPDD_A>);
impl IRQPDD_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQPDD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQPDD_A {
        match self.bits {
            false => IRQPDD_A::_0,
            true => IRQPDD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRQPDD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRQPDD_A::_1
    }
}
impl core::ops::Deref for IRQPDD_R {
    type Target = crate::FieldReader<bool, IRQPDD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQPDD` writer - Interrupt Request (IRQ) Pull Device Disable"]
pub struct IRQPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQPDD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQPDD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IRQ pull device enabled if IRQPE = 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQPDD_A::_0)
    }
    #[doc = "IRQ pull device disabled if IRQPE = 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQPDD_A::_1)
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
impl R {
    #[doc = "Bit 0 - IRQ Detection Mode"]
    #[inline(always)]
    pub fn irqmod(&self) -> IRQMOD_R {
        IRQMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRQ Interrupt Enable"]
    #[inline(always)]
    pub fn irqie(&self) -> IRQIE_R {
        IRQIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IRQ Flag"]
    #[inline(always)]
    pub fn irqf(&self) -> IRQF_R {
        IRQF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IRQ Pin Enable"]
    #[inline(always)]
    pub fn irqpe(&self) -> IRQPE_R {
        IRQPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt Request (IRQ) Edge Select"]
    #[inline(always)]
    pub fn irqedg(&self) -> IRQEDG_R {
        IRQEDG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt Request (IRQ) Pull Device Disable"]
    #[inline(always)]
    pub fn irqpdd(&self) -> IRQPDD_R {
        IRQPDD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRQ Detection Mode"]
    #[inline(always)]
    pub fn irqmod(&mut self) -> IRQMOD_W {
        IRQMOD_W { w: self }
    }
    #[doc = "Bit 1 - IRQ Interrupt Enable"]
    #[inline(always)]
    pub fn irqie(&mut self) -> IRQIE_W {
        IRQIE_W { w: self }
    }
    #[doc = "Bit 2 - IRQ Acknowledge"]
    #[inline(always)]
    pub fn irqack(&mut self) -> IRQACK_W {
        IRQACK_W { w: self }
    }
    #[doc = "Bit 4 - IRQ Pin Enable"]
    #[inline(always)]
    pub fn irqpe(&mut self) -> IRQPE_W {
        IRQPE_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt Request (IRQ) Edge Select"]
    #[inline(always)]
    pub fn irqedg(&mut self) -> IRQEDG_W {
        IRQEDG_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt Request (IRQ) Pull Device Disable"]
    #[inline(always)]
    pub fn irqpdd(&mut self) -> IRQPDD_W {
        IRQPDD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Pin Request Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
