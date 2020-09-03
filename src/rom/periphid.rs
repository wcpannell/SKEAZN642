#[doc = "Reader of register PERIPHID%s"]
pub type R = crate::R<u32, super::PERIPHID>;
#[doc = "Reader of field `PERIPHID`"]
pub type PERIPHID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn periphid(&self) -> PERIPHID_R {
        PERIPHID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
