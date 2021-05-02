use defmt;
use crate::stm32::{
    ltdc,
    LTDC,
};
use crate::gpio::{
    Output,
    PC1,
    PushPull,
};
use crate::hal::digital::v2::OutputPin;

pub trait LtdcExt {
    fn constrain(self, pclk2_freq: u32, pixel_clk_freq: u32, display_pwr: DisplayPwr) -> Ltdc;
}

impl LtdcExt for LTDC {
    fn constrain(self, pclk2_freq: u32, pixel_clk_freq: u32, display_pwr: DisplayPwr) -> Ltdc {
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
            srcr: SRCR { _0: () },
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
            layer1: LAYER1 { clut_filled: false },
            pwr_pins: LtdcPwrPins {
                display_pwr,
            },
            pclk2_freq,
            pixel_clk_freq,
        }
    }
}

pub struct LtdcPwrPins<D: OutputPin>{
    display_pwr: D,
}

impl LtdcPwrPins<DisplayPwr> {
    pub fn display_pwr_on(&mut self) {
        self.display_pwr.set_high().ok();
    }

    pub fn display_pwr_off(&mut self) {
        self.display_pwr.set_low().ok();
    }
}

pub type DisplayPwr = PC1<Output<PushPull>>;

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
    // power pins
    pub pwr_pins: LtdcPwrPins<DisplayPwr>,
    // pclk2 freq as configured in rcc
    pclk2_freq: u32,
    // pixel clock freq as configured in rcc.pllsai2
    pixel_clk_freq: u32,
}

impl Ltdc {
    pub fn config_timings(
        &self,
        hsync_width: u16, 
        vsync_height: u16,
        hbp: u16,
        hfp: u16,
        vbp: u16,
        vfp: u16,
        screen_width: u16,
        screen_height: u16,
    ) {
        let ltdc = unsafe { &*LTDC::ptr() };

        // TODO update struct values ???
        ltdc.sscr.modify(|_, w| { w
            .hsw().bits(hsync_width - 1)
            .vsh().bits(vsync_height - 1)
        });
        stall_after_modify(self.pclk2_freq, self.pixel_clk_freq);

        ltdc.bpcr.modify(|_, w| { w
            .ahbp().bits(hsync_width + hbp - 1)
            .avbp().bits(vsync_height + vbp - 1)
        });
        stall_after_modify(self.pclk2_freq, self.pixel_clk_freq);

        ltdc.awcr.modify(|_, w| { w
            .aaw().bits(hsync_width + hbp + screen_width - 1)
            .aah().bits(vsync_height + vbp + screen_height - 1)
        });
        stall_after_modify(self.pclk2_freq, self.pixel_clk_freq);

        ltdc.twcr.modify(|_, w| { w
            .totalw().bits(hsync_width + hbp + screen_width + hfp - 1)
            .totalh().bits(vsync_height + vbp + screen_height + vfp - 1)
        });
        stall_after_modify(self.pclk2_freq, self.pixel_clk_freq);

    }

    pub fn reload_shadow_reg(&self) {
        self.srcr.set_vbr();
    }

    pub fn wait_for_frame(&self) {
        let cdsr = self.cdsr.reg().read();

        while cdsr.vsyncs().is_active() {};
        while !cdsr.vsyncs().is_active() {};
    }
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

    pub fn read_reg(&mut self) -> u32 {
        self.reg().read().bits()
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
    pub fn update_reg(&self) {
        let gcr = unsafe { &(*LTDC::ptr()).gcr };

        gcr.modify(|_, w| { w
            .hspol().bit(self.hspol)
            .vspol().bit(self.vspol)
            .depol().bit(self.depol)
            .pcpol().bit(self.pcpol)
            .den().bit(self.den)
            .ltdcen().bit(self.ltdcen)
        });
    }

    pub fn hspol(&mut self, positive: bool) -> &mut Self {
        self.hspol = positive;
        self
    }

    pub fn vspol(&mut self, positive: bool) -> &mut Self {
        self.vspol = positive;
        self
    }

    pub fn depol(&mut self, positive: bool) -> &mut Self {
        self.depol = positive;
        self
    }

    pub fn pcpol(&mut self, positive: bool) -> &mut Self {
        self.pcpol = positive;
        self
    }

    pub fn den(&mut self, enabled: bool) -> &mut Self {
        self.den = enabled;
        self
    }

    pub fn ltdcen(&mut self, enabled: bool) -> &mut Self {
        self.ltdcen = enabled;
        self
    }
}

pub struct SRCR {
    _0: (),
}

impl SRCR {
    pub(crate) fn reg(&self) -> &ltdc::SRCR {
        unsafe { &(*LTDC::ptr()).srcr }
    }

    pub fn set_vbr(&self) {
        self.reg().modify(|_, w| {
            w.vbr().set_bit()
        });
    }

    pub fn set_imr(&self) {
        self.reg().modify(|_, w| {
            w.imr().set_bit()
        })
    }
}

pub struct BCCR {
    bcred: Option<u8>,
    bcgreen: Option<u8>,
    bcblue: Option<u8>,
}

impl BCCR {
    pub fn update_reg(&self) {
        let bccr = unsafe { &(*LTDC::ptr()).bccr };

        bccr.modify(|_, w| { w
            .bcred().bits(self.bcred.unwrap_or(0))
            .bcgreen().bits(self.bcgreen.unwrap_or(0))
            .bcblue().bits(self.bcblue.unwrap_or(0))
        });
    }

    pub fn bcred(&mut self, red: u8) -> &mut Self {
        self.bcred = Some (red);
        self
    }

    pub fn bcgreen(&mut self, green: u8) -> &mut Self {
        self.bcgreen = Some (green);
        self
    }

    pub fn bcblue(&mut self, blue: u8) -> &mut Self {
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
    pub fn update_reg(&self) {
        let ier = unsafe { &(*LTDC::ptr()).ier };

        ier.modify(|_, w| { w
            .rrie().bit(self.rrie)
            .terrie().bit(self.terrie)
            .fuie().bit(self.fuie)
            .lie().bit(self.lie)
        });
    }

    pub fn rrie(&mut self, enabled: bool) -> &mut Self {
        self.rrie = enabled;
        self
    }

    pub fn terrie(&mut self, enabled: bool) -> &mut Self {
        self.terrie = enabled;
        self
    }

    pub fn fuie(&mut self, enabled: bool) -> &mut Self {
        self.fuie = enabled;
        self
    }
    
    pub fn lie(&mut self, enabled: bool) -> &mut Self {
        self.lie = enabled;
        self
    }
}

pub struct ISR {
    _0: (),
}

impl ISR {
    #[allow(dead_code)]
    pub(crate) fn reg(&mut self) -> &ltdc::ISR {
        unsafe { &(*LTDC::ptr()).isr }
    }
}

pub struct ICR {
    _0: (),
}

impl ICR {
    #[allow(dead_code)]
    pub(crate) fn reg(&mut self) -> &ltdc::ICR {
        unsafe { &(*LTDC::ptr()).icr }
    }
}

pub struct LIPCR {
    lipos: Option<u16>,
}

impl LIPCR {
    pub fn update_reg(&self) {
        let lipcr = unsafe { &(*LTDC::ptr()).lipcr };

        lipcr.modify(|_, w| { 
            w.lipos().bits(self.lipos.unwrap_or(0)) 
        });
    }

    pub fn lipos(mut self, position: u16) -> Self {
        self.lipos = Some(position);
        self
    }
}

pub struct CPSR {
    _0: (),
}

impl CPSR {
    #[allow(dead_code)]
    pub(crate) fn reg(&mut self) -> &ltdc::CPSR {
        unsafe { &(*LTDC::ptr()).cpsr }
    } 
}

pub struct CDSR {
    _0: (),
}

impl CDSR {
    #[allow(dead_code)]
    pub fn reg(&self) -> &ltdc::CDSR {
        unsafe { &(*LTDC::ptr()).cdsr }
    } 
}

pub struct LAYER1 {
    clut_filled: bool,
}

impl LAYER1 {
    pub fn config_layer(
        &self,
        layer_width: u16,
        layer_height: u16,
        pixel_format: ltdc::layer::pfcr::PF_A,
        buffer_start_address: u32,
    ) {
        let ltdc = unsafe { &*LTDC::ptr() };

        ltdc.layer1.whpcr.modify(|_, w| { w
            .whstpos().bits(0)
            .whsppos().bits(layer_width)
        });

        ltdc.layer1.wvpcr.modify(|_, w| { w
            .wvstpos().bits(0)
            .wvsppos().bits(layer_height)
        });
        
        ltdc.layer1.pfcr.modify(|_, w| { w.pf().variant(pixel_format) });

        ltdc.layer1.cfbar.modify(|_, w| { 
            w.cfbadd().bits(buffer_start_address) 
        });

        let cfbp = layer_width * match pixel_format {
            ltdc::layer::pfcr::PF_A::ARGB8888 => 4,
            ltdc::layer::pfcr::PF_A::RGB888 => 3,
            ltdc::layer::pfcr::PF_A::RGB565 => 2,
            ltdc::layer::pfcr::PF_A::ARGB1555 => 2,
            ltdc::layer::pfcr::PF_A::ARGB4444 => 2,
            ltdc::layer::pfcr::PF_A::L8 => 1,
            ltdc::layer::pfcr::PF_A::AL44 => 1,
            ltdc::layer::pfcr::PF_A::AL88 => 2,
        };

        ltdc.layer1.cfblr.modify(|_, w| { w
            .cfbp().bits(cfbp)
            .cfbll().bits(cfbp + 3)
        });

        ltdc.layer1.cfblnr.modify(|_, w| {
            w.cfblnbr().bits(layer_height)
        });
    }

    pub fn fill_clut_l8(&mut self) {
        let ltdc = unsafe { &*LTDC::ptr() };
        let mut color: u32 = 0;

        while color <= u8::MAX as u32 {
            let red: u32 = (((0b11100000 & color) >> 5)  * 255) / 7;
            let green: u32 = (((0b00011100 & color) >> 2) * 255) / 7;
            let blue: u32 = ((0b00000011 & color) * 255) / 3;

            // TODO figure why this causes issues
            ltdc.layer1.clutwr.write(|w| { w
                .clutadd().bits(color as u8)
                .red().bits(red as u8)
                .green().bits(green as u8)
                .blue().bits(blue as u8)
            });

            color += 1;
        }

        self.clut_filled = true;
    }

    pub fn enable_layer(&self) {
        let ltdc = unsafe { &*LTDC::ptr() };

        ltdc.layer1.cr.modify(|_, w| { w
            .cluten().bit(self.clut_filled)
            .len().set_bit()
        });
    }
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

/// stalls for APB2 bus according to LTDC modifcation timings stated in TRM 29.4.3
pub(crate) fn stall_after_modify(pclk2_freq: u32, pixel_clk_freq: u32) {
    let pclk2_cycles: u32 = 7;
    let pixel_clk_cycles: u32 = 5;
    let total_cycles: u32 = pclk2_cycles + pixel_clk_cycles * (pclk2_freq / pixel_clk_freq + 1);
    cortex_m::asm::delay(total_cycles);
}