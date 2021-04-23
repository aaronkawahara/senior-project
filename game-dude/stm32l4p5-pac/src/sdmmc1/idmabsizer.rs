#[doc = "Reader of register IDMABSIZER"]
pub type R = crate::R<u32, super::IDMABSIZER>;
#[doc = "Writer for register IDMABSIZER"]
pub type W = crate::W<u32, super::IDMABSIZER>;
#[doc = "Register IDMABSIZER `reset()`'s with value 0"]
impl crate::ResetValue for super::IDMABSIZER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMABNDT`"]
pub type IDMABNDT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDMABNDT`"]
pub struct IDMABNDT_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABNDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 5)) | (((value as u32) & 0xff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:12 - Number of bytes per buffer"]
    #[inline(always)]
    pub fn idmabndt(&self) -> IDMABNDT_R {
        IDMABNDT_R::new(((self.bits >> 5) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 5:12 - Number of bytes per buffer"]
    #[inline(always)]
    pub fn idmabndt(&mut self) -> IDMABNDT_W {
        IDMABNDT_W { w: self }
    }
}
