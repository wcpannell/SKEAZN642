#[doc = "Register `CS2` reader"]
pub struct R(crate::R<CS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CS2_SPEC>> for R {
    fn from(reader: crate::R<CS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS2` writer"]
pub struct W(crate::W<CS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS2_SPEC>;
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
impl core::convert::From<crate::W<CS2_SPEC>> for W {
    fn from(writer: crate::W<CS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Watchdog Clock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_A {
    #[doc = "0: Bus clock."]
    _00 = 0,
    #[doc = "1: 1 kHz internal low-power oscillator (LPOCLK)."]
    _01 = 1,
    #[doc = "2: 32 kHz internal oscillator (ICSIRCLK)."]
    _10 = 2,
    #[doc = "3: External clock source."]
    _11 = 3,
}
impl From<CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK` reader - Watchdog Clock"]
pub struct CLK_R(crate::FieldReader<u8, CLK_A>);
impl CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_A {
        match self.bits {
            0 => CLK_A::_00,
            1 => CLK_A::_01,
            2 => CLK_A::_10,
            3 => CLK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == CLK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == CLK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == CLK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == CLK_A::_11
    }
}
impl core::ops::Deref for CLK_R {
    type Target = crate::FieldReader<u8, CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK` writer - Watchdog Clock"]
pub struct CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Bus clock."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLK_A::_00)
    }
    #[doc = "1 kHz internal low-power oscillator (LPOCLK)."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLK_A::_01)
    }
    #[doc = "32 kHz internal oscillator (ICSIRCLK)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLK_A::_10)
    }
    #[doc = "External clock source."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLK_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
#[doc = "Watchdog Prescalar\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRES_A {
    #[doc = "0: 256 prescalar disabled."]
    _0 = 0,
    #[doc = "1: 256 prescalar enabled."]
    _1 = 1,
}
impl From<PRES_A> for bool {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRES` reader - Watchdog Prescalar"]
pub struct PRES_R(crate::FieldReader<bool, PRES_A>);
impl PRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRES_A {
        match self.bits {
            false => PRES_A::_0,
            true => PRES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRES_A::_1
    }
}
impl core::ops::Deref for PRES_R {
    type Target = crate::FieldReader<bool, PRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRES` writer - Watchdog Prescalar"]
pub struct PRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "256 prescalar disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRES_A::_0)
    }
    #[doc = "256 prescalar enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRES_A::_1)
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
#[doc = "Watchdog Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLG_A {
    #[doc = "0: No interrupt occurred."]
    _0 = 0,
    #[doc = "1: An interrupt occurred."]
    _1 = 1,
}
impl From<FLG_A> for bool {
    #[inline(always)]
    fn from(variant: FLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLG` reader - Watchdog Interrupt Flag"]
pub struct FLG_R(crate::FieldReader<bool, FLG_A>);
impl FLG_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLG_A {
        match self.bits {
            false => FLG_A::_0,
            true => FLG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLG_A::_1
    }
}
impl core::ops::Deref for FLG_R {
    type Target = crate::FieldReader<bool, FLG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLG` writer - Watchdog Interrupt Flag"]
pub struct FLG_W<'a> {
    w: &'a mut W,
}
impl<'a> FLG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLG_A::_0)
    }
    #[doc = "An interrupt occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Watchdog Window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIN_A {
    #[doc = "0: Window mode disabled."]
    _0 = 0,
    #[doc = "1: Window mode enabled."]
    _1 = 1,
}
impl From<WIN_A> for bool {
    #[inline(always)]
    fn from(variant: WIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN` reader - Watchdog Window"]
pub struct WIN_R(crate::FieldReader<bool, WIN_A>);
impl WIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIN_A {
        match self.bits {
            false => WIN_A::_0,
            true => WIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WIN_A::_1
    }
}
impl core::ops::Deref for WIN_R {
    type Target = crate::FieldReader<bool, WIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIN` writer - Watchdog Window"]
pub struct WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Window mode disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WIN_A::_0)
    }
    #[doc = "Window mode enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WIN_A::_1)
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
    #[doc = "Bits 0:1 - Watchdog Clock"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Watchdog Prescalar"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn flg(&self) -> FLG_R {
        FLG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Watchdog Window"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Watchdog Clock"]
    #[inline(always)]
    pub fn clk(&mut self) -> CLK_W {
        CLK_W { w: self }
    }
    #[doc = "Bit 4 - Watchdog Prescalar"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W {
        PRES_W { w: self }
    }
    #[doc = "Bit 6 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn flg(&mut self) -> FLG_W {
        FLG_W { w: self }
    }
    #[doc = "Bit 7 - Watchdog Window"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W {
        WIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Control and Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs2](index.html) module"]
pub struct CS2_SPEC;
impl crate::RegisterSpec for CS2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cs2::R](R) reader structure"]
impl crate::Readable for CS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs2::W](W) writer structure"]
impl crate::Writable for CS2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CS2 to value 0x01"]
impl crate::Resettable for CS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
