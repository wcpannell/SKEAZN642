#[doc = "Register `SRSID` reader"]
pub struct R(crate::R<SRSID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SRSID_SPEC>> for R {
    fn from(reader: crate::R<SRSID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Low Voltage Detect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVD_A {
    #[doc = "0: Reset is not caused by LVD trip or POR."]
    _0 = 0,
    #[doc = "1: Reset is caused by LVD trip or POR."]
    _1 = 1,
}
impl From<LVD_A> for bool {
    #[inline(always)]
    fn from(variant: LVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD` reader - Low Voltage Detect"]
pub struct LVD_R(crate::FieldReader<bool, LVD_A>);
impl LVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD_A {
        match self.bits {
            false => LVD_A::_0,
            true => LVD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LVD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LVD_A::_1
    }
}
impl core::ops::Deref for LVD_R {
    type Target = crate::FieldReader<bool, LVD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Internal Clock Source Module Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOC_A {
    #[doc = "0: Reset is not caused by the ICS module."]
    _0 = 0,
    #[doc = "1: Reset is caused by the ICS module."]
    _1 = 1,
}
impl From<LOC_A> for bool {
    #[inline(always)]
    fn from(variant: LOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOC` reader - Internal Clock Source Module Reset"]
pub struct LOC_R(crate::FieldReader<bool, LOC_A>);
impl LOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOC_A {
        match self.bits {
            false => LOC_A::_0,
            true => LOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOC_A::_1
    }
}
impl core::ops::Deref for LOC_R {
    type Target = crate::FieldReader<bool, LOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Watchdog (WDOG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_A {
    #[doc = "0: Reset is not caused by WDOG timeout."]
    _0 = 0,
    #[doc = "1: Reset is caused by WDOG timeout."]
    _1 = 1,
}
impl From<WDOG_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOG` reader - Watchdog (WDOG)"]
pub struct WDOG_R(crate::FieldReader<bool, WDOG_A>);
impl WDOG_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDOG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_A {
        match self.bits {
            false => WDOG_A::_0,
            true => WDOG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WDOG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WDOG_A::_1
    }
}
impl core::ops::Deref for WDOG_R {
    type Target = crate::FieldReader<bool, WDOG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "External Reset Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
    #[doc = "0: Reset is not caused by external reset pin."]
    _0 = 0,
    #[doc = "1: Reset came from external reset pin."]
    _1 = 1,
}
impl From<PIN_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN` reader - External Reset Pin"]
pub struct PIN_R(crate::FieldReader<bool, PIN_A>);
impl PIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_A {
        match self.bits {
            false => PIN_A::_0,
            true => PIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PIN_A::_1
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<bool, PIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Power-On Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_A {
    #[doc = "0: Reset not caused by POR."]
    _0 = 0,
    #[doc = "1: POR caused reset."]
    _1 = 1,
}
impl From<POR_A> for bool {
    #[inline(always)]
    fn from(variant: POR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POR` reader - Power-On Reset"]
pub struct POR_R(crate::FieldReader<bool, POR_A>);
impl POR_R {
    pub(crate) fn new(bits: bool) -> Self {
        POR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_A {
        match self.bits {
            false => POR_A::_0,
            true => POR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == POR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == POR_A::_1
    }
}
impl core::ops::Deref for POR_R {
    type Target = crate::FieldReader<bool, POR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Core Lockup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_A {
    #[doc = "0: Reset is not caused by core LOCKUP event."]
    _0 = 0,
    #[doc = "1: Reset is caused by core LOCKUP event."]
    _1 = 1,
}
impl From<LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP` reader - Core Lockup"]
pub struct LOCKUP_R(crate::FieldReader<bool, LOCKUP_A>);
impl LOCKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_A {
        match self.bits {
            false => LOCKUP_A::_0,
            true => LOCKUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOCKUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOCKUP_A::_1
    }
}
impl core::ops::Deref for LOCKUP_R {
    type Target = crate::FieldReader<bool, LOCKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_A {
    #[doc = "0: Reset is not caused by software setting of SYSRESETREQ bit."]
    _0 = 0,
    #[doc = "1: Reset caused by software setting of SYSRESETREQ bit"]
    _1 = 1,
}
impl From<SW_A> for bool {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW` reader - Software"]
pub struct SW_R(crate::FieldReader<bool, SW_A>);
impl SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            false => SW_A::_0,
            true => SW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SW_A::_1
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<bool, SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MDM-AP System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDMAP_A {
    #[doc = "0: Reset is not caused by host debugger system setting of the System Reset Request bit."]
    _0 = 0,
    #[doc = "1: Reset is caused by host debugger system setting of the System Reset Request bit."]
    _1 = 1,
}
impl From<MDMAP_A> for bool {
    #[inline(always)]
    fn from(variant: MDMAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMAP` reader - MDM-AP System Reset Request"]
pub struct MDMAP_R(crate::FieldReader<bool, MDMAP_A>);
impl MDMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDMAP_A {
        match self.bits {
            false => MDMAP_A::_0,
            true => MDMAP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MDMAP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MDMAP_A::_1
    }
}
impl core::ops::Deref for MDMAP_R {
    type Target = crate::FieldReader<bool, MDMAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stop Mode Acknowledge Error Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACKERR_A {
    #[doc = "0: Reset is not caused by peripheral failure to acknowledge attempt to enter Stop mode."]
    _0 = 0,
    #[doc = "1: Reset is caused by peripheral failure to acknowledge attempt to enter Stop mode."]
    _1 = 1,
}
impl From<SACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: SACKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKERR` reader - Stop Mode Acknowledge Error Reset"]
pub struct SACKERR_R(crate::FieldReader<bool, SACKERR_A>);
impl SACKERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SACKERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SACKERR_A {
        match self.bits {
            false => SACKERR_A::_0,
            true => SACKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SACKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SACKERR_A::_1
    }
}
impl core::ops::Deref for SACKERR_R {
    type Target = crate::FieldReader<bool, SACKERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Device Pin ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINID_A {
    #[doc = "0: 8-pin"]
    _0000 = 0,
    #[doc = "1: 16-pin"]
    _0001 = 1,
    #[doc = "2: 20-pin"]
    _0010 = 2,
    #[doc = "3: 24-pin"]
    _0011 = 3,
    #[doc = "4: 32-pin"]
    _0100 = 4,
    #[doc = "5: 44-pin"]
    _0101 = 5,
    #[doc = "6: 48-pin"]
    _0110 = 6,
    #[doc = "7: 64-pin"]
    _0111 = 7,
    #[doc = "8: 80-pin"]
    _1000 = 8,
    #[doc = "10: 100-pin"]
    _1010 = 10,
}
impl From<PINID_A> for u8 {
    #[inline(always)]
    fn from(variant: PINID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PINID` reader - Device Pin ID"]
pub struct PINID_R(crate::FieldReader<u8, PINID_A>);
impl PINID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PINID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PINID_A> {
        match self.bits {
            0 => Some(PINID_A::_0000),
            1 => Some(PINID_A::_0001),
            2 => Some(PINID_A::_0010),
            3 => Some(PINID_A::_0011),
            4 => Some(PINID_A::_0100),
            5 => Some(PINID_A::_0101),
            6 => Some(PINID_A::_0110),
            7 => Some(PINID_A::_0111),
            8 => Some(PINID_A::_1000),
            10 => Some(PINID_A::_1010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == PINID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == PINID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == PINID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == PINID_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        **self == PINID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        **self == PINID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        **self == PINID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        **self == PINID_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        **self == PINID_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        **self == PINID_A::_1010
    }
}
impl core::ops::Deref for PINID_R {
    type Target = crate::FieldReader<u8, PINID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RevID` reader - Device Revision Number"]
pub struct REVID_R(crate::FieldReader<u8, u8>);
impl REVID_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Kinetis sub-family ID\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBFAMID_A {
    #[doc = "2: KEx2 sub-family"]
    _0010 = 2,
}
impl From<SUBFAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBFAMID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SUBFAMID` reader - Kinetis sub-family ID"]
pub struct SUBFAMID_R(crate::FieldReader<u8, SUBFAMID_A>);
impl SUBFAMID_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUBFAMID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUBFAMID_A> {
        match self.bits {
            2 => Some(SUBFAMID_A::_0010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == SUBFAMID_A::_0010
    }
}
impl core::ops::Deref for SUBFAMID_R {
    type Target = crate::FieldReader<u8, SUBFAMID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Kinetis family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAMID_A {
    #[doc = "0: KE0x family."]
    _0000 = 0,
}
impl From<FAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FAMID` reader - Kinetis family ID"]
pub struct FAMID_R(crate::FieldReader<u8, FAMID_A>);
impl FAMID_R {
    pub(crate) fn new(bits: u8) -> Self {
        FAMID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FAMID_A> {
        match self.bits {
            0 => Some(FAMID_A::_0000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == FAMID_A::_0000
    }
}
impl core::ops::Deref for FAMID_R {
    type Target = crate::FieldReader<u8, FAMID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Low Voltage Detect"]
    #[inline(always)]
    pub fn lvd(&self) -> LVD_R {
        LVD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Internal Clock Source Module Reset"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Watchdog (WDOG)"]
    #[inline(always)]
    pub fn wdog(&self) -> WDOG_R {
        WDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Reset Pin"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Power-On Reset"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Core Lockup"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn mdmap(&self) -> MDMAP_R {
        MDMAP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Stop Mode Acknowledge Error Reset"]
    #[inline(always)]
    pub fn sackerr(&self) -> SACKERR_R {
        SACKERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Device Pin ID"]
    #[inline(always)]
    pub fn pinid(&self) -> PINID_R {
        PINID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Device Revision Number"]
    #[inline(always)]
    pub fn rev_id(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Kinetis sub-family ID"]
    #[inline(always)]
    pub fn subfamid(&self) -> SUBFAMID_R {
        SUBFAMID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Kinetis family ID"]
    #[inline(always)]
    pub fn famid(&self) -> FAMID_R {
        FAMID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "System Reset Status and ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srsid](index.html) module"]
pub struct SRSID_SPEC;
impl crate::RegisterSpec for SRSID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srsid::R](R) reader structure"]
impl crate::Readable for SRSID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRSID to value 0x0200_0002"]
impl crate::Resettable for SRSID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0002
    }
}
