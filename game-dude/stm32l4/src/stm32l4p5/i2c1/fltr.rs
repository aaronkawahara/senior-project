#[doc = "Register `FLTR` reader"]
pub struct R(crate::R<FLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FLTR_SPEC>> for R {
    fn from(reader: crate::R<FLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTR` writer"]
pub struct W(crate::W<FLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTR_SPEC>;
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
impl core::convert::From<crate::W<FLTR_SPEC>> for W {
    fn from(writer: crate::W<FLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANOFF` reader - Analog noise filter"]
pub struct ANOFF_R(crate::FieldReader<bool, bool>);
impl ANOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANOFF` writer - Analog noise filter"]
pub struct ANOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ANOFF_W<'a> {
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
#[doc = "Field `DNF` reader - Digital noise filter"]
pub struct DNF_R(crate::FieldReader<u8, u8>);
impl DNF_R {
    pub(crate) fn new(bits: u8) -> Self {
        DNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNF` writer - Digital noise filter"]
pub struct DNF_W<'a> {
    w: &'a mut W,
}
impl<'a> DNF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Analog noise filter"]
    #[inline(always)]
    pub fn anoff(&self) -> ANOFF_R {
        ANOFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Analog noise filter"]
    #[inline(always)]
    pub fn anoff(&mut self) -> ANOFF_W {
        ANOFF_W { w: self }
    }
    #[doc = "Bits 0:3 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&mut self) -> DNF_W {
        DNF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltr](index.html) module"]
pub struct FLTR_SPEC;
impl crate::RegisterSpec for FLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltr::R](R) reader structure"]
impl crate::Readable for FLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltr::W](W) writer structure"]
impl crate::Writable for FLTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTR to value 0"]
impl crate::Resettable for FLTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
