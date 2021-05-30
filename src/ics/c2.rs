#[doc = "Register `C2` reader"]
pub struct R(crate::R<C2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<C2_SPEC>> for R {
    fn from(reader: crate::R<C2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2` writer"]
pub struct W(crate::W<C2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2_SPEC>;
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
impl core::convert::From<crate::W<C2_SPEC>> for W {
    fn from(writer: crate::W<C2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low Power Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP_A {
    #[doc = "0: FLL is not disabled in bypass mode."]
    _0 = 0,
    #[doc = "1: FLL is disabled in bypass modes unless debug is active."]
    _1 = 1,
}
impl From<LP_A> for bool {
    #[inline(always)]
    fn from(variant: LP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP` reader - Low Power Select"]
pub struct LP_R(crate::FieldReader<bool, LP_A>);
impl LP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_A {
        match self.bits {
            false => LP_A::_0,
            true => LP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LP_A::_1
    }
}
impl core::ops::Deref for LP_R {
    type Target = crate::FieldReader<bool, LP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LP` writer - Low Power Select"]
pub struct LP_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FLL is not disabled in bypass mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LP_A::_0)
    }
    #[doc = "FLL is disabled in bypass modes unless debug is active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LP_A::_1)
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
#[doc = "Bus Frequency Divider\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BDIV_A {
    #[doc = "0: Encoding 0-Divides the selected clock by 1."]
    _000 = 0,
    #[doc = "1: Encoding 1-Divides the selected clock by 2 (reset default)."]
    _001 = 1,
    #[doc = "2: Encoding 2-Divides the selected clock by 4."]
    _010 = 2,
    #[doc = "3: Encoding 3-Divides the selected clock by 8."]
    _011 = 3,
    #[doc = "4: Encoding 4-Divides the selected clock by 16."]
    _100 = 4,
    #[doc = "5: Encoding 5-Divides the selected clock by 32."]
    _101 = 5,
    #[doc = "6: Encoding 6-Divides the selected clock by 64."]
    _110 = 6,
    #[doc = "7: Encoding 7-Divides the selected clock by 128."]
    _111 = 7,
}
impl From<BDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: BDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BDIV` reader - Bus Frequency Divider"]
pub struct BDIV_R(crate::FieldReader<u8, BDIV_A>);
impl BDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        BDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDIV_A {
        match self.bits {
            0 => BDIV_A::_000,
            1 => BDIV_A::_001,
            2 => BDIV_A::_010,
            3 => BDIV_A::_011,
            4 => BDIV_A::_100,
            5 => BDIV_A::_101,
            6 => BDIV_A::_110,
            7 => BDIV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == BDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == BDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == BDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == BDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == BDIV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == BDIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == BDIV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == BDIV_A::_111
    }
}
impl core::ops::Deref for BDIV_R {
    type Target = crate::FieldReader<u8, BDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDIV` writer - Bus Frequency Divider"]
pub struct BDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Encoding 0-Divides the selected clock by 1."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(BDIV_A::_000)
    }
    #[doc = "Encoding 1-Divides the selected clock by 2 (reset default)."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(BDIV_A::_001)
    }
    #[doc = "Encoding 2-Divides the selected clock by 4."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(BDIV_A::_010)
    }
    #[doc = "Encoding 3-Divides the selected clock by 8."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(BDIV_A::_011)
    }
    #[doc = "Encoding 4-Divides the selected clock by 16."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(BDIV_A::_100)
    }
    #[doc = "Encoding 5-Divides the selected clock by 32."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(BDIV_A::_101)
    }
    #[doc = "Encoding 6-Divides the selected clock by 64."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(BDIV_A::_110)
    }
    #[doc = "Encoding 7-Divides the selected clock by 128."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(BDIV_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u8 & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&self) -> LP_R {
        LP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Bus Frequency Divider"]
    #[inline(always)]
    pub fn bdiv(&self) -> BDIV_R {
        BDIV_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&mut self) -> LP_W {
        LP_W { w: self }
    }
    #[doc = "Bits 5:7 - Bus Frequency Divider"]
    #[inline(always)]
    pub fn bdiv(&mut self) -> BDIV_W {
        BDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICS Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c2::R](R) reader structure"]
impl crate::Readable for C2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2::W](W) writer structure"]
impl crate::Writable for C2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2 to value 0x20"]
impl crate::Resettable for C2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
