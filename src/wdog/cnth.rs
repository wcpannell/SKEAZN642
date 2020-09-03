#[doc = "Reader of register CNTH"]
pub type R = crate::R<u8, super::CNTH>;
#[doc = "Reader of field `CNTHIGH`"]
pub type CNTHIGH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - High byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cnthigh(&self) -> CNTHIGH_R {
        CNTHIGH_R::new((self.bits & 0xff) as u8)
    }
}
