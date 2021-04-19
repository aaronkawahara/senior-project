#[doc = "Register `FIFOR14` reader"]
pub struct R(crate::R<FIFOR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FIFOR14_SPEC>> for R {
    fn from(reader: crate::R<FIFOR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOR14` writer"]
pub struct W(crate::W<FIFOR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOR14_SPEC>;
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
impl core::convert::From<crate::W<FIFOR14_SPEC>> for W {
    fn from(writer: crate::W<FIFOR14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFODATA` reader - Receive and transmit FIFO data"]
pub struct FIFODATA_R(crate::FieldReader<u32, u32>);
impl FIFODATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        FIFODATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFODATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFODATA` writer - Receive and transmit FIFO data"]
pub struct FIFODATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFODATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data"]
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive and transmit FIFO data"]
    #[inline(always)]
    pub fn fifodata(&mut self) -> FIFODATA_W {
        FIFODATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data FIFO register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor14](index.html) module"]
pub struct FIFOR14_SPEC;
impl crate::RegisterSpec for FIFOR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifor14::R](R) reader structure"]
impl crate::Readable for FIFOR14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifor14::W](W) writer structure"]
impl crate::Writable for FIFOR14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOR14 to value 0"]
impl crate::Resettable for FIFOR14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}