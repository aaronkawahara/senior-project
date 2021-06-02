pub(super) mod numbers {
    use crate::images::{self, SimpleImage};

    pub const NUMBER_WIDTH: u16 = images::ZeroImage::WIDTH;
    pub const NUMBER_HEIGHT: u16 = images::ZeroImage::HEIGHT;

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

pub(super) mod letters {
    use crate::images::{self, SimpleImage};
    use stm32l4p5_hal::dma2d::Dma2d;

    pub const LETTER_WIDTH: u16 = images::AImage::WIDTH;
    pub const LETTER_HEIGHT: u16 = images::AImage::HEIGHT;
    pub const BORDER_OFFSET_X: u16 = (images::TextHorizontalBordersImage::WIDTH - LETTER_WIDTH) / 2;
    pub const BORDER_OFFSET_Y: u16 = (images::TextVerticalBorderImage::HEIGHT - LETTER_HEIGHT) / 2;
    pub const BORDER_HEIGHT: u16 = images::TextVerticalBorderImage::HEIGHT;

    pub struct TextBox<'a> {
        pub text: &'a str,
        pub x: u16,
        pub y: u16,
        pub border: bool,
    }

    impl TextBox<'_> {
        pub fn draw(&self, dma2d: &Dma2d) {
            let mut x: u32 = u32::from(self.x);
            let y: u32 = u32::from(self.y);

            if self.border {
                self.draw_borders(dma2d);
            }

            for letter in self.text.chars() {
                draw_letter(dma2d, letter, x, y);
                x += u32::from(LETTER_WIDTH);
            }
        }

        fn draw_borders(&self, dma2d: &Dma2d) {
            let mut x: u32 = u32::from(self.x - BORDER_OFFSET_X);
            let y: u32 = u32::from(self.y - BORDER_OFFSET_Y);
            let chars: u32 = self.text.len() as u32;

            for _ in 0..chars {
                dma2d.draw_rgb8_image(
                    images::TextHorizontalBordersImage.data_address(),
                    x,
                    y,
                    images::TextHorizontalBordersImage::WIDTH,
                    images::TextHorizontalBordersImage::HEIGHT,
                );

                x += u32::from(LETTER_WIDTH);
            }

            dma2d.draw_rgb8_image(
                images::TextVerticalBorderImage.data_address(),
                u32::from(self.x - BORDER_OFFSET_X),
                y,
                images::TextVerticalBorderImage::WIDTH,
                images::TextVerticalBorderImage::HEIGHT,
            );

            dma2d.draw_rgb8_image(
                images::TextVerticalBorderImage.data_address(),
                x + u32::from(BORDER_OFFSET_X + images::TextVerticalBorderImage::WIDTH),
                y,
                images::TextVerticalBorderImage::WIDTH,
                images::TextVerticalBorderImage::HEIGHT,
            );
        }

        pub const fn width(&self) -> u16 {
            self.text.len() as u16 * LETTER_WIDTH
        }
    }

    pub const fn center_text_box(text: &str) -> u16 {
        lcd::SCREEN_WIDTH_U16 / 2 - (text.len() as u16 * LETTER_WIDTH / 2) as u16
    }

    fn draw_letter(dma2d: &Dma2d, letter: char, x: u32, y: u32) {
        let (image_address, width, height) = match letter {
            'A' => (
                images::AImage.data_address(),
                images::AImage::WIDTH,
                images::AImage::HEIGHT,
            ),
            'B' => (
                images::BImage.data_address(),
                images::BImage::WIDTH,
                images::BImage::HEIGHT,
            ),
            'C' => (
                images::CImage.data_address(),
                images::CImage::WIDTH,
                images::CImage::HEIGHT,
            ),
            'D' => (
                images::DImage.data_address(),
                images::DImage::WIDTH,
                images::DImage::HEIGHT,
            ),
            'E' => (
                images::EImage.data_address(),
                images::EImage::WIDTH,
                images::EImage::HEIGHT,
            ),
            'F' => (
                images::FImage.data_address(),
                images::FImage::WIDTH,
                images::FImage::HEIGHT,
            ),
            'G' => (
                images::GImage.data_address(),
                images::GImage::WIDTH,
                images::GImage::HEIGHT,
            ),
            'H' => (
                images::HImage.data_address(),
                images::HImage::WIDTH,
                images::HImage::HEIGHT,
            ),
            'I' => (
                images::IImage.data_address(),
                images::IImage::WIDTH,
                images::IImage::HEIGHT,
            ),
            'J' => (
                images::JImage.data_address(),
                images::JImage::WIDTH,
                images::JImage::HEIGHT,
            ),
            'K' => (
                images::KImage.data_address(),
                images::KImage::WIDTH,
                images::KImage::HEIGHT,
            ),
            'L' => (
                images::LImage.data_address(),
                images::LImage::WIDTH,
                images::LImage::HEIGHT,
            ),
            'M' => (
                images::MImage.data_address(),
                images::MImage::WIDTH,
                images::MImage::HEIGHT,
            ),
            'N' => (
                images::NImage.data_address(),
                images::NImage::WIDTH,
                images::NImage::HEIGHT,
            ),
            'O' => (
                images::OImage.data_address(),
                images::OImage::WIDTH,
                images::OImage::HEIGHT,
            ),
            'P' => (
                images::PImage.data_address(),
                images::PImage::WIDTH,
                images::PImage::HEIGHT,
            ),
            'Q' => (
                images::QImage.data_address(),
                images::QImage::WIDTH,
                images::QImage::HEIGHT,
            ),
            'R' => (
                images::RImage.data_address(),
                images::RImage::WIDTH,
                images::RImage::HEIGHT,
            ),
            'S' => (
                images::SImage.data_address(),
                images::SImage::WIDTH,
                images::SImage::HEIGHT,
            ),
            'T' => (
                images::TImage.data_address(),
                images::TImage::WIDTH,
                images::TImage::HEIGHT,
            ),
            'U' => (
                images::UImage.data_address(),
                images::UImage::WIDTH,
                images::UImage::HEIGHT,
            ),
            'V' => (
                images::VImage.data_address(),
                images::VImage::WIDTH,
                images::VImage::HEIGHT,
            ),
            'W' => (
                images::WImage.data_address(),
                images::WImage::WIDTH,
                images::WImage::HEIGHT,
            ),
            'X' => (
                images::XImage.data_address(),
                images::XImage::WIDTH,
                images::XImage::HEIGHT,
            ),
            'Y' => (
                images::YImage.data_address(),
                images::YImage::WIDTH,
                images::YImage::HEIGHT,
            ),
            'Z' => (
                images::ZImage.data_address(),
                images::ZImage::WIDTH,
                images::ZImage::HEIGHT,
            ),
            ' ' => (
                images::SpaceImage.data_address(),
                images::SpaceImage::WIDTH,
                images::SpaceImage::HEIGHT,
            ),
            c => panic!("no alphabet letter provided: {}", c),
        };

        dma2d.draw_rgb8_image(image_address, x, y, width, height);
    }
}
