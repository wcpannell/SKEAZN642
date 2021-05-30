#[doc = "Register `SC2` reader"]
pub struct R(crate::R<SC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC2_SPEC>> for R {
    fn from(reader: crate::R<SC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC2` writer"]
pub struct W(crate::W<SC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC2_SPEC>;
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
impl core::convert::From<crate::W<SC2_SPEC>> for W {
    fn from(writer: crate::W<SC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Default voltage reference pin pair (VREFH/VREFL)."]
    _00 = 0,
    #[doc = "1: Analog supply pin pair (VDDA/VSSA)."]
    _01 = 1,
    #[doc = "2: Reserved."]
    _10 = 2,
    #[doc = "3: Reserved - Selects default voltage reference (VREFH/VREFL) pin pair."]
    _11 = 3,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFSEL` reader - Voltage Reference Selection"]
pub struct REFSEL_R(crate::FieldReader<u8, REFSEL_A>);
impl REFSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::_00,
            1 => REFSEL_A::_01,
            2 => REFSEL_A::_10,
            3 => REFSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == REFSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == REFSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == REFSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == REFSEL_A::_11
    }
}
impl core::ops::Deref for REFSEL_R {
    type Target = crate::FieldReader<u8, REFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFSEL` writer - Voltage Reference Selection"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Default voltage reference pin pair (VREFH/VREFL)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(REFSEL_A::_00)
    }
    #[doc = "Analog supply pin pair (VDDA/VSSA)."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(REFSEL_A::_01)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(REFSEL_A::_10)
    }
    #[doc = "Reserved - Selects default voltage reference (VREFH/VREFL) pin pair."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(REFSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Result FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFULL_A {
    #[doc = "0: Indicates that ADC result FIFO is not full and next conversion data still can be stored into FIFO."]
    _0 = 0,
    #[doc = "1: Indicates that ADC result FIFO is full and next conversion will override old data in case of no read action."]
    _1 = 1,
}
impl From<FFULL_A> for bool {
    #[inline(always)]
    fn from(variant: FFULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFULL` reader - Result FIFO full"]
pub struct FFULL_R(crate::FieldReader<bool, FFULL_A>);
impl FFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFULL_A {
        match self.bits {
            false => FFULL_A::_0,
            true => FFULL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FFULL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FFULL_A::_1
    }
}
impl core::ops::Deref for FFULL_R {
    type Target = crate::FieldReader<bool, FFULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Result FIFO empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEMPTY_A {
    #[doc = "0: Indicates that ADC result FIFO have at least one valid new data."]
    _0 = 0,
    #[doc = "1: Indicates that ADC result FIFO have no valid new data."]
    _1 = 1,
}
impl From<FEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: FEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEMPTY` reader - Result FIFO empty"]
pub struct FEMPTY_R(crate::FieldReader<bool, FEMPTY_A>);
impl FEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEMPTY_A {
        match self.bits {
            false => FEMPTY_A::_0,
            true => FEMPTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FEMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FEMPTY_A::_1
    }
}
impl core::ops::Deref for FEMPTY_R {
    type Target = crate::FieldReader<bool, FEMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Compare Function Greater Than Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACFGT_A {
    #[doc = "0: Compare triggers when input is less than compare level."]
    _0 = 0,
    #[doc = "1: Compare triggers when input is greater than or equal to compare level."]
    _1 = 1,
}
impl From<ACFGT_A> for bool {
    #[inline(always)]
    fn from(variant: ACFGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACFGT` reader - Compare Function Greater Than Enable"]
pub struct ACFGT_R(crate::FieldReader<bool, ACFGT_A>);
impl ACFGT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACFGT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFGT_A {
        match self.bits {
            false => ACFGT_A::_0,
            true => ACFGT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACFGT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACFGT_A::_1
    }
}
impl core::ops::Deref for ACFGT_R {
    type Target = crate::FieldReader<bool, ACFGT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACFGT` writer - Compare Function Greater Than Enable"]
pub struct ACFGT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACFGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACFGT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare triggers when input is less than compare level."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACFGT_A::_0)
    }
    #[doc = "Compare triggers when input is greater than or equal to compare level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACFGT_A::_1)
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
#[doc = "Compare Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACFE_A {
    #[doc = "0: Compare function disabled."]
    _0 = 0,
    #[doc = "1: Compare function enabled."]
    _1 = 1,
}
impl From<ACFE_A> for bool {
    #[inline(always)]
    fn from(variant: ACFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACFE` reader - Compare Function Enable"]
pub struct ACFE_R(crate::FieldReader<bool, ACFE_A>);
impl ACFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFE_A {
        match self.bits {
            false => ACFE_A::_0,
            true => ACFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ACFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ACFE_A::_1
    }
}
impl core::ops::Deref for ACFE_R {
    type Target = crate::FieldReader<bool, ACFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACFE` writer - Compare Function Enable"]
pub struct ACFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare function disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACFE_A::_0)
    }
    #[doc = "Compare function enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACFE_A::_1)
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
#[doc = "Conversion Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADTRG_A {
    #[doc = "0: Software trigger selected."]
    _0 = 0,
    #[doc = "1: Hardware trigger selected."]
    _1 = 1,
}
impl From<ADTRG_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADTRG` reader - Conversion Trigger Select"]
pub struct ADTRG_R(crate::FieldReader<bool, ADTRG_A>);
impl ADTRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADTRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRG_A {
        match self.bits {
            false => ADTRG_A::_0,
            true => ADTRG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADTRG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADTRG_A::_1
    }
}
impl core::ops::Deref for ADTRG_R {
    type Target = crate::FieldReader<bool, ADTRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADTRG` writer - Conversion Trigger Select"]
pub struct ADTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADTRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software trigger selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRG_A::_0)
    }
    #[doc = "Hardware trigger selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRG_A::_1)
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
#[doc = "Conversion Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADACT_A {
    #[doc = "0: Conversion not in progress."]
    _0 = 0,
    #[doc = "1: Conversion in progress."]
    _1 = 1,
}
impl From<ADACT_A> for bool {
    #[inline(always)]
    fn from(variant: ADACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADACT` reader - Conversion Active"]
pub struct ADACT_R(crate::FieldReader<bool, ADACT_A>);
impl ADACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACT_A {
        match self.bits {
            false => ADACT_A::_0,
            true => ADACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADACT_A::_1
    }
}
impl core::ops::Deref for ADACT_R {
    type Target = crate::FieldReader<bool, ADACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Result FIFO full"]
    #[inline(always)]
    pub fn ffull(&self) -> FFULL_R {
        FFULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Result FIFO empty"]
    #[inline(always)]
    pub fn fempty(&self) -> FEMPTY_R {
        FEMPTY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline(always)]
    pub fn acfgt(&self) -> ACFGT_R {
        ACFGT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline(always)]
    pub fn acfe(&self) -> ACFE_R {
        ACFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline(always)]
    pub fn adtrg(&self) -> ADTRG_R {
        ADTRG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Conversion Active"]
    #[inline(always)]
    pub fn adact(&self) -> ADACT_R {
        ADACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline(always)]
    pub fn acfgt(&mut self) -> ACFGT_W {
        ACFGT_W { w: self }
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline(always)]
    pub fn acfe(&mut self) -> ACFE_W {
        ACFE_W { w: self }
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline(always)]
    pub fn adtrg(&mut self) -> ADTRG_W {
        ADTRG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status and Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc2](index.html) module"]
pub struct SC2_SPEC;
impl crate::RegisterSpec for SC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc2::R](R) reader structure"]
impl crate::Readable for SC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc2::W](W) writer structure"]
impl crate::Writable for SC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC2 to value 0x08"]
impl crate::Resettable for SC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
