#![no_std]

use core::{mem, u16, u32};
pub struct Lcd {
    frame_buffer: [u8; 480 * 272],
}

impl Lcd {
    pub fn new() -> Self {
        Self { 
            frame_buffer: [0; Lcd::TOTAL_PIXELS],
        }
    }

    pub fn buffer_address(self) -> u32 {
        unsafe { mem::transmute::<&u8, u32>(&self.frame_buffer[0]) }
    }

    pub const SCREEN_WIDTH: u16 = 480;
    pub const SCREEN_HEIGHT: u16 = 272;
    pub const TOTAL_PIXELS: usize = 130_560;
    pub const HBP: u16 = 36;
    pub const HFP: u16 = 4;
    pub const VBP: u16 = 3;
    pub const VFP: u16 = 2;
    pub const HSYNC_WIDTH: u16 = 1;
    pub const VSYNC_HEIGHT: u16 = 1; // maybe incorporated into BP values
}

// struct for pin assignments
