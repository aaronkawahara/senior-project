#[doc = "Register `DFSDM_FLT1EXMIN` reader"]
pub struct R(crate::R<DFSDM_FLT1EXMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT1EXMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DFSDM_FLT1EXMIN_SPEC>> for R {
    fn from(reader: crate::R<DFSDM_FLT1EXMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXMIN` reader - EXMIN"]
pub struct EXMIN_R(crate::FieldReader<u32, u32>);
impl EXMIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        EXMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXMINCH` reader - Extremes detector minimum data channel"]
pub struct EXMINCH_R(crate::FieldReader<u8, u8>);
impl EXMINCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXMINCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMINCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - Extremes detector minimum data channel"]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1exmin](index.html) module"]
pub struct DFSDM_FLT1EXMIN_SPEC;
impl crate::RegisterSpec for DFSDM_FLT1EXMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt1exmin::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT1EXMIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_FLT1EXMIN to value 0x7fff_ff00"]
impl crate::Resettable for DFSDM_FLT1EXMIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff_ff00
    }
}
