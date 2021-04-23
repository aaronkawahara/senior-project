#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Writer for register CR2"]
pub type W = crate::W<u32, super::CR2>;
#[doc = "Register CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTOEN`"]
pub type RTOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTOEN`"]
pub struct RTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ABREN`"]
pub type ABREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABREN`"]
pub struct ABREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABREN_W<'a> {
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
#[doc = "Reader of field `MSBFIRST`"]
pub type MSBFIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSBFIRST`"]
pub struct MSBFIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBFIRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DATAINV`"]
pub type DATAINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAINV`"]
pub struct DATAINV_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TXINV`"]
pub type TXINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXINV`"]
pub struct TXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINV_W<'a> {
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
#[doc = "Reader of field `RXINV`"]
pub type RXINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXINV`"]
pub struct RXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINV_W<'a> {
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
#[doc = "Reader of field `SWAP`"]
pub type SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP`"]
pub struct SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `LINEN`"]
pub type LINEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LINEN`"]
pub struct LINEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LINEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "STOP bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STOP_A {
    #[doc = "0: 1 stop bit"]
    STOP1 = 0,
    #[doc = "1: 0.5 stop bits"]
    STOP0P5 = 1,
    #[doc = "2: 2 stop bits"]
    STOP2 = 2,
    #[doc = "3: 1.5 stop bits"]
    STOP1P5 = 3,
}
impl From<STOP_A> for u8 {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<u8, STOP_A>;
impl STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            0 => STOP_A::STOP1,
            1 => STOP_A::STOP0P5,
            2 => STOP_A::STOP2,
            3 => STOP_A::STOP1P5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP1`"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP_A::STOP1
    }
    #[doc = "Checks if the value of the field is `STOP0P5`"]
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == STOP_A::STOP0P5
    }
    #[doc = "Checks if the value of the field is `STOP2`"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP_A::STOP2
    }
    #[doc = "Checks if the value of the field is `STOP1P5`"]
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == STOP_A::STOP1P5
    }
}
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut W {
        self.variant(STOP_A::STOP1)
    }
    #[doc = "0.5 stop bits"]
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut W {
        self.variant(STOP_A::STOP0P5)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(STOP_A::STOP2)
    }
    #[doc = "1.5 stop bits"]
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut W {
        self.variant(STOP_A::STOP1P5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKEN_A {
    #[doc = "0: CK pin disabled"]
    DISABLED = 0,
    #[doc = "1: CK pin enabled"]
    ENABLED = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKEN`"]
pub type CLKEN_R = crate::R<bool, CLKEN_A>;
impl CLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::DISABLED,
            true => CLKEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `CLKEN`"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKEN_A::DISABLED)
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: Steady low value on CK pin outside transmission window"]
    LOW = 0,
    #[doc = "1: Steady high value on CK pin outside transmission window"]
    HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::LOW,
            true => CPOL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOL_A::HIGH
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Steady low value on CK pin outside transmission window"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOL_A::LOW)
    }
    #[doc = "Steady high value on CK pin outside transmission window"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOL_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: The first clock transition is the first data capture edge"]
    FIRST = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    SECOND = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::FIRST,
            true => CPHA_A::SECOND,
        }
    }
    #[doc = "Checks if the value of the field is `FIRST`"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CPHA_A::FIRST
    }
    #[doc = "Checks if the value of the field is `SECOND`"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == CPHA_A::SECOND
    }
}
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first(self) -> &'a mut W {
        self.variant(CPHA_A::FIRST)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second(self) -> &'a mut W {
        self.variant(CPHA_A::SECOND)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `LBCL`"]
pub type LBCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBCL`"]
pub struct LBCL_W<'a> {
    w: &'a mut W,
}
impl<'a> LBCL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `LBDIE`"]
pub type LBDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBDIE`"]
pub struct LBDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBDIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LBDL`"]
pub type LBDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBDL`"]
pub struct LBDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LBDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ADDM7`"]
pub type ADDM7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDM7`"]
pub struct ADDM7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDM7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DIS_NSS`"]
pub type DIS_NSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_NSS`"]
pub struct DIS_NSS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_NSS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SLVEN`"]
pub type SLVEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLVEN`"]
pub struct SLVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ADD`"]
pub type ADD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADD`"]
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `ABRMOD`"]
pub type ABRMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ABRMOD`"]
pub struct ABRMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rtoen(&self) -> RTOEN_R {
        RTOEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIS_NSS"]
    #[inline(always)]
    pub fn dis_nss(&self) -> DIS_NSS_R {
        DIS_NSS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Synchronous Slave mode enable"]
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abrmod(&self) -> ABRMOD_R {
        ABRMOD_R::new(((self.bits >> 21) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rtoen(&mut self) -> RTOEN_W {
        RTOEN_W { w: self }
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abren(&mut self) -> ABREN_W {
        ABREN_W { w: self }
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MSBFIRST_W {
        MSBFIRST_W { w: self }
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn datainv(&mut self) -> DATAINV_W {
        DATAINV_W { w: self }
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W {
        TXINV_W { w: self }
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W {
        RXINV_W { w: self }
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W {
        SWAP_W { w: self }
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&mut self) -> LINEN_W {
        LINEN_W { w: self }
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcl(&mut self) -> LBCL_W {
        LBCL_W { w: self }
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W {
        LBDIE_W { w: self }
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    pub fn lbdl(&mut self) -> LBDL_W {
        LBDL_W { w: self }
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&mut self) -> ADDM7_W {
        ADDM7_W { w: self }
    }
    #[doc = "Bit 3 - DIS_NSS"]
    #[inline(always)]
    pub fn dis_nss(&mut self) -> DIS_NSS_W {
        DIS_NSS_W { w: self }
    }
    #[doc = "Bit 0 - Synchronous Slave mode enable"]
    #[inline(always)]
    pub fn slven(&mut self) -> SLVEN_W {
        SLVEN_W { w: self }
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abrmod(&mut self) -> ABRMOD_W {
        ABRMOD_W { w: self }
    }
}
