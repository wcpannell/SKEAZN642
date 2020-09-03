#[doc = "Reader of register TOVAL"]
pub type R = crate::R<u16, super::TOVAL>;
#[doc = "Writer for register TOVAL"]
pub type W = crate::W<u16, super::TOVAL>;
#[doc = "Register TOVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::TOVAL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOVAL`"]
pub type TOVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOVAL`"]
pub struct TOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Watchdog Timeout Value"]
    #[inline(always)]
    pub fn toval(&self) -> TOVAL_R {
        TOVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog Timeout Value"]
    #[inline(always)]
    pub fn toval(&mut self) -> TOVAL_W {
        TOVAL_W { w: self }
    }
}
