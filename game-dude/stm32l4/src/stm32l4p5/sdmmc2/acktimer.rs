#[doc = "Register `ACKTIMER` reader"]
pub struct R(crate::R<ACKTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACKTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ACKTIMER_SPEC>> for R {
    fn from(reader: crate::R<ACKTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACKTIMER` writer"]
pub struct W(crate::W<ACKTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACKTIMER_SPEC>;
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
impl core::convert::From<crate::W<ACKTIMER_SPEC>> for W {
    fn from(writer: crate::W<ACKTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACKTIME` reader - Boot acknowledgment timeout period"]
pub struct ACKTIME_R(crate::FieldReader<u32, u32>);
impl ACKTIME_R {
    pub(crate) fn new(bits: u32) -> Self {
        ACKTIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKTIME_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKTIME` writer - Boot acknowledgment timeout period"]
pub struct ACKTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKTIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | (value as u32 & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - Boot acknowledgment timeout period"]
    #[inline(always)]
    pub fn acktime(&self) -> ACKTIME_R {
        ACKTIME_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - Boot acknowledgment timeout period"]
    #[inline(always)]
    pub fn acktime(&mut self) -> ACKTIME_W {
        ACKTIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "acknowledgment timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acktimer](index.html) module"]
pub struct ACKTIMER_SPEC;
impl crate::RegisterSpec for ACKTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acktimer::R](R) reader structure"]
impl crate::Readable for ACKTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acktimer::W](W) writer structure"]
impl crate::Writable for ACKTIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACKTIMER to value 0"]
impl crate::Resettable for ACKTIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
