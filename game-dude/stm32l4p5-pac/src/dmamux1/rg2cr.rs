#[doc = "Reader of register RG2CR"]
pub type R = crate::R<u32, super::RG2CR>;
#[doc = "Writer for register RG2CR"]
pub type W = crate::W<u32, super::RG2CR>;
#[doc = "Register RG2CR `reset()`'s with value 0"]
impl crate::ResetValue for super::RG2CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GNBREQ`"]
pub type GNBREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GNBREQ`"]
pub struct GNBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> GNBREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `GPOL`"]
pub type GPOL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPOL`"]
pub struct GPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `GE`"]
pub type GE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GE`"]
pub struct GE_W<'a> {
    w: &'a mut W,
}
impl<'a> GE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `OIE`"]
pub type OIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OIE`"]
pub struct OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SIG_ID`"]
pub type SIG_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIG_ID`"]
pub struct SIG_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 19:23 - Number of DMA requests to be generated minus 1"]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - DMA request generator channel 2 enable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - Signal identification"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 19:23 - Number of DMA requests to be generated minus 1"]
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W {
        GNBREQ_W { w: self }
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W {
        GPOL_W { w: self }
    }
    #[doc = "Bit 16 - DMA request generator channel 2 enable"]
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W {
        GE_W { w: self }
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W {
        OIE_W { w: self }
    }
    #[doc = "Bits 0:4 - Signal identification"]
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W {
        SIG_ID_W { w: self }
    }
}
