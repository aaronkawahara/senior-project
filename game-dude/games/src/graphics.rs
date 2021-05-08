use core::convert::Infallible;
pub use embedded_graphics::{
    egcircle, egline, egrectangle, egtext, egtriangle, pixelcolor::raw::RawU8, prelude::{self, *}, 
    primitive_style, primitives::Rectangle, 
    style::{PrimitiveStyle, Styled}
};

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
}

impl PixelColor for RGB8 {
    type Raw = RawU8;
}

impl From<RawU8> for RGB8 {
    fn from(data: RawU8) -> Self {
        Self(data)
    }
}

// renderable trait? that applies to objects/ images?

fn handle_draw(result: Result<(), Infallible>) {
    match result {
        Ok(()) => (),
        Err(_e) => panic!("error drawing shape"),
    }
}