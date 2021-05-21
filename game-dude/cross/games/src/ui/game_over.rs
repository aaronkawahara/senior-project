use crate::images::{self, SimpleImage};
use crate::Games;
use crate::States;

use super::score::Score;

use board::input::Inputs;
use lcd;
use stm32l4p5_hal::dma2d::Dma2d;

pub(crate) fn handle_game_over(
    input: &mut Inputs,
    dma2d: &mut Dma2d,
    draw_and_wait: fn() -> (),
    game: Games,
    score: u32,
) -> States {
    dma2d.draw_rgb8_image(
        images::Game_overImage.data_address(),
        0,
        0,
        lcd::SCREEN_WIDTH_U16,
        lcd::SCREEN_HEIGHT_U16,
    );

    let mut score_digits = Score::default();
    score_digits.parse_score(score);
    const CENTER_Y: u32 = 182;
    score_digits.display_score(dma2d, lcd::SCREEN_WIDTH_U32 / 2, CENTER_Y);

    const YES_X: u32 = 257;
    const NO_X: u32 = 407;
    const YES_NO_Y: u32 = 226;

    dma2d.draw_rgb8_image(
        images::Yes_selectedImage.data_address(),
        YES_X,
        YES_NO_Y,
        images::Yes_selectedImage::WIDTH,
        images::Yes_selectedImage::HEIGHT,
    );
    dma2d.draw_rgb8_image(
        images::No_not_selectedImage.data_address(),
        NO_X,
        YES_NO_Y,
        images::No_not_selectedImage::WIDTH,
        images::No_not_selectedImage::HEIGHT,
    );

    while !input.down_debounced() {
        draw_and_wait();
    }

    States::Play(game)
}
