use board::input::Inputs;
use stm32l4p5_hal::dma2d::Dma2d;

use crate::common::DiscreteSelection;
use crate::high_scores::{Entry, Initials, ONLY_LEVEL_HIGH_SCORES, SQUARE_FIELD_HIGH_SCORES};
use crate::Games;

use super::text::letters::{self, TextBox};

const NEW_HIGH_SCORE_STR: &str = "NEW HIGH SCORE";
const NEW_HIGH_SCORE: TextBox = TextBox {
    text: NEW_HIGH_SCORE_STR,
    x: letters::center_text_box(NEW_HIGH_SCORE_STR),
    y: letters::LETTER_HEIGHT / 2,
    border: false,
};

const ENTER_INITIALS_STR: &str = "ENTER YOUR INITIALS";
const ENTER_INITIALS: TextBox = TextBox {
    text: ENTER_INITIALS_STR,
    x: letters::center_text_box(ENTER_INITIALS_STR),
    y: NEW_HIGH_SCORE.y + letters::LETTER_HEIGHT * 2,
    border: false,
};

pub(crate) fn is_high_score(game: Games, score: u32) -> bool {
    match game {
        Games::SquareField => unsafe { SQUARE_FIELD_HIGH_SCORES.is_new_high_score(score) },
        Games::OnlyOneLevel => unsafe { ONLY_LEVEL_HIGH_SCORES.is_new_high_score(score) },
    }
}

pub(crate) fn handle_high_score(
    game: Games,
    score: u32,
    dma2d: &Dma2d,
    input: &mut Inputs,
    wait_for_vsync: fn() -> (),
) {
    let initials = get_user_initials(dma2d, input, wait_for_vsync);
    let entry: Entry<u32> = Entry::new(initials, score);

    match game {
        Games::SquareField => unsafe { SQUARE_FIELD_HIGH_SCORES.insert_new_high_score(entry) },
        Games::OnlyOneLevel => unsafe { ONLY_LEVEL_HIGH_SCORES.insert_new_high_score(entry) },
    }
}

fn get_user_initials(dma2d: &Dma2d, input: &mut Inputs, wait_for_vsync: fn() -> ()) -> Initials {
    let mut selection: HighScoreSelection = HighScoreSelection::FirstInitial;
    let mut first_initial: char = 'A';
    let mut second_initial: char = 'A';
    let mut third_initial: char = 'A';

    dma2d.fill_background(
        0x00_00_00_00,
        lcd::SCREEN_WIDTH_U16 / 4,
        lcd::SCREEN_HEIGHT_U16,
    );
    NEW_HIGH_SCORE.draw(dma2d);
    ENTER_INITIALS.draw(dma2d);

    while !(matches!(selection, HighScoreSelection::Ok) && input.right_debounced()) {
        if input.right_debounced() {
            selection = selection.next();
        } else if input.left_debounced() {
            selection = selection.previous()
        }

        match selection {
            HighScoreSelection::FirstInitial if input.down_debounced() => {
                first_initial = first_initial.next()
            }
            HighScoreSelection::FirstInitial if input.up_debounced() => {
                first_initial = first_initial.previous()
            }
            HighScoreSelection::SecondInitial if input.down_debounced() => {
                second_initial = second_initial.next()
            }
            HighScoreSelection::SecondInitial if input.up_debounced() => {
                second_initial = second_initial.previous()
            }
            HighScoreSelection::ThirdInitial if input.down_debounced() => {
                third_initial = third_initial.next()
            }
            HighScoreSelection::ThirdInitial if input.up_debounced() => {
                third_initial = third_initial.previous()
            }
            _ => {}
        }

        draw_initials(
            dma2d,
            selection,
            first_initial,
            second_initial,
            third_initial,
        );
        wait_for_vsync();
    }

    [first_initial, second_initial, third_initial]
}

fn draw_initials(
    dma2d: &Dma2d,
    selection: HighScoreSelection,
    first: char,
    second: char,
    third: char,
) {
    const OK: &str = "OK";
    let mut char_buffer: [u8; 4] = [0; 4];
    let y: u16 = ENTER_INITIALS.y + letters::LETTER_HEIGHT * 2;

    dma2d.fill_rgb8_rect(
        lcd::colors::BLACK,
        0,
        y - letters::BORDER_OFFSET_Y,
        lcd::SCREEN_WIDTH_U16,
        letters::BORDER_HEIGHT,
    );

    let first_text_box = TextBox {
        text: first.encode_utf8(&mut char_buffer),
        x: lcd::SCREEN_WIDTH_U16 / 3 - letters::LETTER_WIDTH / 2,
        y,
        border: matches!(selection, HighScoreSelection::FirstInitial),
    };

    first_text_box.draw(dma2d);

    let second_text_box = TextBox {
        text: second.encode_utf8(&mut char_buffer),
        x: lcd::SCREEN_WIDTH_U16 / 2 - letters::LETTER_WIDTH / 2,
        y,
        border: matches!(selection, HighScoreSelection::SecondInitial),
    };

    second_text_box.draw(dma2d);

    let third_text_box = TextBox {
        text: third.encode_utf8(&mut char_buffer),
        x: (lcd::SCREEN_WIDTH_U16 * 2) / 3 - letters::LETTER_WIDTH / 2,
        y,
        border: matches!(selection, HighScoreSelection::ThirdInitial),
    };

    third_text_box.draw(dma2d);

    let ok_text_box = TextBox {
        text: OK,
        x: (lcd::SCREEN_WIDTH_U16 * 5) / 6 - letters::LETTER_WIDTH,
        y,
        border: matches!(selection, HighScoreSelection::Ok),
    };

    ok_text_box.draw(dma2d);
}

#[derive(Clone, Copy)]
enum HighScoreSelection {
    FirstInitial,
    SecondInitial,
    ThirdInitial,
    Ok,
}

impl DiscreteSelection for HighScoreSelection {
    fn next(self) -> Self {
        match self {
            HighScoreSelection::FirstInitial => HighScoreSelection::SecondInitial,
            HighScoreSelection::SecondInitial => HighScoreSelection::ThirdInitial,
            HighScoreSelection::ThirdInitial => HighScoreSelection::Ok,
            HighScoreSelection::Ok => HighScoreSelection::Ok,
        }
    }

    fn previous(self) -> Self {
        match self {
            HighScoreSelection::FirstInitial => HighScoreSelection::FirstInitial,
            HighScoreSelection::SecondInitial => HighScoreSelection::FirstInitial,
            HighScoreSelection::ThirdInitial => HighScoreSelection::SecondInitial,
            HighScoreSelection::Ok => HighScoreSelection::ThirdInitial,
        }
    }
}

impl DiscreteSelection for char {
    fn next(self) -> Self {
        match self {
            'A' => 'B',
            'B' => 'C',
            'C' => 'D',
            'D' => 'E',
            'E' => 'F',
            'F' => 'G',
            'G' => 'H',
            'H' => 'I',
            'I' => 'J',
            'J' => 'K',
            'K' => 'L',
            'L' => 'M',
            'M' => 'N',
            'N' => 'O',
            'O' => 'P',
            'P' => 'Q',
            'Q' => 'R',
            'R' => 'S',
            'S' => 'T',
            'T' => 'U',
            'U' => 'V',
            'V' => 'W',
            'W' => 'X',
            'X' => 'Y',
            'Y' => 'Z',
            'Z' => 'A',
            _ => panic!("invalid char used for DiscreteSelection"),
        }
    }

    fn previous(self) -> Self {
        match self {
            'A' => 'Z',
            'B' => 'A',
            'C' => 'B',
            'D' => 'C',
            'E' => 'D',
            'F' => 'E',
            'G' => 'F',
            'H' => 'G',
            'I' => 'H',
            'J' => 'I',
            'K' => 'J',
            'L' => 'K',
            'M' => 'L',
            'N' => 'M',
            'O' => 'N',
            'P' => 'O',
            'Q' => 'P',
            'R' => 'Q',
            'S' => 'R',
            'T' => 'S',
            'U' => 'T',
            'V' => 'U',
            'W' => 'V',
            'X' => 'W',
            'Y' => 'X',
            'Z' => 'Y',
            _ => panic!("invalid char used for DiscreteSelection"),
        }
    }
}
