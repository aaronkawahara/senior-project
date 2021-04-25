#![no_std]
// Board/peripheral initialization
// note: create new folder for each peripheral/system ex) lcd, buttons, etc.
use stm32l4p5_hal as hal;
use lcd::Lcd;

pub struct Board {
    lcd: Lcd,
}

impl Board {
    pub fn init(/*peripherals*/) -> Self {
        let peripherals = hal::pac::Peripherals::take().unwrap();
        let mut rcc = peripherals.RCC.constrain();
        let mut flash = peripherals.FLASH.constrain();
        let mut pwr = peripherals.PWR.constrain(&mut rcc.apb1r1);

        // Clock Configuration
        // pll = (src_clk / input_divider) * multiplier / output_divider
        // multiplier = pll * input_divider * output_divider / src_clk
        // mutlipier = 120Mhz(max pll) * 1 * 2 / 16Mhz (max pll input)
        // multiplier = 240Mhz / 16Mhz
        // multiplier = 15
        let input_divider = 1;
        let multiplier = 15;
        let pll_config = rcc::PllConfig::new(
            input_divider,
            multiplier,
            rcc::PllDivider::Div2
        );

        let clocks = rcc
            .cfgr
            .pll_source(rcc::PllSource::HSI16)
            .sysclk_with_pll(hal::rcc::MAX_BOOST_SYSCLK, pll_config)
            .hclk(hal::rcc::MAX_BOOST_SYSCLK)
            .pclk1(hal::rcc::MAX_BOOST_SYSCLK) // don't know if needed
            .pclk2(hal::rcc::MAX_BOOST_SYSCLK)
            .freeze(&mut flash.acr, &mut pwr);

        // VCO_in = PLL_in / sai2m_div | 2.66MhHz <= VCO_in <= 8MHz
        // 8Mhz = 16MHz / sai2m_div
        // sai2m_div = 2
        //
        // VCO_out = VCO_in * sai2n_mult | 64MHz <= VCO_out <= 344MHz
        // 96MHz = 8MHz * sai2n_mult
        // sai2n_mult = 12
        //
        // LCDCLK = VCO_out / lcd_div | LCDCLK <= 12MHz (hardware restriction)
        // 12MHz = 96Mhz / lcd_div
        // lcd_div = 8
        rcc.pllsai2cfgr
            .sai2m_div(2)
            .sai2n_mult(12)
            .lcd_div(rcc::pllsai2cfgr::PLLSAI2R_A::DIV8)
            .freeze();
        
        // GPIO Configuration


        // LCD Configuration
        
        Self {
            lcd: Lcd
        }
    }

    /*
    fn init_lcd(rcc: &mut rcc::Rcc) {
        
    } */
}