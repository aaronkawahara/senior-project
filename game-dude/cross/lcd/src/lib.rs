#![no_std]

use core::{convert::TryInto, mem, u16, u32};

pub struct Lcd {
    frame_buffer: [u8; 480 * 272],
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

    pub const PIXEL_CLK_FREQ: u32 = 9_000_000;
    pub const SCREEN_WIDTH: u16 = 480;
    pub const SCREEN_HEIGHT: u16 = 272;
    pub const TOTAL_PIXELS: usize = 130_560;
    pub const HBP: u16 = 40;
    pub const HFP: u16 = 5;
    pub const VBP: u16 = 8;
    pub const VFP: u16 = 8;
    pub const HSYNC_WIDTH: u16 = 4;
    pub const VSYNC_HEIGHT: u16 = 4;
}
