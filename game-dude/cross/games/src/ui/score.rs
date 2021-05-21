use super::ui::numbers;
use stm32l4p5_hal::dma2d::Dma2d;

type ImagePtr = u32;

pub(super) struct Score {
    digits: [Option<ImagePtr>; Self::MAX_DIGITS],
}

impl Score {
    const MAX_DIGITS: usize = 10;

    pub fn parse_score(&mut self, mut score: u32) -> &Self {
        let mut i = Self::MAX_DIGITS - 1;

        while score > 0 {
            let digit: ImagePtr = numbers::get_digit_image_address(score % 10);
            score /= 10;

            self.digits[i] = Some(digit);
            i -= 1;
        }

        self
    }

    pub fn display_score(&self, dma2d: &mut Dma2d, center_x: u32, center_y: u32) {
        let mut width: u32 = Self::MAX_DIGITS as u32 * numbers::NUMBER_WIDTH;
        let mut x: u32 = 0;
        let y: u32 = center_y - numbers::NUMBER_HEIGHT / 2;

        for digit in self.digits.iter() {
            if let Some(image_address) = digit {
                dma2d.draw_rgb8_image(
                    *image_address,
                    x,
                    y,
                    numbers::NUMBER_WIDTH as u16,
                    numbers::NUMBER_HEIGHT as u16,
                );
                x += numbers::NUMBER_WIDTH;
            } else {
                width -= numbers::NUMBER_WIDTH;
                x = center_x - width / 2;
            }
        }
    }
}

impl Default for Score {
    fn default() -> Self {
        Score {
            digits: [None; Self::MAX_DIGITS],
        }
    }
}
