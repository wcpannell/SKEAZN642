#[doc = "Reader of register EEPROT"]
pub type R = crate::R<u8, super::EEPROT>;
#[doc = "Reader of field `DPS`"]
pub type DPS_R = crate::R<u8, u8>;
#[doc = "Reader of field `DPOPEN`"]
pub type DPOPEN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - no description available"]
    #[inline(always)]
    pub fn dps(&self) -> DPS_R {
        DPS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn dpopen(&self) -> DPOPEN_R {
        DPOPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
