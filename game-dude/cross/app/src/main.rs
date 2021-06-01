#![no_std]
#![no_main]

use board::{self, Board};
use cortex_m_rt as rt;
use defmt_rtt as _;
use lcd::Lcd;
use panic_probe as _;
use rt::entry;

#[entry]
fn main() -> ! {
    let lcd = Lcd::new();
    let mut board = Board::new();
    board.init_system_clocks(); 
    board.ltdc().pwr_pins.display_pwr_on();
    board.init_ltdc(lcd.first_element());
    board.init_dma2d(lcd.first_element());
    let input = board.take_inputs();
    let dma2d = board.take_dma2d();
    let wait_for_vsync = board.ltdc().wait_for_vsync();

    games::start_game_machine(input, dma2d, wait_for_vsync);
}
