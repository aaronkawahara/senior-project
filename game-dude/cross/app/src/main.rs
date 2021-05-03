#![no_std]
#![no_main]

use asm_delay::{AsmDelay, bitrate::U32BitrateExt};
use board::{self, Board};
use cortex_m_rt as rt;
use defmt_rtt as _;
use embedded_graphics::{egcircle, egline, egrectangle, egtext, egtriangle, pixelcolor::Gray8, prelude::*, primitive_style, primitives::Rectangle, style::{PrimitiveStyle, Styled}};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use lcd::Lcd;
use panic_probe as _;
use rt::entry;

#[entry]
fn main() -> ! {
    let mut board = Board::new();
    let mut lcd = Lcd::new();
    
    let clocks = board.init_system_clocks();
    let mut _delayer = AsmDelay::new(U32BitrateExt::hz(u32::from(clocks.sysclk())));

    board.ltdc().pwr_pins.display_pwr_on();
    board.init_ltdc(lcd.buffer_address());

    let delta = 5;
    let mut x = 0;
    let mut y = 0;


    loop {
        lcd.set_color(0x00);
        let dpad = board.dpad();

        let dx = match (dpad.left_pressed(), dpad.right_pressed()) {
            (false, false) => 0,
            (true, true) => 0,
            (true, false) => -delta,
            (false, true) => delta,
        };

        let dy = match (dpad.up_pressed(), dpad.down_pressed()) {
            (false, false) => 0,
            (true, true) => 0,
            (true, false) => -delta,
            (false, true) => delta,
        };

        x += dx;
        y += dy;

        let rect: Styled<Rectangle, PrimitiveStyle<Gray8>> = egrectangle!(
            top_left = (x, y),
            bottom_right = (x + 50, y + 50),
            style = primitive_style!(
                stroke_color = Gray8::WHITE,
                fill_color = Gray8::WHITE,
                stroke_width = 1
            )
        );

        handle_draw(rect.draw(&mut lcd));

        board.ltdc().reload_shadow_reg();
        board.ltdc().wait_for_frame();
    }
}

fn handle_draw(result: Result<(), core::convert::Infallible>) {
    match result {
        Ok(()) => (),
        Err(_e) => panic!("error drawing shape"),
    }
}