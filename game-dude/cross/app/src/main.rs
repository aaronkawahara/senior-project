// decide on rtic or not.. will greatly impact flow
#![no_std]
#![no_main]

use cortex_m_rt as rt;

use rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use board::{self, Board};

#[entry]
fn main() -> ! {
    let mut board = Board::init();

    loop {
        board.lcd.set_color(0xFF);
        board.lcd.reload_srcr();
        while board.lcd.vsync_is_high() {}
        while !board.lcd.vsync_is_high() {}
    }
}