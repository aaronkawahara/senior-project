#[doc = "Reader of register IDMACTRLR"]
pub type R = crate::R<u32, super::IDMACTRLR>;
#[doc = "Writer for register IDMACTRLR"]
pub type W = crate::W<u32, super::IDMACTRLR>;
#[doc = "Register IDMACTRLR `reset()`'s with value 0"]
impl crate::ResetValue for super::IDMACTRLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMAEN`"]
pub type IDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDMAEN`"]
pub struct IDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMAEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `IDMABMODE`"]
pub type IDMABMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDMABMODE`"]
pub struct IDMABMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABMODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `IDMABACT`"]
pub type IDMABACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDMABACT`"]
pub struct IDMABACT_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABACT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IDMA enable"]
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Buffer mode selection"]
    #[inline(always)]
    pub fn idmabmode(&self) -> IDMABMODE_R {
        IDMABMODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Double buffer mode active buffer indication"]
    #[inline(always)]
    pub fn idmabact(&self) -> IDMABACT_R {
        IDMABACT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IDMA enable"]
    #[inline(always)]
    pub fn idmaen(&mut self) -> IDMAEN_W {
        IDMAEN_W { w: self }
    }
    #[doc = "Bit 1 - Buffer mode selection"]
    #[inline(always)]
    pub fn idmabmode(&mut self) -> IDMABMODE_W {
        IDMABMODE_W { w: self }
    }
    #[doc = "Bit 2 - Double buffer mode active buffer indication"]
    #[inline(always)]
    pub fn idmabact(&mut self) -> IDMABACT_W {
        IDMABACT_W { w: self }
    }
}
