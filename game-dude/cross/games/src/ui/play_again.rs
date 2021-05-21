use crate::images::{self, SimpleImage};

use stm32l4p5_hal::dma2d::Dma2d;

const YES_X: u32 = 257;
const NO_X: u32 = 357;
const YES_NO_Y: u32 = 226;

pub(super) enum PlayAgainSelected {
    Yes,
    No,
}

pub(super) fn draw_play_again_selection(dma2d: &mut Dma2d, selection: &PlayAgainSelected) {
    dma2d.draw_rgb8_image(
        match selection {
            PlayAgainSelected::Yes => images::YesSelectedImage.data_address(),
            PlayAgainSelected::No => images::YesNotSelectedImage.data_address(),
        },
        YES_X,
        YES_NO_Y,
        images::YesSelectedImage::WIDTH,
        images::YesSelectedImage::HEIGHT,
    );

    dma2d.draw_rgb8_image(
        match selection {
            PlayAgainSelected::Yes => images::NoNotSelectedImage.data_address(),
            PlayAgainSelected::No => images::NoSelectedImage.data_address(),
        },
        NO_X,
        YES_NO_Y,
        images::NoNotSelectedImage::WIDTH,
        images::NoNotSelectedImage::HEIGHT,
    );
}
