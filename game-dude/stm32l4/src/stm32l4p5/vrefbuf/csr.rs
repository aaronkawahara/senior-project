#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CSR_SPEC>> for R {
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl core::convert::From<crate::W<CSR_SPEC>> for W {
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENVR` reader - Voltage reference buffer enable"]
pub struct ENVR_R(crate::FieldReader<bool, bool>);
impl ENVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENVR` writer - Voltage reference buffer enable"]
pub struct ENVR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVR_W<'a> {
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
#[doc = "Field `HIZ` reader - High impedance mode"]
pub struct HIZ_R(crate::FieldReader<bool, bool>);
impl HIZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIZ` writer - High impedance mode"]
pub struct HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIZ_W<'a> {
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
#[doc = "Field `VRS` reader - Voltage reference scale"]
pub struct VRS_R(crate::FieldReader<bool, bool>);
impl VRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VRS` writer - Voltage reference scale"]
pub struct VRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VRS_W<'a> {
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
#[doc = "Field `VRR` reader - Voltage reference buffer ready"]
pub struct VRR_R(crate::FieldReader<bool, bool>);
impl VRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        VRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VRR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Voltage reference buffer enable"]
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - High impedance mode"]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Voltage reference buffer ready"]
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage reference buffer enable"]
    #[inline(always)]
    pub fn envr(&mut self) -> ENVR_W {
        ENVR_W { w: self }
    }
    #[doc = "Bit 1 - High impedance mode"]
    #[inline(always)]
    pub fn hiz(&mut self) -> HIZ_W {
        HIZ_W { w: self }
    }
    #[doc = "Bit 2 - Voltage reference scale"]
    #[inline(always)]
    pub fn vrs(&mut self) -> VRS_W {
        VRS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0x02"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
