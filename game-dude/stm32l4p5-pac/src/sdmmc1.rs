#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power control register"]
    pub power: POWER,
    #[doc = "0x04 - SDI clock control register"]
    pub clkcr: CLKCR,
    #[doc = "0x08 - argument register"]
    pub argr: ARGR,
    #[doc = "0x0c - command register"]
    pub cmdr: CMDR,
    #[doc = "0x10 - command response register"]
    pub respcmdr: RESPCMDR,
    #[doc = "0x14 - response 1..4 register"]
    pub resp1r: RESP1R,
    #[doc = "0x18 - response 1..4 register"]
    pub resp2r: RESP2R,
    #[doc = "0x1c - response 1..4 register"]
    pub resp3r: RESP3R,
    #[doc = "0x20 - response 1..4 register"]
    pub resp4r: RESP4R,
    #[doc = "0x24 - data timer register"]
    pub dtimer: DTIMER,
    #[doc = "0x28 - data length register"]
    pub dlenr: DLENR,
    #[doc = "0x2c - data control register"]
    pub dctrl: DCTRL,
    #[doc = "0x30 - data counter register"]
    pub dcntr: DCNTR,
    #[doc = "0x34 - status register"]
    pub star: STAR,
    #[doc = "0x38 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x3c - mask register"]
    pub maskr: MASKR,
    #[doc = "0x40 - acknowledgment timer register"]
    pub acktimer: ACKTIMER,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - DMA control register"]
    pub idmactrlr: IDMACTRLR,
    #[doc = "0x54 - IDMA buffer size register"]
    pub idmabsizer: IDMABSIZER,
    #[doc = "0x58 - IDMA buffer 0 base address register"]
    pub idmabase0r: IDMABASE0R,
    #[doc = "0x5c - IDMA buffer 0 base address register"]
    pub idmabase1r: IDMABASE1R,
    _reserved21: [u8; 32usize],
    #[doc = "0x80 - data FIFO register 0"]
    pub fifor0: FIFOR0,
    #[doc = "0x84 - data FIFO register 1"]
    pub fifor1: FIFOR1,
    #[doc = "0x88 - data FIFO register 2"]
    pub fifor2: FIFOR2,
    #[doc = "0x8c - data FIFO register 3"]
    pub fifor3: FIFOR3,
    #[doc = "0x90 - data FIFO register 4"]
    pub fifor4: FIFOR4,
    #[doc = "0x94 - data FIFO register 5"]
    pub fifor5: FIFOR5,
    #[doc = "0x98 - data FIFO register 6"]
    pub fifor6: FIFOR6,
    #[doc = "0x9c - data FIFO register 7"]
    pub fifor7: FIFOR7,
    #[doc = "0xa0 - data FIFO register 8"]
    pub fifor8: FIFOR8,
    #[doc = "0xa4 - data FIFO register 9"]
    pub fifor9: FIFOR9,
    #[doc = "0xa8 - data FIFO register 10"]
    pub fifor10: FIFOR10,
    #[doc = "0xac - data FIFO register 11"]
    pub fifor11: FIFOR11,
    #[doc = "0xb0 - data FIFO register 12"]
    pub fifor12: FIFOR12,
    #[doc = "0xb4 - data FIFO register 13"]
    pub fifor13: FIFOR13,
    #[doc = "0xb8 - data FIFO register 14"]
    pub fifor14: FIFOR14,
    #[doc = "0xbc - data FIFO register 15"]
    pub fifor15: FIFOR15,
}
#[doc = "power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](power) module"]
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "power control register"]
pub mod power;
#[doc = "SDI clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcr](clkcr) module"]
pub type CLKCR = crate::Reg<u32, _CLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCR;
#[doc = "`read()` method returns [clkcr::R](clkcr::R) reader structure"]
impl crate::Readable for CLKCR {}
#[doc = "`write(|w| ..)` method takes [clkcr::W](clkcr::W) writer structure"]
impl crate::Writable for CLKCR {}
#[doc = "SDI clock control register"]
pub mod clkcr;
#[doc = "argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [argr](argr) module"]
pub type ARGR = crate::Reg<u32, _ARGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARGR;
#[doc = "`read()` method returns [argr::R](argr::R) reader structure"]
impl crate::Readable for ARGR {}
#[doc = "`write(|w| ..)` method takes [argr::W](argr::W) writer structure"]
impl crate::Writable for ARGR {}
#[doc = "argument register"]
pub mod argr;
#[doc = "command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdr](cmdr) module"]
pub type CMDR = crate::Reg<u32, _CMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDR;
#[doc = "`read()` method returns [cmdr::R](cmdr::R) reader structure"]
impl crate::Readable for CMDR {}
#[doc = "`write(|w| ..)` method takes [cmdr::W](cmdr::W) writer structure"]
impl crate::Writable for CMDR {}
#[doc = "command register"]
pub mod cmdr;
#[doc = "command response register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [respcmdr](respcmdr) module"]
pub type RESPCMDR = crate::Reg<u32, _RESPCMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPCMDR;
#[doc = "`read()` method returns [respcmdr::R](respcmdr::R) reader structure"]
impl crate::Readable for RESPCMDR {}
#[doc = "command response register"]
pub mod respcmdr;
#[doc = "response 1..4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp1r](resp1r) module"]
pub type RESP1R = crate::Reg<u32, _RESP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP1R;
#[doc = "`read()` method returns [resp1r::R](resp1r::R) reader structure"]
impl crate::Readable for RESP1R {}
#[doc = "response 1..4 register"]
pub mod resp1r;
#[doc = "response 1..4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp2r](resp2r) module"]
pub type RESP2R = crate::Reg<u32, _RESP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP2R;
#[doc = "`read()` method returns [resp2r::R](resp2r::R) reader structure"]
impl crate::Readable for RESP2R {}
#[doc = "response 1..4 register"]
pub mod resp2r;
#[doc = "response 1..4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp3r](resp3r) module"]
pub type RESP3R = crate::Reg<u32, _RESP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP3R;
#[doc = "`read()` method returns [resp3r::R](resp3r::R) reader structure"]
impl crate::Readable for RESP3R {}
#[doc = "response 1..4 register"]
pub mod resp3r;
#[doc = "response 1..4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp4r](resp4r) module"]
pub type RESP4R = crate::Reg<u32, _RESP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP4R;
#[doc = "`read()` method returns [resp4r::R](resp4r::R) reader structure"]
impl crate::Readable for RESP4R {}
#[doc = "response 1..4 register"]
pub mod resp4r;
#[doc = "data timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtimer](dtimer) module"]
pub type DTIMER = crate::Reg<u32, _DTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTIMER;
#[doc = "`read()` method returns [dtimer::R](dtimer::R) reader structure"]
impl crate::Readable for DTIMER {}
#[doc = "`write(|w| ..)` method takes [dtimer::W](dtimer::W) writer structure"]
impl crate::Writable for DTIMER {}
#[doc = "data timer register"]
pub mod dtimer;
#[doc = "data length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlenr](dlenr) module"]
pub type DLENR = crate::Reg<u32, _DLENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLENR;
#[doc = "`read()` method returns [dlenr::R](dlenr::R) reader structure"]
impl crate::Readable for DLENR {}
#[doc = "`write(|w| ..)` method takes [dlenr::W](dlenr::W) writer structure"]
impl crate::Writable for DLENR {}
#[doc = "data length register"]
pub mod dlenr;
#[doc = "data control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctrl](dctrl) module"]
pub type DCTRL = crate::Reg<u32, _DCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTRL;
#[doc = "`read()` method returns [dctrl::R](dctrl::R) reader structure"]
impl crate::Readable for DCTRL {}
#[doc = "`write(|w| ..)` method takes [dctrl::W](dctrl::W) writer structure"]
impl crate::Writable for DCTRL {}
#[doc = "data control register"]
pub mod dctrl;
#[doc = "data counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcntr](dcntr) module"]
pub type DCNTR = crate::Reg<u32, _DCNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCNTR;
#[doc = "`read()` method returns [dcntr::R](dcntr::R) reader structure"]
impl crate::Readable for DCNTR {}
#[doc = "data counter register"]
pub mod dcntr;
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [star](star) module"]
pub type STAR = crate::Reg<u32, _STAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAR;
#[doc = "`read()` method returns [star::R](star::R) reader structure"]
impl crate::Readable for STAR {}
#[doc = "status register"]
pub mod star;
#[doc = "interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maskr](maskr) module"]
pub type MASKR = crate::Reg<u32, _MASKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKR;
#[doc = "`read()` method returns [maskr::R](maskr::R) reader structure"]
impl crate::Readable for MASKR {}
#[doc = "`write(|w| ..)` method takes [maskr::W](maskr::W) writer structure"]
impl crate::Writable for MASKR {}
#[doc = "mask register"]
pub mod maskr;
#[doc = "acknowledgment timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acktimer](acktimer) module"]
pub type ACKTIMER = crate::Reg<u32, _ACKTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACKTIMER;
#[doc = "`read()` method returns [acktimer::R](acktimer::R) reader structure"]
impl crate::Readable for ACKTIMER {}
#[doc = "`write(|w| ..)` method takes [acktimer::W](acktimer::W) writer structure"]
impl crate::Writable for ACKTIMER {}
#[doc = "acknowledgment timer register"]
pub mod acktimer;
#[doc = "data FIFO register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor0](fifor0) module"]
pub type FIFOR0 = crate::Reg<u32, _FIFOR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR0;
#[doc = "`read()` method returns [fifor0::R](fifor0::R) reader structure"]
impl crate::Readable for FIFOR0 {}
#[doc = "`write(|w| ..)` method takes [fifor0::W](fifor0::W) writer structure"]
impl crate::Writable for FIFOR0 {}
#[doc = "data FIFO register 0"]
pub mod fifor0;
#[doc = "data FIFO register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor1](fifor1) module"]
pub type FIFOR1 = crate::Reg<u32, _FIFOR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR1;
#[doc = "`read()` method returns [fifor1::R](fifor1::R) reader structure"]
impl crate::Readable for FIFOR1 {}
#[doc = "`write(|w| ..)` method takes [fifor1::W](fifor1::W) writer structure"]
impl crate::Writable for FIFOR1 {}
#[doc = "data FIFO register 1"]
pub mod fifor1;
#[doc = "data FIFO register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor2](fifor2) module"]
pub type FIFOR2 = crate::Reg<u32, _FIFOR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR2;
#[doc = "`read()` method returns [fifor2::R](fifor2::R) reader structure"]
impl crate::Readable for FIFOR2 {}
#[doc = "`write(|w| ..)` method takes [fifor2::W](fifor2::W) writer structure"]
impl crate::Writable for FIFOR2 {}
#[doc = "data FIFO register 2"]
pub mod fifor2;
#[doc = "data FIFO register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor3](fifor3) module"]
pub type FIFOR3 = crate::Reg<u32, _FIFOR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR3;
#[doc = "`read()` method returns [fifor3::R](fifor3::R) reader structure"]
impl crate::Readable for FIFOR3 {}
#[doc = "`write(|w| ..)` method takes [fifor3::W](fifor3::W) writer structure"]
impl crate::Writable for FIFOR3 {}
#[doc = "data FIFO register 3"]
pub mod fifor3;
#[doc = "data FIFO register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor4](fifor4) module"]
pub type FIFOR4 = crate::Reg<u32, _FIFOR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR4;
#[doc = "`read()` method returns [fifor4::R](fifor4::R) reader structure"]
impl crate::Readable for FIFOR4 {}
#[doc = "`write(|w| ..)` method takes [fifor4::W](fifor4::W) writer structure"]
impl crate::Writable for FIFOR4 {}
#[doc = "data FIFO register 4"]
pub mod fifor4;
#[doc = "data FIFO register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor5](fifor5) module"]
pub type FIFOR5 = crate::Reg<u32, _FIFOR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR5;
#[doc = "`read()` method returns [fifor5::R](fifor5::R) reader structure"]
impl crate::Readable for FIFOR5 {}
#[doc = "`write(|w| ..)` method takes [fifor5::W](fifor5::W) writer structure"]
impl crate::Writable for FIFOR5 {}
#[doc = "data FIFO register 5"]
pub mod fifor5;
#[doc = "data FIFO register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor6](fifor6) module"]
pub type FIFOR6 = crate::Reg<u32, _FIFOR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR6;
#[doc = "`read()` method returns [fifor6::R](fifor6::R) reader structure"]
impl crate::Readable for FIFOR6 {}
#[doc = "`write(|w| ..)` method takes [fifor6::W](fifor6::W) writer structure"]
impl crate::Writable for FIFOR6 {}
#[doc = "data FIFO register 6"]
pub mod fifor6;
#[doc = "data FIFO register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor7](fifor7) module"]
pub type FIFOR7 = crate::Reg<u32, _FIFOR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR7;
#[doc = "`read()` method returns [fifor7::R](fifor7::R) reader structure"]
impl crate::Readable for FIFOR7 {}
#[doc = "`write(|w| ..)` method takes [fifor7::W](fifor7::W) writer structure"]
impl crate::Writable for FIFOR7 {}
#[doc = "data FIFO register 7"]
pub mod fifor7;
#[doc = "data FIFO register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor8](fifor8) module"]
pub type FIFOR8 = crate::Reg<u32, _FIFOR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR8;
#[doc = "`read()` method returns [fifor8::R](fifor8::R) reader structure"]
impl crate::Readable for FIFOR8 {}
#[doc = "`write(|w| ..)` method takes [fifor8::W](fifor8::W) writer structure"]
impl crate::Writable for FIFOR8 {}
#[doc = "data FIFO register 8"]
pub mod fifor8;
#[doc = "data FIFO register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor9](fifor9) module"]
pub type FIFOR9 = crate::Reg<u32, _FIFOR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR9;
#[doc = "`read()` method returns [fifor9::R](fifor9::R) reader structure"]
impl crate::Readable for FIFOR9 {}
#[doc = "`write(|w| ..)` method takes [fifor9::W](fifor9::W) writer structure"]
impl crate::Writable for FIFOR9 {}
#[doc = "data FIFO register 9"]
pub mod fifor9;
#[doc = "data FIFO register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor10](fifor10) module"]
pub type FIFOR10 = crate::Reg<u32, _FIFOR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR10;
#[doc = "`read()` method returns [fifor10::R](fifor10::R) reader structure"]
impl crate::Readable for FIFOR10 {}
#[doc = "`write(|w| ..)` method takes [fifor10::W](fifor10::W) writer structure"]
impl crate::Writable for FIFOR10 {}
#[doc = "data FIFO register 10"]
pub mod fifor10;
#[doc = "data FIFO register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor11](fifor11) module"]
pub type FIFOR11 = crate::Reg<u32, _FIFOR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR11;
#[doc = "`read()` method returns [fifor11::R](fifor11::R) reader structure"]
impl crate::Readable for FIFOR11 {}
#[doc = "`write(|w| ..)` method takes [fifor11::W](fifor11::W) writer structure"]
impl crate::Writable for FIFOR11 {}
#[doc = "data FIFO register 11"]
pub mod fifor11;
#[doc = "data FIFO register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor12](fifor12) module"]
pub type FIFOR12 = crate::Reg<u32, _FIFOR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR12;
#[doc = "`read()` method returns [fifor12::R](fifor12::R) reader structure"]
impl crate::Readable for FIFOR12 {}
#[doc = "`write(|w| ..)` method takes [fifor12::W](fifor12::W) writer structure"]
impl crate::Writable for FIFOR12 {}
#[doc = "data FIFO register 12"]
pub mod fifor12;
#[doc = "data FIFO register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor13](fifor13) module"]
pub type FIFOR13 = crate::Reg<u32, _FIFOR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR13;
#[doc = "`read()` method returns [fifor13::R](fifor13::R) reader structure"]
impl crate::Readable for FIFOR13 {}
#[doc = "`write(|w| ..)` method takes [fifor13::W](fifor13::W) writer structure"]
impl crate::Writable for FIFOR13 {}
#[doc = "data FIFO register 13"]
pub mod fifor13;
#[doc = "data FIFO register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor14](fifor14) module"]
pub type FIFOR14 = crate::Reg<u32, _FIFOR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR14;
#[doc = "`read()` method returns [fifor14::R](fifor14::R) reader structure"]
impl crate::Readable for FIFOR14 {}
#[doc = "`write(|w| ..)` method takes [fifor14::W](fifor14::W) writer structure"]
impl crate::Writable for FIFOR14 {}
#[doc = "data FIFO register 14"]
pub mod fifor14;
#[doc = "data FIFO register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifor15](fifor15) module"]
pub type FIFOR15 = crate::Reg<u32, _FIFOR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOR15;
#[doc = "`read()` method returns [fifor15::R](fifor15::R) reader structure"]
impl crate::Readable for FIFOR15 {}
#[doc = "`write(|w| ..)` method takes [fifor15::W](fifor15::W) writer structure"]
impl crate::Writable for FIFOR15 {}
#[doc = "data FIFO register 15"]
pub mod fifor15;
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmactrlr](idmactrlr) module"]
pub type IDMACTRLR = crate::Reg<u32, _IDMACTRLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMACTRLR;
#[doc = "`read()` method returns [idmactrlr::R](idmactrlr::R) reader structure"]
impl crate::Readable for IDMACTRLR {}
#[doc = "`write(|w| ..)` method takes [idmactrlr::W](idmactrlr::W) writer structure"]
impl crate::Writable for IDMACTRLR {}
#[doc = "DMA control register"]
pub mod idmactrlr;
#[doc = "IDMA buffer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmabsizer](idmabsizer) module"]
pub type IDMABSIZER = crate::Reg<u32, _IDMABSIZER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMABSIZER;
#[doc = "`read()` method returns [idmabsizer::R](idmabsizer::R) reader structure"]
impl crate::Readable for IDMABSIZER {}
#[doc = "`write(|w| ..)` method takes [idmabsizer::W](idmabsizer::W) writer structure"]
impl crate::Writable for IDMABSIZER {}
#[doc = "IDMA buffer size register"]
pub mod idmabsizer;
#[doc = "IDMA buffer 0 base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmabase0r](idmabase0r) module"]
pub type IDMABASE0R = crate::Reg<u32, _IDMABASE0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMABASE0R;
#[doc = "`read()` method returns [idmabase0r::R](idmabase0r::R) reader structure"]
impl crate::Readable for IDMABASE0R {}
#[doc = "`write(|w| ..)` method takes [idmabase0r::W](idmabase0r::W) writer structure"]
impl crate::Writable for IDMABASE0R {}
#[doc = "IDMA buffer 0 base address register"]
pub mod idmabase0r;
#[doc = "IDMA buffer 0 base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idmabase1r](idmabase1r) module"]
pub type IDMABASE1R = crate::Reg<u32, _IDMABASE1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDMABASE1R;
#[doc = "`read()` method returns [idmabase1r::R](idmabase1r::R) reader structure"]
impl crate::Readable for IDMABASE1R {}
#[doc = "`write(|w| ..)` method takes [idmabase1r::W](idmabase1r::W) writer structure"]
impl crate::Writable for IDMABASE1R {}
#[doc = "IDMA buffer 0 base address register"]
pub mod idmabase1r;
