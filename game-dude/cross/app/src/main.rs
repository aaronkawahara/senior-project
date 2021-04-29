#![no_std]
#![no_main]

use asm_delay::{AsmDelay, bitrate::U32BitrateExt};
use cortex_m_rt as rt;
use rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use board::{self, Board};
use lcd::Lcd;
use stm32l4p5_hal as hal;
use hal::{prelude::*, time::Hertz};
use embedded_hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut board = Board::new();
    let lcd = Lcd::new();
    let clocks = board.init_system_clocks();
    let clk_spd = clocks.sysclk();
    assert_eq!(clocks.sysclk(), Hertz(120_000_000));
    let mut delayer = AsmDelay::new(U32BitrateExt::mhz(120));
    // board.init_ltdc_clocks();
    // board.init_ltdc(lcd.buffer_address());

    loop {
        // board.lcd.set_color(0xFF);

        // board.ltdc.reload_shadow_reg();
        // board.ltdc.wait_for_frame();
    }
}