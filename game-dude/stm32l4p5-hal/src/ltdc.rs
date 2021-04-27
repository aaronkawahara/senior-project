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
            sscr: SSCR { 
                hsw: None, 
                vsh: None, 
            },
            bpcr: BPCR { 
                ahbp: None, 
                avbp: None, 
            },
            awcr: AWCR { 
                aaw: None,
                aah: None,
            },
            twcr: TWCR { 
                totalw: None,
                totalh: None,
            },
            gcr: GCR { 
                hspol: false,
                vspol: false,
                depol: false,
                pcpol: false,
                den: false,
                ltdcen: false,
            },
            srcr: SRCR { 
                vbr: false,
                imr: false,
            },
            bccr: BCCR {
                bcred: None,
                bcgreen: None,
                bcblue: None,
            },
            ier: IER {
                rrie: false,
                terrie: false,
                fuie: false,
                lie: false,
            },
            isr: ISR { _0: () },
            icr: ICR { _0: () },
            lipcr: LIPCR { 
                lipos: None
            },
            cpsr: CPSR { _0: () },
            cdsr: CDSR { _0: () },
            layer1: LAYER1 { 
                cr: L1CR {
                    cluten: false,
                    colken: false,
                    len: false,
                }, 
                whpcr: L1WHPCR {
                    whsppos: None,
                    whstpos: None,
                }, 
                wvpcr: L1WVPCR {
                    wvsppos: None,
                    wvstpos: None,
                },
                ckcr: L1CKCR {
                    ckred: None,
                    ckgreen: None,
                    ckblue: None,
                },
                pfcr: L1PFCR {
                    pf: None,
                },
                cacr: L1CACR {
                    consta: None,
                },
                dccr: L1DCCR {
                    dcalpha: None,
                    dcred: None,
                    dcgreen: None,
                    dcblue: None,
                },
                bfcr: L1BFCR {
                    bf1: None,
                    bf2: None,
                },
                cfbar: L1CFBAR {
                    cfbadd: None,
                },
                cfblr: L1CFBLR {
                    cfbp: None,
                    cfbll: None,
                },
                cfblnr: L1CFBLNR {
                    cfblnbr: None,
                },
                clutwr: L1CLUTWR {
                    clutadd: None,
                    red: None,
                    green: None,
                    blue: None,
                },
            },
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
    pub layer1: LAYER1,
}

impl Ltdc {
    // enable LTDC clk in RCC_CR The LTDC clock is generated from a specific PLL (PLLSAI2) RCC_PLLCFGR
    // config pixel clk 
    // RCC stuff

    // config synchronous timings for HSYNC + VSYNC
    pub fn config_timings(
        self,
        hsync_width: u16, 
        vsync_height: u16,
        hbp: u16,
        hfp: u16,
        vbp: u16,
        vfp: u16,
        screen_width: u16,
        screen_height: u16,
    ) {
        // TODO delay for stalled register access
        self.sscr
            .hsw(hsync_width - 1)
            .vsh(vsync_height - 1);

        self.bpcr
            .ahbp(hsync_width + hbp - 1)
            .avbp(vsync_height + vbp - 1);

        self.awcr
            .aaw(hsync_width + hbp + screen_width - 1)
            .aah(vsync_height + vbp + screen_height - 1);

        self.twcr
            .totalw(hsync_width + hbp + screen_width + hfp - 1)
            .totalh(vsync_height + vbp + screen_height + vfp - 1);
    }

    // config synchronous signals and clk polarity
    // LTDC_GCR

    // config background color 
    // LTDC_BCCR

    // config interrupts in 
    // LTDC_IER
    // LTDC_LIPCR

    // config l1 params
    // Lx... 

    // enable l1 and CLUT in 
    // LTDC_LxCR

    // dithering and color keying optional
    // GCR LxCKR

    // reload shadow registers to active register
    // LTDC_SRCR

    // enable LCD-TFT controller
    // LTDC_GCR
}

pub struct SSCR {
    hsw: Option<u16>,
    vsh: Option<u16>,
}

impl SSCR {
    pub(crate) fn reg(&mut self) -> &ltdc::SSCR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*LTDC::ptr()).sscr }
    }

    pub fn hsw(mut self, pclk_periods: u16) -> Self {
        self.reg().modify(|_, w| { w.hsw().bits(pclk_periods) });
        self.hsw = Some(pclk_periods);
        self
    }

    pub fn vsh(mut self, horizontal_lines: u16) -> Self {
        self.reg().modify(|_, w| { w.vsh().bits(horizontal_lines) });
        self.vsh = Some(horizontal_lines);
        self
    }
}

pub struct BPCR {
    ahbp: Option<u16>,
    avbp: Option<u16>,
}

impl BPCR {
    pub(crate) fn reg(&mut self) -> &ltdc::BPCR {
        unsafe { &(*LTDC::ptr()).bpcr }
    }

    pub fn ahbp(mut self, pclk_periods: u16) -> Self {
        self.reg().modify(|_, w| { w.ahbp().bits(pclk_periods) });
        self.ahbp = Some(pclk_periods);
        self
    }

    pub fn avbp(mut self, horizontal_lines: u16) -> Self {
        self.reg().modify(|_, w| { w.avbp().bits(horizontal_lines) });
        self.avbp = Some(horizontal_lines);
        self
    }
}

pub struct AWCR {
    aaw: Option<u16>,
    aah: Option<u16>,
}

impl AWCR {
    pub(crate) fn reg(&mut self) -> &ltdc::AWCR {
        unsafe { &(*LTDC::ptr()).awcr }
    }

    pub fn aaw(mut self, pclk_periods: u16) -> Self {
        self.reg().modify(|_, w| { w.aaw().bits(pclk_periods) });
        self.aaw = Some(pclk_periods);
        self
    }

    pub fn aah(mut self, horizontal_lines: u16) -> Self {
        self.reg().modify(|_, w| { w.aah().bits(horizontal_lines) });
        self.aah = Some(horizontal_lines);
        self
    }    
}

pub struct TWCR {
    totalw: Option<u16>,
    totalh: Option<u16>,
}

impl TWCR {
    pub(crate) fn reg(&mut self) -> &ltdc::TWCR {
        unsafe { &(*LTDC::ptr()).twcr }
    }

    pub fn totalw(mut self, pclk_periods: u16) -> Self {
        self.reg().modify(|_, w| { w.totalw().bits(pclk_periods) });
        self.totalw = Some(pclk_periods);
        self
    }

    pub fn totalh(mut self, horizontal_lines: u16) -> Self {
        self.reg().modify(|_, w| { w.totalh().bits(horizontal_lines) });
        self.totalh = Some(horizontal_lines);
        self
    }    
}

pub struct GCR {
    hspol: bool,
    vspol: bool,
    depol: bool,
    pcpol: bool,
    den: bool,
    // drw: Option<u8>,
    // dgw: Option<u8>,
    // dbw: Option<u8>,
    ltdcen: bool,
}

impl GCR {
    pub(crate) fn reg(&mut self) -> &ltdc::GCR {
        unsafe { &(*LTDC::ptr()).gcr }
    }

    pub fn hspol(mut self, positive: bool) -> Self {
        self.reg().modify(|_, w| { w.hspol().bit(positive) });
        self.hspol = positive;
        self
    }

    pub fn vspol(mut self, positive: bool) -> Self {
        self.reg().modify(|_, w| { w.vspol().bit(positive) });
        self.vspol = positive;
        self
    }

    pub fn depol(mut self, positive: bool) -> Self {
        self.reg().modify(|_, w| { w.depol().bit(positive) });
        self.depol = positive;
        self
    }

    pub fn pcpol(mut self, positive: bool) -> Self {
        self.reg().modify(|_, w| { w.pcpol().bit(positive) });
        self.pcpol = positive;
        self
    }

    pub fn den(mut self, enabled: bool) -> Self {
        self.reg().modify(|_, w| { w.den().bit(enabled)});
        self.den = enabled;
        self
    }

    pub fn ltdcen(mut self, enabled: bool) -> Self {
        self.reg().modify(|_, w| { w.ltdcen().bit(enabled) });
        self.ltdcen = enabled;
        self
    }
}

pub struct SRCR {
    vbr: bool,
    imr: bool,
}

impl SRCR {
    pub(crate) fn reg(&mut self) -> &ltdc::SRCR {
        unsafe { &(*LTDC::ptr()).srcr }
    }

    pub fn vbr(mut self, reloaded: bool) -> Self {
        self.reg().modify(|_, w| { w.vbr().bit(reloaded) });
        self.vbr = reloaded;
        self
    }

    pub fn imr(mut self, reloaded: bool) -> Self {
        self.reg().modify(|_, w| { w.imr().bit(reloaded) });
        self.imr = reloaded;
        self
    }
}

pub struct BCCR {
    bcred: Option<u8>,
    bcgreen: Option<u8>,
    bcblue: Option<u8>,
}

impl BCCR {
    pub(crate) fn reg(&mut self) -> &ltdc::BCCR {
        unsafe { &(*LTDC::ptr()).bccr }
    }

    pub fn bcred(mut self, red: u8) -> Self {
        self.reg().modify(|_, w| { w.bcred().bits(red) });
        self.bcred = Some (red);
        self
    }

    pub fn bcgreen(mut self, green: u8) -> Self {
        self.reg().modify(|_, w| { w.bcgreen().bits(green) });
        self.bcgreen = Some (green);
        self
    }

    pub fn bcblue(mut self, blue: u8) -> Self {
        self.reg().modify(|_, w| { w.bcblue().bits(blue) });
        self.bcblue = Some (blue);
        self
    }
}

pub struct IER {
    rrie: bool,
    terrie: bool,
    fuie: bool,
    lie: bool,
}

impl IER {
    pub(crate) fn reg(&mut self) -> &ltdc::IER {
        unsafe { &(*LTDC::ptr()).ier }
    }

    pub fn rrie(mut self, enabled: bool) -> Self {
        self.reg().modify(|_, w| { w.rrie().bit(enabled) });
        self.rrie = enabled;
        self
    }

    pub fn terrie(mut self, enabled: bool) -> Self {
        self.reg().modify(|_, w| { w.terrie().bit(enabled) });
        self.terrie = enabled;
        self
    }

    pub fn fuie(mut self, enabled: bool) -> Self {
        self.reg().modify(|_, w| { w.fuie().bit(enabled) });
        self.fuie = enabled;
        self
    }
    
    pub fn lie(mut self, enabled: bool) -> Self {
        self.reg().modify(|_, w| { w.lie().bit(enabled) });
        self.lie = enabled;
        self
    }
}

pub struct ISR {
    _0: (),
}

impl ISR {
    pub(crate) fn reg(&mut self) -> &ltdc::ISR {
        unsafe { &(*LTDC::ptr()).isr }
    }
}

pub struct ICR {
    _0: (),
}

impl ICR {
    pub(crate) fn reg(&mut self) -> &ltdc::ICR {
        unsafe { &(*LTDC::ptr()).icr }
    }
}

pub struct LIPCR {
    lipos: Option<u16>,
}

impl LIPCR {
    pub(crate) fn reg(&mut self) -> &ltdc::LIPCR {
        unsafe { &(*LTDC::ptr()).lipcr }
    }

    pub fn lipos(mut self, position: u16) -> Self {
        self.reg().modify(|_, w| { w.lipos().bits(position) });
        self.lipos = Some(position);
        self
    }
}

pub struct CPSR {
    _0: (),
}

impl CPSR {
    pub(crate) fn reg(&mut self) -> &ltdc::CPSR {
        unsafe { &(*LTDC::ptr()).cpsr }
    } 
}

pub struct CDSR {
    _0: (),
}

impl CDSR {
    pub(crate) fn reg(&mut self) -> &ltdc::CDSR {
        unsafe { &(*LTDC::ptr()).cdsr }
    } 
}

pub struct LAYER1 {
    cr: L1CR,
    whpcr: L1WHPCR,
    wvpcr: L1WVPCR,
    ckcr: L1CKCR,
    pfcr: L1PFCR,
    cacr: L1CACR,
    dccr: L1DCCR,
    bfcr: L1BFCR,
    cfbar: L1CFBAR,
    cfblr: L1CFBLR,
    cfblnr: L1CFBLNR,
    clutwr: L1CLUTWR,
}

impl LAYER1 {
    pub fn cr(self) -> L1CR { self.cr }
    pub fn whpcr(self) -> L1WHPCR { self.whpcr }
    pub fn wvpcr(self) -> L1WVPCR { self.wvpcr }
    pub fn ckcr(self) -> L1CKCR { self.ckcr }
    pub fn pfcr(self) -> L1PFCR { self.pfcr }
    pub fn cacr(self) -> L1CACR { self.cacr }
    pub fn dccr(self) -> L1DCCR { self.dccr }
    pub fn bfcr(self) -> L1BFCR { self.bfcr }
    pub fn cfbar(self) -> L1CFBAR { self.cfbar }
    pub fn cfblr(self) -> L1CFBLR { self.cfblr }
    pub fn cfblnr(self) -> L1CFBLNR { self.cfblnr }
    pub fn clutwr(self) -> L1CLUTWR { self.clutwr }
}

pub struct L1CR {
    cluten: bool,
    colken: bool,
    len: bool,
}

impl L1CR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::CR {
        unsafe { &(*LTDC::ptr()).layer1.cr }
    } 

    pub fn cluten(mut self, enabled: bool) -> Self {
        self.reg().modify(|_, w| { w.cluten().bit(enabled) });
        self.cluten = enabled;
        self
    }

    pub fn colken(mut self, enabled: bool) -> Self {
        self.reg().modify(|_, w| { w.colken().bit(enabled) });
        self.colken = enabled;
        self
    }

    pub fn len(mut self, enabled: bool) -> Self {
        self.reg().modify(|_, w| { w.len().bit(enabled) });
        self.len = enabled;
        self
    }
}

pub struct L1WHPCR {
    whsppos: Option<u16>,
    whstpos: Option<u16>,
}

impl L1WHPCR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::WHPCR {
        unsafe { &(*LTDC::ptr()).layer1.whpcr }
    } 

    pub fn whsppos(mut self, stop_position: u16) -> Self {
        self.reg().modify(|_, w| { w.whsppos().bits(stop_position) });
        self.whsppos = Some(stop_position);
        self
    }

    pub fn whstpos(mut self, start_position: u16) -> Self {
        self.reg().modify(|_, w| { w.whstpos().bits(start_position) });
        self.whstpos = Some(start_position);
        self
    }
}

pub struct L1WVPCR {
    wvsppos: Option<u16>,
    wvstpos: Option<u16>,
}

impl L1WVPCR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::WVPCR {
        unsafe { &(*LTDC::ptr()).layer1.wvpcr }
    } 

    pub fn wvsppos(mut self, stop_position: u16) -> Self {
        self.reg().modify(|_, w| { w.wvsppos().bits(stop_position) });
        self.wvsppos = Some(stop_position);
        self
    }

    pub fn wvstpos(mut self, start_position: u16) -> Self {
        self.reg().modify(|_, w| { w.wvstpos().bits(start_position) });
        self.wvstpos = Some(start_position);
        self
    }
}

pub struct L1CKCR {
    ckred: Option<u8>,
    ckgreen: Option<u8>,
    ckblue: Option<u8>,
}

impl L1CKCR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::CKCR {
        unsafe { &(*LTDC::ptr()).layer1.ckcr }
    } 

    pub fn ckred(mut self, red: u8) -> Self {
        self.reg().modify(|_, w| { w.ckred().bits(red) });
        self.ckred = Some(red);
        self
    }

    pub fn ckgreen(mut self, green: u8) -> Self {
        self.reg().modify(|_, w| { w.ckgreen().bits(green) });
        self.ckgreen = Some(green);
        self
    }
    
    pub fn ckblue(mut self, blue: u8) -> Self {
        self.reg().modify(|_, w| { w.ckblue().bits(blue) });
        self.ckblue = Some(blue);
        self
    }
}

pub struct L1PFCR {
    pf: Option<ltdc::layer::pfcr::PF_A>
}

impl L1PFCR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::PFCR {
        unsafe { &(*LTDC::ptr()).layer1.pfcr }
    } 

    pub fn pf(mut self, pixel_format: ltdc::layer::pfcr::PF_A) {
        self.reg().modify(|_, w| { w.pf().variant(pixel_format) });
        self.pf = Some(pixel_format);
    }
}

pub struct L1CACR {
    consta: Option<u8>,
}

impl L1CACR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::CACR {
        unsafe { &(*LTDC::ptr()).layer1.cacr }
    } 

    pub fn consta(mut self, alpha: u8) {
        self.reg().modify(|_, w| { w.consta().bits(alpha) });
        self.consta = Some(alpha);
    }
}

pub struct L1DCCR {
    dcalpha: Option<u8>,
    dcred: Option<u8>,
    dcgreen: Option<u8>,
    dcblue: Option<u8>,
}

impl L1DCCR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::DCCR {
        unsafe { &(*LTDC::ptr()).layer1.dccr }
    } 

    pub fn dcalpha(mut self, alpha: u8) -> Self {
        self.reg().modify(|_, w| { w.dcalpha().bits(alpha) });
        self.dcalpha = Some(alpha);
        self
    }

    pub fn dcred(mut self, red: u8) -> Self {
        self.reg().modify(|_, w| { w.dcred().bits(red) });
        self.dcred = Some(red);
        self
    }
    
    pub fn dcgreen(mut self, green: u8) -> Self {
        self.reg().modify(|_, w| { w.dcgreen().bits(green) });
        self.dcgreen = Some(green);
        self
    }
    
    pub fn dcblue(mut self, blue: u8) -> Self {
        self.reg().modify(|_, w| { w.dcblue().bits(blue) });
        self.dcblue = Some(blue);
        self
    }
}

pub struct L1BFCR {
    bf1: Option<ltdc::layer::bfcr::BF1_A>,
    bf2: Option<ltdc::layer::bfcr::BF2_A>,
}

impl L1BFCR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::BFCR {
        unsafe { &(*LTDC::ptr()).layer1.bfcr }
    } 

    pub fn bf1(mut self, blending_factor: ltdc::layer::bfcr::BF1_A) -> Self {
        self.reg().modify(|_, w| { w.bf1().variant(blending_factor) });
        self.bf1 = Some(blending_factor);
        self
    }

    pub fn bf2(mut self, blending_factor: ltdc::layer::bfcr::BF2_A) -> Self {
        self.reg().modify(|_, w| { w.bf2().variant(blending_factor) });
        self.bf2 = Some(blending_factor);
        self
    }
}

pub struct L1CFBAR {
    cfbadd: Option<u32>,
}

impl L1CFBAR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::CFBAR {
        unsafe { &(*LTDC::ptr()).layer1.cfbar }
    } 

    pub fn cfbadd(mut self, start_address: u32) {
        self.reg().modify(|_, w| { w.cfbadd().bits(start_address) });
        self.cfbadd = Some(start_address);
    }
}

pub struct L1CFBLR {
    cfbp: Option<u16>,
    cfbll: Option<u16>,
}

impl L1CFBLR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::CFBLR {
        unsafe { &(*LTDC::ptr()).layer1.cfblr }
    } 

    pub fn cfbp(mut self, buffer_pitch: u16) -> Self {
        self.reg().modify(|_, w| { w.cfbp().bits(buffer_pitch) });
        self.cfbp = Some(buffer_pitch);
        self
    }

    pub fn cfbll(mut self, buffer_length: u16) -> Self {
        self.reg().modify(|_, w| { w.cfbll().bits(buffer_length) });
        self.cfbll = Some(buffer_length);
        self
    }
}

pub struct L1CFBLNR {
    cfblnbr: Option<u16>,
}

impl L1CFBLNR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::CFBLNR {
        unsafe { &(*LTDC::ptr()).layer1.cfblnr }
    } 

    pub fn cfblnbr(mut self, num_lines: u16) -> Self {
        self.reg().modify(|_, w| { w.cfblnbr().bits(num_lines) });
        self.cfblnbr = Some(num_lines);
        self
    }
}

pub struct L1CLUTWR {
    clutadd: Option<u8>,
    red: Option<u8>,
    green: Option<u8>,
    blue: Option<u8>,
}

impl L1CLUTWR {
    pub(crate) fn reg(&mut self) -> &ltdc::layer::CLUTWR {
        unsafe { &(*LTDC::ptr()).layer1.clutwr }
    } 

    pub fn clutadd(mut self, address: u8) -> Self {
        self.reg().write(|w| { w.clutadd().bits(address) });
        self.clutadd = Some(address);
        self
    }

    pub fn red(mut self, red: u8) -> Self {
        self.reg().write(|w| { w.red().bits(red) });
        self.red = Some(red);
        self
    }

    pub fn green(mut self, green: u8) -> Self {
        self.reg().write(|w| { w.green().bits(green) });
        self.green = Some(green);
        self
    }

    pub fn blue(mut self, blue: u8) -> Self {
        self.reg().write(|w| { w.blue().bits(blue) });
        self.blue = Some(blue);
        self
    }
}