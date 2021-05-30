#[doc = "Register `C1` reader"]
pub struct R(crate::R<C1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<C1_SPEC>> for R {
    fn from(reader: crate::R<C1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1` writer"]
pub struct W(crate::W<C1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_SPEC>;
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
impl core::convert::From<crate::W<C1_SPEC>> for W {
    fn from(writer: crate::W<C1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internal Reference Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFSTEN_A {
    #[doc = "0: Internal reference clock is disabled in Stop mode."]
    _0 = 0,
    #[doc = "1: Internal reference clock stays enabled in Stop mode if IRCLKEN is set, or if ICS is in FEI, FBI, or FBILP mode before entering Stop."]
    _1 = 1,
}
impl From<IREFSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: IREFSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFSTEN` reader - Internal Reference Stop Enable"]
pub struct IREFSTEN_R(crate::FieldReader<bool, IREFSTEN_A>);
impl IREFSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IREFSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFSTEN_A {
        match self.bits {
            false => IREFSTEN_A::_0,
            true => IREFSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IREFSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IREFSTEN_A::_1
    }
}
impl core::ops::Deref for IREFSTEN_R {
    type Target = crate::FieldReader<bool, IREFSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREFSTEN` writer - Internal Reference Stop Enable"]
pub struct IREFSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREFSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IREFSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal reference clock is disabled in Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREFSTEN_A::_0)
    }
    #[doc = "Internal reference clock stays enabled in Stop mode if IRCLKEN is set, or if ICS is in FEI, FBI, or FBILP mode before entering Stop."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREFSTEN_A::_1)
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
#[doc = "Internal Reference Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCLKEN_A {
    #[doc = "0: ICSIRCLK is inactive."]
    _0 = 0,
    #[doc = "1: ICSIRCLK is active."]
    _1 = 1,
}
impl From<IRCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: IRCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCLKEN` reader - Internal Reference Clock Enable"]
pub struct IRCLKEN_R(crate::FieldReader<bool, IRCLKEN_A>);
impl IRCLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRCLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRCLKEN_A {
        match self.bits {
            false => IRCLKEN_A::_0,
            true => IRCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IRCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IRCLKEN_A::_1
    }
}
impl core::ops::Deref for IRCLKEN_R {
    type Target = crate::FieldReader<bool, IRCLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRCLKEN` writer - Internal Reference Clock Enable"]
pub struct IRCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRCLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ICSIRCLK is inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRCLKEN_A::_0)
    }
    #[doc = "ICSIRCLK is active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRCLKEN_A::_1)
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
#[doc = "Internal Reference Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFS_A {
    #[doc = "0: External reference clock is selected."]
    _0 = 0,
    #[doc = "1: Internal reference clock is selected."]
    _1 = 1,
}
impl From<IREFS_A> for bool {
    #[inline(always)]
    fn from(variant: IREFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFS` reader - Internal Reference Select"]
pub struct IREFS_R(crate::FieldReader<bool, IREFS_A>);
impl IREFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IREFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFS_A {
        match self.bits {
            false => IREFS_A::_0,
            true => IREFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IREFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IREFS_A::_1
    }
}
impl core::ops::Deref for IREFS_R {
    type Target = crate::FieldReader<bool, IREFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREFS` writer - Internal Reference Select"]
pub struct IREFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IREFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IREFS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External reference clock is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREFS_A::_0)
    }
    #[doc = "Internal reference clock is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREFS_A::_1)
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
#[doc = "Field `RDIV` reader - Reference Divider"]
pub struct RDIV_R(crate::FieldReader<u8, u8>);
impl RDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDIV` writer - Reference Divider"]
pub struct RDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u8 & 0x07) << 3);
        self.w
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKS_A {
    #[doc = "0: Output of FLL is selected."]
    _00 = 0,
    #[doc = "1: Internal reference clock is selected."]
    _01 = 1,
    #[doc = "2: External reference clock is selected."]
    _10 = 2,
    #[doc = "3: Reserved, defaults to 00."]
    _11 = 3,
}
impl From<CLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKS` reader - Clock Source Select"]
pub struct CLKS_R(crate::FieldReader<u8, CLKS_A>);
impl CLKS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKS_A {
        match self.bits {
            0 => CLKS_A::_00,
            1 => CLKS_A::_01,
            2 => CLKS_A::_10,
            3 => CLKS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == CLKS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == CLKS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == CLKS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == CLKS_A::_11
    }
}
impl core::ops::Deref for CLKS_R {
    type Target = crate::FieldReader<u8, CLKS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKS` writer - Clock Source Select"]
pub struct CLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output of FLL is selected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKS_A::_00)
    }
    #[doc = "Internal reference clock is selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKS_A::_01)
    }
    #[doc = "External reference clock is selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKS_A::_10)
    }
    #[doc = "Reserved, defaults to 00."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u8 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    pub fn irefsten(&self) -> IREFSTEN_R {
        IREFSTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    pub fn irclken(&self) -> IRCLKEN_R {
        IRCLKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline(always)]
    pub fn irefs(&self) -> IREFS_R {
        IREFS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Reference Divider"]
    #[inline(always)]
    pub fn rdiv(&self) -> RDIV_R {
        RDIV_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    pub fn irefsten(&mut self) -> IREFSTEN_W {
        IREFSTEN_W { w: self }
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    pub fn irclken(&mut self) -> IRCLKEN_W {
        IRCLKEN_W { w: self }
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline(always)]
    pub fn irefs(&mut self) -> IREFS_W {
        IREFS_W { w: self }
    }
    #[doc = "Bits 3:5 - Reference Divider"]
    #[inline(always)]
    pub fn rdiv(&mut self) -> RDIV_W {
        RDIV_W { w: self }
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    pub fn clks(&mut self) -> CLKS_W {
        CLKS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICS Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](index.html) module"]
pub struct C1_SPEC;
impl crate::RegisterSpec for C1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c1::R](R) reader structure"]
impl crate::Readable for C1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1::W](W) writer structure"]
impl crate::Writable for C1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1 to value 0x04"]
impl crate::Resettable for C1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
