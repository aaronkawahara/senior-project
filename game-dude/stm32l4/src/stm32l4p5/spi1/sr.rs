#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SR_SPEC>> for R {
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<SR_SPEC>> for W {
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXNE` reader - Receive buffer not empty"]
pub struct RXNE_R(crate::FieldReader<bool, bool>);
impl RXNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXE` reader - Transmit buffer empty"]
pub struct TXE_R(crate::FieldReader<bool, bool>);
impl TXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCERR` reader - CRC error flag"]
pub struct CRCERR_R(crate::FieldReader<bool, bool>);
impl CRCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCERR` writer - CRC error flag"]
pub struct CRCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `MODF` reader - Mode fault"]
pub struct MODF_R(crate::FieldReader<bool, bool>);
impl MODF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR` reader - Overrun flag"]
pub struct OVR_R(crate::FieldReader<bool, bool>);
impl OVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSY` reader - Busy flag"]
pub struct BSY_R(crate::FieldReader<bool, bool>);
impl BSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Frame format error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRE_A {
    #[doc = "0: No frame format error"]
    NOERROR = 0,
    #[doc = "1: A frame format error occurred"]
    ERROR = 1,
}
impl From<FRE_A> for bool {
    #[inline(always)]
    fn from(variant: FRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRE` reader - Frame format error"]
pub struct FRE_R(crate::FieldReader<bool, FRE_A>);
impl FRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRE_A {
        match self.bits {
            false => FRE_A::NOERROR,
            true => FRE_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == FRE_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == FRE_A::ERROR
    }
}
impl core::ops::Deref for FRE_R {
    type Target = crate::FieldReader<bool, FRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRLVL` reader - FIFO reception level"]
pub struct FRLVL_R(crate::FieldReader<u8, u8>);
impl FRLVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRLVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTLVL` reader - FIFO transmission level"]
pub struct FTLVL_R(crate::FieldReader<u8, u8>);
impl FTLVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FTLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTLVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mode fault"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overrun flag"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Frame format error"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - FIFO reception level"]
    #[inline(always)]
    pub fn frlvl(&self) -> FRLVL_R {
        FRLVL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - FIFO transmission level"]
    #[inline(always)]
    pub fn ftlvl(&self) -> FTLVL_R {
        FTLVL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W {
        CRCERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0x02"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
