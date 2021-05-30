#[doc = "Register `SC3` reader"]
pub struct R(crate::R<SC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC3_SPEC>> for R {
    fn from(reader: crate::R<SC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC3` writer"]
pub struct W(crate::W<SC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC3_SPEC>;
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
impl core::convert::From<crate::W<SC3_SPEC>> for W {
    fn from(writer: crate::W<SC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADICLK_A {
    #[doc = "0: Bus clock"]
    _00 = 0,
    #[doc = "1: Bus clock divided by 2"]
    _01 = 1,
    #[doc = "2: Alternate clock (ALTCLK)"]
    _10 = 2,
    #[doc = "3: Asynchronous clock (ADACK)"]
    _11 = 3,
}
impl From<ADICLK_A> for u8 {
    #[inline(always)]
    fn from(variant: ADICLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADICLK` reader - Input Clock Select"]
pub struct ADICLK_R(crate::FieldReader<u8, ADICLK_A>);
impl ADICLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADICLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADICLK_A {
        match self.bits {
            0 => ADICLK_A::_00,
            1 => ADICLK_A::_01,
            2 => ADICLK_A::_10,
            3 => ADICLK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == ADICLK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == ADICLK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == ADICLK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == ADICLK_A::_11
    }
}
impl core::ops::Deref for ADICLK_R {
    type Target = crate::FieldReader<u8, ADICLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADICLK` writer - Input Clock Select"]
pub struct ADICLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADICLK_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADICLK_A::_00)
    }
    #[doc = "Bus clock divided by 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADICLK_A::_01)
    }
    #[doc = "Alternate clock (ALTCLK)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADICLK_A::_10)
    }
    #[doc = "Asynchronous clock (ADACK)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADICLK_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Conversion Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 8-bit conversion (N = 8)"]
    _00 = 0,
    #[doc = "1: 10-bit conversion (N = 10)"]
    _01 = 1,
    #[doc = "2: 12-bit conversion (N = 12)"]
    _10 = 2,
    #[doc = "3: Reserved"]
    _11 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Conversion Mode Selection"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::_00,
            1 => MODE_A::_01,
            2 => MODE_A::_10,
            3 => MODE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == MODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == MODE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == MODE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == MODE_A::_11
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Conversion Mode Selection"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8-bit conversion (N = 8)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MODE_A::_00)
    }
    #[doc = "10-bit conversion (N = 10)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MODE_A::_01)
    }
    #[doc = "12-bit conversion (N = 12)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODE_A::_10)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MODE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Long Sample Time Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLSMP_A {
    #[doc = "0: Short sample time."]
    _0 = 0,
    #[doc = "1: Long sample time."]
    _1 = 1,
}
impl From<ADLSMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADLSMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADLSMP` reader - Long Sample Time Configuration"]
pub struct ADLSMP_R(crate::FieldReader<bool, ADLSMP_A>);
impl ADLSMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADLSMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLSMP_A {
        match self.bits {
            false => ADLSMP_A::_0,
            true => ADLSMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADLSMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADLSMP_A::_1
    }
}
impl core::ops::Deref for ADLSMP_R {
    type Target = crate::FieldReader<bool, ADLSMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADLSMP` writer - Long Sample Time Configuration"]
pub struct ADLSMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADLSMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADLSMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Short sample time."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADLSMP_A::_0)
    }
    #[doc = "Long sample time."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADLSMP_A::_1)
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
#[doc = "Clock Divide Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADIV_A {
    #[doc = "0: Divide ration = 1, and clock rate = Input clock."]
    _00 = 0,
    #[doc = "1: Divide ration = 2, and clock rate = Input clock * 2."]
    _01 = 1,
    #[doc = "2: Divide ration = 3, and clock rate = Input clock * 4."]
    _10 = 2,
    #[doc = "3: Divide ration = 4, and clock rate = Input clock * 8."]
    _11 = 3,
}
impl From<ADIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADIV` reader - Clock Divide Select"]
pub struct ADIV_R(crate::FieldReader<u8, ADIV_A>);
impl ADIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIV_A {
        match self.bits {
            0 => ADIV_A::_00,
            1 => ADIV_A::_01,
            2 => ADIV_A::_10,
            3 => ADIV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == ADIV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == ADIV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == ADIV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == ADIV_A::_11
    }
}
impl core::ops::Deref for ADIV_R {
    type Target = crate::FieldReader<u8, ADIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADIV` writer - Clock Divide Select"]
pub struct ADIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide ration = 1, and clock rate = Input clock."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADIV_A::_00)
    }
    #[doc = "Divide ration = 2, and clock rate = Input clock * 2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADIV_A::_01)
    }
    #[doc = "Divide ration = 3, and clock rate = Input clock * 4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADIV_A::_10)
    }
    #[doc = "Divide ration = 4, and clock rate = Input clock * 8."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADIV_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Low-Power Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLPC_A {
    #[doc = "0: High speed configuration."]
    _0 = 0,
    #[doc = "1: Low power configuration:The power is reduced at the expense of maximum clock speed."]
    _1 = 1,
}
impl From<ADLPC_A> for bool {
    #[inline(always)]
    fn from(variant: ADLPC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADLPC` reader - Low-Power Configuration"]
pub struct ADLPC_R(crate::FieldReader<bool, ADLPC_A>);
impl ADLPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADLPC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLPC_A {
        match self.bits {
            false => ADLPC_A::_0,
            true => ADLPC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADLPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADLPC_A::_1
    }
}
impl core::ops::Deref for ADLPC_R {
    type Target = crate::FieldReader<bool, ADLPC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADLPC` writer - Low-Power Configuration"]
pub struct ADLPC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADLPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADLPC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "High speed configuration."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADLPC_A::_0)
    }
    #[doc = "Low power configuration:The power is reduced at the expense of maximum clock speed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADLPC_A::_1)
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
impl R {
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    pub fn adiclk(&self) -> ADICLK_R {
        ADICLK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Conversion Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Long Sample Time Configuration"]
    #[inline(always)]
    pub fn adlsmp(&self) -> ADLSMP_R {
        ADLSMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    pub fn adiv(&self) -> ADIV_R {
        ADIV_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    pub fn adlpc(&self) -> ADLPC_R {
        ADLPC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    pub fn adiclk(&mut self) -> ADICLK_W {
        ADICLK_W { w: self }
    }
    #[doc = "Bits 2:3 - Conversion Mode Selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 4 - Long Sample Time Configuration"]
    #[inline(always)]
    pub fn adlsmp(&mut self) -> ADLSMP_W {
        ADLSMP_W { w: self }
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    pub fn adiv(&mut self) -> ADIV_W {
        ADIV_W { w: self }
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    pub fn adlpc(&mut self) -> ADLPC_W {
        ADLPC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status and Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc3](index.html) module"]
pub struct SC3_SPEC;
impl crate::RegisterSpec for SC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc3::R](R) reader structure"]
impl crate::Readable for SC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc3::W](W) writer structure"]
impl crate::Writable for SC3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC3 to value 0"]
impl crate::Resettable for SC3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
