#[doc = "Register `APB2SMENR` reader"]
pub struct R(crate::R<APB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<APB2SMENR_SPEC>> for R {
    fn from(reader: crate::R<APB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2SMENR` writer"]
pub struct W(crate::W<APB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2SMENR_SPEC>;
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
impl core::convert::From<crate::W<APB2SMENR_SPEC>> for W {
    fn from(writer: crate::W<APB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG clocks enable during Sleep and Stop modes"]
pub struct SYSCFGSMEN_R(crate::FieldReader<bool, bool>);
impl SYSCFGSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCFGSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG clocks enable during Sleep and Stop modes"]
pub struct SYSCFGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGSMEN_W<'a> {
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
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clocks enable during Sleep and Stop modes"]
pub struct TIM1SMEN_R(crate::FieldReader<bool, bool>);
impl TIM1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clocks enable during Sleep and Stop modes"]
pub struct TIM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SPI1SMEN` reader - SPI1 clocks enable during Sleep and Stop modes"]
pub struct SPI1SMEN_R(crate::FieldReader<bool, bool>);
impl SPI1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1SMEN` writer - SPI1 clocks enable during Sleep and Stop modes"]
pub struct SPI1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1SMEN_W<'a> {
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
#[doc = "Field `TIM8SMEN` reader - TIM8 timer clocks enable during Sleep and Stop modes"]
pub struct TIM8SMEN_R(crate::FieldReader<bool, bool>);
impl TIM8SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM8SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8SMEN` writer - TIM8 timer clocks enable during Sleep and Stop modes"]
pub struct TIM8SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8SMEN_W<'a> {
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
#[doc = "Field `USART1SMEN` reader - USART1clocks enable during Sleep and Stop modes"]
pub struct USART1SMEN_R(crate::FieldReader<bool, bool>);
impl USART1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1SMEN` writer - USART1clocks enable during Sleep and Stop modes"]
pub struct USART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SMEN_W<'a> {
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
#[doc = "Field `TIM15SMEN` reader - TIM15 timer clocks enable during Sleep and Stop modes"]
pub struct TIM15SMEN_R(crate::FieldReader<bool, bool>);
impl TIM15SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15SMEN` writer - TIM15 timer clocks enable during Sleep and Stop modes"]
pub struct TIM15SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15SMEN_W<'a> {
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
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clocks enable during Sleep and Stop modes"]
pub struct TIM16SMEN_R(crate::FieldReader<bool, bool>);
impl TIM16SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clocks enable during Sleep and Stop modes"]
pub struct TIM16SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `TIM17SMEN` reader - TIM17 timer clocks enable during Sleep and Stop modes"]
pub struct TIM17SMEN_R(crate::FieldReader<bool, bool>);
impl TIM17SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17SMEN` writer - TIM17 timer clocks enable during Sleep and Stop modes"]
pub struct TIM17SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `SAI1SMEN` reader - SAI1 clocks enable during Sleep and Stop modes"]
pub struct SAI1SMEN_R(crate::FieldReader<bool, bool>);
impl SAI1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1SMEN` writer - SAI1 clocks enable during Sleep and Stop modes"]
pub struct SAI1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SMEN_W<'a> {
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
#[doc = "Field `SAI2SMEN` reader - SAI2 clocks enable during Sleep and Stop modes"]
pub struct SAI2SMEN_R(crate::FieldReader<bool, bool>);
impl SAI2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2SMEN` writer - SAI2 clocks enable during Sleep and Stop modes"]
pub struct SAI2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2SMEN_W<'a> {
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
#[doc = "Field `DFSDM1SMEN` reader - DFSDM timer clocks enable during Sleep and Stop modes"]
pub struct DFSDM1SMEN_R(crate::FieldReader<bool, bool>);
impl DFSDM1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDM1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDM1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDM1SMEN` writer - DFSDM timer clocks enable during Sleep and Stop modes"]
pub struct DFSDM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1SMEN_W<'a> {
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
#[doc = "Field `LTDCSMEN` reader - LCD-TFT timer clocks enable during Sleep and Stop modes"]
pub struct LTDCSMEN_R(crate::FieldReader<bool, bool>);
impl LTDCSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LTDCSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTDCSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTDCSMEN` writer - LCD-TFT timer clocks enable during Sleep and Stop modes"]
pub struct LTDCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `DSISMEN` reader - DSI clocks enable during Sleep and Stop modes"]
pub struct DSISMEN_R(crate::FieldReader<bool, bool>);
impl DSISMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSISMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSISMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSISMEN` writer - DSI clocks enable during Sleep and Stop modes"]
pub struct DSISMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSISMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim8smen(&self) -> TIM8SMEN_R {
        TIM8SMEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim15smen(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai1smen(&self) -> SAI1SMEN_R {
        SAI1SMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai2smen(&self) -> SAI2SMEN_R {
        SAI2SMEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dfsdm1smen(&self) -> DFSDM1SMEN_R {
        DFSDM1SMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LCD-TFT timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ltdcsmen(&self) -> LTDCSMEN_R {
        LTDCSMEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DSI clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dsismen(&self) -> DSISMEN_R {
        DSISMEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W {
        SYSCFGSMEN_W { w: self }
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W {
        TIM1SMEN_W { w: self }
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W {
        SPI1SMEN_W { w: self }
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim8smen(&mut self) -> TIM8SMEN_W {
        TIM8SMEN_W { w: self }
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W {
        USART1SMEN_W { w: self }
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim15smen(&mut self) -> TIM15SMEN_W {
        TIM15SMEN_W { w: self }
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W {
        TIM16SMEN_W { w: self }
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W {
        TIM17SMEN_W { w: self }
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai1smen(&mut self) -> SAI1SMEN_W {
        SAI1SMEN_W { w: self }
    }
    #[doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sai2smen(&mut self) -> SAI2SMEN_W {
        SAI2SMEN_W { w: self }
    }
    #[doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dfsdm1smen(&mut self) -> DFSDM1SMEN_W {
        DFSDM1SMEN_W { w: self }
    }
    #[doc = "Bit 26 - LCD-TFT timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn ltdcsmen(&mut self) -> LTDCSMEN_W {
        LTDCSMEN_W { w: self }
    }
    #[doc = "Bit 27 - DSI clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dsismen(&mut self) -> DSISMEN_W {
        DSISMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2SMENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2smenr](index.html) module"]
pub struct APB2SMENR_SPEC;
impl crate::RegisterSpec for APB2SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2smenr::R](R) reader structure"]
impl crate::Readable for APB2SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2smenr::W](W) writer structure"]
impl crate::Writable for APB2SMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2SMENR to value 0x0167_7c01"]
impl crate::Resettable for APB2SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0167_7c01
    }
}