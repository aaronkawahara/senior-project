#![no_std]
#![no_main]

use board::{self, Board};
use cortex_m_rt as rt;
use defmt_rtt as _;
use games::{cube_field, input::DPad};
use lcd::Lcd;
use panic_probe as _;
use rt::entry;

#[entry]
fn main() -> ! {
    let mut board = Board::new();
    let mut lcd = Lcd::new();
    let _clocks = board.init_system_clocks();

    board.ltdc().pwr_pins.display_pwr_on();
    board.init_ltdc(lcd.first_element());
    let draw_and_wait = board.ltdc().draw_and_wait();

    let inputs = board.inputs();
    let mut dpad = DPad::new(&inputs.up, &inputs.down, &inputs.left, &inputs.right);

    loop {
        cube_field::play(&mut lcd, &mut dpad, draw_and_wait);
    }
}