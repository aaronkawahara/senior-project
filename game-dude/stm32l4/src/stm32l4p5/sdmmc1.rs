#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power control register"]
    pub power: crate::Reg<power::POWER_SPEC>,
    #[doc = "0x04 - SDI clock control register"]
    pub clkcr: crate::Reg<clkcr::CLKCR_SPEC>,
    #[doc = "0x08 - argument register"]
    pub argr: crate::Reg<argr::ARGR_SPEC>,
    #[doc = "0x0c - command register"]
    pub cmdr: crate::Reg<cmdr::CMDR_SPEC>,
    #[doc = "0x10 - command response register"]
    pub respcmdr: crate::Reg<respcmdr::RESPCMDR_SPEC>,
    #[doc = "0x14 - response 1..4 register"]
    pub resp1r: crate::Reg<resp1r::RESP1R_SPEC>,
    #[doc = "0x18 - response 1..4 register"]
    pub resp2r: crate::Reg<resp2r::RESP2R_SPEC>,
    #[doc = "0x1c - response 1..4 register"]
    pub resp3r: crate::Reg<resp3r::RESP3R_SPEC>,
    #[doc = "0x20 - response 1..4 register"]
    pub resp4r: crate::Reg<resp4r::RESP4R_SPEC>,
    #[doc = "0x24 - data timer register"]
    pub dtimer: crate::Reg<dtimer::DTIMER_SPEC>,
    #[doc = "0x28 - data length register"]
    pub dlenr: crate::Reg<dlenr::DLENR_SPEC>,
    #[doc = "0x2c - data control register"]
    pub dctrl: crate::Reg<dctrl::DCTRL_SPEC>,
    #[doc = "0x30 - data counter register"]
    pub dcntr: crate::Reg<dcntr::DCNTR_SPEC>,
    #[doc = "0x34 - status register"]
    pub star: crate::Reg<star::STAR_SPEC>,
    #[doc = "0x38 - interrupt clear register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x3c - mask register"]
    pub maskr: crate::Reg<maskr::MASKR_SPEC>,
    #[doc = "0x40 - acknowledgment timer register"]
    pub acktimer: crate::Reg<acktimer::ACKTIMER_SPEC>,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - DMA control register"]
    pub idmactrlr: crate::Reg<idmactrlr::IDMACTRLR_SPEC>,
    #[doc = "0x54 - IDMA buffer size register"]
    pub idmabsizer: crate::Reg<idmabsizer::IDMABSIZER_SPEC>,
    #[doc = "0x58 - IDMA buffer 0 base address register"]
    pub idmabase0r: crate::Reg<idmabase0r::IDMABASE0R_SPEC>,
    #[doc = "0x5c - IDMA buffer 0 base address register"]
    pub idmabase1r: crate::Reg<idmabase1r::IDMABASE1R_SPEC>,
    _reserved21: [u8; 32usize],
    #[doc = "0x80 - data FIFO register 0"]
    pub fifor0: crate::Reg<fifor0::FIFOR0_SPEC>,
    #[doc = "0x84 - data FIFO register 1"]
    pub fifor1: crate::Reg<fifor1::FIFOR1_SPEC>,
    #[doc = "0x88 - data FIFO register 2"]
    pub fifor2: crate::Reg<fifor2::FIFOR2_SPEC>,
    #[doc = "0x8c - data FIFO register 3"]
    pub fifor3: crate::Reg<fifor3::FIFOR3_SPEC>,
    #[doc = "0x90 - data FIFO register 4"]
    pub fifor4: crate::Reg<fifor4::FIFOR4_SPEC>,
    #[doc = "0x94 - data FIFO register 5"]
    pub fifor5: crate::Reg<fifor5::FIFOR5_SPEC>,
    #[doc = "0x98 - data FIFO register 6"]
    pub fifor6: crate::Reg<fifor6::FIFOR6_SPEC>,
    #[doc = "0x9c - data FIFO register 7"]
    pub fifor7: crate::Reg<fifor7::FIFOR7_SPEC>,
    #[doc = "0xa0 - data FIFO register 8"]
    pub fifor8: crate::Reg<fifor8::FIFOR8_SPEC>,
    #[doc = "0xa4 - data FIFO register 9"]
    pub fifor9: crate::Reg<fifor9::FIFOR9_SPEC>,
    #[doc = "0xa8 - data FIFO register 10"]
    pub fifor10: crate::Reg<fifor10::FIFOR10_SPEC>,
    #[doc = "0xac - data FIFO register 11"]
    pub fifor11: crate::Reg<fifor11::FIFOR11_SPEC>,
    #[doc = "0xb0 - data FIFO register 12"]
    pub fifor12: crate::Reg<fifor12::FIFOR12_SPEC>,
    #[doc = "0xb4 - data FIFO register 13"]
    pub fifor13: crate::Reg<fifor13::FIFOR13_SPEC>,
    #[doc = "0xb8 - data FIFO register 14"]
    pub fifor14: crate::Reg<fifor14::FIFOR14_SPEC>,
    #[doc = "0xbc - data FIFO register 15"]
    pub fifor15: crate::Reg<fifor15::FIFOR15_SPEC>,
}
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "power control register"]
pub mod power;
#[doc = "CLKCR register accessor: an alias for `Reg<CLKCR_SPEC>`"]
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
#[doc = "SDI clock control register"]
pub mod clkcr;
#[doc = "ARGR register accessor: an alias for `Reg<ARGR_SPEC>`"]
pub type ARGR = crate::Reg<argr::ARGR_SPEC>;
#[doc = "argument register"]
pub mod argr;
#[doc = "CMDR register accessor: an alias for `Reg<CMDR_SPEC>`"]
pub type CMDR = crate::Reg<cmdr::CMDR_SPEC>;
#[doc = "command register"]
pub mod cmdr;
#[doc = "RESPCMDR register accessor: an alias for `Reg<RESPCMDR_SPEC>`"]
pub type RESPCMDR = crate::Reg<respcmdr::RESPCMDR_SPEC>;
#[doc = "command response register"]
pub mod respcmdr;
#[doc = "RESP1R register accessor: an alias for `Reg<RESP1R_SPEC>`"]
pub type RESP1R = crate::Reg<resp1r::RESP1R_SPEC>;
#[doc = "response 1..4 register"]
pub mod resp1r;
#[doc = "RESP2R register accessor: an alias for `Reg<RESP2R_SPEC>`"]
pub type RESP2R = crate::Reg<resp2r::RESP2R_SPEC>;
#[doc = "response 1..4 register"]
pub mod resp2r;
#[doc = "RESP3R register accessor: an alias for `Reg<RESP3R_SPEC>`"]
pub type RESP3R = crate::Reg<resp3r::RESP3R_SPEC>;
#[doc = "response 1..4 register"]
pub mod resp3r;
#[doc = "RESP4R register accessor: an alias for `Reg<RESP4R_SPEC>`"]
pub type RESP4R = crate::Reg<resp4r::RESP4R_SPEC>;
#[doc = "response 1..4 register"]
pub mod resp4r;
#[doc = "DTIMER register accessor: an alias for `Reg<DTIMER_SPEC>`"]
pub type DTIMER = crate::Reg<dtimer::DTIMER_SPEC>;
#[doc = "data timer register"]
pub mod dtimer;
#[doc = "DLENR register accessor: an alias for `Reg<DLENR_SPEC>`"]
pub type DLENR = crate::Reg<dlenr::DLENR_SPEC>;
#[doc = "data length register"]
pub mod dlenr;
#[doc = "DCTRL register accessor: an alias for `Reg<DCTRL_SPEC>`"]
pub type DCTRL = crate::Reg<dctrl::DCTRL_SPEC>;
#[doc = "data control register"]
pub mod dctrl;
#[doc = "DCNTR register accessor: an alias for `Reg<DCNTR_SPEC>`"]
pub type DCNTR = crate::Reg<dcntr::DCNTR_SPEC>;
#[doc = "data counter register"]
pub mod dcntr;
#[doc = "STAR register accessor: an alias for `Reg<STAR_SPEC>`"]
pub type STAR = crate::Reg<star::STAR_SPEC>;
#[doc = "status register"]
pub mod star;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "MASKR register accessor: an alias for `Reg<MASKR_SPEC>`"]
pub type MASKR = crate::Reg<maskr::MASKR_SPEC>;
#[doc = "mask register"]
pub mod maskr;
#[doc = "ACKTIMER register accessor: an alias for `Reg<ACKTIMER_SPEC>`"]
pub type ACKTIMER = crate::Reg<acktimer::ACKTIMER_SPEC>;
#[doc = "acknowledgment timer register"]
pub mod acktimer;
#[doc = "FIFOR0 register accessor: an alias for `Reg<FIFOR0_SPEC>`"]
pub type FIFOR0 = crate::Reg<fifor0::FIFOR0_SPEC>;
#[doc = "data FIFO register 0"]
pub mod fifor0;
#[doc = "FIFOR1 register accessor: an alias for `Reg<FIFOR1_SPEC>`"]
pub type FIFOR1 = crate::Reg<fifor1::FIFOR1_SPEC>;
#[doc = "data FIFO register 1"]
pub mod fifor1;
#[doc = "FIFOR2 register accessor: an alias for `Reg<FIFOR2_SPEC>`"]
pub type FIFOR2 = crate::Reg<fifor2::FIFOR2_SPEC>;
#[doc = "data FIFO register 2"]
pub mod fifor2;
#[doc = "FIFOR3 register accessor: an alias for `Reg<FIFOR3_SPEC>`"]
pub type FIFOR3 = crate::Reg<fifor3::FIFOR3_SPEC>;
#[doc = "data FIFO register 3"]
pub mod fifor3;
#[doc = "FIFOR4 register accessor: an alias for `Reg<FIFOR4_SPEC>`"]
pub type FIFOR4 = crate::Reg<fifor4::FIFOR4_SPEC>;
#[doc = "data FIFO register 4"]
pub mod fifor4;
#[doc = "FIFOR5 register accessor: an alias for `Reg<FIFOR5_SPEC>`"]
pub type FIFOR5 = crate::Reg<fifor5::FIFOR5_SPEC>;
#[doc = "data FIFO register 5"]
pub mod fifor5;
#[doc = "FIFOR6 register accessor: an alias for `Reg<FIFOR6_SPEC>`"]
pub type FIFOR6 = crate::Reg<fifor6::FIFOR6_SPEC>;
#[doc = "data FIFO register 6"]
pub mod fifor6;
#[doc = "FIFOR7 register accessor: an alias for `Reg<FIFOR7_SPEC>`"]
pub type FIFOR7 = crate::Reg<fifor7::FIFOR7_SPEC>;
#[doc = "data FIFO register 7"]
pub mod fifor7;
#[doc = "FIFOR8 register accessor: an alias for `Reg<FIFOR8_SPEC>`"]
pub type FIFOR8 = crate::Reg<fifor8::FIFOR8_SPEC>;
#[doc = "data FIFO register 8"]
pub mod fifor8;
#[doc = "FIFOR9 register accessor: an alias for `Reg<FIFOR9_SPEC>`"]
pub type FIFOR9 = crate::Reg<fifor9::FIFOR9_SPEC>;
#[doc = "data FIFO register 9"]
pub mod fifor9;
#[doc = "FIFOR10 register accessor: an alias for `Reg<FIFOR10_SPEC>`"]
pub type FIFOR10 = crate::Reg<fifor10::FIFOR10_SPEC>;
#[doc = "data FIFO register 10"]
pub mod fifor10;
#[doc = "FIFOR11 register accessor: an alias for `Reg<FIFOR11_SPEC>`"]
pub type FIFOR11 = crate::Reg<fifor11::FIFOR11_SPEC>;
#[doc = "data FIFO register 11"]
pub mod fifor11;
#[doc = "FIFOR12 register accessor: an alias for `Reg<FIFOR12_SPEC>`"]
pub type FIFOR12 = crate::Reg<fifor12::FIFOR12_SPEC>;
#[doc = "data FIFO register 12"]
pub mod fifor12;
#[doc = "FIFOR13 register accessor: an alias for `Reg<FIFOR13_SPEC>`"]
pub type FIFOR13 = crate::Reg<fifor13::FIFOR13_SPEC>;
#[doc = "data FIFO register 13"]
pub mod fifor13;
#[doc = "FIFOR14 register accessor: an alias for `Reg<FIFOR14_SPEC>`"]
pub type FIFOR14 = crate::Reg<fifor14::FIFOR14_SPEC>;
#[doc = "data FIFO register 14"]
pub mod fifor14;
#[doc = "FIFOR15 register accessor: an alias for `Reg<FIFOR15_SPEC>`"]
pub type FIFOR15 = crate::Reg<fifor15::FIFOR15_SPEC>;
#[doc = "data FIFO register 15"]
pub mod fifor15;
#[doc = "IDMACTRLR register accessor: an alias for `Reg<IDMACTRLR_SPEC>`"]
pub type IDMACTRLR = crate::Reg<idmactrlr::IDMACTRLR_SPEC>;
#[doc = "DMA control register"]
pub mod idmactrlr;
#[doc = "IDMABSIZER register accessor: an alias for `Reg<IDMABSIZER_SPEC>`"]
pub type IDMABSIZER = crate::Reg<idmabsizer::IDMABSIZER_SPEC>;
#[doc = "IDMA buffer size register"]
pub mod idmabsizer;
#[doc = "IDMABASE0R register accessor: an alias for `Reg<IDMABASE0R_SPEC>`"]
pub type IDMABASE0R = crate::Reg<idmabase0r::IDMABASE0R_SPEC>;
#[doc = "IDMA buffer 0 base address register"]
pub mod idmabase0r;
#[doc = "IDMABASE1R register accessor: an alias for `Reg<IDMABASE1R_SPEC>`"]
pub type IDMABASE1R = crate::Reg<idmabase1r::IDMABASE1R_SPEC>;
#[doc = "IDMA buffer 0 base address register"]
pub mod idmabase1r;