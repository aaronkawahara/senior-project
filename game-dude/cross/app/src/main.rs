#![no_std]
#![no_main]

use board::{self, Board};
use cortex_m_rt as rt;
use defmt_rtt as _;
use games;
use lcd::Lcd;
use panic_probe as _;
use rt::entry;

#[entry]
fn main() -> ! {
    let mut board = Board::new();
    let lcd = Lcd::new();
    let _clocks = board.init_system_clocks(); 

    board.ltdc().pwr_pins.display_pwr_on();
    board.init_ltdc(lcd.first_element());
    let draw_and_wait = board.ltdc().draw_and_wait();

    let dma2d = board.init_dma2d(lcd.first_element());

    let input = board.take_inputs();

    games::start_game_machine(input, dma2d, draw_and_wait);
}
