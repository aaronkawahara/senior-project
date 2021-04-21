#![no_std]
// Board/peripheral initialization
// note: create new folder for each peripheral/system ex) lcd, buttons, etc.
use stm32l4p5_hal as hal;
use lcd::Lcd;

pub struct Board {
    lcd: Lcd,
}

// 120Mhz is achievable if PWR_CR5 reg is set to boost mode, otherwise 80Mhz is max
pub const MAX_SYSCLK: u32 = 120_000_000;

impl Board {
    pub fn init(/*peripherals*/) -> Self {
        let peripherals = hal::pac::Peripherals::take().unwrap();
        // let mut rcc = peripherals.RCC.constrain();
        // let mut flash = peripherals.FLASH.constrain();
        // let mut pwr = peripherals.PWR.constrain(&mut rcc.apb1r1);

        // pll = (src_clk / input_divider) * multiplier / output_divider
        // multiplier = pll * input_divider * output_divider / src_clk
        // mutlipier = 80Mhz(max pll) * 1 * 2 / 16Mhz (max pll input)
        // multiplier = 160Mhz / 16Mhz = 10
        // let input_divider = 1;
        // let multiplier = 10;
        // let pll_config = rcc::PllConfig::new(
        //     input_divider,
        //     multiplier,
        //      rcc::PllDivider::Div2
        // );

        // let clocks = rcc
        //     .cfgr
        //     .msi(rcc::MsiFreq::RANGE48M)
        //     .pll_source(rcc::PllSource::HSI16)
        //     .sysclk_with_pll(MAX_SYSCLK, pll_config)
        //     .pclk1(MAX_SYSCLK)
        //     .pclk2(MAX_SYSCLK)
        //     .freeze(&mut flash.acr, &mut pwr);

        // let pllsai1 = stm32::rcc::pllsai2cfgr::PLL;

        // Board::init_lcd(&mut rcc);

        Self {
            lcd: Lcd
        }
    }

    /*fn init_rcc(rcc: &mut rcc::Rcc, flash: &mut flash::Parts, pwr: &mut Pwr) {
        let input_divider = 1;
        let multiplier = 10;
        let pll_config = rcc::PllConfig::new(
            input_divider,
            multiplier,
             rcc::PllDivider::Div2
        );

        let clocks = rcc
            .cfgr
            .msi(rcc::MsiFreq::RANGE48M)
            .pll_source(rcc::PllSource::HSI16)
            .sysclk_with_pll(MAX_SYSCLK, pll_config)
            .pclk1(MAX_SYSCLK)
            .pclk2(MAX_SYSCLK)
            .freeze(&mut flash.acr, &mut pwr);

        // let pllsai1 = stm32::rcc::pllsai1cfgr();
    }

    fn init_pwr(pwr: &mut Pwr) {
        // set PWR_CR5 to boost mode
    }

    fn init_lcd(rcc: &mut rcc::Rcc) {
        // enable LTDC clk in RCC_CR
        // The LTDC clock. The LTDC clock is generated from a specific PLL (PLLSAI2) RCC_PLLCFGR

        // config pixel clk
        // config synchronous timings for HSYNC + VSYNC
        // config synchronous signals and clk polarity in LTDC_GCR reg
        // config background color
        // config interrupts in LTDC_IER & LTDC_LIPCR
        // config l1 params
        // enable l1 and CLUT in LTDC_LxCR reg
        // dithering and color keying optional
        // reload shadow registers to active register through LTDC_SRCR
        // enable LCD-TFT controller in LTDC_GCR
    } */
}