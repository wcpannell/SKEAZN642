#[doc = "Reader of register FSEC"]
pub type R = crate::R<u8, super::FSEC>;
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEYEN`"]
pub type KEYEN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Flash Security"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Backdoor Key Security Enable"]
    #[inline(always)]
    pub fn keyen(&self) -> KEYEN_R {
        KEYEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
