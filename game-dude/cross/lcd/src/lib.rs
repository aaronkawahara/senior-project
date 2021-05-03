#![no_std]

use core::{convert::TryInto, mem, u16, u32, usize};
use embedded_graphics::{drawable, DrawTarget, pixelcolor::Gray8, prelude::*};

#[repr(C)]
pub struct Lcd {
    frame_buffer: [u8; Lcd::TOTAL_PIXELS],
}

impl Lcd {
    pub fn new() -> Self {
        Self { 
            frame_buffer: [0; Lcd::TOTAL_PIXELS]
        }
    }

    pub fn buffer_address(&self) -> u32 {
        unsafe { mem::transmute::<&u8, u32>(&self.frame_buffer[0]) }
    }

    pub fn buffer_size(&self) -> usize {
        self.frame_buffer.len().try_into().unwrap()
    }

    pub fn set_color(&mut self, color: u8) {
        for pixel in 0..self.frame_buffer.len() {
            self.frame_buffer[pixel] = color;
        }
    }

    pub fn horizontal_line(&mut self, color: u8, y: u32) {
        let starting_pixel: usize = (y as usize) * 480;

        for pixel in 0..480 {
            self.frame_buffer[starting_pixel + pixel] = color;
        }
    }

    pub fn vertical_line(&mut self, color: u8, x: u32) {
        for pixel in 0..Lcd::SCREEN_HEIGHT {
            let index: u32 = x + pixel as u32 * Lcd::SCREEN_WIDTH as u32;
            self.frame_buffer[index as usize] = color;
        }
    }

    pub const PIXEL_CLK_FREQ: u32 = 9_000_000;
    pub const SCREEN_WIDTH: u16 = 480;
    pub const SCREEN_HEIGHT: u16 = 272;
    pub const TOTAL_PIXELS: usize = 130_560;
    pub const HBP: u16 = 40;
    pub const HFP: u16 = 5;
    pub const VBP: u16 = 8;
    pub const VFP: u16 = 8;
    pub const HSYNC_WIDTH: u16 = 1;
    pub const VSYNC_HEIGHT: u16 = 1;
}

impl DrawTarget<Gray8> for Lcd {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: drawable::Pixel<Gray8>) -> Result<(), Self::Error> {
        let Pixel(Point { x, y }, color) = pixel;

        if 0 <= x && x < 479 && 0 <= y && y < 271 {
            let x_: usize = x.try_into().unwrap();
            let y_: usize = y.try_into().unwrap();
            
            let index: usize = x_ + y_ * 480;
            self.frame_buffer[index] = color.luma();
        }

        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(Lcd::SCREEN_WIDTH.into(), Lcd::SCREEN_HEIGHT.into())
    }
}
