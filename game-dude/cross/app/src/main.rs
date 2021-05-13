#![no_std]
#![no_main]

use board::{self, Board};
use cortex_m_rt as rt;
use defmt_rtt as _;
use games::{cube_field, input::DPad};
use lcd::Lcd;
use panic_probe as _;
use rand::{SeedableRng, rngs::SmallRng};
use rt::entry;

const RNG_SEED: [u8; 16] = [
    57, 15, 4, 218, 230, 117, 34, 242, 173, 21, 102, 234, 23, 225, 59, 137,
    // 180, 233, 32, 108, 41, 189, 248, 144, 83, 48, 250, 211, 129, 61, 22, 137
];

#[entry]
fn main() -> ! {
    let mut board = Board::new();
    let mut lcd = Lcd::new();
    let _clocks = board.init_system_clocks();

    board.ltdc().pwr_pins.display_pwr_on();
    board.init_ltdc(lcd.first_element());
    let wait_for_frame = board.ltdc().wait_for_frame();
    let draw_and_wait = board.ltdc().draw_and_wait();

    let mut dma2d = board.init_dma2d(lcd.first_element());

    let inputs = board.inputs();
    let mut dpad = DPad::new(&inputs.up, &inputs.down, &inputs.left, &inputs.right);

    let mut rng = SmallRng::from_seed(RNG_SEED);

    loop {
        cube_field::play(&mut lcd, &mut dpad, &mut dma2d, &mut rng, draw_and_wait);
    }
}