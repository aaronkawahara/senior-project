#![no_std]

use lcd;
use stm32l4p5_hal as stm32hal;
use stm32hal::{dma2d::{Dma2d, Dma2dExt}, flash::{self, FlashExt}, gpio::{self, GpioExt, Input, PullUp}, ltdc::{Ltdc, LtdcExt}, pac, pwr::{Pwr, PwrExt}, rcc::{self, Clocks, Rcc, RccExt, PllConfig, PllDivider, PllSource}};

pub struct Board {
    rcc: Rcc,
    flash: flash::Parts,
    pwr: Pwr,
    ltdc: Ltdc,
    inputs: Inputs,
    dma2d: Option<Dma2d>,
}

pub struct Inputs
where
{
    pub up: UpPin,
    pub down: DownPin,
    pub left: LeftPin,
    pub right: RightPin,
}

pub type UpPin = gpio::PC10<Input<PullUp>>;
pub type DownPin = gpio::PC11<Input<PullUp>>;
pub type LeftPin = gpio::PC12<Input<PullUp>>;
pub type RightPin = gpio::PD2<Input<PullUp>>;

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

        self.rcc.pllsai2cfgr
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

        self.ltdc.gcr
            .hspol(false)
            .vspol(false)
            .depol(false)
            .pcpol(false)
            .den(false)
            .update_reg();

        self.ltdc.bccr
            .bcred(0x00)
            .bcgreen(0x00)
            .bcblue(0x00)
            .update_reg();

        self.ltdc.ier
            .fuie(true)
            .terrie(true)
            .update_reg();

        let buffer_start_address: u32 = unsafe { 
            core::mem::transmute::<&u8, u32>(first_buffer_element) 
        };

        self.ltdc.layer1.config_layer(
            lcd::SCREEN_WIDTH_U16,
            lcd::SCREEN_HEIGHT_U16,
            pac::ltdc::layer::pfcr::PF_A::L8,
            buffer_start_address
        );

        // TODO figure out why clut causes issues
        self.ltdc.layer1.fill_clut_l8();
        self.ltdc.layer1.enable_layer();
        self.ltdc.srcr.set_imr();
        self.ltdc.gcr.ltdcen(true).update_reg();
    }

    pub fn init_dma2d(&mut self, first_buffer_element: &u8) -> Dma2d {
        self.dma2d.as_mut().unwrap().init(
            pac::dma2d::cr::MODE_A::REGISTERTOMEMORY,
            lcd::SCREEN_WIDTH_U16,
            unsafe { core::mem::transmute::<&u8, u32>(first_buffer_element) }
        );

        self.dma2d.take().unwrap()
    }

    pub fn ltdc(&mut self) -> &mut Ltdc {
        &mut self.ltdc
    }

    pub fn rcc(&mut self) -> &mut Rcc {
        &mut self.rcc
    }

    pub fn inputs(&mut self) -> &mut Inputs {
        &mut self.inputs
    }

    pub fn new() -> Board {
        let peripherals = stm32hal::pac::Peripherals::take().unwrap();
        let mut rcc = peripherals.RCC.constrain();
        let flash = peripherals.FLASH.constrain();
        let pwr = peripherals.PWR.constrain(&mut rcc.apb1r1);

        // forced to initialized GPIO here because the HAL I copied sucks
        let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb2);
        let mut gpioc = peripherals.GPIOC.split(&mut rcc.ahb2);
        let mut gpiod = peripherals.GPIOD.split(&mut rcc.ahb2);
        let mut gpioe = peripherals.GPIOE.split(&mut rcc.ahb2);
        let mut gpiof = peripherals.GPIOF.split(&mut rcc.ahb2);
        let mut gpiog = peripherals.GPIOG.split(&mut rcc.ahb2);

        // GPIO config
        let up: UpPin = gpioc
            .pc10
            .into_pull_up_input(
                &mut gpioc.moder,
                &mut gpioc.pupdr
            );

        let down: DownPin = gpioc
            .pc11
            .into_pull_up_input(
                &mut gpioc.moder,
                &mut gpioc.pupdr
            );

        let left: LeftPin = gpioc
            .pc12
            .into_pull_up_input(
                &mut gpioc.moder,
                &mut gpioc.pupdr
            );

        let right: RightPin = gpiod
            .pd2
            .into_pull_up_input(
                &mut gpiod.moder,
                &mut gpiod.pupdr
            );

        let display_pwr = gpioc
            .pc1
            .into_push_pull_output_with_state(
                &mut gpioc.moder,
                &mut gpioc.otyper,
                gpio::State::Low
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
            .pc8 // TODO verify if this should be pc7 or pc8
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

        let ltdc = peripherals.LTDC.constrain(
            rcc::MAX_BOOST_SYSCLK, 
            lcd::PIXEL_CLK_FREQ, 
            display_pwr
        );

        let dma2d = peripherals.DMA2D.constrain();

        let inputs = Inputs {
            up,
            down,
            left,
            right,
        };

        Board {
            rcc,
            flash,
            pwr,
            ltdc,
            inputs,
            dma2d: Some(dma2d),
        }
    }
}