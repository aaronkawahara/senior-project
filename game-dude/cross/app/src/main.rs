#![no_std]
#![no_main]

use board::{self, Board};
use cortex_m_rt as rt;
use defmt_rtt as _;
use games::{square_field, input::DPad};
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

    let mut dma2d = board.init_dma2d(lcd.first_element());

    let inputs = board.inputs();
    let mut dpad = DPad::new(&inputs.up, &inputs.down, &inputs.left, &inputs.right);
    let mut state: States = States::PlayCubeField;

    loop {
        state = match state {
            States::PlayCubeField => {
                let score: u32 = square_field::play(&mut lcd, &mut dpad, &mut dma2d, draw_and_wait);
                States::GameOver { score, game: Games::CubeField }
            }
            States::GameOver { score, game} => {
                States::PlayCubeField
            }
            States::MainMenu => {
                States::MainMenu
            }
        };
    }
}

enum States {
    GameOver{ score: u32, game: Games },
    MainMenu,
    PlayCubeField,
}

#[derive(Clone, Copy)]
enum Games {
    CubeField,
}