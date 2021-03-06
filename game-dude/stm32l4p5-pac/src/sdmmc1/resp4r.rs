#[doc = "Reader of register RESP4R"]
pub type R = crate::R<u32, super::RESP4R>;
#[doc = "Reader of field `CARDSTATUS4`"]
pub type CARDSTATUS4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - see Table 347"]
    #[inline(always)]
    pub fn cardstatus4(&self) -> CARDSTATUS4_R {
        CARDSTATUS4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
