#![no_std]

pub const PIXEL_CLK_FREQ: u32 = 9_000_000;
pub const SCREEN_WIDTH: u16 = 480;
pub const SCREEN_HEIGHT: u16 = 272;
pub const TOTAL_PIXELS: usize = 130_560;
pub const QUARTER_PIXELS: usize = 32_640;
pub const HBP: u16 = 40;
pub const HFP: u16 = 5;
pub const VBP: u16 = 8;
pub const VFP: u16 = 8;
pub const HSYNC_WIDTH: u16 = 1;
pub const VSYNC_HEIGHT: u16 = 1;

use core::{convert::TryInto, u16, u32, usize};

use core::convert::Infallible;
pub use embedded_graphics::{
    drawable::Pixel, egcircle, egline, egrectangle, egtext, egtriangle, pixelcolor::raw::RawU8, prelude::{self, *}, 
    primitive_style, primitives::{Rectangle, Triangle},
    style::{PrimitiveStyle, Styled}
};

pub struct Lcd {
    frame_buffer: [u8; TOTAL_PIXELS],
}

impl Lcd {
    pub fn new() -> Self {
        Self { 
            frame_buffer: [0; TOTAL_PIXELS]
        }
    }

    pub fn first_element(&self) -> &u8 {
        &self.frame_buffer[0]
    }

    pub fn buffer_size(&self) -> usize {
        self.frame_buffer.len().try_into().unwrap()
    }

    pub fn set_color(&mut self, color: u8) {
        for pixel in self.frame_buffer.iter_mut() {
            *pixel = color;
        }
    }

    pub fn horizontal_line(&mut self, color: u8, y: u32) {
        let starting_pixel: usize = (y as usize) * 480;

        for pixel in 0..480 {
            self.frame_buffer[starting_pixel + pixel] = color;
        }
    }

    pub fn vertical_line(&mut self, color: u8, x: u32) {
        for pixel in 0..SCREEN_HEIGHT {
            let index: u32 = x + pixel as u32 * SCREEN_WIDTH as u32;
            self.frame_buffer[index as usize] = color;
        }
    }
}

impl DrawTarget<RGB8> for Lcd {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<RGB8>) -> Result<(), Self::Error> {
        let Pixel(Point { x, y }, color) = pixel;

        if 0 <= x && x < 479 && 0 <= y && y < 271 {
            let x_: usize = x.try_into().unwrap();
            let y_: usize = y.try_into().unwrap();
            
            let index: usize = x_ + y_ * 480;
            self.frame_buffer[index] = color.rgb();
        }

        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(SCREEN_WIDTH.into(), SCREEN_HEIGHT.into())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RGB8(RawU8);

impl RGB8 {
    pub const RED: u8 = 0b11100000;
    pub const GREEN: u8 = 0b00011100;
    pub const BLUE: u8 = 0b00000011;
    pub const BLACK: u8 = 0b00000000;
    pub const WHITE: u8 = 0b11111111;

    pub fn new(rgb: u8) -> Self {
        Self(RawU8::new(rgb))
    }

    pub fn rgb(&self) -> u8 {
        self.0.into_inner()
    }
}

impl PixelColor for RGB8 {
    type Raw = RawU8;
}

impl From<RawU8> for RGB8 {
    fn from(data: RawU8) -> Self {
        Self(data)
    }
}

pub fn handle_draw(result: Result<(), Infallible>) {
    match result {
        Ok(()) => (),
        Err(_e) => panic!("error drawing shape"),
    }
}