#[doc = "Reader of register PLLCFGR"]
pub type R = crate::R<u32, super::PLLCFGR>;
#[doc = "Writer for register PLLCFGR"]
pub type W = crate::W<u32, super::PLLCFGR>;
#[doc = "Register PLLCFGR `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::PLLCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Reader of field `PLLPDIV`"]
pub type PLLPDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLPDIV`"]
pub struct PLLPDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `PLLR`"]
pub type PLLR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLR`"]
pub struct PLLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
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
#[doc = "Reader of field `PLLREN`"]
pub type PLLREN_R = crate::R<bool, PLLREN_A>;
impl PLLREN_R {
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
        *self == PLLREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLREN_A::ENABLED
    }
}
#[doc = "Write proxy for field `PLLREN`"]
pub struct PLLREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
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
#[doc = "Reader of field `PLLQ`"]
pub type PLLQ_R = crate::R<u8, PLLQ_A>;
impl PLLQ_R {
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
        *self == PLLQ_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLQ_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLQ_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLQ_A::DIV8
    }
}
#[doc = "Write proxy for field `PLLQ`"]
pub struct PLLQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Main PLL PLLUSB1CLK output enable"]
pub type PLLQEN_A = PLLREN_A;
#[doc = "Reader of field `PLLQEN`"]
pub type PLLQEN_R = crate::R<bool, PLLREN_A>;
#[doc = "Write proxy for field `PLLQEN`"]
pub struct PLLQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
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
#[doc = "Reader of field `PLLP`"]
pub type PLLP_R = crate::R<bool, PLLP_A>;
impl PLLP_R {
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
        *self == PLLP_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLP_A::DIV17
    }
}
#[doc = "Write proxy for field `PLLP`"]
pub struct PLLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Main PLL PLLSAI3CLK output enable"]
pub type PLLPEN_A = PLLREN_A;
#[doc = "Reader of field `PLLPEN`"]
pub type PLLPEN_R = crate::R<bool, PLLREN_A>;
#[doc = "Write proxy for field `PLLPEN`"]
pub struct PLLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PLLN`"]
pub type PLLN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLN`"]
pub struct PLLN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PLLM`"]
pub type PLLM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLM`"]
pub struct PLLM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
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
#[doc = "Reader of field `PLLSRC`"]
pub type PLLSRC_R = crate::R<u8, PLLSRC_A>;
impl PLLSRC_R {
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
        *self == PLLSRC_A::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `MSI`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == PLLSRC_A::MSI
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLLSRC_A::HSI16
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC_A::HSE
    }
}
#[doc = "Write proxy for field `PLLSRC`"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
}
