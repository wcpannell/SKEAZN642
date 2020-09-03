#[doc = "Reader of register WIN"]
pub type R = crate::R<u16, super::WIN>;
#[doc = "Writer for register WIN"]
pub type W = crate::W<u16, super::WIN>;
#[doc = "Register WIN `reset()`'s with value 0"]
impl crate::ResetValue for super::WIN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WIN`"]
pub type WIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WIN`"]
pub struct WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Watchdog Window Value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog Window Value"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W {
        WIN_W { w: self }
    }
}
