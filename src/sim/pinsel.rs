#[doc = "Reader of register PINSEL"]
pub type R = crate::R<u32, super::PINSEL>;
#[doc = "Writer for register PINSEL"]
pub type W = crate::W<u32, super::PINSEL>;
#[doc = "Register PINSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RTCO Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCPS_A {
    #[doc = "0: RTCO is mapped on PTC4."]
    _0 = 0,
    #[doc = "1: RTCO is mapped on PTC5."]
    _1 = 1,
}
impl From<RTCPS_A> for bool {
    #[inline(always)]
    fn from(variant: RTCPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCPS`"]
pub type RTCPS_R = crate::R<bool, RTCPS_A>;
impl RTCPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCPS_A {
        match self.bits {
            false => RTCPS_A::_0,
            true => RTCPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCPS_A::_1
    }
}
#[doc = "Write proxy for field `RTCPS`"]
pub struct RTCPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTCO is mapped on PTC4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCPS_A::_0)
    }
    #[doc = "RTCO is mapped on PTC5."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCPS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "I2C0 Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0PS_A {
    #[doc = "0: I2C0_SCL and I2C0_SDA are mapped on PTA3 and PTA2, respectively."]
    _0 = 0,
    #[doc = "1: I2C0_SCL and I2C0_SDA are mapped on PTB7 and PTB6, respectively."]
    _1 = 1,
}
impl From<I2C0PS_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C0PS`"]
pub type I2C0PS_R = crate::R<bool, I2C0PS_A>;
impl I2C0PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0PS_A {
        match self.bits {
            false => I2C0PS_A::_0,
            true => I2C0PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C0PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C0PS_A::_1
    }
}
#[doc = "Write proxy for field `I2C0PS`"]
pub struct I2C0PS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C0_SCL and I2C0_SDA are mapped on PTA3 and PTA2, respectively."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C0PS_A::_0)
    }
    #[doc = "I2C0_SCL and I2C0_SDA are mapped on PTB7 and PTB6, respectively."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C0PS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "SPI0 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0PS_A {
    #[doc = "0: SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS0 are mapped on PTB2, PTB3, PTB4, and PTB5."]
    _0 = 0,
    #[doc = "1: SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS0 are mapped on PTE0, PTE1, PTE2, and PTE3."]
    _1 = 1,
}
impl From<SPI0PS_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0PS`"]
pub type SPI0PS_R = crate::R<bool, SPI0PS_A>;
impl SPI0PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0PS_A {
        match self.bits {
            false => SPI0PS_A::_0,
            true => SPI0PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI0PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI0PS_A::_1
    }
}
#[doc = "Write proxy for field `SPI0PS`"]
pub struct SPI0PS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS0 are mapped on PTB2, PTB3, PTB4, and PTB5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0PS_A::_0)
    }
    #[doc = "SPI0_SCK, SPI0_MOSI, SPI0_MISO, and SPI0_PCS0 are mapped on PTE0, PTE1, PTE2, and PTE3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0PS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "UART0 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0PS_A {
    #[doc = "0: UART0_RX and UART0_TX are mapped on PTB0 and PTB1."]
    _0 = 0,
    #[doc = "1: UART0_RX and UART0_TX are mapped on PTA2 and PTA3."]
    _1 = 1,
}
impl From<UART0PS_A> for bool {
    #[inline(always)]
    fn from(variant: UART0PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0PS`"]
pub type UART0PS_R = crate::R<bool, UART0PS_A>;
impl UART0PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0PS_A {
        match self.bits {
            false => UART0PS_A::_0,
            true => UART0PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART0PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART0PS_A::_1
    }
}
#[doc = "Write proxy for field `UART0PS`"]
pub struct UART0PS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART0_RX and UART0_TX are mapped on PTB0 and PTB1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0PS_A::_0)
    }
    #[doc = "UART0_RX and UART0_TX are mapped on PTA2 and PTA3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0PS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "FTM0\\[0\\]
Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0PS0_A {
    #[doc = "0: FTM0\\[0\\]
channels are mapped on PTA0."]
    _0 = 0,
    #[doc = "1: FTM0\\[0\\]
channels are mapped on PTB2."]
    _1 = 1,
}
impl From<FTM0PS0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0PS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM0PS0`"]
pub type FTM0PS0_R = crate::R<bool, FTM0PS0_A>;
impl FTM0PS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0PS0_A {
        match self.bits {
            false => FTM0PS0_A::_0,
            true => FTM0PS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0PS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0PS0_A::_1
    }
}
#[doc = "Write proxy for field `FTM0PS0`"]
pub struct FTM0PS0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0PS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0PS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0\\[0\\]
channels are mapped on PTA0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0PS0_A::_0)
    }
    #[doc = "FTM0\\[0\\]
channels are mapped on PTB2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0PS0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "FTM0\\[1\\]
Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0PS1_A {
    #[doc = "0: FTM0\\[1\\]
channels are mapped on PTA1."]
    _0 = 0,
    #[doc = "1: FTM0\\[1\\]
channels are mapped on PTB3."]
    _1 = 1,
}
impl From<FTM0PS1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0PS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM0PS1`"]
pub type FTM0PS1_R = crate::R<bool, FTM0PS1_A>;
impl FTM0PS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0PS1_A {
        match self.bits {
            false => FTM0PS1_A::_0,
            true => FTM0PS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0PS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0PS1_A::_1
    }
}
#[doc = "Write proxy for field `FTM0PS1`"]
pub struct FTM0PS1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0PS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0PS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM0\\[1\\]
channels are mapped on PTA1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0PS1_A::_0)
    }
    #[doc = "FTM0\\[1\\]
channels are mapped on PTB3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0PS1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "FTM1\\[0\\]
Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1PS0_A {
    #[doc = "0: FTM1\\[0\\]
channels are mapped on PTC4."]
    _0 = 0,
    #[doc = "1: FTM1\\[0\\]
channels are mapped on PTH2."]
    _1 = 1,
}
impl From<FTM1PS0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1PS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM1PS0`"]
pub type FTM1PS0_R = crate::R<bool, FTM1PS0_A>;
impl FTM1PS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1PS0_A {
        match self.bits {
            false => FTM1PS0_A::_0,
            true => FTM1PS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1PS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1PS0_A::_1
    }
}
#[doc = "Write proxy for field `FTM1PS0`"]
pub struct FTM1PS0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1PS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1PS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM1\\[0\\]
channels are mapped on PTC4."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1PS0_A::_0)
    }
    #[doc = "FTM1\\[0\\]
channels are mapped on PTH2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1PS0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "FTM1\\[1\\]
Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1PS1_A {
    #[doc = "0: FTM1\\[1\\]
channels are mapped on PTC5."]
    _0 = 0,
    #[doc = "1: FTM1\\[1\\]
channels are mapped on PTE7."]
    _1 = 1,
}
impl From<FTM1PS1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1PS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM1PS1`"]
pub type FTM1PS1_R = crate::R<bool, FTM1PS1_A>;
impl FTM1PS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1PS1_A {
        match self.bits {
            false => FTM1PS1_A::_0,
            true => FTM1PS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1PS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1PS1_A::_1
    }
}
#[doc = "Write proxy for field `FTM1PS1`"]
pub struct FTM1PS1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1PS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1PS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM1\\[1\\]
channels are mapped on PTC5."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1PS1_A::_0)
    }
    #[doc = "FTM1\\[1\\]
channels are mapped on PTE7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1PS1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "FTM2\\[0\\]
Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2PS0_A {
    #[doc = "0: FTM2\\[0\\]
channels are mapped on PTC0."]
    _0 = 0,
    #[doc = "1: FTM2\\[0\\]
channels are mapped on PTH0."]
    _1 = 1,
}
impl From<FTM2PS0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2PS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM2PS0`"]
pub type FTM2PS0_R = crate::R<bool, FTM2PS0_A>;
impl FTM2PS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2PS0_A {
        match self.bits {
            false => FTM2PS0_A::_0,
            true => FTM2PS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2PS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2PS0_A::_1
    }
}
#[doc = "Write proxy for field `FTM2PS0`"]
pub struct FTM2PS0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM2\\[0\\]
channels are mapped on PTC0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2PS0_A::_0)
    }
    #[doc = "FTM2\\[0\\]
channels are mapped on PTH0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2PS0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "FTM2\\[1\\]
Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2PS1_A {
    #[doc = "0: FTM2\\[1\\]
channels are mapped on PTC1."]
    _0 = 0,
    #[doc = "1: FTM2\\[1\\]
channels are mapped on PTH1."]
    _1 = 1,
}
impl From<FTM2PS1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2PS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM2PS1`"]
pub type FTM2PS1_R = crate::R<bool, FTM2PS1_A>;
impl FTM2PS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2PS1_A {
        match self.bits {
            false => FTM2PS1_A::_0,
            true => FTM2PS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2PS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2PS1_A::_1
    }
}
#[doc = "Write proxy for field `FTM2PS1`"]
pub struct FTM2PS1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM2\\[1\\]
channels are mapped on PTC1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2PS1_A::_0)
    }
    #[doc = "FTM2\\[1\\]
channels are mapped on PTH1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2PS1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "FTM2\\[2\\]
Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2PS2_A {
    #[doc = "0: FTM2\\[2\\]
channels are mapped on PTC2."]
    _0 = 0,
    #[doc = "1: FTM2\\[2\\]
channels are mapped on PTD0."]
    _1 = 1,
}
impl From<FTM2PS2_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2PS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM2PS2`"]
pub type FTM2PS2_R = crate::R<bool, FTM2PS2_A>;
impl FTM2PS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2PS2_A {
        match self.bits {
            false => FTM2PS2_A::_0,
            true => FTM2PS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2PS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2PS2_A::_1
    }
}
#[doc = "Write proxy for field `FTM2PS2`"]
pub struct FTM2PS2_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM2\\[2\\]
channels are mapped on PTC2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2PS2_A::_0)
    }
    #[doc = "FTM2\\[2\\]
channels are mapped on PTD0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2PS2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "FTM2\\[3\\]
Port Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2PS3_A {
    #[doc = "0: FTM2\\[3\\]
channels are mapped on PTC3."]
    _0 = 0,
    #[doc = "1: FTM2\\[3\\]
channels are mapped on PTD1."]
    _1 = 1,
}
impl From<FTM2PS3_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2PS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM2PS3`"]
pub type FTM2PS3_R = crate::R<bool, FTM2PS3_A>;
impl FTM2PS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2PS3_A {
        match self.bits {
            false => FTM2PS3_A::_0,
            true => FTM2PS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2PS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2PS3_A::_1
    }
}
#[doc = "Write proxy for field `FTM2PS3`"]
pub struct FTM2PS3_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2PS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2PS3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM2\\[3\\]
channels are mapped on PTC3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2PS3_A::_0)
    }
    #[doc = "FTM2\\[3\\]
channels are mapped on PTD1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2PS3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - RTCO Pin Select"]
    #[inline(always)]
    pub fn rtcps(&self) -> RTCPS_R {
        RTCPS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C0 Port Pin Select"]
    #[inline(always)]
    pub fn i2c0ps(&self) -> I2C0PS_R {
        I2C0PS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI0 Pin Select"]
    #[inline(always)]
    pub fn spi0ps(&self) -> SPI0PS_R {
        SPI0PS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART0 Pin Select"]
    #[inline(always)]
    pub fn uart0ps(&self) -> UART0PS_R {
        UART0PS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FTM0\\[0\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps0(&self) -> FTM0PS0_R {
        FTM0PS0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FTM0\\[1\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps1(&self) -> FTM0PS1_R {
        FTM0PS1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FTM1\\[0\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm1ps0(&self) -> FTM1PS0_R {
        FTM1PS0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FTM1\\[1\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm1ps1(&self) -> FTM1PS1_R {
        FTM1PS1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FTM2\\[0\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps0(&self) -> FTM2PS0_R {
        FTM2PS0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - FTM2\\[1\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps1(&self) -> FTM2PS1_R {
        FTM2PS1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FTM2\\[2\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps2(&self) -> FTM2PS2_R {
        FTM2PS2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FTM2\\[3\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps3(&self) -> FTM2PS3_R {
        FTM2PS3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RTCO Pin Select"]
    #[inline(always)]
    pub fn rtcps(&mut self) -> RTCPS_W {
        RTCPS_W { w: self }
    }
    #[doc = "Bit 5 - I2C0 Port Pin Select"]
    #[inline(always)]
    pub fn i2c0ps(&mut self) -> I2C0PS_W {
        I2C0PS_W { w: self }
    }
    #[doc = "Bit 6 - SPI0 Pin Select"]
    #[inline(always)]
    pub fn spi0ps(&mut self) -> SPI0PS_W {
        SPI0PS_W { w: self }
    }
    #[doc = "Bit 7 - UART0 Pin Select"]
    #[inline(always)]
    pub fn uart0ps(&mut self) -> UART0PS_W {
        UART0PS_W { w: self }
    }
    #[doc = "Bit 8 - FTM0\\[0\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps0(&mut self) -> FTM0PS0_W {
        FTM0PS0_W { w: self }
    }
    #[doc = "Bit 9 - FTM0\\[1\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm0ps1(&mut self) -> FTM0PS1_W {
        FTM0PS1_W { w: self }
    }
    #[doc = "Bit 10 - FTM1\\[0\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm1ps0(&mut self) -> FTM1PS0_W {
        FTM1PS0_W { w: self }
    }
    #[doc = "Bit 11 - FTM1\\[1\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm1ps1(&mut self) -> FTM1PS1_W {
        FTM1PS1_W { w: self }
    }
    #[doc = "Bit 12 - FTM2\\[0\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps0(&mut self) -> FTM2PS0_W {
        FTM2PS0_W { w: self }
    }
    #[doc = "Bit 13 - FTM2\\[1\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps1(&mut self) -> FTM2PS1_W {
        FTM2PS1_W { w: self }
    }
    #[doc = "Bit 14 - FTM2\\[2\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps2(&mut self) -> FTM2PS2_W {
        FTM2PS2_W { w: self }
    }
    #[doc = "Bit 15 - FTM2\\[3\\]
Port Pin Select"]
    #[inline(always)]
    pub fn ftm2ps3(&mut self) -> FTM2PS3_W {
        FTM2PS3_W { w: self }
    }
}
