use stm32l4p5_hal::dma2d::Dma2d;

use crate::{
    collisions::BoundingBox,
    common::{Position, Velocity},
    images::{
        SimpleImage, SquareBrightImage, SquareMonochromeImage, SquareNeonImage, SquareNormalImage,
    },
};

use super::color_schemes::ColorSchemes;

#[derive(Clone, Copy)]
pub struct Square<'a> {
    pub hit_box: BoundingBox,
    pub velocity: Velocity,
    dma2d: &'a Dma2d,
}

impl<'a> Square<'a> {
    pub const WIDTH: u16 = SquareNormalImage::WIDTH;
    pub const HEIGHT: u16 = SquareNormalImage::HEIGHT;

    pub fn new(dma2d: &'a Dma2d) -> Self {
        let hit_box = BoundingBox::new(
            Position::new(0, 0),
            Position::new(i32::from(Self::WIDTH), i32::from(Self::HEIGHT)),
        );

        Square {
            hit_box,
            velocity: Velocity::default(),
            dma2d,
        }
    }

    pub fn draw(&self, color_scheme: ColorSchemes) {
        let cropped_box = BoundingBox::new(
            Position::new(
                self.hit_box.top_left.x.clamp(0, lcd::SCREEN_WIDTH_I32),
                self.hit_box.top_left.y.clamp(0, lcd::SCREEN_HEIGHT_I32),
            ),
            Position::new(
                self.hit_box.bottom_right.x.clamp(0, lcd::SCREEN_WIDTH_I32),
                self.hit_box.bottom_right.y.clamp(0, lcd::SCREEN_HEIGHT_I32),
            ),
        );

        let cropped_offset = cropped_box.top_left - self.hit_box.top_left;

        let cropped_address: u32 = match color_scheme {
            ColorSchemes::Normal => SquareNormalImage
                .data_address_offset(cropped_offset.x as u16, cropped_offset.y as u16),
            ColorSchemes::Neon => SquareNeonImage
                .data_address_offset(cropped_offset.x as u16, cropped_offset.y as u16),
            ColorSchemes::Monochrome => SquareMonochromeImage
                .data_address_offset(cropped_offset.x as u16, cropped_offset.y as u16),
            ColorSchemes::Bright => SquareBrightImage
                .data_address_offset(cropped_offset.x as u16, cropped_offset.y as u16),
        };

        self.dma2d.draw_rgb8_image(
            cropped_address,
            cropped_box.top_left.x as u32,
            cropped_box.top_left.y as u32,
            cropped_box.width() as u16,
            cropped_box.height() as u16,
        );
    }
}
