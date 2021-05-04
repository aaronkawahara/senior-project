#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert_eq;
    use board::{self, Board};
    use lcd::Lcd;
    use stm32l4p5_hal::{pac, rcc};

    #[init]
    fn init() -> Board {
        Board::new()
    }

    #[test]
    fn will_pass() {
        assert!(true);
    }

    #[test]
    fn check_clocks(board: &mut Board) {
        let expected_freq: u32 = 120_000_000;
        let expected_hsi48 = false;
        let expected_msi: Option<rcc::MsiFreq> = None;
        let expected_lsi = false;
        let expected_lse = false;
        let expected_pll = Some(rcc::PllSource::HSI16);

        // let expected_clocks = rcc::Clocks {
        //     hclk: expected_freq,
        //     hsi48: false,
        //     msi: None,
        //     lsi: false,
        //     lse: false,
        //     pclk1: expected_freq,
        //     pclk2: expected_freq,
        //     ppre1: 1,
        //     ppre2: 1,
        //     sysclk: expected_freq,
        //     pll_source: Some(rcc::PllSource::HSI16),
        // };

        let clocks = board.init_system_clocks();

        assert_eq!(expected_freq, clocks.hclk().0);
        assert_eq!(expected_hsi48, clocks.hsi48());
        assert!(expected_msi == clocks.msi());
        assert_eq!(expected_lsi, clocks.lsi());
        assert_eq!(expected_lse, clocks.lse());
        assert_eq!(expected_freq, clocks.pclk1().0);
        assert_eq!(expected_freq, clocks.pclk2().0);
        assert!(expected_pll == clocks.pll_source());

        let rcc = board.rcc();
        
        let expected_cfgr: u32 = 0x0000000F;
        assert_eq!(expected_cfgr, rcc.cfgr.read_reg());
    }

    #[test]
    fn check_ltdc_registers(board: &mut Board) {
        let lcd = Lcd::new();

        board.ltdc().pwr_pins.display_pwr_on();
        board.init_ltdc(lcd.buffer_address());

        let rcc = board.rcc();

        let expected_cr: u32 = 0x33000560;
        assert_eq!(expected_cr, unsafe { (&(*pac::RCC::ptr())).cr.read().bits() });

        let expected_cfgr: u32 = 0xf;
        assert_eq!(expected_cfgr, rcc.cfgr.read_reg());

        let expected_pllcfgr: u32 = 0x1001E12;
        assert_eq!(expected_pllcfgr, rcc.pllcfgr.read_reg());

        let expected_apb2enr: u32 = 0x04000000;
        assert_eq!(expected_apb2enr, rcc.apb2.enr().read().bits());

        let expected_pllsai2cfgr: u32 = 0x03000910;
        assert_eq!(expected_pllsai2cfgr, rcc.pllsai2cfgr.read_reg());

        let ltdc = unsafe { &(*pac::LTDC::ptr()) };

        let expected_sscr: u32 = 0x30003;
        assert_eq!(expected_sscr, ltdc.sscr.read().bits());

        let expected_bpcr: u32 = 0x2b000b;
        assert_eq!(expected_bpcr, ltdc.bpcr.read().bits());

        let expected_awcr: u32 = 0x20b011b;
        assert_eq!(expected_awcr, ltdc.awcr.read().bits());

        let expected_twcr: u32 = 0x2100123;
        assert_eq!(expected_twcr, ltdc.twcr.read().bits());

        let expected_gcr: u32 = 0x2221;
        assert_eq!(expected_gcr, ltdc.gcr.read().bits());
    }
}