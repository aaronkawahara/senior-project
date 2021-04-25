#![no_std]
pub struct Lcd;

pub const PIXEL_WIDTH: u32 = 480;
pub const PIXEL_HEIGHT: u32 = 272;
pub const HBP: u8 = 36;
pub const HFP: u8 = 4;
pub const VBP: u8 = 3;
pub const VFP: u8 = 2;
pub const HSYNC_WIDTH: u8 = 1;
pub const VSYNC_WIDTH: u8 = 1; // maybe incorporated into BP values

