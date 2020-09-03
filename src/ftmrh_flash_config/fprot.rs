#[doc = "Reader of register FPROT"]
pub type R = crate::R<u8, super::FPROT>;
#[doc = "Reader of field `FPLS`"]
pub type FPLS_R = crate::R<u8, u8>;
#[doc = "Reader of field `FPLDIS`"]
pub type FPLDIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPHS`"]
pub type FPHS_R = crate::R<u8, u8>;
#[doc = "Reader of field `FPHDIS`"]
pub type FPHDIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPOPEN`"]
pub type FPOPEN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - no description available"]
    #[inline(always)]
    pub fn fpls(&self) -> FPLS_R {
        FPLS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn fpldis(&self) -> FPLDIS_R {
        FPLDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - no description available"]
    #[inline(always)]
    pub fn fphs(&self) -> FPHS_R {
        FPHS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn fphdis(&self) -> FPHDIS_R {
        FPHDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn fpopen(&self) -> FPOPEN_R {
        FPOPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
