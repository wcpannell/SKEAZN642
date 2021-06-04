#[doc = "Register `SPMSC1` reader"]
pub struct R(crate::R<SPMSC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPMSC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPMSC1_SPEC>> for R {
    fn from(reader: crate::R<SPMSC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPMSC1` writer"]
pub struct W(crate::W<SPMSC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPMSC1_SPEC>;
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
impl core::convert::From<crate::W<SPMSC1_SPEC>> for W {
    fn from(writer: crate::W<SPMSC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bandgap Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGBE_A {
    #[doc = "0: Bandgap buffer is disabled."]
    _0 = 0,
    #[doc = "1: Bandgap buffer is enabled."]
    _1 = 1,
}
impl From<BGBE_A> for bool {
    #[inline(always)]
    fn from(variant: BGBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGBE` reader - Bandgap Buffer Enable"]
pub struct BGBE_R(crate::FieldReader<bool, BGBE_A>);
impl BGBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGBE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGBE_A {
        match self.bits {
            false => BGBE_A::_0,
            true => BGBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BGBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BGBE_A::_1
    }
}
impl core::ops::Deref for BGBE_R {
    type Target = crate::FieldReader<bool, BGBE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGBE` writer - Bandgap Buffer Enable"]
pub struct BGBE_W<'a> {
    w: &'a mut W,
}
impl<'a> BGBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGBE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bandgap buffer is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGBE_A::_0)
    }
    #[doc = "Bandgap buffer is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGBE_A::_1)
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
#[doc = "Low-Voltage Detect Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDE_A {
    #[doc = "0: LVD logic is disabled."]
    _0 = 0,
    #[doc = "1: LVD logic is enabled."]
    _1 = 1,
}
impl From<LVDE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDE` reader - Low-Voltage Detect Enable"]
pub struct LVDE_R(crate::FieldReader<bool, LVDE_A>);
impl LVDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDE_A {
        match self.bits {
            false => LVDE_A::_0,
            true => LVDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVDE_A::_1
    }
}
impl core::ops::Deref for LVDE_R {
    type Target = crate::FieldReader<bool, LVDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVDE` writer - Low-Voltage Detect Enable"]
pub struct LVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LVD logic is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDE_A::_0)
    }
    #[doc = "LVD logic is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDE_A::_1)
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
#[doc = "Low-Voltage Detect Stop Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDSE_A {
    #[doc = "0: Low-voltage detect is disabled during Stop mode."]
    _0 = 0,
    #[doc = "1: Low-voltage detect is enabled during Stop mode."]
    _1 = 1,
}
impl From<LVDSE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDSE` reader - Low-Voltage Detect Stop Enable"]
pub struct LVDSE_R(crate::FieldReader<bool, LVDSE_A>);
impl LVDSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVDSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDSE_A {
        match self.bits {
            false => LVDSE_A::_0,
            true => LVDSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVDSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVDSE_A::_1
    }
}
impl core::ops::Deref for LVDSE_R {
    type Target = crate::FieldReader<bool, LVDSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVDSE` writer - Low-Voltage Detect Stop Enable"]
pub struct LVDSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low-voltage detect is disabled during Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDSE_A::_0)
    }
    #[doc = "Low-voltage detect is enabled during Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDSE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Low-Voltage Detect Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDRE_A {
    #[doc = "0: LVD events do not generate hardware resets."]
    _0 = 0,
    #[doc = "1: Forces an MCU reset when an enabled low-voltage detect event occurs."]
    _1 = 1,
}
impl From<LVDRE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDRE` reader - Low-Voltage Detect Reset Enable"]
pub struct LVDRE_R(crate::FieldReader<bool, LVDRE_A>);
impl LVDRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVDRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDRE_A {
        match self.bits {
            false => LVDRE_A::_0,
            true => LVDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVDRE_A::_1
    }
}
impl core::ops::Deref for LVDRE_R {
    type Target = crate::FieldReader<bool, LVDRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVDRE` writer - Low-Voltage Detect Reset Enable"]
pub struct LVDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LVD events do not generate hardware resets."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDRE_A::_0)
    }
    #[doc = "Forces an MCU reset when an enabled low-voltage detect event occurs."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDRE_A::_1)
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
#[doc = "Low-Voltage Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWIE_A {
    #[doc = "0: Hardware interrupt is disabled (use polling)."]
    _0 = 0,
    #[doc = "1: Requests a hardware interrupt when LVWF = 1."]
    _1 = 1,
}
impl From<LVWIE_A> for bool {
    #[inline(always)]
    fn from(variant: LVWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVWIE` reader - Low-Voltage Warning Interrupt Enable"]
pub struct LVWIE_R(crate::FieldReader<bool, LVWIE_A>);
impl LVWIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVWIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWIE_A {
        match self.bits {
            false => LVWIE_A::_0,
            true => LVWIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVWIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVWIE_A::_1
    }
}
impl core::ops::Deref for LVWIE_R {
    type Target = crate::FieldReader<bool, LVWIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVWIE` writer - Low-Voltage Warning Interrupt Enable"]
pub struct LVWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVWIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware interrupt is disabled (use polling)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVWIE_A::_0)
    }
    #[doc = "Requests a hardware interrupt when LVWF = 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVWIE_A::_1)
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
#[doc = "Field `LVWACK` reader - Low-Voltage Warning Acknowledge"]
pub struct LVWACK_R(crate::FieldReader<bool, bool>);
impl LVWACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVWACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVWACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVWACK` writer - Low-Voltage Warning Acknowledge"]
pub struct LVWACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LVWACK_W<'a> {
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
#[doc = "Low-Voltage Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWF_A {
    #[doc = "0: Low-voltage warning is not present."]
    _0 = 0,
    #[doc = "1: Low-voltage warning is present or was present."]
    _1 = 1,
}
impl From<LVWF_A> for bool {
    #[inline(always)]
    fn from(variant: LVWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVWF` reader - Low-Voltage Warning Flag"]
pub struct LVWF_R(crate::FieldReader<bool, LVWF_A>);
impl LVWF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVWF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWF_A {
        match self.bits {
            false => LVWF_A::_0,
            true => LVWF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVWF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVWF_A::_1
    }
}
impl core::ops::Deref for LVWF_R {
    type Target = crate::FieldReader<bool, LVWF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline(always)]
    pub fn bgbe(&self) -> BGBE_R {
        BGBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low-Voltage Detect Enable"]
    #[inline(always)]
    pub fn lvde(&self) -> LVDE_R {
        LVDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Low-Voltage Detect Stop Enable"]
    #[inline(always)]
    pub fn lvdse(&self) -> LVDSE_R {
        LVDSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Low-Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&self) -> LVDRE_R {
        LVDRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    pub fn lvwie(&self) -> LVWIE_R {
        LVWIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Low-Voltage Warning Acknowledge"]
    #[inline(always)]
    pub fn lvwack(&self) -> LVWACK_R {
        LVWACK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low-Voltage Warning Flag"]
    #[inline(always)]
    pub fn lvwf(&self) -> LVWF_R {
        LVWF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline(always)]
    pub fn bgbe(&mut self) -> BGBE_W {
        BGBE_W { w: self }
    }
    #[doc = "Bit 2 - Low-Voltage Detect Enable"]
    #[inline(always)]
    pub fn lvde(&mut self) -> LVDE_W {
        LVDE_W { w: self }
    }
    #[doc = "Bit 3 - Low-Voltage Detect Stop Enable"]
    #[inline(always)]
    pub fn lvdse(&mut self) -> LVDSE_W {
        LVDSE_W { w: self }
    }
    #[doc = "Bit 4 - Low-Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&mut self) -> LVDRE_W {
        LVDRE_W { w: self }
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    pub fn lvwie(&mut self) -> LVWIE_W {
        LVWIE_W { w: self }
    }
    #[doc = "Bit 6 - Low-Voltage Warning Acknowledge"]
    #[inline(always)]
    pub fn lvwack(&mut self) -> LVWACK_W {
        LVWACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Power Management Status and Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spmsc1](index.html) module"]
pub struct SPMSC1_SPEC;
impl crate::RegisterSpec for SPMSC1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spmsc1::R](R) reader structure"]
impl crate::Readable for SPMSC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spmsc1::W](W) writer structure"]
impl crate::Writable for SPMSC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPMSC1 to value 0x1c"]
impl crate::Resettable for SPMSC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1c
    }
}
