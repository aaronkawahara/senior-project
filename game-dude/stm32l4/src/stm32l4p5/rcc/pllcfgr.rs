#[doc = "Register `PLLCFGR` reader"]
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PLLCFGR_SPEC>> for R {
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFGR` writer"]
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
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
impl core::convert::From<crate::W<PLLCFGR_SPEC>> for W {
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Main PLL division factor for PLLSAI2CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLPDIV_A {
    #[doc = "0: PLLSAI3CLK is controlled by the bit PLLP"]
    PLLPUSELECTED = 0,
}
impl From<PLLPDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLPDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLPDIV` reader - Main PLL division factor for PLLSAI2CLK"]
pub struct PLLPDIV_R(crate::FieldReader<u8, PLLPDIV_A>);
impl PLLPDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLPDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLPDIV_A> {
        match self.bits {
            0 => Some(PLLPDIV_A::PLLPUSELECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLLPUSELECTED`"]
    #[inline(always)]
    pub fn is_pllpuselected(&self) -> bool {
        **self == PLLPDIV_A::PLLPUSELECTED
    }
}
impl core::ops::Deref for PLLPDIV_R {
    type Target = crate::FieldReader<u8, PLLPDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLPDIV` writer - Main PLL division factor for PLLSAI2CLK"]
pub struct PLLPDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLPDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PLLSAI3CLK is controlled by the bit PLLP"]
    #[inline(always)]
    pub fn pllpuselected(self) -> &'a mut W {
        self.variant(PLLPDIV_A::PLLPUSELECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
#[doc = "Field `PLLR` reader - Main PLL division factor for PLLCLK (system clock)"]
pub struct PLLR_R(crate::FieldReader<u8, u8>);
impl PLLR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLR` writer - Main PLL division factor for PLLCLK (system clock)"]
pub struct PLLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Main PLL PLLCLK output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLREN_A {
    #[doc = "0: output disabled"]
    DISABLED = 0,
    #[doc = "1: output enabled"]
    ENABLED = 1,
}
impl From<PLLREN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLREN` reader - Main PLL PLLCLK output enable"]
pub struct PLLREN_R(crate::FieldReader<bool, PLLREN_A>);
impl PLLREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLREN_A {
        match self.bits {
            false => PLLREN_A::DISABLED,
            true => PLLREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PLLREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PLLREN_A::ENABLED
    }
}
impl core::ops::Deref for PLLREN_R {
    type Target = crate::FieldReader<bool, PLLREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLREN` writer - Main PLL PLLCLK output enable"]
pub struct PLLREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLREN_A::DISABLED)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLREN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Main PLL division factor for PLLUSB1CLK(48 MHz clock)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLQ_A {
    #[doc = "0: PLLQ = 2"]
    DIV2 = 0,
    #[doc = "1: PLLQ = 4"]
    DIV4 = 1,
    #[doc = "2: PLLQ = 6"]
    DIV6 = 2,
    #[doc = "3: PLLQ = 8"]
    DIV8 = 3,
}
impl From<PLLQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLQ` reader - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
pub struct PLLQ_R(crate::FieldReader<u8, PLLQ_A>);
impl PLLQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLQ_A {
        match self.bits {
            0 => PLLQ_A::DIV2,
            1 => PLLQ_A::DIV4,
            2 => PLLQ_A::DIV6,
            3 => PLLQ_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PLLQ_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PLLQ_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PLLQ_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PLLQ_A::DIV8
    }
}
impl core::ops::Deref for PLLQ_R {
    type Target = crate::FieldReader<u8, PLLQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLQ` writer - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
pub struct PLLQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLQ_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PLLQ = 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV2)
    }
    #[doc = "PLLQ = 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV4)
    }
    #[doc = "PLLQ = 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV6)
    }
    #[doc = "PLLQ = 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLQ_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Main PLL PLLUSB1CLK output enable"]
pub type PLLQEN_A = PLLREN_A;
#[doc = "Field `PLLQEN` reader - Main PLL PLLUSB1CLK output enable"]
pub type PLLQEN_R = PLLREN_R;
#[doc = "Field `PLLQEN` writer - Main PLL PLLUSB1CLK output enable"]
pub struct PLLQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLQEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLQEN_A::DISABLED)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLQEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLP_A {
    #[doc = "0: PLLP = 7"]
    DIV7 = 0,
    #[doc = "1: PLLP = 17"]
    DIV17 = 1,
}
impl From<PLLP_A> for bool {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLP` reader - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
pub struct PLLP_R(crate::FieldReader<bool, PLLP_A>);
impl PLLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLP_A {
        match self.bits {
            false => PLLP_A::DIV7,
            true => PLLP_A::DIV17,
        }
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        **self == PLLP_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        **self == PLLP_A::DIV17
    }
}
impl core::ops::Deref for PLLP_R {
    type Target = crate::FieldReader<bool, PLLP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLP` writer - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
pub struct PLLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLLP = 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLP_A::DIV7)
    }
    #[doc = "PLLP = 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLP_A::DIV17)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Main PLL PLLSAI3CLK output enable"]
pub type PLLPEN_A = PLLREN_A;
#[doc = "Field `PLLPEN` reader - Main PLL PLLSAI3CLK output enable"]
pub type PLLPEN_R = PLLREN_R;
#[doc = "Field `PLLPEN` writer - Main PLL PLLSAI3CLK output enable"]
pub struct PLLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLPEN_A::DISABLED)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PLLN` reader - Main PLL multiplication factor for VCO"]
pub struct PLLN_R(crate::FieldReader<u8, u8>);
impl PLLN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN` writer - Main PLL multiplication factor for VCO"]
pub struct PLLN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `PLLM` reader - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
pub struct PLLM_R(crate::FieldReader<u8, u8>);
impl PLLM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLM` writer - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
pub struct PLLM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Main PLL, PLLSAI1 and PLLSAI2 entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "0: No clock"]
    NOCLOCK = 0,
    #[doc = "1: MSI clock selected as PLL clock entry"]
    MSI = 1,
    #[doc = "2: HSI16 clock selected as PLL clock entry"]
    HSI16 = 2,
    #[doc = "3: HSE clock selected as PLL clock entry"]
    HSE = 3,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLSRC` reader - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub struct PLLSRC_R(crate::FieldReader<u8, PLLSRC_A>);
impl PLLSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            0 => PLLSRC_A::NOCLOCK,
            1 => PLLSRC_A::MSI,
            2 => PLLSRC_A::HSI16,
            3 => PLLSRC_A::HSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        **self == PLLSRC_A::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `MSI`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        **self == PLLSRC_A::MSI
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == PLLSRC_A::HSI16
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == PLLSRC_A::HSE
    }
}
impl core::ops::Deref for PLLSRC_R {
    type Target = crate::FieldReader<u8, PLLSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSRC` writer - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(PLLSRC_A::NOCLOCK)
    }
    #[doc = "MSI clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(PLLSRC_A::MSI)
    }
    #[doc = "HSI16 clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI16)
    }
    #[doc = "HSE clock selected as PLL clock entry"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31 - Main PLL division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllpdiv(&self) -> PLLPDIV_R {
        PLLPDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLLCLK (system clock)"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Main PLL PLLCLK output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Main PLL PLLUSB1CLK output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Main PLL PLLSAI3CLK output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - Main PLL division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllpdiv(&mut self) -> PLLPDIV_W {
        PLLPDIV_W { w: self }
    }
    #[doc = "Bits 25:26 - Main PLL division factor for PLLCLK (system clock)"]
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W {
        PLLR_W { w: self }
    }
    #[doc = "Bit 24 - Main PLL PLLCLK output enable"]
    #[inline(always)]
    pub fn pllren(&mut self) -> PLLREN_W {
        PLLREN_W { w: self }
    }
    #[doc = "Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W {
        PLLQ_W { w: self }
    }
    #[doc = "Bit 20 - Main PLL PLLUSB1CLK output enable"]
    #[inline(always)]
    pub fn pllqen(&mut self) -> PLLQEN_W {
        PLLQEN_W { w: self }
    }
    #[doc = "Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W {
        PLLP_W { w: self }
    }
    #[doc = "Bit 16 - Main PLL PLLSAI3CLK output enable"]
    #[inline(always)]
    pub fn pllpen(&mut self) -> PLLPEN_W {
        PLLPEN_W { w: self }
    }
    #[doc = "Bits 8:14 - Main PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W {
        PLLN_W { w: self }
    }
    #[doc = "Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W {
        PLLM_W { w: self }
    }
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](index.html) module"]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfgr::R](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x1000"]
impl crate::Resettable for PLLCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
