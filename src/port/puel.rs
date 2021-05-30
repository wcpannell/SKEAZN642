#[doc = "Register `PUEL` reader"]
pub struct R(crate::R<PUEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PUEL_SPEC>> for R {
    fn from(reader: crate::R<PUEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUEL` writer"]
pub struct W(crate::W<PUEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUEL_SPEC>;
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
impl core::convert::From<crate::W<PUEL_SPEC>> for W {
    fn from(writer: crate::W<PUEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pull Enable for Port A Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE0_A {
    #[doc = "0: Pullup is disabled for port A bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 0."]
    _1 = 1,
}
impl From<PTAPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTAPE0` reader - Pull Enable for Port A Bit 0"]
pub struct PTAPE0_R(crate::FieldReader<bool, PTAPE0_A>);
impl PTAPE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTAPE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE0_A {
        match self.bits {
            false => PTAPE0_A::_0,
            true => PTAPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTAPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTAPE0_A::_1
    }
}
impl core::ops::Deref for PTAPE0_R {
    type Target = crate::FieldReader<bool, PTAPE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTAPE0` writer - Pull Enable for Port A Bit 0"]
pub struct PTAPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port A bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE0_A::_1)
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
#[doc = "Pull Enable for Port A Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE1_A {
    #[doc = "0: Pullup is disabled for port A bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 1."]
    _1 = 1,
}
impl From<PTAPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTAPE1` reader - Pull Enable for Port A Bit 1"]
pub struct PTAPE1_R(crate::FieldReader<bool, PTAPE1_A>);
impl PTAPE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTAPE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE1_A {
        match self.bits {
            false => PTAPE1_A::_0,
            true => PTAPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTAPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTAPE1_A::_1
    }
}
impl core::ops::Deref for PTAPE1_R {
    type Target = crate::FieldReader<bool, PTAPE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTAPE1` writer - Pull Enable for Port A Bit 1"]
pub struct PTAPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port A bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE1_A::_1)
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
#[doc = "Pull Enable for Port A Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE2_A {
    #[doc = "0: Pullup is disabled for port A bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 2."]
    _1 = 1,
}
impl From<PTAPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTAPE2` reader - Pull Enable for Port A Bit 2"]
pub struct PTAPE2_R(crate::FieldReader<bool, PTAPE2_A>);
impl PTAPE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTAPE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE2_A {
        match self.bits {
            false => PTAPE2_A::_0,
            true => PTAPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTAPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTAPE2_A::_1
    }
}
impl core::ops::Deref for PTAPE2_R {
    type Target = crate::FieldReader<bool, PTAPE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTAPE2` writer - Pull Enable for Port A Bit 2"]
pub struct PTAPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port A bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE2_A::_1)
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
#[doc = "Pull Enable for Port A Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE3_A {
    #[doc = "0: Pullup is disabled for port A bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 3."]
    _1 = 1,
}
impl From<PTAPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTAPE3` reader - Pull Enable for Port A Bit 3"]
pub struct PTAPE3_R(crate::FieldReader<bool, PTAPE3_A>);
impl PTAPE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTAPE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE3_A {
        match self.bits {
            false => PTAPE3_A::_0,
            true => PTAPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTAPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTAPE3_A::_1
    }
}
impl core::ops::Deref for PTAPE3_R {
    type Target = crate::FieldReader<bool, PTAPE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTAPE3` writer - Pull Enable for Port A Bit 3"]
pub struct PTAPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port A bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE3_A::_1)
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
#[doc = "Pull Enable for Port A Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE4_A {
    #[doc = "0: Pullup is disabled for port A bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 4."]
    _1 = 1,
}
impl From<PTAPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTAPE4` reader - Pull Enable for Port A Bit 4"]
pub struct PTAPE4_R(crate::FieldReader<bool, PTAPE4_A>);
impl PTAPE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTAPE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE4_A {
        match self.bits {
            false => PTAPE4_A::_0,
            true => PTAPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTAPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTAPE4_A::_1
    }
}
impl core::ops::Deref for PTAPE4_R {
    type Target = crate::FieldReader<bool, PTAPE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTAPE4` writer - Pull Enable for Port A Bit 4"]
pub struct PTAPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port A bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE4_A::_1)
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
#[doc = "Pull Enable for Port A Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE5_A {
    #[doc = "0: Pullup is disabled for port A bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 5."]
    _1 = 1,
}
impl From<PTAPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTAPE5` reader - Pull Enable for Port A Bit 5"]
pub struct PTAPE5_R(crate::FieldReader<bool, PTAPE5_A>);
impl PTAPE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTAPE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE5_A {
        match self.bits {
            false => PTAPE5_A::_0,
            true => PTAPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTAPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTAPE5_A::_1
    }
}
impl core::ops::Deref for PTAPE5_R {
    type Target = crate::FieldReader<bool, PTAPE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTAPE5` writer - Pull Enable for Port A Bit 5"]
pub struct PTAPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port A bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE5_A::_1)
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
#[doc = "Pull Enable for Port A Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE6_A {
    #[doc = "0: Pullup is disabled for port A bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 6."]
    _1 = 1,
}
impl From<PTAPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTAPE6` reader - Pull Enable for Port A Bit 6"]
pub struct PTAPE6_R(crate::FieldReader<bool, PTAPE6_A>);
impl PTAPE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTAPE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE6_A {
        match self.bits {
            false => PTAPE6_A::_0,
            true => PTAPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTAPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTAPE6_A::_1
    }
}
impl core::ops::Deref for PTAPE6_R {
    type Target = crate::FieldReader<bool, PTAPE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTAPE6` writer - Pull Enable for Port A Bit 6"]
pub struct PTAPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port A bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE6_A::_1)
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
#[doc = "Pull Enable for Port A Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTAPE7_A {
    #[doc = "0: Pullup is disabled for port A bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port A bit 7."]
    _1 = 1,
}
impl From<PTAPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTAPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTAPE7` reader - Pull Enable for Port A Bit 7"]
pub struct PTAPE7_R(crate::FieldReader<bool, PTAPE7_A>);
impl PTAPE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTAPE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTAPE7_A {
        match self.bits {
            false => PTAPE7_A::_0,
            true => PTAPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTAPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTAPE7_A::_1
    }
}
impl core::ops::Deref for PTAPE7_R {
    type Target = crate::FieldReader<bool, PTAPE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTAPE7` writer - Pull Enable for Port A Bit 7"]
pub struct PTAPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTAPE7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port A bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTAPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port A bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTAPE7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Pull Enable for Port B Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE0_A {
    #[doc = "0: Pullup is disabled for port B bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 0."]
    _1 = 1,
}
impl From<PTBPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTBPE0` reader - Pull Enable for Port B Bit 0"]
pub struct PTBPE0_R(crate::FieldReader<bool, PTBPE0_A>);
impl PTBPE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTBPE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE0_A {
        match self.bits {
            false => PTBPE0_A::_0,
            true => PTBPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTBPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTBPE0_A::_1
    }
}
impl core::ops::Deref for PTBPE0_R {
    type Target = crate::FieldReader<bool, PTBPE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTBPE0` writer - Pull Enable for Port B Bit 0"]
pub struct PTBPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port B bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Pull Enable for Port B Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE1_A {
    #[doc = "0: Pullup is disabled for port B bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 1."]
    _1 = 1,
}
impl From<PTBPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTBPE1` reader - Pull Enable for Port B Bit 1"]
pub struct PTBPE1_R(crate::FieldReader<bool, PTBPE1_A>);
impl PTBPE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTBPE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE1_A {
        match self.bits {
            false => PTBPE1_A::_0,
            true => PTBPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTBPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTBPE1_A::_1
    }
}
impl core::ops::Deref for PTBPE1_R {
    type Target = crate::FieldReader<bool, PTBPE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTBPE1` writer - Pull Enable for Port B Bit 1"]
pub struct PTBPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port B bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Pull Enable for Port B Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE2_A {
    #[doc = "0: Pullup is disabled for port B bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 2."]
    _1 = 1,
}
impl From<PTBPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTBPE2` reader - Pull Enable for Port B Bit 2"]
pub struct PTBPE2_R(crate::FieldReader<bool, PTBPE2_A>);
impl PTBPE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTBPE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE2_A {
        match self.bits {
            false => PTBPE2_A::_0,
            true => PTBPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTBPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTBPE2_A::_1
    }
}
impl core::ops::Deref for PTBPE2_R {
    type Target = crate::FieldReader<bool, PTBPE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTBPE2` writer - Pull Enable for Port B Bit 2"]
pub struct PTBPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port B bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Pull Enable for Port B Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE3_A {
    #[doc = "0: Pullup is disabled for port B bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 3."]
    _1 = 1,
}
impl From<PTBPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTBPE3` reader - Pull Enable for Port B Bit 3"]
pub struct PTBPE3_R(crate::FieldReader<bool, PTBPE3_A>);
impl PTBPE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTBPE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE3_A {
        match self.bits {
            false => PTBPE3_A::_0,
            true => PTBPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTBPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTBPE3_A::_1
    }
}
impl core::ops::Deref for PTBPE3_R {
    type Target = crate::FieldReader<bool, PTBPE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTBPE3` writer - Pull Enable for Port B Bit 3"]
pub struct PTBPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port B bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Pull Enable for Port B Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE4_A {
    #[doc = "0: Pullup is disabled for port B bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 4."]
    _1 = 1,
}
impl From<PTBPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTBPE4` reader - Pull Enable for Port B Bit 4"]
pub struct PTBPE4_R(crate::FieldReader<bool, PTBPE4_A>);
impl PTBPE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTBPE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE4_A {
        match self.bits {
            false => PTBPE4_A::_0,
            true => PTBPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTBPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTBPE4_A::_1
    }
}
impl core::ops::Deref for PTBPE4_R {
    type Target = crate::FieldReader<bool, PTBPE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTBPE4` writer - Pull Enable for Port B Bit 4"]
pub struct PTBPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port B bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Pull Enable for Port B Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE5_A {
    #[doc = "0: Pullup is disabled for port B bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 5."]
    _1 = 1,
}
impl From<PTBPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTBPE5` reader - Pull Enable for Port B Bit 5"]
pub struct PTBPE5_R(crate::FieldReader<bool, PTBPE5_A>);
impl PTBPE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTBPE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE5_A {
        match self.bits {
            false => PTBPE5_A::_0,
            true => PTBPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTBPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTBPE5_A::_1
    }
}
impl core::ops::Deref for PTBPE5_R {
    type Target = crate::FieldReader<bool, PTBPE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTBPE5` writer - Pull Enable for Port B Bit 5"]
pub struct PTBPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port B bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Pull Enable for Port B Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE6_A {
    #[doc = "0: Pullup is disabled for port B bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 6."]
    _1 = 1,
}
impl From<PTBPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTBPE6` reader - Pull Enable for Port B Bit 6"]
pub struct PTBPE6_R(crate::FieldReader<bool, PTBPE6_A>);
impl PTBPE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTBPE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE6_A {
        match self.bits {
            false => PTBPE6_A::_0,
            true => PTBPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTBPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTBPE6_A::_1
    }
}
impl core::ops::Deref for PTBPE6_R {
    type Target = crate::FieldReader<bool, PTBPE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTBPE6` writer - Pull Enable for Port B Bit 6"]
pub struct PTBPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port B bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Pull Enable for Port B Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTBPE7_A {
    #[doc = "0: Pullup is disabled for port B bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port B bit 7."]
    _1 = 1,
}
impl From<PTBPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTBPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTBPE7` reader - Pull Enable for Port B Bit 7"]
pub struct PTBPE7_R(crate::FieldReader<bool, PTBPE7_A>);
impl PTBPE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTBPE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTBPE7_A {
        match self.bits {
            false => PTBPE7_A::_0,
            true => PTBPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTBPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTBPE7_A::_1
    }
}
impl core::ops::Deref for PTBPE7_R {
    type Target = crate::FieldReader<bool, PTBPE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTBPE7` writer - Pull Enable for Port B Bit 7"]
pub struct PTBPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTBPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTBPE7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port B bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTBPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port B bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTBPE7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE0_A {
    #[doc = "0: Pullup is disabled for port C bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 0."]
    _1 = 1,
}
impl From<PTCPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCPE0` reader - Pull Enable for Port C Bit 0"]
pub struct PTCPE0_R(crate::FieldReader<bool, PTCPE0_A>);
impl PTCPE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTCPE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE0_A {
        match self.bits {
            false => PTCPE0_A::_0,
            true => PTCPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTCPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTCPE0_A::_1
    }
}
impl core::ops::Deref for PTCPE0_R {
    type Target = crate::FieldReader<bool, PTCPE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTCPE0` writer - Pull Enable for Port C Bit 0"]
pub struct PTCPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port C bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE1_A {
    #[doc = "0: Pullup is disabled for port C bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 1."]
    _1 = 1,
}
impl From<PTCPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCPE1` reader - Pull Enable for Port C Bit 1"]
pub struct PTCPE1_R(crate::FieldReader<bool, PTCPE1_A>);
impl PTCPE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTCPE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE1_A {
        match self.bits {
            false => PTCPE1_A::_0,
            true => PTCPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTCPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTCPE1_A::_1
    }
}
impl core::ops::Deref for PTCPE1_R {
    type Target = crate::FieldReader<bool, PTCPE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTCPE1` writer - Pull Enable for Port C Bit 1"]
pub struct PTCPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port C bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE2_A {
    #[doc = "0: Pullup is disabled for port C bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 2."]
    _1 = 1,
}
impl From<PTCPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCPE2` reader - Pull Enable for Port C Bit 2"]
pub struct PTCPE2_R(crate::FieldReader<bool, PTCPE2_A>);
impl PTCPE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTCPE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE2_A {
        match self.bits {
            false => PTCPE2_A::_0,
            true => PTCPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTCPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTCPE2_A::_1
    }
}
impl core::ops::Deref for PTCPE2_R {
    type Target = crate::FieldReader<bool, PTCPE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTCPE2` writer - Pull Enable for Port C Bit 2"]
pub struct PTCPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port C bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE3_A {
    #[doc = "0: Pullup is disabled for port C bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 3."]
    _1 = 1,
}
impl From<PTCPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCPE3` reader - Pull Enable for Port C Bit 3"]
pub struct PTCPE3_R(crate::FieldReader<bool, PTCPE3_A>);
impl PTCPE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTCPE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE3_A {
        match self.bits {
            false => PTCPE3_A::_0,
            true => PTCPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTCPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTCPE3_A::_1
    }
}
impl core::ops::Deref for PTCPE3_R {
    type Target = crate::FieldReader<bool, PTCPE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTCPE3` writer - Pull Enable for Port C Bit 3"]
pub struct PTCPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port C bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE4_A {
    #[doc = "0: Pullup is disabled for port C bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 4."]
    _1 = 1,
}
impl From<PTCPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCPE4` reader - Pull Enable for Port C Bit 4"]
pub struct PTCPE4_R(crate::FieldReader<bool, PTCPE4_A>);
impl PTCPE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTCPE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE4_A {
        match self.bits {
            false => PTCPE4_A::_0,
            true => PTCPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTCPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTCPE4_A::_1
    }
}
impl core::ops::Deref for PTCPE4_R {
    type Target = crate::FieldReader<bool, PTCPE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTCPE4` writer - Pull Enable for Port C Bit 4"]
pub struct PTCPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port C bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE5_A {
    #[doc = "0: Pullup is disabled for port C bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 5."]
    _1 = 1,
}
impl From<PTCPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCPE5` reader - Pull Enable for Port C Bit 5"]
pub struct PTCPE5_R(crate::FieldReader<bool, PTCPE5_A>);
impl PTCPE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTCPE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE5_A {
        match self.bits {
            false => PTCPE5_A::_0,
            true => PTCPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTCPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTCPE5_A::_1
    }
}
impl core::ops::Deref for PTCPE5_R {
    type Target = crate::FieldReader<bool, PTCPE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTCPE5` writer - Pull Enable for Port C Bit 5"]
pub struct PTCPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port C bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE6_A {
    #[doc = "0: Pullup is disabled for port C bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 6."]
    _1 = 1,
}
impl From<PTCPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCPE6` reader - Pull Enable for Port C Bit 6"]
pub struct PTCPE6_R(crate::FieldReader<bool, PTCPE6_A>);
impl PTCPE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTCPE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE6_A {
        match self.bits {
            false => PTCPE6_A::_0,
            true => PTCPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTCPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTCPE6_A::_1
    }
}
impl core::ops::Deref for PTCPE6_R {
    type Target = crate::FieldReader<bool, PTCPE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTCPE6` writer - Pull Enable for Port C Bit 6"]
pub struct PTCPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port C bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Pull Enable for Port C Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCPE7_A {
    #[doc = "0: Pullup is disabled for port C bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port C bit 7."]
    _1 = 1,
}
impl From<PTCPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTCPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCPE7` reader - Pull Enable for Port C Bit 7"]
pub struct PTCPE7_R(crate::FieldReader<bool, PTCPE7_A>);
impl PTCPE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTCPE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTCPE7_A {
        match self.bits {
            false => PTCPE7_A::_0,
            true => PTCPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTCPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTCPE7_A::_1
    }
}
impl core::ops::Deref for PTCPE7_R {
    type Target = crate::FieldReader<bool, PTCPE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTCPE7` writer - Pull Enable for Port C Bit 7"]
pub struct PTCPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTCPE7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port C bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port C bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCPE7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE0_A {
    #[doc = "0: Pullup is disabled for port D bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 0."]
    _1 = 1,
}
impl From<PTDPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTDPE0` reader - Pull Enable for Port D Bit 0"]
pub struct PTDPE0_R(crate::FieldReader<bool, PTDPE0_A>);
impl PTDPE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTDPE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE0_A {
        match self.bits {
            false => PTDPE0_A::_0,
            true => PTDPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTDPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTDPE0_A::_1
    }
}
impl core::ops::Deref for PTDPE0_R {
    type Target = crate::FieldReader<bool, PTDPE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTDPE0` writer - Pull Enable for Port D Bit 0"]
pub struct PTDPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port D bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE1_A {
    #[doc = "0: Pullup is disabled for port D bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 1."]
    _1 = 1,
}
impl From<PTDPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTDPE1` reader - Pull Enable for Port D Bit 1"]
pub struct PTDPE1_R(crate::FieldReader<bool, PTDPE1_A>);
impl PTDPE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTDPE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE1_A {
        match self.bits {
            false => PTDPE1_A::_0,
            true => PTDPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTDPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTDPE1_A::_1
    }
}
impl core::ops::Deref for PTDPE1_R {
    type Target = crate::FieldReader<bool, PTDPE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTDPE1` writer - Pull Enable for Port D Bit 1"]
pub struct PTDPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port D bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE2_A {
    #[doc = "0: Pullup is disabled for port D bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 2."]
    _1 = 1,
}
impl From<PTDPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTDPE2` reader - Pull Enable for Port D Bit 2"]
pub struct PTDPE2_R(crate::FieldReader<bool, PTDPE2_A>);
impl PTDPE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTDPE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE2_A {
        match self.bits {
            false => PTDPE2_A::_0,
            true => PTDPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTDPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTDPE2_A::_1
    }
}
impl core::ops::Deref for PTDPE2_R {
    type Target = crate::FieldReader<bool, PTDPE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTDPE2` writer - Pull Enable for Port D Bit 2"]
pub struct PTDPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port D bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE3_A {
    #[doc = "0: Pullup is disabled for port D bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 3."]
    _1 = 1,
}
impl From<PTDPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTDPE3` reader - Pull Enable for Port D Bit 3"]
pub struct PTDPE3_R(crate::FieldReader<bool, PTDPE3_A>);
impl PTDPE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTDPE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE3_A {
        match self.bits {
            false => PTDPE3_A::_0,
            true => PTDPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTDPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTDPE3_A::_1
    }
}
impl core::ops::Deref for PTDPE3_R {
    type Target = crate::FieldReader<bool, PTDPE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTDPE3` writer - Pull Enable for Port D Bit 3"]
pub struct PTDPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port D bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE4_A {
    #[doc = "0: Pullup is disabled for port D bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 4."]
    _1 = 1,
}
impl From<PTDPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTDPE4` reader - Pull Enable for Port D Bit 4"]
pub struct PTDPE4_R(crate::FieldReader<bool, PTDPE4_A>);
impl PTDPE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTDPE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE4_A {
        match self.bits {
            false => PTDPE4_A::_0,
            true => PTDPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTDPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTDPE4_A::_1
    }
}
impl core::ops::Deref for PTDPE4_R {
    type Target = crate::FieldReader<bool, PTDPE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTDPE4` writer - Pull Enable for Port D Bit 4"]
pub struct PTDPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port D bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE5_A {
    #[doc = "0: Pullup is disabled for port D bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 5."]
    _1 = 1,
}
impl From<PTDPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTDPE5` reader - Pull Enable for Port D Bit 5"]
pub struct PTDPE5_R(crate::FieldReader<bool, PTDPE5_A>);
impl PTDPE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTDPE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE5_A {
        match self.bits {
            false => PTDPE5_A::_0,
            true => PTDPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTDPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTDPE5_A::_1
    }
}
impl core::ops::Deref for PTDPE5_R {
    type Target = crate::FieldReader<bool, PTDPE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTDPE5` writer - Pull Enable for Port D Bit 5"]
pub struct PTDPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port D bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE6_A {
    #[doc = "0: Pullup is disabled for port D bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 6."]
    _1 = 1,
}
impl From<PTDPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTDPE6` reader - Pull Enable for Port D Bit 6"]
pub struct PTDPE6_R(crate::FieldReader<bool, PTDPE6_A>);
impl PTDPE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTDPE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE6_A {
        match self.bits {
            false => PTDPE6_A::_0,
            true => PTDPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTDPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTDPE6_A::_1
    }
}
impl core::ops::Deref for PTDPE6_R {
    type Target = crate::FieldReader<bool, PTDPE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTDPE6` writer - Pull Enable for Port D Bit 6"]
pub struct PTDPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port D bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Pull Enable for Port D Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTDPE7_A {
    #[doc = "0: Pullup is disabled for port D bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port D bit 7."]
    _1 = 1,
}
impl From<PTDPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTDPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTDPE7` reader - Pull Enable for Port D Bit 7"]
pub struct PTDPE7_R(crate::FieldReader<bool, PTDPE7_A>);
impl PTDPE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTDPE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTDPE7_A {
        match self.bits {
            false => PTDPE7_A::_0,
            true => PTDPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTDPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTDPE7_A::_1
    }
}
impl core::ops::Deref for PTDPE7_R {
    type Target = crate::FieldReader<bool, PTDPE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTDPE7` writer - Pull Enable for Port D Bit 7"]
pub struct PTDPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTDPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTDPE7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port D bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTDPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port D bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTDPE7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pull Enable for Port A Bit 0"]
    #[inline(always)]
    pub fn ptape0(&self) -> PTAPE0_R {
        PTAPE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pull Enable for Port A Bit 1"]
    #[inline(always)]
    pub fn ptape1(&self) -> PTAPE1_R {
        PTAPE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pull Enable for Port A Bit 2"]
    #[inline(always)]
    pub fn ptape2(&self) -> PTAPE2_R {
        PTAPE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pull Enable for Port A Bit 3"]
    #[inline(always)]
    pub fn ptape3(&self) -> PTAPE3_R {
        PTAPE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pull Enable for Port A Bit 4"]
    #[inline(always)]
    pub fn ptape4(&self) -> PTAPE4_R {
        PTAPE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pull Enable for Port A Bit 5"]
    #[inline(always)]
    pub fn ptape5(&self) -> PTAPE5_R {
        PTAPE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pull Enable for Port A Bit 6"]
    #[inline(always)]
    pub fn ptape6(&self) -> PTAPE6_R {
        PTAPE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pull Enable for Port A Bit 7"]
    #[inline(always)]
    pub fn ptape7(&self) -> PTAPE7_R {
        PTAPE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pull Enable for Port B Bit 0"]
    #[inline(always)]
    pub fn ptbpe0(&self) -> PTBPE0_R {
        PTBPE0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pull Enable for Port B Bit 1"]
    #[inline(always)]
    pub fn ptbpe1(&self) -> PTBPE1_R {
        PTBPE1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pull Enable for Port B Bit 2"]
    #[inline(always)]
    pub fn ptbpe2(&self) -> PTBPE2_R {
        PTBPE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pull Enable for Port B Bit 3"]
    #[inline(always)]
    pub fn ptbpe3(&self) -> PTBPE3_R {
        PTBPE3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pull Enable for Port B Bit 4"]
    #[inline(always)]
    pub fn ptbpe4(&self) -> PTBPE4_R {
        PTBPE4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pull Enable for Port B Bit 5"]
    #[inline(always)]
    pub fn ptbpe5(&self) -> PTBPE5_R {
        PTBPE5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pull Enable for Port B Bit 6"]
    #[inline(always)]
    pub fn ptbpe6(&self) -> PTBPE6_R {
        PTBPE6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pull Enable for Port B Bit 7"]
    #[inline(always)]
    pub fn ptbpe7(&self) -> PTBPE7_R {
        PTBPE7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pull Enable for Port C Bit 0"]
    #[inline(always)]
    pub fn ptcpe0(&self) -> PTCPE0_R {
        PTCPE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pull Enable for Port C Bit 1"]
    #[inline(always)]
    pub fn ptcpe1(&self) -> PTCPE1_R {
        PTCPE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pull Enable for Port C Bit 2"]
    #[inline(always)]
    pub fn ptcpe2(&self) -> PTCPE2_R {
        PTCPE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Pull Enable for Port C Bit 3"]
    #[inline(always)]
    pub fn ptcpe3(&self) -> PTCPE3_R {
        PTCPE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pull Enable for Port C Bit 4"]
    #[inline(always)]
    pub fn ptcpe4(&self) -> PTCPE4_R {
        PTCPE4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Pull Enable for Port C Bit 5"]
    #[inline(always)]
    pub fn ptcpe5(&self) -> PTCPE5_R {
        PTCPE5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Pull Enable for Port C Bit 6"]
    #[inline(always)]
    pub fn ptcpe6(&self) -> PTCPE6_R {
        PTCPE6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Pull Enable for Port C Bit 7"]
    #[inline(always)]
    pub fn ptcpe7(&self) -> PTCPE7_R {
        PTCPE7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pull Enable for Port D Bit 0"]
    #[inline(always)]
    pub fn ptdpe0(&self) -> PTDPE0_R {
        PTDPE0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pull Enable for Port D Bit 1"]
    #[inline(always)]
    pub fn ptdpe1(&self) -> PTDPE1_R {
        PTDPE1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Pull Enable for Port D Bit 2"]
    #[inline(always)]
    pub fn ptdpe2(&self) -> PTDPE2_R {
        PTDPE2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Pull Enable for Port D Bit 3"]
    #[inline(always)]
    pub fn ptdpe3(&self) -> PTDPE3_R {
        PTDPE3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Pull Enable for Port D Bit 4"]
    #[inline(always)]
    pub fn ptdpe4(&self) -> PTDPE4_R {
        PTDPE4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Pull Enable for Port D Bit 5"]
    #[inline(always)]
    pub fn ptdpe5(&self) -> PTDPE5_R {
        PTDPE5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Pull Enable for Port D Bit 6"]
    #[inline(always)]
    pub fn ptdpe6(&self) -> PTDPE6_R {
        PTDPE6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Pull Enable for Port D Bit 7"]
    #[inline(always)]
    pub fn ptdpe7(&self) -> PTDPE7_R {
        PTDPE7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull Enable for Port A Bit 0"]
    #[inline(always)]
    pub fn ptape0(&mut self) -> PTAPE0_W {
        PTAPE0_W { w: self }
    }
    #[doc = "Bit 1 - Pull Enable for Port A Bit 1"]
    #[inline(always)]
    pub fn ptape1(&mut self) -> PTAPE1_W {
        PTAPE1_W { w: self }
    }
    #[doc = "Bit 2 - Pull Enable for Port A Bit 2"]
    #[inline(always)]
    pub fn ptape2(&mut self) -> PTAPE2_W {
        PTAPE2_W { w: self }
    }
    #[doc = "Bit 3 - Pull Enable for Port A Bit 3"]
    #[inline(always)]
    pub fn ptape3(&mut self) -> PTAPE3_W {
        PTAPE3_W { w: self }
    }
    #[doc = "Bit 4 - Pull Enable for Port A Bit 4"]
    #[inline(always)]
    pub fn ptape4(&mut self) -> PTAPE4_W {
        PTAPE4_W { w: self }
    }
    #[doc = "Bit 5 - Pull Enable for Port A Bit 5"]
    #[inline(always)]
    pub fn ptape5(&mut self) -> PTAPE5_W {
        PTAPE5_W { w: self }
    }
    #[doc = "Bit 6 - Pull Enable for Port A Bit 6"]
    #[inline(always)]
    pub fn ptape6(&mut self) -> PTAPE6_W {
        PTAPE6_W { w: self }
    }
    #[doc = "Bit 7 - Pull Enable for Port A Bit 7"]
    #[inline(always)]
    pub fn ptape7(&mut self) -> PTAPE7_W {
        PTAPE7_W { w: self }
    }
    #[doc = "Bit 8 - Pull Enable for Port B Bit 0"]
    #[inline(always)]
    pub fn ptbpe0(&mut self) -> PTBPE0_W {
        PTBPE0_W { w: self }
    }
    #[doc = "Bit 9 - Pull Enable for Port B Bit 1"]
    #[inline(always)]
    pub fn ptbpe1(&mut self) -> PTBPE1_W {
        PTBPE1_W { w: self }
    }
    #[doc = "Bit 10 - Pull Enable for Port B Bit 2"]
    #[inline(always)]
    pub fn ptbpe2(&mut self) -> PTBPE2_W {
        PTBPE2_W { w: self }
    }
    #[doc = "Bit 11 - Pull Enable for Port B Bit 3"]
    #[inline(always)]
    pub fn ptbpe3(&mut self) -> PTBPE3_W {
        PTBPE3_W { w: self }
    }
    #[doc = "Bit 12 - Pull Enable for Port B Bit 4"]
    #[inline(always)]
    pub fn ptbpe4(&mut self) -> PTBPE4_W {
        PTBPE4_W { w: self }
    }
    #[doc = "Bit 13 - Pull Enable for Port B Bit 5"]
    #[inline(always)]
    pub fn ptbpe5(&mut self) -> PTBPE5_W {
        PTBPE5_W { w: self }
    }
    #[doc = "Bit 14 - Pull Enable for Port B Bit 6"]
    #[inline(always)]
    pub fn ptbpe6(&mut self) -> PTBPE6_W {
        PTBPE6_W { w: self }
    }
    #[doc = "Bit 15 - Pull Enable for Port B Bit 7"]
    #[inline(always)]
    pub fn ptbpe7(&mut self) -> PTBPE7_W {
        PTBPE7_W { w: self }
    }
    #[doc = "Bit 16 - Pull Enable for Port C Bit 0"]
    #[inline(always)]
    pub fn ptcpe0(&mut self) -> PTCPE0_W {
        PTCPE0_W { w: self }
    }
    #[doc = "Bit 17 - Pull Enable for Port C Bit 1"]
    #[inline(always)]
    pub fn ptcpe1(&mut self) -> PTCPE1_W {
        PTCPE1_W { w: self }
    }
    #[doc = "Bit 18 - Pull Enable for Port C Bit 2"]
    #[inline(always)]
    pub fn ptcpe2(&mut self) -> PTCPE2_W {
        PTCPE2_W { w: self }
    }
    #[doc = "Bit 19 - Pull Enable for Port C Bit 3"]
    #[inline(always)]
    pub fn ptcpe3(&mut self) -> PTCPE3_W {
        PTCPE3_W { w: self }
    }
    #[doc = "Bit 20 - Pull Enable for Port C Bit 4"]
    #[inline(always)]
    pub fn ptcpe4(&mut self) -> PTCPE4_W {
        PTCPE4_W { w: self }
    }
    #[doc = "Bit 21 - Pull Enable for Port C Bit 5"]
    #[inline(always)]
    pub fn ptcpe5(&mut self) -> PTCPE5_W {
        PTCPE5_W { w: self }
    }
    #[doc = "Bit 22 - Pull Enable for Port C Bit 6"]
    #[inline(always)]
    pub fn ptcpe6(&mut self) -> PTCPE6_W {
        PTCPE6_W { w: self }
    }
    #[doc = "Bit 23 - Pull Enable for Port C Bit 7"]
    #[inline(always)]
    pub fn ptcpe7(&mut self) -> PTCPE7_W {
        PTCPE7_W { w: self }
    }
    #[doc = "Bit 24 - Pull Enable for Port D Bit 0"]
    #[inline(always)]
    pub fn ptdpe0(&mut self) -> PTDPE0_W {
        PTDPE0_W { w: self }
    }
    #[doc = "Bit 25 - Pull Enable for Port D Bit 1"]
    #[inline(always)]
    pub fn ptdpe1(&mut self) -> PTDPE1_W {
        PTDPE1_W { w: self }
    }
    #[doc = "Bit 26 - Pull Enable for Port D Bit 2"]
    #[inline(always)]
    pub fn ptdpe2(&mut self) -> PTDPE2_W {
        PTDPE2_W { w: self }
    }
    #[doc = "Bit 27 - Pull Enable for Port D Bit 3"]
    #[inline(always)]
    pub fn ptdpe3(&mut self) -> PTDPE3_W {
        PTDPE3_W { w: self }
    }
    #[doc = "Bit 28 - Pull Enable for Port D Bit 4"]
    #[inline(always)]
    pub fn ptdpe4(&mut self) -> PTDPE4_W {
        PTDPE4_W { w: self }
    }
    #[doc = "Bit 29 - Pull Enable for Port D Bit 5"]
    #[inline(always)]
    pub fn ptdpe5(&mut self) -> PTDPE5_W {
        PTDPE5_W { w: self }
    }
    #[doc = "Bit 30 - Pull Enable for Port D Bit 6"]
    #[inline(always)]
    pub fn ptdpe6(&mut self) -> PTDPE6_W {
        PTDPE6_W { w: self }
    }
    #[doc = "Bit 31 - Pull Enable for Port D Bit 7"]
    #[inline(always)]
    pub fn ptdpe7(&mut self) -> PTDPE7_W {
        PTDPE7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Pullup Enable Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [puel](index.html) module"]
pub struct PUEL_SPEC;
impl crate::RegisterSpec for PUEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [puel::R](R) reader structure"]
impl crate::Readable for PUEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [puel::W](W) writer structure"]
impl crate::Writable for PUEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUEL to value 0x0010_0000"]
impl crate::Resettable for PUEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0000
    }
}
