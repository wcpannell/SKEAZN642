#[doc = "Reader of register S"]
pub type R = crate::R<u8, super::S>;
#[doc = "Clock Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKST_A {
    #[doc = "0: Output of FLL is selected."]
    _00 = 0,
    #[doc = "1: FLL Bypassed, internal reference clock is selected."]
    _01 = 1,
    #[doc = "2: FLL Bypassed, external reference clock is selected."]
    _10 = 2,
    #[doc = "3: Reserved."]
    _11 = 3,
}
impl From<CLKST_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKST`"]
pub type CLKST_R = crate::R<u8, CLKST_A>;
impl CLKST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKST_A {
        match self.bits {
            0 => CLKST_A::_00,
            1 => CLKST_A::_01,
            2 => CLKST_A::_10,
            3 => CLKST_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLKST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLKST_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLKST_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLKST_A::_11
    }
}
#[doc = "Internal Reference Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFST_A {
    #[doc = "0: Source of reference clock is external clock."]
    _0 = 0,
    #[doc = "1: Source of reference clock is internal clock."]
    _1 = 1,
}
impl From<IREFST_A> for bool {
    #[inline(always)]
    fn from(variant: IREFST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IREFST`"]
pub type IREFST_R = crate::R<bool, IREFST_A>;
impl IREFST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFST_A {
        match self.bits {
            false => IREFST_A::_0,
            true => IREFST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREFST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREFST_A::_1
    }
}
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: FLL is currently unlocked."]
    _0 = 0,
    #[doc = "1: FLL is currently locked."]
    _1 = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::_0,
            true => LOCK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCK_A::_1
    }
}
#[doc = "Loss of Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLS_A {
    #[doc = "0: FLL has not lost lock since LOLS was last cleared."]
    _0 = 0,
    #[doc = "1: FLL has lost lock since LOLS was last cleared."]
    _1 = 1,
}
impl From<LOLS_A> for bool {
    #[inline(always)]
    fn from(variant: LOLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOLS`"]
pub type LOLS_R = crate::R<bool, LOLS_A>;
impl LOLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOLS_A {
        match self.bits {
            false => LOLS_A::_0,
            true => LOLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOLS_A::_1
    }
}
impl R {
    #[doc = "Bits 2:3 - Clock Mode Status"]
    #[inline(always)]
    pub fn clkst(&self) -> CLKST_R {
        CLKST_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Internal Reference Status"]
    #[inline(always)]
    pub fn irefst(&self) -> IREFST_R {
        IREFST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loss of Lock Status"]
    #[inline(always)]
    pub fn lols(&self) -> LOLS_R {
        LOLS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
