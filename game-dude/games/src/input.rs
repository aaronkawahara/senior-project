use core::convert::Infallible;
use embedded_hal::digital::v2::InputPin;

trait DirectionalInput {
    fn up_pressed(&self) -> bool;
    fn down_pressed(&self) -> bool;
    fn left_pressed(&self) -> bool;
    fn right_pressed(&self) -> bool;
}

type ButtonInput = dyn InputPin<Error = Infallible>;

pub struct DPad<'a>{
    up: &'a ButtonInput,
    down: &'a ButtonInput,
    left: &'a ButtonInput,
    right: &'a ButtonInput,
}

impl<'a> DPad<'a> {
    pub fn new(
        up: &'a ButtonInput, 
        down: &'a ButtonInput, 
        left: &'a ButtonInput, 
        right: &'a ButtonInput
    ) -> Self {
        DPad {
            up,
            down,
            left,
            right,
        }
    }
}

impl<'a> DirectionalInput for DPad<'a> {
    fn up_pressed(&self) -> bool {
        self.up.is_low().unwrap()
    }

    fn down_pressed(&self) -> bool {
        self.down.is_low().unwrap()
    }

    fn left_pressed(&self) -> bool {
        self.left.is_low().unwrap()
    }

    fn right_pressed(&self) -> bool {
        self.right.is_low().unwrap()
    }
}
