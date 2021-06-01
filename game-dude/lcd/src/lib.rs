#![no_std]

pub const PIXEL_CLK_FREQ: u32 = 9_000_000;
pub const SCREEN_WIDTH_U16: u16 = 480;
pub const SCREEN_HEIGHT_U16: u16 = 272;
pub const SCREEN_WIDTH_U32: u32 = 480;
pub const SCREEN_HEIGHT_U32: u32 = 272;
pub const SCREEN_WIDTH_I32: i32 = 480;
pub const SCREEN_HEIGHT_I32: i32 = 272;
pub const TOTAL_PIXELS: usize = 130_560;
pub const QUARTER_PIXELS: usize = 32_640;
pub const HBP: u16 = 40;
pub const HFP: u16 = 5;
pub const VBP: u16 = 8;
pub const VFP: u16 = 8;
pub const HSYNC_WIDTH: u16 = 1;
pub const VSYNC_HEIGHT: u16 = 1;

use core::{convert::TryInto, i32, u16, u32, usize};

pub use embedded_graphics::{
    drawable::Pixel,
    egcircle, egline, egrectangle, egtext, egtriangle,
    pixelcolor::raw::RawU8,
    prelude::{self, *},
    primitive_style,
    primitives::{Rectangle, Triangle},
    style::{PrimitiveStyle, Styled},
};

pub struct Lcd {
    frame_buffer: [u8; TOTAL_PIXELS],
}

impl Lcd {
    pub fn new() -> Self {
        Self {
            frame_buffer: [0; TOTAL_PIXELS],
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
        for pixel in 0..SCREEN_HEIGHT_U32 {
            let index: u32 = x + pixel as u32 * SCREEN_WIDTH_U32;
            self.frame_buffer[index as usize] = color;
        }
    }
}

pub mod colors {
    pub const RED: u8 = 0b11100000;
    pub const GREEN: u8 = 0b00011100;
    pub const BLUE: u8 = 0b00000011;
    pub const BLACK: u8 = 0b00000000;
    pub const WHITE: u8 = 0b11111111;
}