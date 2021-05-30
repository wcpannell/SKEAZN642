#[doc = "Register `FCNFG` reader"]
pub struct R(crate::R<FCNFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCNFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FCNFG_SPEC>> for R {
    fn from(reader: crate::R<FCNFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCNFG` writer"]
pub struct W(crate::W<FCNFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCNFG_SPEC>;
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
impl core::convert::From<crate::W<FCNFG_SPEC>> for W {
    fn from(writer: crate::W<FCNFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Force Single Bit Fault Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSFD_A {
    #[doc = "0: Flash array read operations will set the SFDIF flag in the FERSTAT register only if a single bit fault is detected."]
    _0 = 0,
    #[doc = "1: Flash array read operation will force the SFDIF flag in the FERSTAT register to be set and an interrupt will be generated as long as FERCNFG\\[SFDIE\\]
is set."]
    _1 = 1,
}
impl From<FSFD_A> for bool {
    #[inline(always)]
    fn from(variant: FSFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSFD` reader - Force Single Bit Fault Detect"]
pub struct FSFD_R(crate::FieldReader<bool, FSFD_A>);
impl FSFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSFD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSFD_A {
        match self.bits {
            false => FSFD_A::_0,
            true => FSFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FSFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FSFD_A::_1
    }
}
impl core::ops::Deref for FSFD_R {
    type Target = crate::FieldReader<bool, FSFD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSFD` writer - Force Single Bit Fault Detect"]
pub struct FSFD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSFD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash array read operations will set the SFDIF flag in the FERSTAT register only if a single bit fault is detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSFD_A::_0)
    }
    #[doc = "Flash array read operation will force the SFDIF flag in the FERSTAT register to be set and an interrupt will be generated as long as FERCNFG\\[SFDIE\\]
is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSFD_A::_1)
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
#[doc = "Force Double Bit Fault Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDFD_A {
    #[doc = "0: Flash array read operations will set the FERSTAT\\[DFDIF\\]
flag only if a double bit fault is detected."]
    _0 = 0,
    #[doc = "1: Any flash array read operation will force the FERSTAT\\[DFDIF\\]
flag to be set and an interrupt will be generated as long as FERCNFG\\[DFDIE\\]
is set."]
    _1 = 1,
}
impl From<FDFD_A> for bool {
    #[inline(always)]
    fn from(variant: FDFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDFD` reader - Force Double Bit Fault Detect"]
pub struct FDFD_R(crate::FieldReader<bool, FDFD_A>);
impl FDFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDFD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDFD_A {
        match self.bits {
            false => FDFD_A::_0,
            true => FDFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FDFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FDFD_A::_1
    }
}
impl core::ops::Deref for FDFD_R {
    type Target = crate::FieldReader<bool, FDFD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDFD` writer - Force Double Bit Fault Detect"]
pub struct FDFD_W<'a> {
    w: &'a mut W,
}
impl<'a> FDFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDFD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash array read operations will set the FERSTAT\\[DFDIF\\]
flag only if a double bit fault is detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDFD_A::_0)
    }
    #[doc = "Any flash array read operation will force the FERSTAT\\[DFDIF\\]
flag to be set and an interrupt will be generated as long as FERCNFG\\[DFDIE\\]
is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDFD_A::_1)
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
#[doc = "Ignore Single Bit Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGNSF_A {
    #[doc = "0: All single-bit faults detected during array reads are reported."]
    _0 = 0,
    #[doc = "1: Single-bit faults detected during array reads are not reported and the single bit fault interrupt will not be generated."]
    _1 = 1,
}
impl From<IGNSF_A> for bool {
    #[inline(always)]
    fn from(variant: IGNSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IGNSF` reader - Ignore Single Bit Fault"]
pub struct IGNSF_R(crate::FieldReader<bool, IGNSF_A>);
impl IGNSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IGNSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGNSF_A {
        match self.bits {
            false => IGNSF_A::_0,
            true => IGNSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IGNSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IGNSF_A::_1
    }
}
impl core::ops::Deref for IGNSF_R {
    type Target = crate::FieldReader<bool, IGNSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGNSF` writer - Ignore Single Bit Fault"]
pub struct IGNSF_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IGNSF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All single-bit faults detected during array reads are reported."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IGNSF_A::_0)
    }
    #[doc = "Single-bit faults detected during array reads are not reported and the single bit fault interrupt will not be generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IGNSF_A::_1)
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
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIE_A {
    #[doc = "0: Command complete interrupt is disabled."]
    _0 = 0,
    #[doc = "1: An interrupt will be requested whenever the CCIF flag in the FSTAT register is set."]
    _1 = 1,
}
impl From<CCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIE` reader - Command Complete Interrupt Enable"]
pub struct CCIE_R(crate::FieldReader<bool, CCIE_A>);
impl CCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIE_A {
        match self.bits {
            false => CCIE_A::_0,
            true => CCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CCIE_A::_1
    }
}
impl core::ops::Deref for CCIE_R {
    type Target = crate::FieldReader<bool, CCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCIE` writer - Command Complete Interrupt Enable"]
pub struct CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Command complete interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIE_A::_0)
    }
    #[doc = "An interrupt will be requested whenever the CCIF flag in the FSTAT register is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIE_A::_1)
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
    #[doc = "Bit 0 - Force Single Bit Fault Detect"]
    #[inline(always)]
    pub fn fsfd(&self) -> FSFD_R {
        FSFD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn fdfd(&self) -> FDFD_R {
        FDFD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Ignore Single Bit Fault"]
    #[inline(always)]
    pub fn ignsf(&self) -> IGNSF_R {
        IGNSF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force Single Bit Fault Detect"]
    #[inline(always)]
    pub fn fsfd(&mut self) -> FSFD_W {
        FSFD_W { w: self }
    }
    #[doc = "Bit 1 - Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn fdfd(&mut self) -> FDFD_W {
        FDFD_W { w: self }
    }
    #[doc = "Bit 4 - Ignore Single Bit Fault"]
    #[inline(always)]
    pub fn ignsf(&mut self) -> IGNSF_W {
        IGNSF_W { w: self }
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CCIE_W {
        CCIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcnfg](index.html) module"]
pub struct FCNFG_SPEC;
impl crate::RegisterSpec for FCNFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fcnfg::R](R) reader structure"]
impl crate::Readable for FCNFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcnfg::W](W) writer structure"]
impl crate::Writable for FCNFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCNFG to value 0"]
impl crate::Resettable for FCNFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
