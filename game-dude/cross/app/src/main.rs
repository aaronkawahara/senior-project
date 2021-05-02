#![no_std]
#![no_main]

use asm_delay::{AsmDelay, bitrate::U32BitrateExt};
use board::{self, Board};
use cortex_m_rt as rt;
use defmt_rtt as _;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use lcd::Lcd;
use panic_probe as _;
use rt::entry;

#[entry]
fn main() -> ! {
    let mut board = Board::new();
    let mut lcd = Lcd::new();
    
    let clocks = board.init_system_clocks();
    let mut _delayer = AsmDelay::new(U32BitrateExt::hz(u32::from(clocks.sysclk())));

    board.ltdc().pwr_pins.display_pwr_on();
    board.init_ltdc(lcd.buffer_address());

    let mut color: u8 = 0xFF;
    loop {
        lcd.set_color(color);
        defmt::info!("color set!");
        board.ltdc().reload_shadow_reg();
        board.ltdc().wait_for_frame();
        color ^= 0xFF;
        defmt::info!("color changed!");
    }
}