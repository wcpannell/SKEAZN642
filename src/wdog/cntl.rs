#[doc = "Reader of register CNTL"]
pub type R = crate::R<u8, super::CNTL>;
#[doc = "Reader of field `CNTLOW`"]
pub type CNTLOW_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub fn cntlow(&self) -> CNTLOW_R {
        CNTLOW_R::new((self.bits & 0xff) as u8)
    }
}
