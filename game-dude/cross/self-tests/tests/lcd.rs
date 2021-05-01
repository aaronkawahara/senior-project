#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert_eq;
    use board::{self, Board};
    use lcd::Lcd;
    use stm32l4p5_hal::rcc;

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
        let mut lcd = Lcd::new();

        board.ltdc().pwr_pins.display_pwr_on();
        board.init_ltdc(lcd.buffer_address());

        let rcc = board.rcc();

        let expected_apb2enr: u32 = 0x04000000;
        assert_eq!(expected_apb2enr, rcc.apb2.enr().read().bits());

        let expected_pllsai2cfgr: u32 = 0x07000910;
        assert_eq!(expected_pllsai2cfgr, rcc.pllsai2cfgr.read_reg());
        // expected: 0000_0111000000000000100100010000
        // read:     0000_0110000000000000100100010000

        let ltdc = board.ltdc();

        // TODO maybe try initiliazing everything in one mega function?
        // TODO test modifying registers directly
        let expected_sscr: u32 = (((Lcd::HSYNC_WIDTH - 1) as u32) << 16) | ((Lcd::VSYNC_HEIGHT - 1) as u32);
        assert_eq!(expected_sscr, ltdc.sscr.read_reg());
    }
}