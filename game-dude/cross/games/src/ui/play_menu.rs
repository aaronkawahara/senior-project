use crate::images;
use crate::images::SimpleImage;
use crate::Games;

use board::input::Inputs;
use lcd;
use stm32l4p5_hal::dma2d::Dma2d;

pub(crate) fn get_game_selection(
    input: &mut Inputs,
    dma2d: &mut Dma2d,
    draw_and_wait: fn() -> (),
) -> Games {
    dma2d.fill_background(
        0x00_00_00_00,
        lcd::SCREEN_WIDTH_U16 / 4,
        lcd::SCREEN_HEIGHT_U16,
    );

    let mut selection: Games = Games::SquareField;
    draw_game_selection(dma2d, &selection);

    while !input.right_debounced() {
        if input.down_debounced() {
            selection = selection.next();
        } else if input.up_debounced() {
            selection = selection.previous();
        }

        draw_and_wait();
    }

    selection
}

fn draw_game_selection(dma2d: &mut Dma2d, selection: &Games) {
    const DY: u32 = lcd::SCREEN_HEIGHT_U32 / (core::mem::variant_count::<Games>() + 1) as u32;
    const SQUARE_FIELD_X: u32 =
        (lcd::SCREEN_WIDTH_U16 - images::SquareFieldSelectedImage::WIDTH) as u32 / 2;
    const SQUARE_FIELD_Y: u32 = DY - images::SquareFieldSelectedImage::HEIGHT as u32 / 2;

    dma2d.draw_rgb8_image(
        if let Games::SquareField = selection {
            images::SquareFieldSelectedImage.data_address()
        } else {
            images::SquareFieldNotSelectedImage.data_address()
        },
        SQUARE_FIELD_X,
        SQUARE_FIELD_Y,
        images::SquareFieldSelectedImage::WIDTH,
        images::SquareFieldSelectedImage::HEIGHT,
    );
}

trait DiscreteSelection {
    fn next(self) -> Self;
    fn previous(self) -> Self;
}

impl DiscreteSelection for Games {
    fn next(self) -> Self {
        match self {
            Games::SquareField => Games::SquareField,
        }
    }

    fn previous(self) -> Self {
        match self {
            Games::SquareField => Games::SquareField,
        }
    }
}
