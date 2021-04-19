#[doc = "Register `IDMACTRLR` reader"]
pub struct R(crate::R<IDMACTRLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDMACTRLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IDMACTRLR_SPEC>> for R {
    fn from(reader: crate::R<IDMACTRLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDMACTRLR` writer"]
pub struct W(crate::W<IDMACTRLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDMACTRLR_SPEC>;
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
impl core::convert::From<crate::W<IDMACTRLR_SPEC>> for W {
    fn from(writer: crate::W<IDMACTRLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMAEN` reader - IDMA enable"]
pub struct IDMAEN_R(crate::FieldReader<bool, bool>);
impl IDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMAEN` writer - IDMA enable"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `IDMABMODE` reader - Buffer mode selection"]
pub struct IDMABMODE_R(crate::FieldReader<bool, bool>);
impl IDMABMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDMABMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMABMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMABMODE` writer - Buffer mode selection"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `IDMABACT` reader - Double buffer mode active buffer indication"]
pub struct IDMABACT_R(crate::FieldReader<bool, bool>);
impl IDMABACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDMABACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMABACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMABACT` writer - Double buffer mode active buffer indication"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmactrlr](index.html) module"]
pub struct IDMACTRLR_SPEC;
impl crate::RegisterSpec for IDMACTRLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idmactrlr::R](R) reader structure"]
impl crate::Readable for IDMACTRLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idmactrlr::W](W) writer structure"]
impl crate::Writable for IDMACTRLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDMACTRLR to value 0"]
impl crate::Resettable for IDMACTRLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
