#[doc = "Register `SC1` reader"]
pub struct R(crate::R<SC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC1_SPEC>> for R {
    fn from(reader: crate::R<SC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC1` writer"]
pub struct W(crate::W<SC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC1_SPEC>;
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
impl core::convert::From<crate::W<SC1_SPEC>> for W {
    fn from(writer: crate::W<SC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input Channel Select\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCH_A {
    #[doc = "22: Temperature Sensor"]
    _10110 = 22,
    #[doc = "23: Bandgap"]
    _10111 = 23,
    #[doc = "29: VREFH"]
    _11101 = 29,
    #[doc = "30: VREFL"]
    _11110 = 30,
    #[doc = "31: Module disabled Reset FIFO in FIFO mode."]
    _11111 = 31,
}
impl From<ADCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCH` reader - Input Channel Select"]
pub struct ADCH_R(crate::FieldReader<u8, ADCH_A>);
impl ADCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCH_A> {
        match self.bits {
            22 => Some(ADCH_A::_10110),
            23 => Some(ADCH_A::_10111),
            29 => Some(ADCH_A::_11101),
            30 => Some(ADCH_A::_11110),
            31 => Some(ADCH_A::_11111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        **self == ADCH_A::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        **self == ADCH_A::_10111
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        **self == ADCH_A::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        **self == ADCH_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        **self == ADCH_A::_11111
    }
}
impl core::ops::Deref for ADCH_R {
    type Target = crate::FieldReader<u8, ADCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCH` writer - Input Channel Select"]
pub struct ADCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Temperature Sensor"]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut W {
        self.variant(ADCH_A::_10110)
    }
    #[doc = "Bandgap"]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut W {
        self.variant(ADCH_A::_10111)
    }
    #[doc = "VREFH"]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut W {
        self.variant(ADCH_A::_11101)
    }
    #[doc = "VREFL"]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(ADCH_A::_11110)
    }
    #[doc = "Module disabled Reset FIFO in FIFO mode."]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(ADCH_A::_11111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Continuous Conversion Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCO_A {
    #[doc = "0: One conversion following a write to the ADC_SC1 when software triggered operation is selected, or one conversion following assertion of ADHWT when hardware triggered operation is selected. When the FIFO function is enabled (AFDEP > 0), a set of conversions are triggered."]
    _0 = 0,
    #[doc = "1: Continuous conversions are initiated following a write to ADC_SC1 when software triggered operation is selected. Continuous conversions are initiated by an ADHWT event when hardware triggered operation is selected. When the FIFO function is enabled (AFDEP > 0), a set of conversions are loop triggered."]
    _1 = 1,
}
impl From<ADCO_A> for bool {
    #[inline(always)]
    fn from(variant: ADCO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCO` reader - Continuous Conversion Enable"]
pub struct ADCO_R(crate::FieldReader<bool, ADCO_A>);
impl ADCO_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCO_A {
        match self.bits {
            false => ADCO_A::_0,
            true => ADCO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCO_A::_1
    }
}
impl core::ops::Deref for ADCO_R {
    type Target = crate::FieldReader<bool, ADCO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCO` writer - Continuous Conversion Enable"]
pub struct ADCO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "One conversion following a write to the ADC_SC1 when software triggered operation is selected, or one conversion following assertion of ADHWT when hardware triggered operation is selected. When the FIFO function is enabled (AFDEP > 0), a set of conversions are triggered."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCO_A::_0)
    }
    #[doc = "Continuous conversions are initiated following a write to ADC_SC1 when software triggered operation is selected. Continuous conversions are initiated by an ADHWT event when hardware triggered operation is selected. When the FIFO function is enabled (AFDEP > 0), a set of conversions are loop triggered."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCO_A::_1)
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
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIEN_A {
    #[doc = "0: Conversion complete interrupt disabled."]
    _0 = 0,
    #[doc = "1: Conversion complete interrupt enabled."]
    _1 = 1,
}
impl From<AIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIEN` reader - Interrupt Enable"]
pub struct AIEN_R(crate::FieldReader<bool, AIEN_A>);
impl AIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIEN_A {
        match self.bits {
            false => AIEN_A::_0,
            true => AIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AIEN_A::_1
    }
}
impl core::ops::Deref for AIEN_R {
    type Target = crate::FieldReader<bool, AIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIEN` writer - Interrupt Enable"]
pub struct AIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Conversion complete interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AIEN_A::_0)
    }
    #[doc = "Conversion complete interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AIEN_A::_1)
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
#[doc = "Conversion Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COCO_A {
    #[doc = "0: Conversion not completed."]
    _0 = 0,
    #[doc = "1: Conversion completed."]
    _1 = 1,
}
impl From<COCO_A> for bool {
    #[inline(always)]
    fn from(variant: COCO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COCO` reader - Conversion Complete Flag"]
pub struct COCO_R(crate::FieldReader<bool, COCO_A>);
impl COCO_R {
    pub(crate) fn new(bits: bool) -> Self {
        COCO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COCO_A {
        match self.bits {
            false => COCO_A::_0,
            true => COCO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COCO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COCO_A::_1
    }
}
impl core::ops::Deref for COCO_R {
    type Target = crate::FieldReader<bool, COCO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Input Channel Select"]
    #[inline(always)]
    pub fn adch(&self) -> ADCH_R {
        ADCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Continuous Conversion Enable"]
    #[inline(always)]
    pub fn adco(&self) -> ADCO_R {
        ADCO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&self) -> AIEN_R {
        AIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Conversion Complete Flag"]
    #[inline(always)]
    pub fn coco(&self) -> COCO_R {
        COCO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input Channel Select"]
    #[inline(always)]
    pub fn adch(&mut self) -> ADCH_W {
        ADCH_W { w: self }
    }
    #[doc = "Bit 5 - Continuous Conversion Enable"]
    #[inline(always)]
    pub fn adco(&mut self) -> ADCO_W {
        ADCO_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&mut self) -> AIEN_W {
        AIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status and Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc1](index.html) module"]
pub struct SC1_SPEC;
impl crate::RegisterSpec for SC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc1::R](R) reader structure"]
impl crate::Readable for SC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc1::W](W) writer structure"]
impl crate::Writable for SC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC1 to value 0x1f"]
impl crate::Resettable for SC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
