#[doc = "Register `PUEH` reader"]
pub struct R(crate::R<PUEH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUEH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PUEH_SPEC>> for R {
    fn from(reader: crate::R<PUEH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUEH` writer"]
pub struct W(crate::W<PUEH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUEH_SPEC>;
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
impl core::convert::From<crate::W<PUEH_SPEC>> for W {
    fn from(writer: crate::W<PUEH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pull Enable for Port E Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE0_A {
    #[doc = "0: Pullup is disabled for port E bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 0."]
    _1 = 1,
}
impl From<PTEPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTEPE0` reader - Pull Enable for Port E Bit 0"]
pub struct PTEPE0_R(crate::FieldReader<bool, PTEPE0_A>);
impl PTEPE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTEPE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE0_A {
        match self.bits {
            false => PTEPE0_A::_0,
            true => PTEPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTEPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTEPE0_A::_1
    }
}
impl core::ops::Deref for PTEPE0_R {
    type Target = crate::FieldReader<bool, PTEPE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEPE0` writer - Pull Enable for Port E Bit 0"]
pub struct PTEPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port E bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE0_A::_1)
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
#[doc = "Pull Enable for Port E Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE1_A {
    #[doc = "0: Pullup is disabled for port E bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 1."]
    _1 = 1,
}
impl From<PTEPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTEPE1` reader - Pull Enable for Port E Bit 1"]
pub struct PTEPE1_R(crate::FieldReader<bool, PTEPE1_A>);
impl PTEPE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTEPE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE1_A {
        match self.bits {
            false => PTEPE1_A::_0,
            true => PTEPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTEPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTEPE1_A::_1
    }
}
impl core::ops::Deref for PTEPE1_R {
    type Target = crate::FieldReader<bool, PTEPE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEPE1` writer - Pull Enable for Port E Bit 1"]
pub struct PTEPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port E bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE1_A::_1)
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
#[doc = "Pull Enable for Port E Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE2_A {
    #[doc = "0: Pullup is disabled for port E bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 2."]
    _1 = 1,
}
impl From<PTEPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTEPE2` reader - Pull Enable for Port E Bit 2"]
pub struct PTEPE2_R(crate::FieldReader<bool, PTEPE2_A>);
impl PTEPE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTEPE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE2_A {
        match self.bits {
            false => PTEPE2_A::_0,
            true => PTEPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTEPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTEPE2_A::_1
    }
}
impl core::ops::Deref for PTEPE2_R {
    type Target = crate::FieldReader<bool, PTEPE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEPE2` writer - Pull Enable for Port E Bit 2"]
pub struct PTEPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port E bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE2_A::_1)
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
#[doc = "Pull Enable for Port E Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE3_A {
    #[doc = "0: Pullup is disabled for port E bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 3."]
    _1 = 1,
}
impl From<PTEPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTEPE3` reader - Pull Enable for Port E Bit 3"]
pub struct PTEPE3_R(crate::FieldReader<bool, PTEPE3_A>);
impl PTEPE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTEPE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE3_A {
        match self.bits {
            false => PTEPE3_A::_0,
            true => PTEPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTEPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTEPE3_A::_1
    }
}
impl core::ops::Deref for PTEPE3_R {
    type Target = crate::FieldReader<bool, PTEPE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEPE3` writer - Pull Enable for Port E Bit 3"]
pub struct PTEPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port E bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE3_A::_1)
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
#[doc = "Pull Enable for Port E Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE4_A {
    #[doc = "0: Pullup is disabled for port E bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 4."]
    _1 = 1,
}
impl From<PTEPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTEPE4` reader - Pull Enable for Port E Bit 4"]
pub struct PTEPE4_R(crate::FieldReader<bool, PTEPE4_A>);
impl PTEPE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTEPE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE4_A {
        match self.bits {
            false => PTEPE4_A::_0,
            true => PTEPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTEPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTEPE4_A::_1
    }
}
impl core::ops::Deref for PTEPE4_R {
    type Target = crate::FieldReader<bool, PTEPE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEPE4` writer - Pull Enable for Port E Bit 4"]
pub struct PTEPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port E bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE4_A::_1)
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
#[doc = "Pull Enable for Port E Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE5_A {
    #[doc = "0: Pullup is disabled for port E bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 5."]
    _1 = 1,
}
impl From<PTEPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTEPE5` reader - Pull Enable for Port E Bit 5"]
pub struct PTEPE5_R(crate::FieldReader<bool, PTEPE5_A>);
impl PTEPE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTEPE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE5_A {
        match self.bits {
            false => PTEPE5_A::_0,
            true => PTEPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTEPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTEPE5_A::_1
    }
}
impl core::ops::Deref for PTEPE5_R {
    type Target = crate::FieldReader<bool, PTEPE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEPE5` writer - Pull Enable for Port E Bit 5"]
pub struct PTEPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port E bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE5_A::_1)
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
#[doc = "Pull Enable for Port E Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE6_A {
    #[doc = "0: Pullup is disabled for port E bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 6."]
    _1 = 1,
}
impl From<PTEPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTEPE6` reader - Pull Enable for Port E Bit 6"]
pub struct PTEPE6_R(crate::FieldReader<bool, PTEPE6_A>);
impl PTEPE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTEPE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE6_A {
        match self.bits {
            false => PTEPE6_A::_0,
            true => PTEPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTEPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTEPE6_A::_1
    }
}
impl core::ops::Deref for PTEPE6_R {
    type Target = crate::FieldReader<bool, PTEPE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEPE6` writer - Pull Enable for Port E Bit 6"]
pub struct PTEPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port E bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE6_A::_1)
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
#[doc = "Pull Enable for Port E Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTEPE7_A {
    #[doc = "0: Pullup is disabled for port E bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port E bit 7."]
    _1 = 1,
}
impl From<PTEPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTEPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTEPE7` reader - Pull Enable for Port E Bit 7"]
pub struct PTEPE7_R(crate::FieldReader<bool, PTEPE7_A>);
impl PTEPE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTEPE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTEPE7_A {
        match self.bits {
            false => PTEPE7_A::_0,
            true => PTEPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTEPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTEPE7_A::_1
    }
}
impl core::ops::Deref for PTEPE7_R {
    type Target = crate::FieldReader<bool, PTEPE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEPE7` writer - Pull Enable for Port E Bit 7"]
pub struct PTEPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTEPE7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port E bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTEPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port E bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTEPE7_A::_1)
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
#[doc = "Pull Enable for Port F Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE0_A {
    #[doc = "0: Pullup is disabled for port F bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 0."]
    _1 = 1,
}
impl From<PTFPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTFPE0` reader - Pull Enable for Port F Bit 0"]
pub struct PTFPE0_R(crate::FieldReader<bool, PTFPE0_A>);
impl PTFPE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTFPE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE0_A {
        match self.bits {
            false => PTFPE0_A::_0,
            true => PTFPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTFPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTFPE0_A::_1
    }
}
impl core::ops::Deref for PTFPE0_R {
    type Target = crate::FieldReader<bool, PTFPE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTFPE0` writer - Pull Enable for Port F Bit 0"]
pub struct PTFPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port F bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE0_A::_1)
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
#[doc = "Pull Enable for Port F Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE1_A {
    #[doc = "0: Pullup is disabled for port F bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 1."]
    _1 = 1,
}
impl From<PTFPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTFPE1` reader - Pull Enable for Port F Bit 1"]
pub struct PTFPE1_R(crate::FieldReader<bool, PTFPE1_A>);
impl PTFPE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTFPE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE1_A {
        match self.bits {
            false => PTFPE1_A::_0,
            true => PTFPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTFPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTFPE1_A::_1
    }
}
impl core::ops::Deref for PTFPE1_R {
    type Target = crate::FieldReader<bool, PTFPE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTFPE1` writer - Pull Enable for Port F Bit 1"]
pub struct PTFPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port F bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE1_A::_1)
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
#[doc = "Pull Enable for Port F Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE2_A {
    #[doc = "0: Pullup is disabled for port F bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 2."]
    _1 = 1,
}
impl From<PTFPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTFPE2` reader - Pull Enable for Port F Bit 2"]
pub struct PTFPE2_R(crate::FieldReader<bool, PTFPE2_A>);
impl PTFPE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTFPE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE2_A {
        match self.bits {
            false => PTFPE2_A::_0,
            true => PTFPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTFPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTFPE2_A::_1
    }
}
impl core::ops::Deref for PTFPE2_R {
    type Target = crate::FieldReader<bool, PTFPE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTFPE2` writer - Pull Enable for Port F Bit 2"]
pub struct PTFPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port F bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE2_A::_1)
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
#[doc = "Pull Enable for Port F Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE3_A {
    #[doc = "0: Pullup is disabled for port F bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 3."]
    _1 = 1,
}
impl From<PTFPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTFPE3` reader - Pull Enable for Port F Bit 3"]
pub struct PTFPE3_R(crate::FieldReader<bool, PTFPE3_A>);
impl PTFPE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTFPE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE3_A {
        match self.bits {
            false => PTFPE3_A::_0,
            true => PTFPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTFPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTFPE3_A::_1
    }
}
impl core::ops::Deref for PTFPE3_R {
    type Target = crate::FieldReader<bool, PTFPE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTFPE3` writer - Pull Enable for Port F Bit 3"]
pub struct PTFPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port F bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE3_A::_1)
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
#[doc = "Pull Enable for Port F Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE4_A {
    #[doc = "0: Pullup is disabled for port F bit 4."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 4."]
    _1 = 1,
}
impl From<PTFPE4_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTFPE4` reader - Pull Enable for Port F Bit 4"]
pub struct PTFPE4_R(crate::FieldReader<bool, PTFPE4_A>);
impl PTFPE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTFPE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE4_A {
        match self.bits {
            false => PTFPE4_A::_0,
            true => PTFPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTFPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTFPE4_A::_1
    }
}
impl core::ops::Deref for PTFPE4_R {
    type Target = crate::FieldReader<bool, PTFPE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTFPE4` writer - Pull Enable for Port F Bit 4"]
pub struct PTFPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port F bit 4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE4_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 4."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE4_A::_1)
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
#[doc = "Pull Enable for Port F Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE5_A {
    #[doc = "0: Pullup is disabled for port F bit 5."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 5."]
    _1 = 1,
}
impl From<PTFPE5_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTFPE5` reader - Pull Enable for Port F Bit 5"]
pub struct PTFPE5_R(crate::FieldReader<bool, PTFPE5_A>);
impl PTFPE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTFPE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE5_A {
        match self.bits {
            false => PTFPE5_A::_0,
            true => PTFPE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTFPE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTFPE5_A::_1
    }
}
impl core::ops::Deref for PTFPE5_R {
    type Target = crate::FieldReader<bool, PTFPE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTFPE5` writer - Pull Enable for Port F Bit 5"]
pub struct PTFPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port F bit 5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE5_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE5_A::_1)
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
#[doc = "Pull Enable for Port F Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE6_A {
    #[doc = "0: Pullup is disabled for port F bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 6."]
    _1 = 1,
}
impl From<PTFPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTFPE6` reader - Pull Enable for Port F Bit 6"]
pub struct PTFPE6_R(crate::FieldReader<bool, PTFPE6_A>);
impl PTFPE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTFPE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE6_A {
        match self.bits {
            false => PTFPE6_A::_0,
            true => PTFPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTFPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTFPE6_A::_1
    }
}
impl core::ops::Deref for PTFPE6_R {
    type Target = crate::FieldReader<bool, PTFPE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTFPE6` writer - Pull Enable for Port F Bit 6"]
pub struct PTFPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port F bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE6_A::_1)
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
#[doc = "Pull Enable for Port F Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTFPE7_A {
    #[doc = "0: Pullup is disabled for port F bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port F bit 7."]
    _1 = 1,
}
impl From<PTFPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTFPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTFPE7` reader - Pull Enable for Port F Bit 7"]
pub struct PTFPE7_R(crate::FieldReader<bool, PTFPE7_A>);
impl PTFPE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTFPE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTFPE7_A {
        match self.bits {
            false => PTFPE7_A::_0,
            true => PTFPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTFPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTFPE7_A::_1
    }
}
impl core::ops::Deref for PTFPE7_R {
    type Target = crate::FieldReader<bool, PTFPE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTFPE7` writer - Pull Enable for Port F Bit 7"]
pub struct PTFPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTFPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTFPE7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port F bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTFPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port F bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTFPE7_A::_1)
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
#[doc = "Pull Enable for Port G Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE0_A {
    #[doc = "0: Pullup is disabled for port G bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 0."]
    _1 = 1,
}
impl From<PTGPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTGPE0` reader - Pull Enable for Port G Bit 0"]
pub struct PTGPE0_R(crate::FieldReader<bool, PTGPE0_A>);
impl PTGPE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTGPE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE0_A {
        match self.bits {
            false => PTGPE0_A::_0,
            true => PTGPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTGPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTGPE0_A::_1
    }
}
impl core::ops::Deref for PTGPE0_R {
    type Target = crate::FieldReader<bool, PTGPE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTGPE0` writer - Pull Enable for Port G Bit 0"]
pub struct PTGPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port G bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE0_A::_1)
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
#[doc = "Pull Enable for Port G Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE1_A {
    #[doc = "0: Pullup is disabled for port G bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 1."]
    _1 = 1,
}
impl From<PTGPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTGPE1` reader - Pull Enable for Port G Bit 1"]
pub struct PTGPE1_R(crate::FieldReader<bool, PTGPE1_A>);
impl PTGPE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTGPE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE1_A {
        match self.bits {
            false => PTGPE1_A::_0,
            true => PTGPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTGPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTGPE1_A::_1
    }
}
impl core::ops::Deref for PTGPE1_R {
    type Target = crate::FieldReader<bool, PTGPE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTGPE1` writer - Pull Enable for Port G Bit 1"]
pub struct PTGPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port G bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE1_A::_1)
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
#[doc = "Pull Enable for Port G Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE2_A {
    #[doc = "0: Pullup is disabled for port G bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 2."]
    _1 = 1,
}
impl From<PTGPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTGPE2` reader - Pull Enable for Port G Bit 2"]
pub struct PTGPE2_R(crate::FieldReader<bool, PTGPE2_A>);
impl PTGPE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTGPE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE2_A {
        match self.bits {
            false => PTGPE2_A::_0,
            true => PTGPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTGPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTGPE2_A::_1
    }
}
impl core::ops::Deref for PTGPE2_R {
    type Target = crate::FieldReader<bool, PTGPE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTGPE2` writer - Pull Enable for Port G Bit 2"]
pub struct PTGPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port G bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE2_A::_1)
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
#[doc = "Pull Enable for Port G Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTGPE3_A {
    #[doc = "0: Pullup is disabled for port G bit 3."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port G bit 3."]
    _1 = 1,
}
impl From<PTGPE3_A> for bool {
    #[inline(always)]
    fn from(variant: PTGPE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTGPE3` reader - Pull Enable for Port G Bit 3"]
pub struct PTGPE3_R(crate::FieldReader<bool, PTGPE3_A>);
impl PTGPE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTGPE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTGPE3_A {
        match self.bits {
            false => PTGPE3_A::_0,
            true => PTGPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTGPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTGPE3_A::_1
    }
}
impl core::ops::Deref for PTGPE3_R {
    type Target = crate::FieldReader<bool, PTGPE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTGPE3` writer - Pull Enable for Port G Bit 3"]
pub struct PTGPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PTGPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTGPE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port G bit 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTGPE3_A::_0)
    }
    #[doc = "Pullup is enabled for port G bit 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTGPE3_A::_1)
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
#[doc = "Pull Enable for Port H Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE0_A {
    #[doc = "0: Pullup is disabled for port H bit 0."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 0."]
    _1 = 1,
}
impl From<PTHPE0_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTHPE0` reader - Pull Enable for Port H Bit 0"]
pub struct PTHPE0_R(crate::FieldReader<bool, PTHPE0_A>);
impl PTHPE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTHPE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE0_A {
        match self.bits {
            false => PTHPE0_A::_0,
            true => PTHPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTHPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTHPE0_A::_1
    }
}
impl core::ops::Deref for PTHPE0_R {
    type Target = crate::FieldReader<bool, PTHPE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTHPE0` writer - Pull Enable for Port H Bit 0"]
pub struct PTHPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port H bit 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE0_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE0_A::_1)
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
#[doc = "Pull Enable for Port H Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE1_A {
    #[doc = "0: Pullup is disabled for port H bit 1."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 1."]
    _1 = 1,
}
impl From<PTHPE1_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTHPE1` reader - Pull Enable for Port H Bit 1"]
pub struct PTHPE1_R(crate::FieldReader<bool, PTHPE1_A>);
impl PTHPE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTHPE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE1_A {
        match self.bits {
            false => PTHPE1_A::_0,
            true => PTHPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTHPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTHPE1_A::_1
    }
}
impl core::ops::Deref for PTHPE1_R {
    type Target = crate::FieldReader<bool, PTHPE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTHPE1` writer - Pull Enable for Port H Bit 1"]
pub struct PTHPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port H bit 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE1_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE1_A::_1)
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
#[doc = "Pull Enable for Port H Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE2_A {
    #[doc = "0: Pullup is disabled for port H bit 2."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 2."]
    _1 = 1,
}
impl From<PTHPE2_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTHPE2` reader - Pull Enable for Port H Bit 2"]
pub struct PTHPE2_R(crate::FieldReader<bool, PTHPE2_A>);
impl PTHPE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTHPE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE2_A {
        match self.bits {
            false => PTHPE2_A::_0,
            true => PTHPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTHPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTHPE2_A::_1
    }
}
impl core::ops::Deref for PTHPE2_R {
    type Target = crate::FieldReader<bool, PTHPE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTHPE2` writer - Pull Enable for Port H Bit 2"]
pub struct PTHPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port H bit 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE2_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE2_A::_1)
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
#[doc = "Pull Enable for Port H Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE6_A {
    #[doc = "0: Pullup is disabled for port H bit 6."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 6."]
    _1 = 1,
}
impl From<PTHPE6_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTHPE6` reader - Pull Enable for Port H Bit 6"]
pub struct PTHPE6_R(crate::FieldReader<bool, PTHPE6_A>);
impl PTHPE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTHPE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE6_A {
        match self.bits {
            false => PTHPE6_A::_0,
            true => PTHPE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTHPE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTHPE6_A::_1
    }
}
impl core::ops::Deref for PTHPE6_R {
    type Target = crate::FieldReader<bool, PTHPE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTHPE6` writer - Pull Enable for Port H Bit 6"]
pub struct PTHPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port H bit 6."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE6_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 6."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE6_A::_1)
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
#[doc = "Pull Enable for Port H Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTHPE7_A {
    #[doc = "0: Pullup is disabled for port H bit 7."]
    _0 = 0,
    #[doc = "1: Pullup is enabled for port H bit 7."]
    _1 = 1,
}
impl From<PTHPE7_A> for bool {
    #[inline(always)]
    fn from(variant: PTHPE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTHPE7` reader - Pull Enable for Port H Bit 7"]
pub struct PTHPE7_R(crate::FieldReader<bool, PTHPE7_A>);
impl PTHPE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTHPE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTHPE7_A {
        match self.bits {
            false => PTHPE7_A::_0,
            true => PTHPE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PTHPE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PTHPE7_A::_1
    }
}
impl core::ops::Deref for PTHPE7_R {
    type Target = crate::FieldReader<bool, PTHPE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTHPE7` writer - Pull Enable for Port H Bit 7"]
pub struct PTHPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTHPE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTHPE7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup is disabled for port H bit 7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTHPE7_A::_0)
    }
    #[doc = "Pullup is enabled for port H bit 7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTHPE7_A::_1)
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
    #[doc = "Bit 0 - Pull Enable for Port E Bit 0"]
    #[inline(always)]
    pub fn ptepe0(&self) -> PTEPE0_R {
        PTEPE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pull Enable for Port E Bit 1"]
    #[inline(always)]
    pub fn ptepe1(&self) -> PTEPE1_R {
        PTEPE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pull Enable for Port E Bit 2"]
    #[inline(always)]
    pub fn ptepe2(&self) -> PTEPE2_R {
        PTEPE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pull Enable for Port E Bit 3"]
    #[inline(always)]
    pub fn ptepe3(&self) -> PTEPE3_R {
        PTEPE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pull Enable for Port E Bit 4"]
    #[inline(always)]
    pub fn ptepe4(&self) -> PTEPE4_R {
        PTEPE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pull Enable for Port E Bit 5"]
    #[inline(always)]
    pub fn ptepe5(&self) -> PTEPE5_R {
        PTEPE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pull Enable for Port E Bit 6"]
    #[inline(always)]
    pub fn ptepe6(&self) -> PTEPE6_R {
        PTEPE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pull Enable for Port E Bit 7"]
    #[inline(always)]
    pub fn ptepe7(&self) -> PTEPE7_R {
        PTEPE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pull Enable for Port F Bit 0"]
    #[inline(always)]
    pub fn ptfpe0(&self) -> PTFPE0_R {
        PTFPE0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pull Enable for Port F Bit 1"]
    #[inline(always)]
    pub fn ptfpe1(&self) -> PTFPE1_R {
        PTFPE1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pull Enable for Port F Bit 2"]
    #[inline(always)]
    pub fn ptfpe2(&self) -> PTFPE2_R {
        PTFPE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pull Enable for Port F Bit 3"]
    #[inline(always)]
    pub fn ptfpe3(&self) -> PTFPE3_R {
        PTFPE3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pull Enable for Port F Bit 4"]
    #[inline(always)]
    pub fn ptfpe4(&self) -> PTFPE4_R {
        PTFPE4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pull Enable for Port F Bit 5"]
    #[inline(always)]
    pub fn ptfpe5(&self) -> PTFPE5_R {
        PTFPE5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pull Enable for Port F Bit 6"]
    #[inline(always)]
    pub fn ptfpe6(&self) -> PTFPE6_R {
        PTFPE6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pull Enable for Port F Bit 7"]
    #[inline(always)]
    pub fn ptfpe7(&self) -> PTFPE7_R {
        PTFPE7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pull Enable for Port G Bit 0"]
    #[inline(always)]
    pub fn ptgpe0(&self) -> PTGPE0_R {
        PTGPE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pull Enable for Port G Bit 1"]
    #[inline(always)]
    pub fn ptgpe1(&self) -> PTGPE1_R {
        PTGPE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pull Enable for Port G Bit 2"]
    #[inline(always)]
    pub fn ptgpe2(&self) -> PTGPE2_R {
        PTGPE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Pull Enable for Port G Bit 3"]
    #[inline(always)]
    pub fn ptgpe3(&self) -> PTGPE3_R {
        PTGPE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pull Enable for Port H Bit 0"]
    #[inline(always)]
    pub fn pthpe0(&self) -> PTHPE0_R {
        PTHPE0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pull Enable for Port H Bit 1"]
    #[inline(always)]
    pub fn pthpe1(&self) -> PTHPE1_R {
        PTHPE1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Pull Enable for Port H Bit 2"]
    #[inline(always)]
    pub fn pthpe2(&self) -> PTHPE2_R {
        PTHPE2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Pull Enable for Port H Bit 6"]
    #[inline(always)]
    pub fn pthpe6(&self) -> PTHPE6_R {
        PTHPE6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Pull Enable for Port H Bit 7"]
    #[inline(always)]
    pub fn pthpe7(&self) -> PTHPE7_R {
        PTHPE7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull Enable for Port E Bit 0"]
    #[inline(always)]
    pub fn ptepe0(&mut self) -> PTEPE0_W {
        PTEPE0_W { w: self }
    }
    #[doc = "Bit 1 - Pull Enable for Port E Bit 1"]
    #[inline(always)]
    pub fn ptepe1(&mut self) -> PTEPE1_W {
        PTEPE1_W { w: self }
    }
    #[doc = "Bit 2 - Pull Enable for Port E Bit 2"]
    #[inline(always)]
    pub fn ptepe2(&mut self) -> PTEPE2_W {
        PTEPE2_W { w: self }
    }
    #[doc = "Bit 3 - Pull Enable for Port E Bit 3"]
    #[inline(always)]
    pub fn ptepe3(&mut self) -> PTEPE3_W {
        PTEPE3_W { w: self }
    }
    #[doc = "Bit 4 - Pull Enable for Port E Bit 4"]
    #[inline(always)]
    pub fn ptepe4(&mut self) -> PTEPE4_W {
        PTEPE4_W { w: self }
    }
    #[doc = "Bit 5 - Pull Enable for Port E Bit 5"]
    #[inline(always)]
    pub fn ptepe5(&mut self) -> PTEPE5_W {
        PTEPE5_W { w: self }
    }
    #[doc = "Bit 6 - Pull Enable for Port E Bit 6"]
    #[inline(always)]
    pub fn ptepe6(&mut self) -> PTEPE6_W {
        PTEPE6_W { w: self }
    }
    #[doc = "Bit 7 - Pull Enable for Port E Bit 7"]
    #[inline(always)]
    pub fn ptepe7(&mut self) -> PTEPE7_W {
        PTEPE7_W { w: self }
    }
    #[doc = "Bit 8 - Pull Enable for Port F Bit 0"]
    #[inline(always)]
    pub fn ptfpe0(&mut self) -> PTFPE0_W {
        PTFPE0_W { w: self }
    }
    #[doc = "Bit 9 - Pull Enable for Port F Bit 1"]
    #[inline(always)]
    pub fn ptfpe1(&mut self) -> PTFPE1_W {
        PTFPE1_W { w: self }
    }
    #[doc = "Bit 10 - Pull Enable for Port F Bit 2"]
    #[inline(always)]
    pub fn ptfpe2(&mut self) -> PTFPE2_W {
        PTFPE2_W { w: self }
    }
    #[doc = "Bit 11 - Pull Enable for Port F Bit 3"]
    #[inline(always)]
    pub fn ptfpe3(&mut self) -> PTFPE3_W {
        PTFPE3_W { w: self }
    }
    #[doc = "Bit 12 - Pull Enable for Port F Bit 4"]
    #[inline(always)]
    pub fn ptfpe4(&mut self) -> PTFPE4_W {
        PTFPE4_W { w: self }
    }
    #[doc = "Bit 13 - Pull Enable for Port F Bit 5"]
    #[inline(always)]
    pub fn ptfpe5(&mut self) -> PTFPE5_W {
        PTFPE5_W { w: self }
    }
    #[doc = "Bit 14 - Pull Enable for Port F Bit 6"]
    #[inline(always)]
    pub fn ptfpe6(&mut self) -> PTFPE6_W {
        PTFPE6_W { w: self }
    }
    #[doc = "Bit 15 - Pull Enable for Port F Bit 7"]
    #[inline(always)]
    pub fn ptfpe7(&mut self) -> PTFPE7_W {
        PTFPE7_W { w: self }
    }
    #[doc = "Bit 16 - Pull Enable for Port G Bit 0"]
    #[inline(always)]
    pub fn ptgpe0(&mut self) -> PTGPE0_W {
        PTGPE0_W { w: self }
    }
    #[doc = "Bit 17 - Pull Enable for Port G Bit 1"]
    #[inline(always)]
    pub fn ptgpe1(&mut self) -> PTGPE1_W {
        PTGPE1_W { w: self }
    }
    #[doc = "Bit 18 - Pull Enable for Port G Bit 2"]
    #[inline(always)]
    pub fn ptgpe2(&mut self) -> PTGPE2_W {
        PTGPE2_W { w: self }
    }
    #[doc = "Bit 19 - Pull Enable for Port G Bit 3"]
    #[inline(always)]
    pub fn ptgpe3(&mut self) -> PTGPE3_W {
        PTGPE3_W { w: self }
    }
    #[doc = "Bit 24 - Pull Enable for Port H Bit 0"]
    #[inline(always)]
    pub fn pthpe0(&mut self) -> PTHPE0_W {
        PTHPE0_W { w: self }
    }
    #[doc = "Bit 25 - Pull Enable for Port H Bit 1"]
    #[inline(always)]
    pub fn pthpe1(&mut self) -> PTHPE1_W {
        PTHPE1_W { w: self }
    }
    #[doc = "Bit 26 - Pull Enable for Port H Bit 2"]
    #[inline(always)]
    pub fn pthpe2(&mut self) -> PTHPE2_W {
        PTHPE2_W { w: self }
    }
    #[doc = "Bit 30 - Pull Enable for Port H Bit 6"]
    #[inline(always)]
    pub fn pthpe6(&mut self) -> PTHPE6_W {
        PTHPE6_W { w: self }
    }
    #[doc = "Bit 31 - Pull Enable for Port H Bit 7"]
    #[inline(always)]
    pub fn pthpe7(&mut self) -> PTHPE7_W {
        PTHPE7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Pullup Enable High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pueh](index.html) module"]
pub struct PUEH_SPEC;
impl crate::RegisterSpec for PUEH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pueh::R](R) reader structure"]
impl crate::Readable for PUEH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pueh::W](W) writer structure"]
impl crate::Writable for PUEH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUEH to value 0"]
impl crate::Resettable for PUEH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
