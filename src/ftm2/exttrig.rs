#[doc = "Register `EXTTRIG` reader"]
pub struct R(crate::R<EXTTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EXTTRIG_SPEC>> for R {
    fn from(reader: crate::R<EXTTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTTRIG` writer"]
pub struct W(crate::W<EXTTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTTRIG_SPEC>;
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
impl core::convert::From<crate::W<EXTTRIG_SPEC>> for W {
    fn from(writer: crate::W<EXTTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 2 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2TRIG_A {
    #[doc = "0: The generation of the channel trigger is disabled."]
    _0 = 0,
    #[doc = "1: The generation of the channel trigger is enabled."]
    _1 = 1,
}
impl From<CH2TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: CH2TRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2TRIG` reader - Channel 2 Trigger Enable"]
pub struct CH2TRIG_R(crate::FieldReader<bool, CH2TRIG_A>);
impl CH2TRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2TRIG_A {
        match self.bits {
            false => CH2TRIG_A::_0,
            true => CH2TRIG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH2TRIG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH2TRIG_A::_1
    }
}
impl core::ops::Deref for CH2TRIG_R {
    type Target = crate::FieldReader<bool, CH2TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2TRIG` writer - Channel 2 Trigger Enable"]
pub struct CH2TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2TRIG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The generation of the channel trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2TRIG_A::_0)
    }
    #[doc = "The generation of the channel trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2TRIG_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Channel 3 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3TRIG_A {
    #[doc = "0: The generation of the channel trigger is disabled."]
    _0 = 0,
    #[doc = "1: The generation of the channel trigger is enabled."]
    _1 = 1,
}
impl From<CH3TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: CH3TRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3TRIG` reader - Channel 3 Trigger Enable"]
pub struct CH3TRIG_R(crate::FieldReader<bool, CH3TRIG_A>);
impl CH3TRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3TRIG_A {
        match self.bits {
            false => CH3TRIG_A::_0,
            true => CH3TRIG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH3TRIG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH3TRIG_A::_1
    }
}
impl core::ops::Deref for CH3TRIG_R {
    type Target = crate::FieldReader<bool, CH3TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3TRIG` writer - Channel 3 Trigger Enable"]
pub struct CH3TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3TRIG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The generation of the channel trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3TRIG_A::_0)
    }
    #[doc = "The generation of the channel trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3TRIG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Channel 4 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4TRIG_A {
    #[doc = "0: The generation of the channel trigger is disabled."]
    _0 = 0,
    #[doc = "1: The generation of the channel trigger is enabled."]
    _1 = 1,
}
impl From<CH4TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: CH4TRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4TRIG` reader - Channel 4 Trigger Enable"]
pub struct CH4TRIG_R(crate::FieldReader<bool, CH4TRIG_A>);
impl CH4TRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4TRIG_A {
        match self.bits {
            false => CH4TRIG_A::_0,
            true => CH4TRIG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH4TRIG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH4TRIG_A::_1
    }
}
impl core::ops::Deref for CH4TRIG_R {
    type Target = crate::FieldReader<bool, CH4TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4TRIG` writer - Channel 4 Trigger Enable"]
pub struct CH4TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4TRIG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The generation of the channel trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4TRIG_A::_0)
    }
    #[doc = "The generation of the channel trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4TRIG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Channel 5 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5TRIG_A {
    #[doc = "0: The generation of the channel trigger is disabled."]
    _0 = 0,
    #[doc = "1: The generation of the channel trigger is enabled."]
    _1 = 1,
}
impl From<CH5TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: CH5TRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5TRIG` reader - Channel 5 Trigger Enable"]
pub struct CH5TRIG_R(crate::FieldReader<bool, CH5TRIG_A>);
impl CH5TRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5TRIG_A {
        match self.bits {
            false => CH5TRIG_A::_0,
            true => CH5TRIG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH5TRIG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH5TRIG_A::_1
    }
}
impl core::ops::Deref for CH5TRIG_R {
    type Target = crate::FieldReader<bool, CH5TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5TRIG` writer - Channel 5 Trigger Enable"]
pub struct CH5TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5TRIG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The generation of the channel trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5TRIG_A::_0)
    }
    #[doc = "The generation of the channel trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5TRIG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Channel 0 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0TRIG_A {
    #[doc = "0: The generation of the channel trigger is disabled."]
    _0 = 0,
    #[doc = "1: The generation of the channel trigger is enabled."]
    _1 = 1,
}
impl From<CH0TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: CH0TRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0TRIG` reader - Channel 0 Trigger Enable"]
pub struct CH0TRIG_R(crate::FieldReader<bool, CH0TRIG_A>);
impl CH0TRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0TRIG_A {
        match self.bits {
            false => CH0TRIG_A::_0,
            true => CH0TRIG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH0TRIG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH0TRIG_A::_1
    }
}
impl core::ops::Deref for CH0TRIG_R {
    type Target = crate::FieldReader<bool, CH0TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0TRIG` writer - Channel 0 Trigger Enable"]
pub struct CH0TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0TRIG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The generation of the channel trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0TRIG_A::_0)
    }
    #[doc = "The generation of the channel trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0TRIG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Channel 1 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1TRIG_A {
    #[doc = "0: The generation of the channel trigger is disabled."]
    _0 = 0,
    #[doc = "1: The generation of the channel trigger is enabled."]
    _1 = 1,
}
impl From<CH1TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: CH1TRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1TRIG` reader - Channel 1 Trigger Enable"]
pub struct CH1TRIG_R(crate::FieldReader<bool, CH1TRIG_A>);
impl CH1TRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1TRIG_A {
        match self.bits {
            false => CH1TRIG_A::_0,
            true => CH1TRIG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH1TRIG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH1TRIG_A::_1
    }
}
impl core::ops::Deref for CH1TRIG_R {
    type Target = crate::FieldReader<bool, CH1TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1TRIG` writer - Channel 1 Trigger Enable"]
pub struct CH1TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1TRIG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The generation of the channel trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1TRIG_A::_0)
    }
    #[doc = "The generation of the channel trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1TRIG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Initialization Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITTRIGEN_A {
    #[doc = "0: The generation of initialization trigger is disabled."]
    _0 = 0,
    #[doc = "1: The generation of initialization trigger is enabled."]
    _1 = 1,
}
impl From<INITTRIGEN_A> for bool {
    #[inline(always)]
    fn from(variant: INITTRIGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITTRIGEN` reader - Initialization Trigger Enable"]
pub struct INITTRIGEN_R(crate::FieldReader<bool, INITTRIGEN_A>);
impl INITTRIGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITTRIGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITTRIGEN_A {
        match self.bits {
            false => INITTRIGEN_A::_0,
            true => INITTRIGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INITTRIGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INITTRIGEN_A::_1
    }
}
impl core::ops::Deref for INITTRIGEN_R {
    type Target = crate::FieldReader<bool, INITTRIGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITTRIGEN` writer - Initialization Trigger Enable"]
pub struct INITTRIGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INITTRIGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INITTRIGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The generation of initialization trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITTRIGEN_A::_0)
    }
    #[doc = "The generation of initialization trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITTRIGEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Channel Trigger Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGF_A {
    #[doc = "0: No channel trigger was generated."]
    _0 = 0,
    #[doc = "1: A channel trigger was generated."]
    _1 = 1,
}
impl From<TRIGF_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGF` reader - Channel Trigger Flag"]
pub struct TRIGF_R(crate::FieldReader<bool, TRIGF_A>);
impl TRIGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRIGF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGF_A {
        match self.bits {
            false => TRIGF_A::_0,
            true => TRIGF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRIGF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRIGF_A::_1
    }
}
impl core::ops::Deref for TRIGF_R {
    type Target = crate::FieldReader<bool, TRIGF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Channel 2 Trigger Enable"]
    #[inline(always)]
    pub fn ch2trig(&self) -> CH2TRIG_R {
        CH2TRIG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 3 Trigger Enable"]
    #[inline(always)]
    pub fn ch3trig(&self) -> CH3TRIG_R {
        CH3TRIG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 4 Trigger Enable"]
    #[inline(always)]
    pub fn ch4trig(&self) -> CH4TRIG_R {
        CH4TRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 5 Trigger Enable"]
    #[inline(always)]
    pub fn ch5trig(&self) -> CH5TRIG_R {
        CH5TRIG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Trigger Enable"]
    #[inline(always)]
    pub fn ch0trig(&self) -> CH0TRIG_R {
        CH0TRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Trigger Enable"]
    #[inline(always)]
    pub fn ch1trig(&self) -> CH1TRIG_R {
        CH1TRIG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Initialization Trigger Enable"]
    #[inline(always)]
    pub fn inittrigen(&self) -> INITTRIGEN_R {
        INITTRIGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel Trigger Flag"]
    #[inline(always)]
    pub fn trigf(&self) -> TRIGF_R {
        TRIGF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 2 Trigger Enable"]
    #[inline(always)]
    pub fn ch2trig(&mut self) -> CH2TRIG_W {
        CH2TRIG_W { w: self }
    }
    #[doc = "Bit 1 - Channel 3 Trigger Enable"]
    #[inline(always)]
    pub fn ch3trig(&mut self) -> CH3TRIG_W {
        CH3TRIG_W { w: self }
    }
    #[doc = "Bit 2 - Channel 4 Trigger Enable"]
    #[inline(always)]
    pub fn ch4trig(&mut self) -> CH4TRIG_W {
        CH4TRIG_W { w: self }
    }
    #[doc = "Bit 3 - Channel 5 Trigger Enable"]
    #[inline(always)]
    pub fn ch5trig(&mut self) -> CH5TRIG_W {
        CH5TRIG_W { w: self }
    }
    #[doc = "Bit 4 - Channel 0 Trigger Enable"]
    #[inline(always)]
    pub fn ch0trig(&mut self) -> CH0TRIG_W {
        CH0TRIG_W { w: self }
    }
    #[doc = "Bit 5 - Channel 1 Trigger Enable"]
    #[inline(always)]
    pub fn ch1trig(&mut self) -> CH1TRIG_W {
        CH1TRIG_W { w: self }
    }
    #[doc = "Bit 6 - Initialization Trigger Enable"]
    #[inline(always)]
    pub fn inittrigen(&mut self) -> INITTRIGEN_W {
        INITTRIGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FTM External Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exttrig](index.html) module"]
pub struct EXTTRIG_SPEC;
impl crate::RegisterSpec for EXTTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exttrig::R](R) reader structure"]
impl crate::Readable for EXTTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exttrig::W](W) writer structure"]
impl crate::Writable for EXTTRIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTTRIG to value 0"]
impl crate::Resettable for EXTTRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
