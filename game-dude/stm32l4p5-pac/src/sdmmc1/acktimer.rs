#[doc = "Reader of register ACKTIMER"]
pub type R = crate::R<u32, super::ACKTIMER>;
#[doc = "Writer for register ACKTIMER"]
pub type W = crate::W<u32, super::ACKTIMER>;
#[doc = "Register ACKTIMER `reset()`'s with value 0"]
impl crate::ResetValue for super::ACKTIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACKTIME`"]
pub type ACKTIME_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ACKTIME`"]
pub struct ACKTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKTIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - Boot acknowledgment timeout period"]
    #[inline(always)]
    pub fn acktime(&self) -> ACKTIME_R {
        ACKTIME_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - Boot acknowledgment timeout period"]
    #[inline(always)]
    pub fn acktime(&mut self) -> ACKTIME_W {
        ACKTIME_W { w: self }
    }
}
