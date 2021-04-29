#![no_std]

use stm32l4p5_hal as hal;
use hal::{flash::{self, FlashExt}, gpio::{self, GpioExt, Output, PB7, PC1, PushPull}, ltdc::{Ltdc, LtdcExt}, pac::{self, Peripherals}, prelude::{_embedded_hal_digital_v2_OutputPin, _embedded_hal_digital_v2_StatefulOutputPin}, pwr::{Pwr, PwrExt}, rcc::{Clocks, Rcc, RccExt, PllConfig, PllDivider, PllSource}};
use lcd::Lcd;

pub struct Board {
    rcc: Rcc,
    flash: flash::Parts,
    pwr: Pwr,
    ltdc: Ltdc,
}

impl Board {
    pub fn init_system_clocks(&mut self) -> Clocks {
        // VCO_in = PLL_in / pllm | 2.66MhHz <= VCO_in <= 8MHz
        // 8Mhz = 16MHz / pllm
        let pllm: u8 = 2;

        // VCO_out = VCO_in * plln | 64MHz <= VCO_out <= 344MHz
        // 240MHz = 8MHz * plln
        let plln: u8 = 30;

        // output_clk = VCO_out / pllp_div | output_clk = 120MHz max
        // 120MHz = 240MHz / pllp_div
        let pllp_div = PllDivider::Div2;

        let pll_config = PllConfig::new(
            pllm,
            plln,
            pllp_div
        );

        self.rcc
            .cfgr
            .pll_source(PllSource::HSI16)
            .sysclk_with_pll(hal::rcc::MAX_BOOST_SYSCLK, pll_config)
            .hclk(hal::rcc::MAX_BOOST_SYSCLK)
            .pclk1(hal::rcc::MAX_BOOST_SYSCLK) // don't know if needed
            .pclk2(hal::rcc::MAX_BOOST_SYSCLK)
            .freeze(&mut self.flash.acr, &mut self.pwr)
    }

    pub fn init_ltdc_clocks(&mut self) {
        // VCO_in = PLL_in / sai2m_div | 2.66MhHz <= VCO_in <= 8MHz
        // 8Mhz = 16MHz / sai2m_div
        let sai2m_div: u8 = 2;
        
        // VCO_out = VCO_in * sai2n_mult | 64MHz <= VCO_out <= 344MHz
        // 96MHz = 8MHz * sai2n_mult
        let sai2n_mult: u8 = 12;

        // LCDCLK = VCO_out / lcd_div | LCDCLK <= 12MHz (hardware restriction)
        // 12MHz = 96Mhz / lcd_div
        let lcd_div = pac::rcc::pllsai2cfgr::PLLSAI2R_A::DIV8;

        self.rcc.pllsai2cfgr
            .sai2m_div(sai2m_div)
            .sai2n_mult(sai2n_mult)
            .lcd_div(lcd_div)
            .freeze();
            
        self.rcc.apb2.enr().modify(|_, w| { w.ltdcen().set_bit() });
    }

    pub fn init_ltdc(&mut self, buffer_start_address: u32) {
        // LTDC configuration
        self.ltdc.config_timings(
            Lcd::HSYNC_WIDTH,
            Lcd::VSYNC_HEIGHT,
            Lcd::HBP,
            Lcd::HFP,
            Lcd::VBP,
            Lcd::VFP,
            Lcd::SCREEN_WIDTH,
            Lcd::SCREEN_HEIGHT,
        );

        // config synchronous signals and clk polarity
        self.ltdc.gcr
            .hspol(false)
            .vspol(false)
            .depol(false)
            .pcpol(true)
            .den(false)
            .update_reg();

        // config background color 
        self.ltdc.bccr
            .bcred(0x00)
            .bcgreen(0x00)
            .bcblue(0x00)
            .update_reg();

        // // config interrupts
        // self.self.ltdc.ier.lie(true).update_reg();
        // self.self.ltdc.lipcr.lipos(Lcd::SCREEN_HEIGHT).update_reg();
        
        self.ltdc.layer1.config_layer(
            Lcd::SCREEN_WIDTH,
            Lcd::SCREEN_HEIGHT,
            pac::ltdc::layer::pfcr::PF_A::L8,
            buffer_start_address
        );

        self.ltdc.layer1.fill_clut_l8();
        self.ltdc.layer1.enable_layer();
        self.ltdc.srcr.set_imr();
        self.ltdc.gcr.ltdcen(true).update_reg();
    }

    pub fn new() -> Board {
        let peripherals = hal::pac::Peripherals::take().unwrap();
        let mut rcc = peripherals.RCC.constrain();
        let flash = peripherals.FLASH.constrain();
        let ltdc = peripherals.LTDC.constrain();
        let pwr = peripherals.PWR.constrain(&mut rcc.apb1r1);

        // forced to initialized GPIO here because the HAL I copied sucks
        let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb2);
        let mut gpioc = peripherals.GPIOC.split(&mut rcc.ahb2);
        let mut gpiod = peripherals.GPIOD.split(&mut rcc.ahb2);
        let mut gpioe = peripherals.GPIOE.split(&mut rcc.ahb2);
        let mut gpiof = peripherals.GPIOF.split(&mut rcc.ahb2);
        let mut gpiog = peripherals.GPIOG.split(&mut rcc.ahb2);

        // GPIO config
        let _display_on = gpioc
        .pc1
        .into_push_pull_output_with_state(
            &mut gpioc.moder,
            &mut gpioc.otyper,
            gpio::State::High
        );

        let _pixel_clk = gpioa
        .pa4
        .into_push_pull_output_with_state(
            &mut gpioa.moder,
            &mut gpioa.otyper,
            gpio::State::Low
        )
        .into_af11(&mut gpioa.moder, &mut gpioa.afrl)
        .set_speed(gpio::Speed::VeryHigh);

        let _de = gpioc
        .pc0
        .into_push_pull_output_with_state(
            &mut gpioc.moder,
            &mut gpioc.otyper,
            gpio::State::Low
        )
        .into_af11(&mut gpioc.moder, &mut gpioc.afrl)
        .set_speed(gpio::Speed::VeryHigh);

        let _hsync = gpioe
        .pe0
        .into_push_pull_output_with_state(
            &mut gpioe.moder,
            &mut gpioe.otyper,
            gpio::State::Low
        )
        .into_af11(&mut gpioe.moder, &mut gpioe.afrl)
        .set_speed(gpio::Speed::VeryHigh);

        let _vsync = gpioe
        .pe1
        .into_push_pull_output_with_state(
            &mut gpioe.moder,
            &mut gpioe.otyper,
            gpio::State::Low
        )
        .into_af11(&mut gpioe.moder, &mut gpioe.afrl)
        .set_speed(gpio::Speed::VeryHigh);

        let _r0 = gpiog
        .pg13
        .into_push_pull_output_with_state(
            &mut gpiog.moder, 
            &mut gpiog.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiog.moder, &mut gpiog.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _r1 = gpiog
        .pg14
        .into_push_pull_output_with_state(
            &mut gpiog.moder, 
            &mut gpiog.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiog.moder, &mut gpiog.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _r2 = gpioe
        .pe15
        .into_push_pull_output_with_state(
            &mut gpioe.moder, 
            &mut gpioe.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpioe.moder, &mut gpioe.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _r3 = gpiod
        .pd8
        .into_push_pull_output_with_state(
            &mut gpiod.moder, 
            &mut gpiod.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiod.moder, &mut gpiod.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _r4 = gpiod
        .pd9
        .into_push_pull_output_with_state(
            &mut gpiod.moder, 
            &mut gpiod.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiod.moder, &mut gpiod.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _r5 = gpiod
        .pd10
        .into_push_pull_output_with_state(
            &mut gpiod.moder, 
            &mut gpiod.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiod.moder, &mut gpiod.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _r6 = gpiod
        .pd11
        .into_push_pull_output_with_state(
            &mut gpiod.moder, 
            &mut gpiod.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiod.moder, &mut gpiod.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _r7 = gpiod
        .pd12
        .into_push_pull_output_with_state(
            &mut gpiod.moder, 
            &mut gpiod.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiod.moder, &mut gpiod.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _g0 = gpiof
        .pf14
        .into_push_pull_output_with_state(
            &mut gpiof.moder, 
            &mut gpiof.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiof.moder, &mut gpiof.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _g1 = gpiof
        .pf15
        .into_push_pull_output_with_state(
            &mut gpiof.moder, 
            &mut gpiof.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiof.moder, &mut gpiof.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _g2 = gpioe
        .pe9
        .into_push_pull_output_with_state(
            &mut gpioe.moder, 
            &mut gpioe.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpioe.moder, &mut gpioe.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _g3 = gpioe
        .pe10
        .into_push_pull_output_with_state(
            &mut gpioe.moder, 
            &mut gpioe.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpioe.moder, &mut gpioe.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _g4 = gpioe
        .pe11
        .into_push_pull_output_with_state(
            &mut gpioe.moder, 
            &mut gpioe.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpioe.moder, &mut gpioe.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _g5 = gpioe
        .pe12
        .into_push_pull_output_with_state(
            &mut gpioe.moder, 
            &mut gpioe.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpioe.moder, &mut gpioe.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _g6 = gpioe
        .pe13
        .into_push_pull_output_with_state(
            &mut gpioe.moder, 
            &mut gpioe.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpioe.moder, &mut gpioe.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _g7 = gpioe
        .pe14
        .into_push_pull_output_with_state(
            &mut gpioe.moder, 
            &mut gpioe.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpioe.moder, &mut gpioe.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _b0 = gpiof
        .pf12
        .into_push_pull_output_with_state(
            &mut gpiof.moder, 
            &mut gpiof.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiof.moder, &mut gpiof.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _b1 = gpiof
        .pf13
        .into_push_pull_output_with_state(
            &mut gpiof.moder, 
            &mut gpiof.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiof.moder, &mut gpiof.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _b2 = gpiod
        .pd14
        .into_push_pull_output_with_state(
            &mut gpiod.moder, 
            &mut gpiod.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiod.moder, &mut gpiod.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _b3 = gpiod
        .pd15
        .into_push_pull_output_with_state(
            &mut gpiod.moder, 
            &mut gpiod.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiod.moder, &mut gpiod.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _b4 = gpiod
        .pd0
        .into_push_pull_output_with_state(
            &mut gpiod.moder, 
            &mut gpiod.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiod.moder, &mut gpiod.afrl)
        .set_speed(gpio::Speed::VeryHigh);

        let _b5 = gpiod
        .pd1
        .into_push_pull_output_with_state(
            &mut gpiod.moder, 
            &mut gpiod.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpiod.moder, &mut gpiod.afrl)
        .set_speed(gpio::Speed::VeryHigh);

        let _b6 = gpioc
        .pc8
        .into_push_pull_output_with_state(
            &mut gpioc.moder, 
            &mut gpioc.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpioc.moder, &mut gpioc.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        let _b7 = gpioa
        .pa8
        .into_push_pull_output_with_state(
            &mut gpioa.moder, 
            &mut gpioa.otyper, 
            gpio::State::Low
        )
        .into_af11(&mut gpioa.moder, &mut gpioa.afrh)
        .set_speed(gpio::Speed::VeryHigh);

        Board {
            rcc,
            flash,
            pwr,
            ltdc,
        }
    }
}