#[doc = "Reader of register FCNFG"]
pub type R = crate::R<u8, super::FCNFG>;
#[doc = "Writer for register FCNFG"]
pub type W = crate::W<u8, super::FCNFG>;
#[doc = "Register FCNFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FCNFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `FSFD`"]
pub type FSFD_R = crate::R<bool, FSFD_A>;
impl FSFD_R {
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
        *self == FSFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSFD_A::_1
    }
}
#[doc = "Write proxy for field `FSFD`"]
pub struct FSFD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
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
#[doc = "Reader of field `FDFD`"]
pub type FDFD_R = crate::R<bool, FDFD_A>;
impl FDFD_R {
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
        *self == FDFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDFD_A::_1
    }
}
#[doc = "Write proxy for field `FDFD`"]
pub struct FDFD_W<'a> {
    w: &'a mut W,
}
impl<'a> FDFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
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
#[doc = "Reader of field `IGNSF`"]
pub type IGNSF_R = crate::R<bool, IGNSF_A>;
impl IGNSF_R {
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
        *self == IGNSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IGNSF_A::_1
    }
}
#[doc = "Write proxy for field `IGNSF`"]
pub struct IGNSF_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IGNSF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
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
#[doc = "Reader of field `CCIE`"]
pub type CCIE_R = crate::R<bool, CCIE_A>;
impl CCIE_R {
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
        *self == CCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCIE_A::_1
    }
}
#[doc = "Write proxy for field `CCIE`"]
pub struct CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
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
}
