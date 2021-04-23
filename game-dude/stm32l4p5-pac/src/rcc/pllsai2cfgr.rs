#[doc = "Reader of register PLLSAI2CFGR"]
pub type R = crate::R<u32, super::PLLSAI2CFGR>;
#[doc = "Writer for register PLLSAI2CFGR"]
pub type W = crate::W<u32, super::PLLSAI2CFGR>;
#[doc = "Register PLLSAI2CFGR `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::PLLSAI2CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Reader of field `PLLSAI2PDIV`"]
pub type PLLSAI2PDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAI2PDIV`"]
pub struct PLLSAI2PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2PDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "PLLSAI2 division factor for PLLADC2CLK (ADC clock)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSAI2R_A {
    #[doc = "0: PLLSAI2R = 2"]
    DIV2 = 0,
    #[doc = "1: PLLSAI2R = 4"]
    DIV4 = 1,
    #[doc = "2: PLLSAI2R = 6"]
    DIV6 = 2,
    #[doc = "3: PLLSAI2R = 8"]
    DIV8 = 3,
}
impl From<PLLSAI2R_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI2R_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLLSAI2R`"]
pub type PLLSAI2R_R = crate::R<u8, PLLSAI2R_A>;
impl PLLSAI2R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2R_A {
        match self.bits {
            0 => PLLSAI2R_A::DIV2,
            1 => PLLSAI2R_A::DIV4,
            2 => PLLSAI2R_A::DIV6,
            3 => PLLSAI2R_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI2R_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI2R_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI2R_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI2R_A::DIV8
    }
}
#[doc = "Write proxy for field `PLLSAI2R`"]
pub struct PLLSAI2R_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSAI2R_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PLLSAI2R = 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAI2R_A::DIV2)
    }
    #[doc = "PLLSAI2R = 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAI2R_A::DIV4)
    }
    #[doc = "PLLSAI2R = 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAI2R_A::DIV6)
    }
    #[doc = "PLLSAI2R = 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAI2R_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "PLLSAI2 PLLADC2CLK output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSAI2REN_A {
    #[doc = "0: output disabled"]
    DISABLED = 0,
    #[doc = "1: output enabled"]
    ENABLED = 1,
}
impl From<PLLSAI2REN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2REN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLSAI2REN`"]
pub type PLLSAI2REN_R = crate::R<bool, PLLSAI2REN_A>;
impl PLLSAI2REN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2REN_A {
        match self.bits {
            false => PLLSAI2REN_A::DISABLED,
            true => PLLSAI2REN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2REN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2REN_A::ENABLED
    }
}
#[doc = "Write proxy for field `PLLSAI2REN`"]
pub struct PLLSAI2REN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2REN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSAI2REN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI2REN_A::DISABLED)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI2REN_A::ENABLED)
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
#[doc = "SAI2PLL PLLSAI2CLK output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSAI2Q_A {
    #[doc = "0: PLLSAI2Q = 2"]
    DIV2 = 0,
    #[doc = "1: PLLSAI2Q = 4"]
    DIV4 = 1,
    #[doc = "2: PLLSAI2Q = 6"]
    DIV6 = 2,
    #[doc = "3: PLLSAI2Q = 8"]
    DIV8 = 3,
}
impl From<PLLSAI2Q_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAI2Q_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLLSAI2Q`"]
pub type PLLSAI2Q_R = crate::R<u8, PLLSAI2Q_A>;
impl PLLSAI2Q_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2Q_A {
        match self.bits {
            0 => PLLSAI2Q_A::DIV2,
            1 => PLLSAI2Q_A::DIV4,
            2 => PLLSAI2Q_A::DIV6,
            3 => PLLSAI2Q_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAI2Q_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAI2Q_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAI2Q_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAI2Q_A::DIV8
    }
}
#[doc = "Write proxy for field `PLLSAI2Q`"]
pub struct PLLSAI2Q_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2Q_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSAI2Q_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PLLSAI2Q = 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAI2Q_A::DIV2)
    }
    #[doc = "PLLSAI2Q = 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAI2Q_A::DIV4)
    }
    #[doc = "PLLSAI2Q = 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAI2Q_A::DIV6)
    }
    #[doc = "PLLSAI2Q = 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAI2Q_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "PLLSAI2 division factor for PLLDISCLK"]
pub type PLLSAI2QEN_A = PLLSAI2REN_A;
#[doc = "Reader of field `PLLSAI2QEN`"]
pub type PLLSAI2QEN_R = crate::R<bool, PLLSAI2REN_A>;
#[doc = "Write proxy for field `PLLSAI2QEN`"]
pub struct PLLSAI2QEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2QEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSAI2QEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI2REN_A::DISABLED)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI2REN_A::ENABLED)
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
#[doc = "SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSAI2P_A {
    #[doc = "0: PLLSAI2P = 7"]
    DIV7 = 0,
    #[doc = "1: PLLSAI2P = 17"]
    DIV17 = 1,
}
impl From<PLLSAI2P_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLSAI2P`"]
pub type PLLSAI2P_R = crate::R<bool, PLLSAI2P_A>;
impl PLLSAI2P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2P_A {
        match self.bits {
            false => PLLSAI2P_A::DIV7,
            true => PLLSAI2P_A::DIV17,
        }
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLSAI2P_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLSAI2P_A::DIV17
    }
}
#[doc = "Write proxy for field `PLLSAI2P`"]
pub struct PLLSAI2P_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSAI2P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLLSAI2P = 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLSAI2P_A::DIV7)
    }
    #[doc = "PLLSAI2P = 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLSAI2P_A::DIV17)
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
#[doc = "SAI2PLL PLLSAI2CLK output enable"]
pub type PLLSAI2PEN_A = PLLSAI2REN_A;
#[doc = "Reader of field `PLLSAI2PEN`"]
pub type PLLSAI2PEN_R = crate::R<bool, PLLSAI2REN_A>;
#[doc = "Write proxy for field `PLLSAI2PEN`"]
pub struct PLLSAI2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2PEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSAI2PEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI2REN_A::DISABLED)
    }
    #[doc = "output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI2REN_A::ENABLED)
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
#[doc = "Reader of field `PLLSAI2N`"]
pub type PLLSAI2N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAI2N`"]
pub struct PLLSAI2N_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PLLSAI2M`"]
pub type PLLSAI2M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAI2M`"]
pub struct PLLSAI2M_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllsai2pdiv(&self) -> PLLSAI2PDIV_R {
        PLLSAI2PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai2r(&self) -> PLLSAI2R_R {
        PLLSAI2R_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2ren(&self) -> PLLSAI2REN_R {
        PLLSAI2REN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2q(&self) -> PLLSAI2Q_R {
        PLLSAI2Q_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - PLLSAI2 division factor for PLLDISCLK"]
    #[inline(always)]
    pub fn pllsai2qen(&self) -> PLLSAI2QEN_R {
        PLLSAI2QEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai2p(&self) -> PLLSAI2P_R {
        PLLSAI2P_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2pen(&self) -> PLLSAI2PEN_R {
        PLLSAI2PEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai2n(&self) -> PLLSAI2N_R {
        PLLSAI2N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 4:7 - Division factor for PLLSAI2 input clock"]
    #[inline(always)]
    pub fn pllsai2m(&self) -> PLLSAI2M_R {
        PLLSAI2M_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllsai2pdiv(&mut self) -> PLLSAI2PDIV_W {
        PLLSAI2PDIV_W { w: self }
    }
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai2r(&mut self) -> PLLSAI2R_W {
        PLLSAI2R_W { w: self }
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2ren(&mut self) -> PLLSAI2REN_W {
        PLLSAI2REN_W { w: self }
    }
    #[doc = "Bits 21:22 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2q(&mut self) -> PLLSAI2Q_W {
        PLLSAI2Q_W { w: self }
    }
    #[doc = "Bit 20 - PLLSAI2 division factor for PLLDISCLK"]
    #[inline(always)]
    pub fn pllsai2qen(&mut self) -> PLLSAI2QEN_W {
        PLLSAI2QEN_W { w: self }
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai2p(&mut self) -> PLLSAI2P_W {
        PLLSAI2P_W { w: self }
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2pen(&mut self) -> PLLSAI2PEN_W {
        PLLSAI2PEN_W { w: self }
    }
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai2n(&mut self) -> PLLSAI2N_W {
        PLLSAI2N_W { w: self }
    }
    #[doc = "Bits 4:7 - Division factor for PLLSAI2 input clock"]
    #[inline(always)]
    pub fn pllsai2m(&mut self) -> PLLSAI2M_W {
        PLLSAI2M_W { w: self }
    }
}
