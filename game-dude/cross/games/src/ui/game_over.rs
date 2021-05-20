use crate::input::DPad;
use crate::States;
use crate::{input::DirectionalInput, Games};

use super::score::Score;

use lcd;
use stm32l4p5_hal::dma2d::Dma2d;

pub(crate) fn handle_game_over(
    dpad: &mut DPad,
    dma2d: &mut Dma2d,
    draw_and_wait: fn() -> (),
    game: Games,
    score: u32,
) -> States {
    dma2d.fill_background(
        0x00_00_00_00,
        lcd::SCREEN_WIDTH_U16 / 4,
        lcd::SCREEN_HEIGHT_U16,
    );
    
    let mut score_digits = Score::default();
    score_digits.parse_score(score);

    while !dpad.down_pressed() {
        score_digits.display_score(dma2d, lcd::SCREEN_WIDTH_U32 / 2, lcd::SCREEN_HEIGHT_U32 / 2);
        draw_and_wait();
    }

    States::Play(game)
}
