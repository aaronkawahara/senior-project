#![no_std]
#![feature(option_result_contains)]

use stm32hal::{
    dma2d::{Dma2d, Dma2dExt},
    flash::{self, FlashExt},
    gpio::{self, GpioExt},
    ltdc::{Ltdc, LtdcExt},
    pac,
    pwr::{Pwr, PwrExt},
    rcc::{self, Clocks, PllConfig, PllDivider, PllSource, Rcc, RccExt},
};
use stm32l4p5_hal as stm32hal;

pub mod input;
use crate::input::{DownPin, Inputs, LeftPin, RightPin, UpPin};

pub struct Board {
    rcc: Rcc,
    flash: flash::Parts,
    pwr: Pwr,
    ltdc: Ltdc,
    input: Option<Inputs>,
    dma2d: Option<Dma2d>,
}

impl Board {
    pub fn init_system_clocks(&mut self) -> Clocks {
        defmt::info!("working in board");

        // VCO_in = PLL_in / pllm | 2.66MhHz <= VCO_in <= 8MHz
        // 8Mhz = 16MHz / pllm
        let pll_m: u8 = 2;

        // VCO_out = VCO_in * plln | 64MHz <= VCO_out <= 344MHz
        // 240MHz = 8MHz * plln
        let pll_n: u8 = 30;

        // output_clk = VCO_out / pllp_div | output_clk = 120MHz max
        // 120MHz = 240MHz / pllp_div
        let pll_p = PllDivider::Div2;

        let pll_config = PllConfig::new(pll_m, pll_n, pll_p);

        self.rcc
            .cfgr
            .pll_source(PllSource::HSI16)
            .sysclk_with_pll(rcc::MAX_BOOST_SYSCLK, pll_config)
            .hclk(rcc::MAX_BOOST_SYSCLK)
            .pclk1(rcc::MAX_BOOST_SYSCLK) // don't know if needed
            .pclk2(rcc::MAX_BOOST_SYSCLK)
            .freeze(&mut self.flash.acr, &mut self.pwr)
    }

    fn init_ltdc_clocks(&mut self) {
        // // VCO_in = PLL_in / sai2m_div | 2.66MhHz <= VCO_in <= 8MHz
        // // 8Mhz = 16MHz / sai2m_div
        // let sai2m_div: u8 = 2;

        // // VCO_out = VCO_in * sai2n_mult | 64MHz <= VCO_out <= 344MHz
        // // 192MHz = 8MHz * sai2n_mult
        // let sai2n_mult: u8 = 24;

        // // PLLSAI2_R = VCO_out / lcd_div | target = 18MHz (hardware restriction)
        // // 24MHz = 192MHz / lcd_div
        // let lcd_div = pac::rcc::pllsai2cfgr::PLLSAI2R_A::DIV8;

        // // LTDC_CLK = PLLSAI2_R / divr | LTDC_CLK <= 12Mhz (hardware restriction)
        // let sai2_divr: u8 = 0;

        let sai2m_div: u8 = 2;
        let sai2n_mult: u8 = 10;
        let lcd_div = pac::rcc::pllsai2cfgr::PLLSAI2R_A::DIV8;
        let sai2_divr: u8 = 0;

        self.rcc
            .pllsai2cfgr
            .sai2m_div(sai2m_div)
            .sai2n_mult(sai2n_mult)
            .lcd_div(lcd_div)
            .sai2_divr(sai2_divr)
            .lcd_enabled(true)
            .freeze();
    }

    pub fn init_ltdc(&mut self, first_buffer_element: &u8) {
        self.init_ltdc_clocks();

        self.ltdc.config_timings(
            lcd::HSYNC_WIDTH,
            lcd::VSYNC_HEIGHT,
            lcd::HBP,
            lcd::HFP,
            lcd::VBP,
            lcd::VFP,
            lcd::SCREEN_WIDTH_U16,
            lcd::SCREEN_HEIGHT_U16,
        );

        self.ltdc
            .gcr
            .hspol(false)
            .vspol(false)
            .depol(false)
            .pcpol(false)
            .den(false)
            .update_reg();

        self.ltdc
            .bccr
            .bcred(0x00)
            .bcgreen(0x00)
            .bcblue(0x00)
            .update_reg();

        self.ltdc.ier.fuie(true).terrie(true).update_reg();

        let buffer_start_address: u32 =
            unsafe { core::mem::transmute::<&u8, u32>(first_buffer_element) };

        self.ltdc.layer1.config_layer(
            lcd::SCREEN_WIDTH_U16,
            lcd::SCREEN_HEIGHT_U16,
            pac::ltdc::layer::pfcr::PF_A::L8,
            buffer_start_address,
        );

        // TODO figure out why clut causes issues
        self.ltdc.layer1.fill_clut_l8();
        self.ltdc.layer1.enable_layer();
        self.ltdc.srcr.set_imr();
        self.ltdc.gcr.ltdcen(true).update_reg();
    }

    pub fn init_dma2d(&mut self, first_buffer_element: &u8) {
        self.dma2d.as_mut().unwrap().init(
            pac::dma2d::cr::MODE_A::REGISTERTOMEMORY,
            lcd::SCREEN_WIDTH_U16,
            unsafe { core::mem::transmute::<&u8, u32>(first_buffer_element) },
        );
    }

    pub fn ltdc(&mut self) -> &mut Ltdc {
        &mut self.ltdc
    }

    pub fn rcc(&mut self) -> &mut Rcc {
        &mut self.rcc
    }

    pub fn take_dma2d(&mut self) -> Dma2d {
        self.dma2d.take().unwrap()
    }

    pub fn take_inputs(&mut self) -> Inputs {
        self.input.take().unwrap()
    }

    pub fn new() -> Board {
        let peripherals = stm32hal::pac::Peripherals::take().unwrap();
        let mut rcc = peripherals.RCC.constrain();
        let flash = peripherals.FLASH.constrain();
        let pwr = peripherals.PWR.constrain(&mut rcc.apb1r1);

        // forced to initialized GPIO here because the HAL I copied sucks
        let mut gpio_a = peripherals.GPIOA.split(&mut rcc.ahb2);
        let mut gpio_c = peripherals.GPIOC.split(&mut rcc.ahb2);
        let mut gpio_d = peripherals.GPIOD.split(&mut rcc.ahb2);
        let mut gpio_e = peripherals.GPIOE.split(&mut rcc.ahb2);
        let mut gpio_f = peripherals.GPIOF.split(&mut rcc.ahb2);
        let mut gpio_g = peripherals.GPIOG.split(&mut rcc.ahb2);

        // GPIO config
        let up: UpPin = gpio_c
            .pc10
            .into_pull_up_input(&mut gpio_c.moder, &mut gpio_c.pupdr);

        let down: DownPin = gpio_c
            .pc11
            .into_pull_up_input(&mut gpio_c.moder, &mut gpio_c.pupdr);

        let left: LeftPin = gpio_c
            .pc12
            .into_pull_up_input(&mut gpio_c.moder, &mut gpio_c.pupdr);

        let right: RightPin = gpio_d
            .pd2
            .into_pull_up_input(&mut gpio_d.moder, &mut gpio_d.pupdr);

        let display_pwr = gpio_c.pc1.into_push_pull_output_with_state(
            &mut gpio_c.moder,
            &mut gpio_c.otyper,
            gpio::State::Low,
        );

        let _pixel_clk = gpio_a
            .pa4
            .into_push_pull_output_with_state(
                &mut gpio_a.moder,
                &mut gpio_a.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_a.moder, &mut gpio_a.afrl)
            .set_speed(gpio::Speed::VeryHigh);

        let _de = gpio_c
            .pc0
            .into_push_pull_output_with_state(
                &mut gpio_c.moder,
                &mut gpio_c.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_c.moder, &mut gpio_c.afrl)
            .set_speed(gpio::Speed::VeryHigh);

        let _hsync = gpio_e
            .pe0
            .into_push_pull_output_with_state(
                &mut gpio_e.moder,
                &mut gpio_e.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_e.moder, &mut gpio_e.afrl)
            .set_speed(gpio::Speed::VeryHigh);

        let _vsync = gpio_e
            .pe1
            .into_push_pull_output_with_state(
                &mut gpio_e.moder,
                &mut gpio_e.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_e.moder, &mut gpio_e.afrl)
            .set_speed(gpio::Speed::VeryHigh);

        let _r0 = gpio_g
            .pg13
            .into_push_pull_output_with_state(
                &mut gpio_g.moder,
                &mut gpio_g.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_g.moder, &mut gpio_g.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _r1 = gpio_g
            .pg14
            .into_push_pull_output_with_state(
                &mut gpio_g.moder,
                &mut gpio_g.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_g.moder, &mut gpio_g.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _r2 = gpio_e
            .pe15
            .into_push_pull_output_with_state(
                &mut gpio_e.moder,
                &mut gpio_e.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_e.moder, &mut gpio_e.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _r3 = gpio_d
            .pd8
            .into_push_pull_output_with_state(
                &mut gpio_d.moder,
                &mut gpio_d.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_d.moder, &mut gpio_d.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _r4 = gpio_d
            .pd9
            .into_push_pull_output_with_state(
                &mut gpio_d.moder,
                &mut gpio_d.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_d.moder, &mut gpio_d.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _r5 = gpio_d
            .pd10
            .into_push_pull_output_with_state(
                &mut gpio_d.moder,
                &mut gpio_d.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_d.moder, &mut gpio_d.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _r6 = gpio_d
            .pd11
            .into_push_pull_output_with_state(
                &mut gpio_d.moder,
                &mut gpio_d.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_d.moder, &mut gpio_d.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _r7 = gpio_d
            .pd12
            .into_push_pull_output_with_state(
                &mut gpio_d.moder,
                &mut gpio_d.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_d.moder, &mut gpio_d.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _g0 = gpio_f
            .pf14
            .into_push_pull_output_with_state(
                &mut gpio_f.moder,
                &mut gpio_f.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_f.moder, &mut gpio_f.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _g1 = gpio_f
            .pf15
            .into_push_pull_output_with_state(
                &mut gpio_f.moder,
                &mut gpio_f.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_f.moder, &mut gpio_f.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _g2 = gpio_e
            .pe9
            .into_push_pull_output_with_state(
                &mut gpio_e.moder,
                &mut gpio_e.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_e.moder, &mut gpio_e.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _g3 = gpio_e
            .pe10
            .into_push_pull_output_with_state(
                &mut gpio_e.moder,
                &mut gpio_e.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_e.moder, &mut gpio_e.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _g4 = gpio_e
            .pe11
            .into_push_pull_output_with_state(
                &mut gpio_e.moder,
                &mut gpio_e.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_e.moder, &mut gpio_e.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _g5 = gpio_e
            .pe12
            .into_push_pull_output_with_state(
                &mut gpio_e.moder,
                &mut gpio_e.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_e.moder, &mut gpio_e.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _g6 = gpio_e
            .pe13
            .into_push_pull_output_with_state(
                &mut gpio_e.moder,
                &mut gpio_e.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_e.moder, &mut gpio_e.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _g7 = gpio_e
            .pe14
            .into_push_pull_output_with_state(
                &mut gpio_e.moder,
                &mut gpio_e.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_e.moder, &mut gpio_e.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _b0 = gpio_f
            .pf12
            .into_push_pull_output_with_state(
                &mut gpio_f.moder,
                &mut gpio_f.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_f.moder, &mut gpio_f.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _b1 = gpio_f
            .pf13
            .into_push_pull_output_with_state(
                &mut gpio_f.moder,
                &mut gpio_f.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_f.moder, &mut gpio_f.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _b2 = gpio_d
            .pd14
            .into_push_pull_output_with_state(
                &mut gpio_d.moder,
                &mut gpio_d.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_d.moder, &mut gpio_d.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _b3 = gpio_d
            .pd15
            .into_push_pull_output_with_state(
                &mut gpio_d.moder,
                &mut gpio_d.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_d.moder, &mut gpio_d.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _b4 = gpio_d
            .pd0
            .into_push_pull_output_with_state(
                &mut gpio_d.moder,
                &mut gpio_d.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_d.moder, &mut gpio_d.afrl)
            .set_speed(gpio::Speed::VeryHigh);

        let _b5 = gpio_d
            .pd1
            .into_push_pull_output_with_state(
                &mut gpio_d.moder,
                &mut gpio_d.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_d.moder, &mut gpio_d.afrl)
            .set_speed(gpio::Speed::VeryHigh);

        let _b6 = gpio_c
            .pc8 // TODO verify if this should be pc7 or pc8
            .into_push_pull_output_with_state(
                &mut gpio_c.moder,
                &mut gpio_c.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_c.moder, &mut gpio_c.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let _b7 = gpio_a
            .pa8
            .into_push_pull_output_with_state(
                &mut gpio_a.moder,
                &mut gpio_a.otyper,
                gpio::State::Low,
            )
            .into_af11(&mut gpio_a.moder, &mut gpio_a.afrh)
            .set_speed(gpio::Speed::VeryHigh);

        let ltdc =
            peripherals
                .LTDC
                .constrain(rcc::MAX_BOOST_SYSCLK, lcd::PIXEL_CLK_FREQ, display_pwr);

        let dma2d = peripherals.DMA2D.constrain();

        let inputs = Inputs::new(up, down, left, right);

        Board {
            rcc,
            flash,
            pwr,
            ltdc,
            input: Some(inputs),
            dma2d: Some(dma2d),
        }
    }
}
