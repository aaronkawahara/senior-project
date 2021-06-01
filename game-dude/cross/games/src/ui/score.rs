use stm32l4p5_hal::dma2d::Dma2d;

use super::text::numbers;

type ImagePtr = u32;

pub(super) struct Score {
    digits: [Option<ImagePtr>; Self::MAX_DIGITS],
}

impl Score {
    const MAX_DIGITS: usize = 10;

    pub fn parse_score(&mut self, mut score: u32) -> &Self {
        let mut i = 0;

        while score > 0 {
            let digit: ImagePtr = numbers::get_digit_image_address(score % 10);
            score /= 10;

            self.digits[i] = Some(digit);
            i += 1;
        }

        self
    }

    pub fn display_score(&self, dma2d: &Dma2d, right_x: u16, y: u16) {
        let mut x: u16 = right_x - numbers::NUMBER_WIDTH;

        for digit in &self.digits {
            if let Some(image_address) = digit {
                dma2d.draw_rgb8_image(
                    *image_address,
                    u32::from(x),
                    u32::from(y),
                    numbers::NUMBER_WIDTH,
                    numbers::NUMBER_HEIGHT,
                );

                x -= numbers::NUMBER_WIDTH;
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
