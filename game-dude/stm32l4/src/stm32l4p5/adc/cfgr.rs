#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CFGR_SPEC>> for R {
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl core::convert::From<crate::W<CFGR_SPEC>> for W {
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JQDIS` reader - Injected Queue disable"]
pub struct JQDIS_R(crate::FieldReader<bool, bool>);
impl JQDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        JQDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JQDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JQDIS` writer - Injected Queue disable"]
pub struct JQDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> JQDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `AWDCH1CH` reader - AWDCH1CH"]
pub struct AWDCH1CH_R(crate::FieldReader<u8, u8>);
impl AWDCH1CH_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWDCH1CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWDCH1CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWDCH1CH` writer - AWDCH1CH"]
pub struct AWDCH1CH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDCH1CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | ((value as u32 & 0x1f) << 26);
        self.w
    }
}
#[doc = "Field `JAUTO` reader - JAUTO"]
pub struct JAUTO_R(crate::FieldReader<bool, bool>);
impl JAUTO_R {
    pub(crate) fn new(bits: bool) -> Self {
        JAUTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JAUTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JAUTO` writer - JAUTO"]
pub struct JAUTO_W<'a> {
    w: &'a mut W,
}
impl<'a> JAUTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `JAWD1EN` reader - JAWD1EN"]
pub struct JAWD1EN_R(crate::FieldReader<bool, bool>);
impl JAWD1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        JAWD1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JAWD1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JAWD1EN` writer - JAWD1EN"]
pub struct JAWD1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> JAWD1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `AWD1EN` reader - AWD1EN"]
pub struct AWD1EN_R(crate::FieldReader<bool, bool>);
impl AWD1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD1EN` writer - AWD1EN"]
pub struct AWD1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `AWD1SGL` reader - AWD1SGL"]
pub struct AWD1SGL_R(crate::FieldReader<bool, bool>);
impl AWD1SGL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD1SGL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD1SGL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD1SGL` writer - AWD1SGL"]
pub struct AWD1SGL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD1SGL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `JQM` reader - JQM"]
pub struct JQM_R(crate::FieldReader<bool, bool>);
impl JQM_R {
    pub(crate) fn new(bits: bool) -> Self {
        JQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JQM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JQM` writer - JQM"]
pub struct JQM_W<'a> {
    w: &'a mut W,
}
impl<'a> JQM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `JDISCEN` reader - JDISCEN"]
pub struct JDISCEN_R(crate::FieldReader<bool, bool>);
impl JDISCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        JDISCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDISCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JDISCEN` writer - JDISCEN"]
pub struct JDISCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JDISCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `DISCNUM` reader - DISCNUM"]
pub struct DISCNUM_R(crate::FieldReader<u8, u8>);
impl DISCNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DISCNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCNUM` writer - DISCNUM"]
pub struct DISCNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `DISCEN` reader - DISCEN"]
pub struct DISCEN_R(crate::FieldReader<bool, bool>);
impl DISCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCEN` writer - DISCEN"]
pub struct DISCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `AUTDLY` reader - AUTDLY"]
pub struct AUTDLY_R(crate::FieldReader<bool, bool>);
impl AUTDLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTDLY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTDLY` writer - AUTDLY"]
pub struct AUTDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTDLY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CONT` reader - CONT"]
pub struct CONT_R(crate::FieldReader<bool, bool>);
impl CONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT` writer - CONT"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `OVRMOD` reader - OVRMOD"]
pub struct OVRMOD_R(crate::FieldReader<bool, bool>);
impl OVRMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRMOD` writer - OVRMOD"]
pub struct OVRMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `EXTEN` reader - EXTEN"]
pub struct EXTEN_R(crate::FieldReader<u8, u8>);
impl EXTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEN` writer - EXTEN"]
pub struct EXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `EXTSEL3` reader - EXTSEL3"]
pub struct EXTSEL3_R(crate::FieldReader<bool, bool>);
impl EXTSEL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTSEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSEL3` writer - EXTSEL3"]
pub struct EXTSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `EXTSEL2` reader - EXTSEL2"]
pub struct EXTSEL2_R(crate::FieldReader<bool, bool>);
impl EXTSEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTSEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSEL2` writer - EXTSEL2"]
pub struct EXTSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSEL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `EXTSEL1` reader - EXTSEL1"]
pub struct EXTSEL1_R(crate::FieldReader<bool, bool>);
impl EXTSEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTSEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSEL1` writer - EXTSEL1"]
pub struct EXTSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSEL1_W<'a> {
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
#[doc = "Field `EXTSEL0` reader - EXTSEL0"]
pub struct EXTSEL0_R(crate::FieldReader<bool, bool>);
impl EXTSEL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTSEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTSEL0` writer - EXTSEL0"]
pub struct EXTSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSEL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ALIGN` reader - ALIGN"]
pub struct ALIGN_R(crate::FieldReader<bool, bool>);
impl ALIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIGN` writer - ALIGN"]
pub struct ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RES` reader - RES"]
pub struct RES_R(crate::FieldReader<u8, u8>);
impl RES_R {
    pub(crate) fn new(bits: u8) -> Self {
        RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES` writer - RES"]
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `DFSDMCFG` reader - DFSDM mode configuration"]
pub struct DFSDMCFG_R(crate::FieldReader<bool, bool>);
impl DFSDMCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDMCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDMCFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDMCFG` writer - DFSDM mode configuration"]
pub struct DFSDMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMCFG_W<'a> {
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
#[doc = "Field `DMACFG` reader - DMACFG"]
pub struct DMACFG_R(crate::FieldReader<bool, bool>);
impl DMACFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMACFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMACFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMACFG` writer - DMACFG"]
pub struct DMACFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACFG_W<'a> {
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
#[doc = "Field `DMAEN` reader - DMAEN"]
pub struct DMAEN_R(crate::FieldReader<bool, bool>);
impl DMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMAEN"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
    #[doc = "Bit 31 - Injected Queue disable"]
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - AWDCH1CH"]
    #[inline(always)]
    pub fn awdch1ch(&self) -> AWDCH1CH_R {
        AWDCH1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - EXTSEL3"]
    #[inline(always)]
    pub fn extsel3(&self) -> EXTSEL3_R {
        EXTSEL3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EXTSEL2"]
    #[inline(always)]
    pub fn extsel2(&self) -> EXTSEL2_R {
        EXTSEL2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EXTSEL1"]
    #[inline(always)]
    pub fn extsel1(&self) -> EXTSEL1_R {
        EXTSEL1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EXTSEL0"]
    #[inline(always)]
    pub fn extsel0(&self) -> EXTSEL0_R {
        EXTSEL0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ALIGN"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - DFSDM mode configuration"]
    #[inline(always)]
    pub fn dfsdmcfg(&self) -> DFSDMCFG_R {
        DFSDMCFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Injected Queue disable"]
    #[inline(always)]
    pub fn jqdis(&mut self) -> JQDIS_W {
        JQDIS_W { w: self }
    }
    #[doc = "Bits 26:30 - AWDCH1CH"]
    #[inline(always)]
    pub fn awdch1ch(&mut self) -> AWDCH1CH_W {
        AWDCH1CH_W { w: self }
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W {
        JAUTO_W { w: self }
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&mut self) -> JAWD1EN_W {
        JAWD1EN_W { w: self }
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W {
        AWD1EN_W { w: self }
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W {
        AWD1SGL_W { w: self }
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&mut self) -> JQM_W {
        JQM_W { w: self }
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W {
        JDISCEN_W { w: self }
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W {
        DISCNUM_W { w: self }
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W {
        DISCEN_W { w: self }
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&mut self) -> AUTDLY_W {
        AUTDLY_W { w: self }
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W {
        OVRMOD_W { w: self }
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W {
        EXTEN_W { w: self }
    }
    #[doc = "Bit 9 - EXTSEL3"]
    #[inline(always)]
    pub fn extsel3(&mut self) -> EXTSEL3_W {
        EXTSEL3_W { w: self }
    }
    #[doc = "Bit 8 - EXTSEL2"]
    #[inline(always)]
    pub fn extsel2(&mut self) -> EXTSEL2_W {
        EXTSEL2_W { w: self }
    }
    #[doc = "Bit 7 - EXTSEL1"]
    #[inline(always)]
    pub fn extsel1(&mut self) -> EXTSEL1_W {
        EXTSEL1_W { w: self }
    }
    #[doc = "Bit 6 - EXTSEL0"]
    #[inline(always)]
    pub fn extsel0(&mut self) -> EXTSEL0_W {
        EXTSEL0_W { w: self }
    }
    #[doc = "Bit 5 - ALIGN"]
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W {
        ALIGN_W { w: self }
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    #[doc = "Bit 2 - DFSDM mode configuration"]
    #[inline(always)]
    pub fn dfsdmcfg(&mut self) -> DFSDMCFG_W {
        DFSDMCFG_W { w: self }
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W {
        DMACFG_W { w: self }
    }
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
