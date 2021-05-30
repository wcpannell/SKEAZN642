#[doc = "Register `SCGC` reader"]
pub struct R(crate::R<SCGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SCGC_SPEC>> for R {
    fn from(reader: crate::R<SCGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGC` writer"]
pub struct W(crate::W<SCGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGC_SPEC>;
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
impl core::convert::From<crate::W<SCGC_SPEC>> for W {
    fn from(writer: crate::W<SCGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_A {
    #[doc = "0: Bus clock to the RTC module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the RTC module is enabled."]
    _1 = 1,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` reader - RTC Clock Gate Control"]
pub struct RTC_R(crate::FieldReader<bool, RTC_A>);
impl RTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::_0,
            true => RTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RTC_A::_1
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<bool, RTC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC` writer - RTC Clock Gate Control"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the RTC module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTC_A::_0)
    }
    #[doc = "Bus clock to the RTC module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTC_A::_1)
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
#[doc = "PIT Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIT_A {
    #[doc = "0: Bus clock to the PIT module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the PIT module is enabled."]
    _1 = 1,
}
impl From<PIT_A> for bool {
    #[inline(always)]
    fn from(variant: PIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIT` reader - PIT Clock Gate Control"]
pub struct PIT_R(crate::FieldReader<bool, PIT_A>);
impl PIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT_A {
        match self.bits {
            false => PIT_A::_0,
            true => PIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PIT_A::_1
    }
}
impl core::ops::Deref for PIT_R {
    type Target = crate::FieldReader<bool, PIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIT` writer - PIT Clock Gate Control"]
pub struct PIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the PIT module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIT_A::_0)
    }
    #[doc = "Bus clock to the PIT module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIT_A::_1)
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
#[doc = "FTM0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0_A {
    #[doc = "0: Bus clock to the FTM0 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the FTM0 module is enabled."]
    _1 = 1,
}
impl From<FTM0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM0` reader - FTM0 Clock Gate Control"]
pub struct FTM0_R(crate::FieldReader<bool, FTM0_A>);
impl FTM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0_A {
        match self.bits {
            false => FTM0_A::_0,
            true => FTM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM0_A::_1
    }
}
impl core::ops::Deref for FTM0_R {
    type Target = crate::FieldReader<bool, FTM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM0` writer - FTM0 Clock Gate Control"]
pub struct FTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the FTM0 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0_A::_0)
    }
    #[doc = "Bus clock to the FTM0 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0_A::_1)
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
#[doc = "FTM1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1_A {
    #[doc = "0: Bus clock to the FTM1 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the FTM1 module is enabled."]
    _1 = 1,
}
impl From<FTM1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM1` reader - FTM1 Clock Gate Control"]
pub struct FTM1_R(crate::FieldReader<bool, FTM1_A>);
impl FTM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1_A {
        match self.bits {
            false => FTM1_A::_0,
            true => FTM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM1_A::_1
    }
}
impl core::ops::Deref for FTM1_R {
    type Target = crate::FieldReader<bool, FTM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM1` writer - FTM1 Clock Gate Control"]
pub struct FTM1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the FTM1 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1_A::_0)
    }
    #[doc = "Bus clock to the FTM1 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1_A::_1)
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
#[doc = "FTM2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2_A {
    #[doc = "0: Bus clock to the FTM2 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the FTM2 module is enabled."]
    _1 = 1,
}
impl From<FTM2_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM2` reader - FTM2 Clock Gate Control"]
pub struct FTM2_R(crate::FieldReader<bool, FTM2_A>);
impl FTM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTM2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2_A {
        match self.bits {
            false => FTM2_A::_0,
            true => FTM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM2_A::_1
    }
}
impl core::ops::Deref for FTM2_R {
    type Target = crate::FieldReader<bool, FTM2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM2` writer - FTM2 Clock Gate Control"]
pub struct FTM2_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the FTM2 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2_A::_0)
    }
    #[doc = "Bus clock to the FTM2 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2_A::_1)
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
#[doc = "CRC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_A {
    #[doc = "0: Bus clock to the CRC module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the CRC module is enabled."]
    _1 = 1,
}
impl From<CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC` reader - CRC Clock Gate Control"]
pub struct CRC_R(crate::FieldReader<bool, CRC_A>);
impl CRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::_0,
            true => CRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRC_A::_1
    }
}
impl core::ops::Deref for CRC_R {
    type Target = crate::FieldReader<bool, CRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC` writer - CRC Clock Gate Control"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the CRC module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_A::_0)
    }
    #[doc = "Bus clock to the CRC module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_A::_1)
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
#[doc = "Flash Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_A {
    #[doc = "0: Bus clock to the flash module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the flash module is enabled."]
    _1 = 1,
}
impl From<FLASH_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH` reader - Flash Clock Gate Control"]
pub struct FLASH_R(crate::FieldReader<bool, FLASH_A>);
impl FLASH_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_A {
        match self.bits {
            false => FLASH_A::_0,
            true => FLASH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLASH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLASH_A::_1
    }
}
impl core::ops::Deref for FLASH_R {
    type Target = crate::FieldReader<bool, FLASH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH` writer - Flash Clock Gate Control"]
pub struct FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the flash module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLASH_A::_0)
    }
    #[doc = "Bus clock to the flash module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLASH_A::_1)
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
#[doc = "SWD (single wire debugger) Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWD_A {
    #[doc = "0: Bus clock to the SWD module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the SWD module is enabled."]
    _1 = 1,
}
impl From<SWD_A> for bool {
    #[inline(always)]
    fn from(variant: SWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWD` reader - SWD (single wire debugger) Clock Gate Control"]
pub struct SWD_R(crate::FieldReader<bool, SWD_A>);
impl SWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWD_A {
        match self.bits {
            false => SWD_A::_0,
            true => SWD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SWD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SWD_A::_1
    }
}
impl core::ops::Deref for SWD_R {
    type Target = crate::FieldReader<bool, SWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWD` writer - SWD (single wire debugger) Clock Gate Control"]
pub struct SWD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the SWD module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWD_A::_0)
    }
    #[doc = "Bus clock to the SWD module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWD_A::_1)
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
#[doc = "I2C Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_A {
    #[doc = "0: Bus clock to the IIC module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the IIC module is enabled."]
    _1 = 1,
}
impl From<I2C_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C` reader - I2C Clock Gate Control"]
pub struct I2C_R(crate::FieldReader<bool, I2C_A>);
impl I2C_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_A {
        match self.bits {
            false => I2C_A::_0,
            true => I2C_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == I2C_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == I2C_A::_1
    }
}
impl core::ops::Deref for I2C_R {
    type Target = crate::FieldReader<bool, I2C_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C` writer - I2C Clock Gate Control"]
pub struct I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the IIC module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C_A::_0)
    }
    #[doc = "Bus clock to the IIC module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C_A::_1)
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
#[doc = "SPI0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_A {
    #[doc = "0: Bus clock to the SPI0 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the SPI0 module is enabled."]
    _1 = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0` reader - SPI0 Clock Gate Control"]
pub struct SPI0_R(crate::FieldReader<bool, SPI0_A>);
impl SPI0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::_0,
            true => SPI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPI0_A::_1
    }
}
impl core::ops::Deref for SPI0_R {
    type Target = crate::FieldReader<bool, SPI0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0` writer - SPI0 Clock Gate Control"]
pub struct SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the SPI0 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0_A::_0)
    }
    #[doc = "Bus clock to the SPI0 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0_A::_1)
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
#[doc = "SPI1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_A {
    #[doc = "0: Bus clock to the SPI1 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the SPI1 module is enabled."]
    _1 = 1,
}
impl From<SPI1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1` reader - SPI1 Clock Gate Control"]
pub struct SPI1_R(crate::FieldReader<bool, SPI1_A>);
impl SPI1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::_0,
            true => SPI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPI1_A::_1
    }
}
impl core::ops::Deref for SPI1_R {
    type Target = crate::FieldReader<bool, SPI1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1` writer - SPI1 Clock Gate Control"]
pub struct SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the SPI1 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1_A::_0)
    }
    #[doc = "Bus clock to the SPI1 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1_A::_1)
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
#[doc = "UART0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_A {
    #[doc = "0: Bus clock to the UART0 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the UART0 module is enabled."]
    _1 = 1,
}
impl From<UART0_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0` reader - UART0 Clock Gate Control"]
pub struct UART0_R(crate::FieldReader<bool, UART0_A>);
impl UART0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_A {
        match self.bits {
            false => UART0_A::_0,
            true => UART0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART0_A::_1
    }
}
impl core::ops::Deref for UART0_R {
    type Target = crate::FieldReader<bool, UART0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0` writer - UART0 Clock Gate Control"]
pub struct UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the UART0 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0_A::_0)
    }
    #[doc = "Bus clock to the UART0 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0_A::_1)
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
#[doc = "UART1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_A {
    #[doc = "0: Bus clock to the UART1 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the UART1 module is enabled."]
    _1 = 1,
}
impl From<UART1_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1` reader - UART1 Clock Gate Control"]
pub struct UART1_R(crate::FieldReader<bool, UART1_A>);
impl UART1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_A {
        match self.bits {
            false => UART1_A::_0,
            true => UART1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART1_A::_1
    }
}
impl core::ops::Deref for UART1_R {
    type Target = crate::FieldReader<bool, UART1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1` writer - UART1 Clock Gate Control"]
pub struct UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the UART1 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1_A::_0)
    }
    #[doc = "Bus clock to the UART1 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1_A::_1)
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
#[doc = "UART2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_A {
    #[doc = "0: Bus clock to the UART2 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the UART2 module is enabled."]
    _1 = 1,
}
impl From<UART2_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART2` reader - UART2 Clock Gate Control"]
pub struct UART2_R(crate::FieldReader<bool, UART2_A>);
impl UART2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2_A {
        match self.bits {
            false => UART2_A::_0,
            true => UART2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UART2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UART2_A::_1
    }
}
impl core::ops::Deref for UART2_R {
    type Target = crate::FieldReader<bool, UART2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2` writer - UART2 Clock Gate Control"]
pub struct UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the UART2 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART2_A::_0)
    }
    #[doc = "Bus clock to the UART2 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART2_A::_1)
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
#[doc = "KBI0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KBI0_A {
    #[doc = "0: Bus clock to the KBI0 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the KBI0 module is enabled."]
    _1 = 1,
}
impl From<KBI0_A> for bool {
    #[inline(always)]
    fn from(variant: KBI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KBI0` reader - KBI0 Clock Gate Control"]
pub struct KBI0_R(crate::FieldReader<bool, KBI0_A>);
impl KBI0_R {
    pub(crate) fn new(bits: bool) -> Self {
        KBI0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KBI0_A {
        match self.bits {
            false => KBI0_A::_0,
            true => KBI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == KBI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == KBI0_A::_1
    }
}
impl core::ops::Deref for KBI0_R {
    type Target = crate::FieldReader<bool, KBI0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBI0` writer - KBI0 Clock Gate Control"]
pub struct KBI0_W<'a> {
    w: &'a mut W,
}
impl<'a> KBI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBI0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the KBI0 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBI0_A::_0)
    }
    #[doc = "Bus clock to the KBI0 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBI0_A::_1)
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
#[doc = "KBI1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KBI1_A {
    #[doc = "0: Bus clock to the KBI1 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the KBI1 module is enabled."]
    _1 = 1,
}
impl From<KBI1_A> for bool {
    #[inline(always)]
    fn from(variant: KBI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KBI1` reader - KBI1 Clock Gate Control"]
pub struct KBI1_R(crate::FieldReader<bool, KBI1_A>);
impl KBI1_R {
    pub(crate) fn new(bits: bool) -> Self {
        KBI1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KBI1_A {
        match self.bits {
            false => KBI1_A::_0,
            true => KBI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == KBI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == KBI1_A::_1
    }
}
impl core::ops::Deref for KBI1_R {
    type Target = crate::FieldReader<bool, KBI1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBI1` writer - KBI1 Clock Gate Control"]
pub struct KBI1_W<'a> {
    w: &'a mut W,
}
impl<'a> KBI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KBI1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the KBI1 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KBI1_A::_0)
    }
    #[doc = "Bus clock to the KBI1 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KBI1_A::_1)
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
#[doc = "IRQ Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ_A {
    #[doc = "0: Bus clock to the IRQ module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the IRQ module is enabled."]
    _1 = 1,
}
impl From<IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ` reader - IRQ Clock Gate Control"]
pub struct IRQ_R(crate::FieldReader<bool, IRQ_A>);
impl IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ_A {
        match self.bits {
            false => IRQ_A::_0,
            true => IRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRQ_A::_1
    }
}
impl core::ops::Deref for IRQ_R {
    type Target = crate::FieldReader<bool, IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ` writer - IRQ Clock Gate Control"]
pub struct IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the IRQ module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQ_A::_0)
    }
    #[doc = "Bus clock to the IRQ module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQ_A::_1)
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
#[doc = "ADC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: Bus clock to the ADC module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the ADC module is enabled."]
    _1 = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC` reader - ADC Clock Gate Control"]
pub struct ADC_R(crate::FieldReader<bool, ADC_A>);
impl ADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::_0,
            true => ADC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADC_A::_1
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<bool, ADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC` writer - ADC Clock Gate Control"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the ADC module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC_A::_0)
    }
    #[doc = "Bus clock to the ADC module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC_A::_1)
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
#[doc = "ACMP0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP0_A {
    #[doc = "0: Bus clock to the ACMP0 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the ACMP0 module is enabled."]
    _1 = 1,
}
impl From<ACMP0_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP0` reader - ACMP0 Clock Gate Control"]
pub struct ACMP0_R(crate::FieldReader<bool, ACMP0_A>);
impl ACMP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP0_A {
        match self.bits {
            false => ACMP0_A::_0,
            true => ACMP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACMP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACMP0_A::_1
    }
}
impl core::ops::Deref for ACMP0_R {
    type Target = crate::FieldReader<bool, ACMP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP0` writer - ACMP0 Clock Gate Control"]
pub struct ACMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the ACMP0 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMP0_A::_0)
    }
    #[doc = "Bus clock to the ACMP0 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMP0_A::_1)
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
#[doc = "ACMP1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1_A {
    #[doc = "0: Bus clock to the ACMP1 module is disabled."]
    _0 = 0,
    #[doc = "1: Bus clock to the ACMP1 module is enabled."]
    _1 = 1,
}
impl From<ACMP1_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP1` reader - ACMP1 Clock Gate Control"]
pub struct ACMP1_R(crate::FieldReader<bool, ACMP1_A>);
impl ACMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP1_A {
        match self.bits {
            false => ACMP1_A::_0,
            true => ACMP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACMP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACMP1_A::_1
    }
}
impl core::ops::Deref for ACMP1_R {
    type Target = crate::FieldReader<bool, ACMP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP1` writer - ACMP1 Clock Gate Control"]
pub struct ACMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock to the ACMP1 module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMP1_A::_0)
    }
    #[doc = "Bus clock to the ACMP1 module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMP1_A::_1)
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
    #[doc = "Bit 0 - RTC Clock Gate Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&self) -> PIT_R {
        PIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FTM0 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm0(&self) -> FTM0_R {
        FTM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FTM1 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm1(&self) -> FTM1_R {
        FTM1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&self) -> FTM2_R {
        FTM2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Flash Clock Gate Control"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SWD (single wire debugger) Clock Gate Control"]
    #[inline(always)]
    pub fn swd(&self) -> SWD_R {
        SWD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 17 - I2C Clock Gate Control"]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SPI1 Clock Gate Control"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - UART0 Clock Gate Control"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - UART1 Clock Gate Control"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - UART2 Clock Gate Control"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - KBI0 Clock Gate Control"]
    #[inline(always)]
    pub fn kbi0(&self) -> KBI0_R {
        KBI0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - KBI1 Clock Gate Control"]
    #[inline(always)]
    pub fn kbi1(&self) -> KBI1_R {
        KBI1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IRQ Clock Gate Control"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ADC Clock Gate Control"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ACMP0 Clock Gate Control"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ACMP1 Clock Gate Control"]
    #[inline(always)]
    pub fn acmp1(&self) -> ACMP1_R {
        ACMP1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Clock Gate Control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 1 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&mut self) -> PIT_W {
        PIT_W { w: self }
    }
    #[doc = "Bit 5 - FTM0 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm0(&mut self) -> FTM0_W {
        FTM0_W { w: self }
    }
    #[doc = "Bit 6 - FTM1 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm1(&mut self) -> FTM1_W {
        FTM1_W { w: self }
    }
    #[doc = "Bit 7 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&mut self) -> FTM2_W {
        FTM2_W { w: self }
    }
    #[doc = "Bit 10 - CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 12 - Flash Clock Gate Control"]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W {
        FLASH_W { w: self }
    }
    #[doc = "Bit 13 - SWD (single wire debugger) Clock Gate Control"]
    #[inline(always)]
    pub fn swd(&mut self) -> SWD_W {
        SWD_W { w: self }
    }
    #[doc = "Bit 17 - I2C Clock Gate Control"]
    #[inline(always)]
    pub fn i2c(&mut self) -> I2C_W {
        I2C_W { w: self }
    }
    #[doc = "Bit 18 - SPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W {
        SPI0_W { w: self }
    }
    #[doc = "Bit 19 - SPI1 Clock Gate Control"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W {
        SPI1_W { w: self }
    }
    #[doc = "Bit 20 - UART0 Clock Gate Control"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W { w: self }
    }
    #[doc = "Bit 21 - UART1 Clock Gate Control"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W { w: self }
    }
    #[doc = "Bit 22 - UART2 Clock Gate Control"]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART2_W {
        UART2_W { w: self }
    }
    #[doc = "Bit 24 - KBI0 Clock Gate Control"]
    #[inline(always)]
    pub fn kbi0(&mut self) -> KBI0_W {
        KBI0_W { w: self }
    }
    #[doc = "Bit 25 - KBI1 Clock Gate Control"]
    #[inline(always)]
    pub fn kbi1(&mut self) -> KBI1_W {
        KBI1_W { w: self }
    }
    #[doc = "Bit 27 - IRQ Clock Gate Control"]
    #[inline(always)]
    pub fn irq(&mut self) -> IRQ_W {
        IRQ_W { w: self }
    }
    #[doc = "Bit 29 - ADC Clock Gate Control"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 30 - ACMP0 Clock Gate Control"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> ACMP0_W {
        ACMP0_W { w: self }
    }
    #[doc = "Bit 31 - ACMP1 Clock Gate Control"]
    #[inline(always)]
    pub fn acmp1(&mut self) -> ACMP1_W {
        ACMP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc](index.html) module"]
pub struct SCGC_SPEC;
impl crate::RegisterSpec for SCGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgc::R](R) reader structure"]
impl crate::Readable for SCGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgc::W](W) writer structure"]
impl crate::Writable for SCGC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGC to value 0x3000"]
impl crate::Resettable for SCGC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3000
    }
}
