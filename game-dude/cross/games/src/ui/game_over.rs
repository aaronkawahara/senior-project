use crate::ui::high_score_entry;
use crate::Games;
use crate::States;

use super::score::Score;
use super::text::letters::{self, TextBox};

use board::input::Inputs;
use stm32l4p5_hal::dma2d::Dma2d;

pub(crate) fn handle_game_over(
    input: &mut Inputs,
    dma2d: &Dma2d,
    wait_for_vsync: fn() -> (),
    game: Games,
    score: u32,
) -> States {
    if high_score_entry::is_high_score(game, score) {
        high_score_entry::handle_high_score(game, score, dma2d, input, wait_for_vsync);
    }

    draw_game_over(dma2d, score);

    let mut selection: PlayAgainSelected = PlayAgainSelected::Yes;
    draw_play_again_selection(dma2d, selection);

    while !input.down_debounced() {
        if input.right_debounced() {
            selection = PlayAgainSelected::No;
            draw_play_again_selection(dma2d, selection);
        } else if input.left_debounced() {
            selection = PlayAgainSelected::Yes;
            draw_play_again_selection(dma2d, selection);
        }

        wait_for_vsync();
    }

    match selection {
        PlayAgainSelected::Yes => States::Play(game),
        PlayAgainSelected::No => States::PlayMenu,
    }
}

fn draw_game_over(dma2d: &Dma2d, score: u32) {
    dma2d.fill_background(
        0x00_00_00_00,
        lcd::SCREEN_WIDTH_U16 / 4,
        lcd::SCREEN_HEIGHT_U16,
    );

    GAME_OVER.draw(dma2d);
    SCORE.draw(dma2d);

    let mut score_digits = Score::default();
    score_digits.parse_score(score);
    score_digits.display_score(
        dma2d,
        lcd::SCREEN_WIDTH_U16 - letters::LETTER_WIDTH,
        SCORE.y,
    );
}

fn draw_play_again_selection(dma2d: &Dma2d, selection: PlayAgainSelected) {
    const YES_STR: &str = "YES";
    const NO_STR: &str = "NO";

    let yes: TextBox = TextBox {
        text: YES_STR,
        x: letters::LETTER_WIDTH * 3 + PLAY_AGAIN.width(),
        y: PLAY_AGAIN.y,
        border: matches!(selection, PlayAgainSelected::Yes),
    };

    let no: TextBox = TextBox {
        text: NO_STR,
        x: yes.x + 100,
        y: PLAY_AGAIN.y,
        border: matches!(selection, PlayAgainSelected::No),
    };

    dma2d.fill_rgb8_rect(
        lcd::colors::BLACK,
        0,
        yes.y - letters::BORDER_OFFSET_Y,
        lcd::SCREEN_WIDTH_U16,
        letters::BORDER_HEIGHT,
    );

    PLAY_AGAIN.draw(dma2d);
    yes.draw(dma2d);
    no.draw(dma2d);
}

#[derive(Clone, Copy)]
enum PlayAgainSelected {
    Yes,
    No,
}

const GAME_OVER_STR: &str = "GAME OVER";
const GAME_OVER: TextBox = TextBox {
    text: GAME_OVER_STR,
    x: letters::center_text_box(GAME_OVER_STR),
    y: letters::LETTER_HEIGHT,
    border: false,
};

const SCORE_STR: &str = "SCORE";
const SCORE: TextBox = TextBox {
    text: SCORE_STR,
    x: letters::LETTER_WIDTH,
    y: GAME_OVER.y + (letters::LETTER_HEIGHT * 3) / 2,
    border: false,
};

const PLAY_AGAIN_STR: &str = "PLAY AGAIN";
const PLAY_AGAIN: TextBox = TextBox {
    text: PLAY_AGAIN_STR,
    x: letters::LETTER_WIDTH,
    y: lcd::SCREEN_HEIGHT_U16 - letters::LETTER_HEIGHT * 2,
    border: false,
};