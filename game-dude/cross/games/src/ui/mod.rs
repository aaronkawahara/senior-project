pub(crate) mod game_over;
pub(super) mod play_menu;
pub(super) mod score;
pub(super) mod play_again;

pub(super) mod numbers {
    use crate::images::{self, SimpleImage};

    pub const NUMBER_WIDTH: u32 = images::ZeroImage::WIDTH as u32;
    pub const NUMBER_HEIGHT: u32 = images::ZeroImage::HEIGHT as u32;

    pub fn get_digit_image_address(digit: u32) -> u32 {
        match digit {
            0 => images::ZeroImage.data_address(),
            1 => images::OneImage.data_address(),
            2 => images::TwoImage.data_address(),
            3 => images::ThreeImage.data_address(),
            4 => images::FourImage.data_address(),
            5 => images::FiveImage.data_address(),
            6 => images::SixImage.data_address(),
            7 => images::SevenImage.data_address(),
            8 => images::EightImage.data_address(),
            9 => images::NineImage.data_address(),
            _ => panic!("provided number is more than a single digit"),
        }
    }
}