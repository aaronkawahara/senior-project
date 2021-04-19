#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CR_SPEC>> for R {
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl core::convert::From<crate::W<CR_SPEC>> for W {
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REV_OUT` reader - Reverse output data"]
pub struct REV_OUT_R(crate::FieldReader<bool, bool>);
impl REV_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV_OUT` writer - Reverse output data"]
pub struct REV_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `REV_IN` reader - Reverse input data"]
pub struct REV_IN_R(crate::FieldReader<u8, u8>);
impl REV_IN_R {
    pub(crate) fn new(bits: u8) -> Self {
        REV_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV_IN` writer - Reverse input data"]
pub struct REV_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Polynomial size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POLYSIZE_A {
    #[doc = "0: 32-bit polynomial"]
    POLYSIZE32 = 0,
    #[doc = "1: 16-bit polynomial"]
    POLYSIZE16 = 1,
    #[doc = "2: 8-bit polynomial"]
    POLYSIZE8 = 2,
    #[doc = "3: 7-bit polynomial"]
    POLYSIZE7 = 3,
}
impl From<POLYSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: POLYSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POLYSIZE` reader - Polynomial size"]
pub struct POLYSIZE_R(crate::FieldReader<u8, POLYSIZE_A>);
impl POLYSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        POLYSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLYSIZE_A {
        match self.bits {
            0 => POLYSIZE_A::POLYSIZE32,
            1 => POLYSIZE_A::POLYSIZE16,
            2 => POLYSIZE_A::POLYSIZE8,
            3 => POLYSIZE_A::POLYSIZE7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POLYSIZE32`"]
    #[inline(always)]
    pub fn is_polysize32(&self) -> bool {
        **self == POLYSIZE_A::POLYSIZE32
    }
    #[doc = "Checks if the value of the field is `POLYSIZE16`"]
    #[inline(always)]
    pub fn is_polysize16(&self) -> bool {
        **self == POLYSIZE_A::POLYSIZE16
    }
    #[doc = "Checks if the value of the field is `POLYSIZE8`"]
    #[inline(always)]
    pub fn is_polysize8(&self) -> bool {
        **self == POLYSIZE_A::POLYSIZE8
    }
    #[doc = "Checks if the value of the field is `POLYSIZE7`"]
    #[inline(always)]
    pub fn is_polysize7(&self) -> bool {
        **self == POLYSIZE_A::POLYSIZE7
    }
}
impl core::ops::Deref for POLYSIZE_R {
    type Target = crate::FieldReader<u8, POLYSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLYSIZE` writer - Polynomial size"]
pub struct POLYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> POLYSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLYSIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "32-bit polynomial"]
    #[inline(always)]
    pub fn polysize32(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE32)
    }
    #[doc = "16-bit polynomial"]
    #[inline(always)]
    pub fn polysize16(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE16)
    }
    #[doc = "8-bit polynomial"]
    #[inline(always)]
    pub fn polysize8(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE8)
    }
    #[doc = "7-bit polynomial"]
    #[inline(always)]
    pub fn polysize7(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "RESET bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_AW {
    #[doc = "1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF"]
    RESET = 1,
}
impl From<RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - RESET bit"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESET_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    pub fn rev_out(&mut self) -> REV_OUT_W {
        REV_OUT_W { w: self }
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    pub fn rev_in(&mut self) -> REV_IN_W {
        REV_IN_W { w: self }
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    pub fn polysize(&mut self) -> POLYSIZE_W {
        POLYSIZE_W { w: self }
    }
    #[doc = "Bit 0 - RESET bit"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
