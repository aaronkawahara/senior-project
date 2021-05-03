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
    lcd.set_color(0x00);

    loop {
        let x = 0;
        let y = 0;

        let rect: Styled<Rectangle, PrimitiveStyle<Gray8>> = egrectangle!(
            top_left = (x, y),
            bottom_right = (x + 50, y + 50),
            style = primitive_style!(
                stroke_color = Gray8::WHITE,
                fill_color = Gray8::WHITE,
                stroke_width = 1
            )
        );

        // lcd.half_color(0x0f);

        handle_draw(rect.draw(&mut lcd));

        // lcd.horizontal_line(0xFF, 0);
        // lcd.horizontal_line(0xF0, 50);
        // lcd.horizontal_line(0x0F, 100);

        // lcd.vertical_line(0xFF, 0);
        // lcd.vertical_line(0xF0, 50);
        // lcd.vertical_line(0x0F, 479);

        // for y in 0..272 {
        //     lcd.horizontal_line(0xFF, y);
        // }

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