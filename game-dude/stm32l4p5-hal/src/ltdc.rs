use crate::stm32::{
    ltdc,
    LTDC,
};


pub trait LtdcExt {
    fn constrain(self) -> Ltdc;
}

impl LtdcExt for LTDC {
    fn constrain(self) -> Ltdc {
        Ltdc {
            sscr: SSCR { _0: () },
            bpcr: BPCR { _0: () },
            awcr: AWCR { _0: () },
            twcr: TWCR { _0: () },
            gcr: GCR { _0: () },
            srcr: SRCR{ _0: () },
            bccr: BCCR { _0: () },
            ier: IER { _0: () },
            isr: ISR { _0: () },
            icr: ICR { _0: () },
            lipcr: LIPCR { _0: () },
            cpsr: CPSR { _0: () },
            cdsr: CDSR { _0: () },
            layer1: LAYER { _0: () },
        }
    }
}

// constrained LTDC peripheral
pub struct Ltdc {
    // LTDC Synchronization Size Configuration Register"]
    pub sscr: SSCR,
    // LTDC Back Porch Configuration Register"]
    pub bpcr: BPCR,
    // LTDC Active Width Configuration Register"]
    pub awcr: AWCR,
    // LTDC Total Width Configuration Register"]
    pub twcr: TWCR,
    // LTDC Global Control Register"]
    pub gcr: GCR,
    // LTDC Shadow Reload Configuration Register"]
    pub srcr: SRCR,
    // LTDC Background Color Configuration Register"]
    pub bccr: BCCR,
    // LTDC Interrupt Enable Register"]
    pub ier: IER,
    // LTDC Interrupt Status Register"]
    pub isr: ISR,
    // LTDC Interrupt Clear Register"]
    pub icr: ICR,
    // LTDC Line Interrupt Position Configuration Register"]
    pub lipcr: LIPCR,
    // LTDC Current Position Status Register"]
    pub cpsr: CPSR,
    // LTDC Current Display Status Register"]
    pub cdsr: CDSR,
    // Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    pub layer1: LAYER,
}

pub struct SSCR {
    _0: (),
}

impl SSCR {
    pub(crate) fn sscr(&mut self) -> &ltdc::SSCR {
        unsafe { &(*LTDC::ptr()).sscr }
    }
}

pub struct BPCR {
    _0: (),
}

pub struct AWCR {
    _0: (),
}

pub struct TWCR {
    _0: (),
}

pub struct GCR {
    _0: (),
}

pub struct SRCR {
    _0: (),
}

pub struct BCCR {
    _0: (),
}

pub struct IER {
    _0: (),
}

pub struct ISR {
    _0: (),
}

pub struct ICR {
    _0: (),
}

pub struct LIPCR {
    _0: (),
}

pub struct CPSR {
    _0: (),
}

pub struct CDSR {
    _0: (),
}

pub struct LAYER {
    _0: (),
}
