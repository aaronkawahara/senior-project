use stm32l4p5_hal::dma2d::Dma2d;

use crate::{
    collisions::BoundingBox,
    common::Position,
    images::{
        SimpleImage, SquareFieldPlayerBrightImage, SquareFieldPlayerMonochromeImage,
        SquareFieldPlayerNeonImage, SquareFieldPlayerNormalImage,
    },
};

use super::color_schemes::ColorSchemes;

pub struct Player<'a> {
    pub hit_box: BoundingBox,
    dma2d: &'a Dma2d,
}

impl<'a> Player<'a> {
    const WIDTH: u16 = SquareFieldPlayerNormalImage::WIDTH;
    const HEIGHT: u16 = SquareFieldPlayerNormalImage::HEIGHT;
    pub const MOVEMENT_SPEED: i32 = 15;

    pub fn new(dma2d: &'a Dma2d) -> Self {
        let x: i32 = lcd::SCREEN_WIDTH_I32 / 2 - i32::from(SquareFieldPlayerNormalImage::WIDTH) / 2;
        let y: i32 = lcd::SCREEN_HEIGHT_I32 - 10 - i32::from(SquareFieldPlayerNormalImage::HEIGHT);

        Player {
            hit_box: BoundingBox::new(
                Position::new(x, y),
                Position::new(
                    x + i32::from(SquareFieldPlayerNormalImage::WIDTH),
                    y + i32::from(SquareFieldPlayerNormalImage::HEIGHT),
                ),
            ),
            dma2d,
        }
    }

    pub fn draw(&self, color_scheme: ColorSchemes) {
        self.dma2d.draw_rgb8_image(
            match color_scheme {
                ColorSchemes::Normal => SquareFieldPlayerNormalImage.data_address(),
                ColorSchemes::Neon => SquareFieldPlayerNeonImage.data_address(),
                ColorSchemes::Monochrome => SquareFieldPlayerMonochromeImage.data_address(),
                ColorSchemes::Bright => SquareFieldPlayerBrightImage.data_address(),
            },
            self.hit_box.top_left.x as u32,
            self.hit_box.top_left.y as u32,
            Self::WIDTH,
            Self::HEIGHT,
        );
    }
}
