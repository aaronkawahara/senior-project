#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - LTDC Synchronization Size Configuration Register"]
    pub sscr: crate::Reg<sscr::SSCR_SPEC>,
    #[doc = "0x0c - LTDC Back Porch Configuration Register"]
    pub bpcr: crate::Reg<bpcr::BPCR_SPEC>,
    #[doc = "0x10 - LTDC Active Width Configuration Register"]
    pub awcr: crate::Reg<awcr::AWCR_SPEC>,
    #[doc = "0x14 - LTDC Total Width Configuration Register"]
    pub twcr: crate::Reg<twcr::TWCR_SPEC>,
    #[doc = "0x18 - LTDC Global Control Register"]
    pub gcr: crate::Reg<gcr::GCR_SPEC>,
    _reserved5: [u8; 8usize],
    #[doc = "0x24 - LTDC Shadow Reload Configuration Register"]
    pub srcr: crate::Reg<srcr::SRCR_SPEC>,
    _reserved6: [u8; 4usize],
    #[doc = "0x2c - LTDC Background Color Configuration Register"]
    pub bccr: crate::Reg<bccr::BCCR_SPEC>,
    _reserved7: [u8; 4usize],
    #[doc = "0x34 - LTDC Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x38 - LTDC Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x3c - LTDC Interrupt Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x40 - LTDC Line Interrupt Position Configuration Register"]
    pub lipcr: crate::Reg<lipcr::LIPCR_SPEC>,
    #[doc = "0x44 - LTDC Current Position Status Register"]
    pub cpsr: crate::Reg<cpsr::CPSR_SPEC>,
    #[doc = "0x48 - LTDC Current Display Status Register"]
    pub cdsr: crate::Reg<cdsr::CDSR_SPEC>,
    _reserved13: [u8; 56usize],
    #[doc = "0x84 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    pub layer1: LAYER,
    _reserved14: [u8; 60usize],
    #[doc = "0x104 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    pub layer2: LAYER,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct LAYER {
    #[doc = "0x00 - LTDC Layer Control Register"]
    pub cr: crate::Reg<self::layer::cr::CR_SPEC>,
    #[doc = "0x04 - LTDC Layer Window Horizontal Position Configuration Register"]
    pub whpcr: crate::Reg<self::layer::whpcr::WHPCR_SPEC>,
    #[doc = "0x08 - LTDC Layer Window Vertical Position Configuration Register"]
    pub wvpcr: crate::Reg<self::layer::wvpcr::WVPCR_SPEC>,
    #[doc = "0x0c - LTDC Layer Color Keying Configuration Register"]
    pub ckcr: crate::Reg<self::layer::ckcr::CKCR_SPEC>,
    #[doc = "0x10 - LTDC Layer Pixel Format Configuration Register"]
    pub pfcr: crate::Reg<self::layer::pfcr::PFCR_SPEC>,
    #[doc = "0x14 - LTDC Layer Constant Alpha Configuration Register"]
    pub cacr: crate::Reg<self::layer::cacr::CACR_SPEC>,
    #[doc = "0x18 - LTDC Layer Default Color Configuration Register"]
    pub dccr: crate::Reg<self::layer::dccr::DCCR_SPEC>,
    #[doc = "0x1c - LTDC Layer Blending Factors Configuration Register"]
    pub bfcr: crate::Reg<self::layer::bfcr::BFCR_SPEC>,
    _reserved8: [u8; 8usize],
    #[doc = "0x28 - LTDC Layer Color Frame Buffer Address Register"]
    pub cfbar: crate::Reg<self::layer::cfbar::CFBAR_SPEC>,
    #[doc = "0x2c - LTDC Layer Color Frame Buffer Length Register"]
    pub cfblr: crate::Reg<self::layer::cfblr::CFBLR_SPEC>,
    #[doc = "0x30 - LTDC Layer ColorFrame Buffer Line Number Register"]
    pub cfblnr: crate::Reg<self::layer::cfblnr::CFBLNR_SPEC>,
    _reserved11: [u8; 12usize],
    #[doc = "0x40 - LTDC Layerx CLUT Write Register"]
    pub clutwr: crate::Reg<self::layer::clutwr::CLUTWR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
pub mod layer;
#[doc = "SSCR register accessor: an alias for `Reg<SSCR_SPEC>`"]
pub type SSCR = crate::Reg<sscr::SSCR_SPEC>;
#[doc = "LTDC Synchronization Size Configuration Register"]
pub mod sscr;
#[doc = "BPCR register accessor: an alias for `Reg<BPCR_SPEC>`"]
pub type BPCR = crate::Reg<bpcr::BPCR_SPEC>;
#[doc = "LTDC Back Porch Configuration Register"]
pub mod bpcr;
#[doc = "AWCR register accessor: an alias for `Reg<AWCR_SPEC>`"]
pub type AWCR = crate::Reg<awcr::AWCR_SPEC>;
#[doc = "LTDC Active Width Configuration Register"]
pub mod awcr;
#[doc = "TWCR register accessor: an alias for `Reg<TWCR_SPEC>`"]
pub type TWCR = crate::Reg<twcr::TWCR_SPEC>;
#[doc = "LTDC Total Width Configuration Register"]
pub mod twcr;
#[doc = "GCR register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "LTDC Global Control Register"]
pub mod gcr;
#[doc = "SRCR register accessor: an alias for `Reg<SRCR_SPEC>`"]
pub type SRCR = crate::Reg<srcr::SRCR_SPEC>;
#[doc = "LTDC Shadow Reload Configuration Register"]
pub mod srcr;
#[doc = "BCCR register accessor: an alias for `Reg<BCCR_SPEC>`"]
pub type BCCR = crate::Reg<bccr::BCCR_SPEC>;
#[doc = "LTDC Background Color Configuration Register"]
pub mod bccr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "LTDC Interrupt Enable Register"]
pub mod ier;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "LTDC Interrupt Status Register"]
pub mod isr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "LTDC Interrupt Clear Register"]
pub mod icr;
#[doc = "LIPCR register accessor: an alias for `Reg<LIPCR_SPEC>`"]
pub type LIPCR = crate::Reg<lipcr::LIPCR_SPEC>;
#[doc = "LTDC Line Interrupt Position Configuration Register"]
pub mod lipcr;
#[doc = "CPSR register accessor: an alias for `Reg<CPSR_SPEC>`"]
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
#[doc = "LTDC Current Position Status Register"]
pub mod cpsr;
#[doc = "CDSR register accessor: an alias for `Reg<CDSR_SPEC>`"]
pub type CDSR = crate::Reg<cdsr::CDSR_SPEC>;
#[doc = "LTDC Current Display Status Register"]
pub mod cdsr;
