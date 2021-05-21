use crate::images::{self, SimpleImage};

use stm32l4p5_hal::dma2d::Dma2d;

const YES_X: u32 = 257;
const NO_X: u32 = 407;
const YES_NO_Y: u32 = 226;

pub(super) enum PlayAgainSelected {
    Yes,
    No,
}

pub(super) fn draw_play_again_selection(dma2d: &mut Dma2d, selection: &PlayAgainSelected) {
    dma2d.draw_rgb8_image(
        match selection {
            PlayAgainSelected::Yes => images::Yes_selectedImage.data_address(),
            PlayAgainSelected::No => images::Yes_not_selectedImage.data_address(),
        },
        YES_X,
        YES_NO_Y,
        images::Yes_selectedImage::WIDTH,
        images::Yes_selectedImage::HEIGHT,
    );

    dma2d.draw_rgb8_image(
        match selection {
            PlayAgainSelected::Yes => images::No_not_selectedImage.data_address(),
            PlayAgainSelected::No => images::No_selectedImage.data_address(),
        },
        NO_X,
        YES_NO_Y,
        images::No_not_selectedImage::WIDTH,
        images::No_not_selectedImage::HEIGHT,
    );
}
