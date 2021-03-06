#[doc = "Reader of register IDMABASE0R"]
pub type R = crate::R<u32, super::IDMABASE0R>;
#[doc = "Writer for register IDMABASE0R"]
pub type W = crate::W<u32, super::IDMABASE0R>;
#[doc = "Register IDMABASE0R `reset()`'s with value 0"]
impl crate::ResetValue for super::IDMABASE0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMABASE0`"]
pub type IDMABASE0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IDMABASE0`"]
pub struct IDMABASE0_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABASE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only)"]
    #[inline(always)]
    pub fn idmabase0(&self) -> IDMABASE0_R {
        IDMABASE0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only)"]
    #[inline(always)]
    pub fn idmabase0(&mut self) -> IDMABASE0_W {
        IDMABASE0_W { w: self }
    }
}
