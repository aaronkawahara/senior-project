#[doc = "Reader of register BRR"]
pub type R = crate::R<u32, super::BRR>;
#[doc = "Writer for register BRR"]
pub type W = crate::W<u32, super::BRR>;
#[doc = "Register BRR `reset()`'s with value 0"]
impl crate::ResetValue for super::BRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRR`"]
pub type BRR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BRR`"]
pub struct BRR_W<'a> {
    w: &'a mut W,
}
impl<'a> BRR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - USARTDIV"]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - USARTDIV"]
    #[inline(always)]
    pub fn brr(&mut self) -> BRR_W {
        BRR_W { w: self }
    }
}
