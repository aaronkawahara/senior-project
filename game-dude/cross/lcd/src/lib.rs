#![no_std]

use core::{convert::TryInto, mem, u16, u32};
use stm32l4p5_hal::ltdc::Ltdc;

pub struct Lcd {
    frame_buffer: [u8; 480 * 272],
    ltdc: Option<Ltdc>,
}

impl Lcd {
    pub fn new() -> Self {
        Self { 
            frame_buffer: [0; Lcd::TOTAL_PIXELS],
            ltdc: None,
        }
    }

    pub fn set_ltdc(&mut self, ltdc: Ltdc) {
        self.ltdc = Some(ltdc);
    }

    pub fn buffer_address(&self) -> u32 {
        unsafe { mem::transmute::<&u8, u32>(&self.frame_buffer[0]) }
    }

    pub fn buffer_size(&self) -> usize {
        self.frame_buffer.len().try_into().unwrap()
    }

    pub fn reload_srcr(&mut self) {
        if let Some(ltdc) = &self.ltdc {
            ltdc.srcr.set_vbr()
        } else {
            panic!("ltdc not initialized")
        }
    }

    pub fn vsync_is_high(&mut self) -> bool {
        if let Some(ltdc) = &self.ltdc {
            ltdc.cdsr.reg().read().vsyncs().is_active()
        } else {
            panic!("ltdc not initialized")
        }
    }

    pub fn hsync_is_high(&self) -> bool {
        if let Some(ltdc) = &self.ltdc {
            ltdc.cdsr.reg().read().hsyncs().is_active()
        } else {
            panic!("ltdc not initialized")
        }
    }

    pub fn set_color(&mut self, color: u8) {
        for pixel in 0..self.frame_buffer.len() {
            self.frame_buffer[pixel] = color;
        }
    }

    pub const SCREEN_WIDTH: u16 = 480;
    pub const SCREEN_HEIGHT: u16 = 272;
    pub const TOTAL_PIXELS: usize = 130_560;
    pub const HBP: u16 = 255;
    pub const HFP: u16 = 65;
    pub const VBP: u16 = 31;
    pub const VFP: u16 = 93;
    pub const HSYNC_WIDTH: u16 = 1;
    pub const VSYNC_HEIGHT: u16 = 4;
}
