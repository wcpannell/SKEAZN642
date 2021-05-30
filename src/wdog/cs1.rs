#[doc = "Register `CS1` reader"]
pub struct R(crate::R<CS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CS1_SPEC>> for R {
    fn from(reader: crate::R<CS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS1` writer"]
pub struct W(crate::W<CS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS1_SPEC>;
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
impl core::convert::From<crate::W<CS1_SPEC>> for W {
    fn from(writer: crate::W<CS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    #[doc = "0: Watchdog disabled in chip stop mode."]
    _0 = 0,
    #[doc = "1: Watchdog enabled in chip stop mode."]
    _1 = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop Enable"]
pub struct STOP_R(crate::FieldReader<bool, STOP_A>);
impl STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::_0,
            true => STOP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STOP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STOP_A::_1
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - Stop Enable"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog disabled in chip stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOP_A::_0)
    }
    #[doc = "Watchdog enabled in chip stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOP_A::_1)
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
#[doc = "Wait Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAIT_A {
    #[doc = "0: Watchdog disabled in chip wait mode."]
    _0 = 0,
    #[doc = "1: Watchdog enabled in chip wait mode."]
    _1 = 1,
}
impl From<WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAIT` reader - Wait Enable"]
pub struct WAIT_R(crate::FieldReader<bool, WAIT_A>);
impl WAIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_A {
        match self.bits {
            false => WAIT_A::_0,
            true => WAIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WAIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WAIT_A::_1
    }
}
impl core::ops::Deref for WAIT_R {
    type Target = crate::FieldReader<bool, WAIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT` writer - Wait Enable"]
pub struct WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog disabled in chip wait mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAIT_A::_0)
    }
    #[doc = "Watchdog enabled in chip wait mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAIT_A::_1)
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
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_A {
    #[doc = "0: Watchdog disabled in chip debug mode."]
    _0 = 0,
    #[doc = "1: Watchdog enabled in chip debug mode."]
    _1 = 1,
}
impl From<DBG_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG` reader - Debug Enable"]
pub struct DBG_R(crate::FieldReader<bool, DBG_A>);
impl DBG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_A {
        match self.bits {
            false => DBG_A::_0,
            true => DBG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBG_A::_1
    }
}
impl core::ops::Deref for DBG_R {
    type Target = crate::FieldReader<bool, DBG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG` writer - Debug Enable"]
pub struct DBG_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog disabled in chip debug mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBG_A::_0)
    }
    #[doc = "Watchdog enabled in chip debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBG_A::_1)
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
#[doc = "Watchdog Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TST_A {
    #[doc = "0: Watchdog test mode disabled."]
    _00 = 0,
    #[doc = "1: Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    _01 = 1,
    #[doc = "2: Watchdog test mode enabled, only the low byte is used. WDOG_CNTL is compared with WDOG_TOVALL."]
    _10 = 2,
    #[doc = "3: Watchdog test mode enabled, only the high byte is used. WDOG_CNTH is compared with WDOG_TOVALH."]
    _11 = 3,
}
impl From<TST_A> for u8 {
    #[inline(always)]
    fn from(variant: TST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TST` reader - Watchdog Test"]
pub struct TST_R(crate::FieldReader<u8, TST_A>);
impl TST_R {
    pub(crate) fn new(bits: u8) -> Self {
        TST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TST_A {
        match self.bits {
            0 => TST_A::_00,
            1 => TST_A::_01,
            2 => TST_A::_10,
            3 => TST_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == TST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == TST_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TST_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TST_A::_11
    }
}
impl core::ops::Deref for TST_R {
    type Target = crate::FieldReader<u8, TST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TST` writer - Watchdog Test"]
pub struct TST_W<'a> {
    w: &'a mut W,
}
impl<'a> TST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TST_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Watchdog test mode disabled."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TST_A::_00)
    }
    #[doc = "Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TST_A::_01)
    }
    #[doc = "Watchdog test mode enabled, only the low byte is used. WDOG_CNTL is compared with WDOG_TOVALL."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TST_A::_10)
    }
    #[doc = "Watchdog test mode enabled, only the high byte is used. WDOG_CNTH is compared with WDOG_TOVALH."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TST_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u8 & 0x03) << 3);
        self.w
    }
}
#[doc = "Allow updates\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDATE_A {
    #[doc = "0: Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    _0 = 0,
    #[doc = "1: Updates allowed. Software can modify the watchdog configuration registers within 128 bus clocks after performing the unlock write sequence."]
    _1 = 1,
}
impl From<UPDATE_A> for bool {
    #[inline(always)]
    fn from(variant: UPDATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDATE` reader - Allow updates"]
pub struct UPDATE_R(crate::FieldReader<bool, UPDATE_A>);
impl UPDATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPDATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDATE_A {
        match self.bits {
            false => UPDATE_A::_0,
            true => UPDATE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UPDATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UPDATE_A::_1
    }
}
impl core::ops::Deref for UPDATE_R {
    type Target = crate::FieldReader<bool, UPDATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDATE` writer - Allow updates"]
pub struct UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UPDATE_A::_0)
    }
    #[doc = "Updates allowed. Software can modify the watchdog configuration registers within 128 bus clocks after performing the unlock write sequence."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UPDATE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Watchdog Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_A {
    #[doc = "0: Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    _0 = 0,
    #[doc = "1: Watchdog interrupts are enabled. Watchdog resets are delayed by 128 bus clocks."]
    _1 = 1,
}
impl From<INT_A> for bool {
    #[inline(always)]
    fn from(variant: INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT` reader - Watchdog Interrupt"]
pub struct INT_R(crate::FieldReader<bool, INT_A>);
impl INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_A {
        match self.bits {
            false => INT_A::_0,
            true => INT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INT_A::_1
    }
}
impl core::ops::Deref for INT_R {
    type Target = crate::FieldReader<bool, INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT` writer - Watchdog Interrupt"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT_A::_0)
    }
    #[doc = "Watchdog interrupts are enabled. Watchdog resets are delayed by 128 bus clocks."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT_A::_1)
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
#[doc = "Watchdog Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Watchdog disabled."]
    _0 = 0,
    #[doc = "1: Watchdog enabled."]
    _1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Watchdog Enable"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::_0,
            true => EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EN_A::_1
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Watchdog Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
    #[doc = "Watchdog enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
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
    #[doc = "Bit 0 - Stop Enable"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wait Enable"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Debug Enable"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Watchdog Test"]
    #[inline(always)]
    pub fn tst(&self) -> TST_R {
        TST_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Allow updates"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Watchdog Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Enable"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 1 - Wait Enable"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W {
        WAIT_W { w: self }
    }
    #[doc = "Bit 2 - Debug Enable"]
    #[inline(always)]
    pub fn dbg(&mut self) -> DBG_W {
        DBG_W { w: self }
    }
    #[doc = "Bits 3:4 - Watchdog Test"]
    #[inline(always)]
    pub fn tst(&mut self) -> TST_W {
        TST_W { w: self }
    }
    #[doc = "Bit 5 - Allow updates"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W {
        UPDATE_W { w: self }
    }
    #[doc = "Bit 6 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bit 7 - Watchdog Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Control and Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs1](index.html) module"]
pub struct CS1_SPEC;
impl crate::RegisterSpec for CS1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cs1::R](R) reader structure"]
impl crate::Readable for CS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs1::W](W) writer structure"]
impl crate::Writable for CS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CS1 to value 0x80"]
impl crate::Resettable for CS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
