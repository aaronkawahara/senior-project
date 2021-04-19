#[doc = "Register `IDMABASE0R` reader"]
pub struct R(crate::R<IDMABASE0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDMABASE0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IDMABASE0R_SPEC>> for R {
    fn from(reader: crate::R<IDMABASE0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDMABASE0R` writer"]
pub struct W(crate::W<IDMABASE0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDMABASE0R_SPEC>;
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
impl core::convert::From<crate::W<IDMABASE0R_SPEC>> for W {
    fn from(writer: crate::W<IDMABASE0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMABASE0` reader - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only)"]
pub struct IDMABASE0_R(crate::FieldReader<u32, u32>);
impl IDMABASE0_R {
    pub(crate) fn new(bits: u32) -> Self {
        IDMABASE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMABASE0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMABASE0` writer - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only)"]
pub struct IDMABASE0_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABASE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IDMA buffer 0 base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmabase0r](index.html) module"]
pub struct IDMABASE0R_SPEC;
impl crate::RegisterSpec for IDMABASE0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idmabase0r::R](R) reader structure"]
impl crate::Readable for IDMABASE0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idmabase0r::W](W) writer structure"]
impl crate::Writable for IDMABASE0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDMABASE0R to value 0"]
impl crate::Resettable for IDMABASE0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}