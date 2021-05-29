use crate::images::{self, SimpleImage};
use crate::Games;
use crate::States;

use super::play_again::{draw_play_again_selection, PlayAgainSelected};
use super::score::Score;

use board::input::Inputs;
use stm32l4p5_hal::dma2d::Dma2d;

pub(crate) fn handle_game_over(
    input: &mut Inputs,
    dma2d: &mut Dma2d,
    draw_and_wait: fn() -> (),
    game: Games,
    score: u32,
) -> States {
    const CENTER_Y: u32 = 182;
    
    dma2d.draw_rgb8_image(
        images::GameOverImage.data_address(),
        0,
        0,
        lcd::SCREEN_WIDTH_U16,
        lcd::SCREEN_HEIGHT_U16,
    );

    let mut score_digits = Score::default();
    score_digits.parse_score(score);
    score_digits.display_score(dma2d, lcd::SCREEN_WIDTH_U32 / 2, CENTER_Y);

    let mut selection: PlayAgainSelected = PlayAgainSelected::Yes;
    draw_play_again_selection(dma2d, &selection);

    while !input.down_debounced() {
        if input.right_debounced() {
            selection = PlayAgainSelected::No;
            draw_play_again_selection(dma2d, &selection);
        } else if input.left_debounced() {
            selection = PlayAgainSelected::Yes;
            draw_play_again_selection(dma2d, &selection);
        }

        draw_and_wait();
    }

    match selection {
        PlayAgainSelected::Yes => States::Play(game),
        PlayAgainSelected::No => States::PlayMenu,
    }
}
