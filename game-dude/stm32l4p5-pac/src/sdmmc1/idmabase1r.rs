#[doc = "Reader of register IDMABASE1R"]
pub type R = crate::R<u32, super::IDMABASE1R>;
#[doc = "Writer for register IDMABASE1R"]
pub type W = crate::W<u32, super::IDMABASE1R>;
#[doc = "Register IDMABASE1R `reset()`'s with value 0"]
impl crate::ResetValue for super::IDMABASE1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMABASE1`"]
pub type IDMABASE1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IDMABASE1`"]
pub struct IDMABASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABASE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only)"]
    #[inline(always)]
    pub fn idmabase1(&self) -> IDMABASE1_R {
        IDMABASE1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer 1 memory base address, shall be word aligned (bit \\[1:0\\]
are always 0 and read only)"]
    #[inline(always)]
    pub fn idmabase1(&mut self) -> IDMABASE1_W {
        IDMABASE1_W { w: self }
    }
}
