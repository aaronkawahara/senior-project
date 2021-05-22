use debounced_pin::{prelude::*, ActiveLow};
use embedded_hal::digital::v2::InputPin;
use stm32l4p5_hal::gpio::{self, Input, PullUp};

pub type UpPin = gpio::PC10<Input<PullUp>>;
pub type DownPin = gpio::PC11<Input<PullUp>>;
pub type LeftPin = gpio::PC12<Input<PullUp>>;
pub type RightPin = gpio::PD2<Input<PullUp>>;

pub type DebouncedUpPin = DebouncedInputPin<UpPin, ActiveLow>;
pub type DebouncedDownPin = DebouncedInputPin<DownPin, ActiveLow>;
pub type DebouncedLeftPin = DebouncedInputPin<LeftPin, ActiveLow>;
pub type DebouncedRightPin = DebouncedInputPin<RightPin, ActiveLow>;

pub struct Inputs {
    up: DebouncedUpPin,
    down: DebouncedDownPin,
    left: DebouncedLeftPin,
    right: DebouncedRightPin,
}

impl Inputs {
    pub fn new(up: UpPin, down: DownPin, left: LeftPin, right: RightPin) -> Self {
        Inputs {
            up: DebouncedInputPin::new(up, ActiveLow),
            down: DebouncedInputPin::new(down, ActiveLow),
            left: DebouncedInputPin::new(left, ActiveLow),
            right: DebouncedInputPin::new(right, ActiveLow),
        }
    }

    pub fn up_pressed(&mut self) -> bool {
        self.up.is_pressed()
    }

    pub fn down_pressed(&mut self) -> bool {
        self.down.is_pressed()
    }

    pub fn left_pressed(&mut self) -> bool {
        self.left.is_pressed()
    }

    pub fn right_pressed(&mut self) -> bool {
        self.right.is_pressed()
    }

    pub fn up_debounced(&mut self) -> bool {
        self.up.is_debounced()
    }

    pub fn down_debounced(&mut self) -> bool {
        self.down.is_debounced()
    }

    pub fn left_debounced(&mut self) -> bool {
        self.left.is_debounced()
    }

    pub fn right_debounced(&mut self) -> bool {
        self.right.is_debounced()
    }
}

pub trait ButtonPressed {
    fn is_pressed(&mut self) -> bool;
}

impl<T: InputPin> ButtonPressed for DebouncedInputPin<T, ActiveLow> {
    fn is_pressed(&mut self) -> bool {
        if let Some(state) = self.update().ok() {
            state == DebounceState::Debouncing || state == DebounceState::Active
        } else {
            false
        }
    }
}

pub trait ButtonDebounced {
    fn is_debounced(&mut self) -> bool;
}

impl<T: InputPin> ButtonDebounced for DebouncedInputPin<T, ActiveLow> {
    fn is_debounced(&mut self) -> bool {
        if self.update().ok() == Some(DebounceState::Debouncing) {
            while self.update().ok() == Some(DebounceState::Debouncing) {}
            self.is_active()
        } else {
            false
        }
    }
}
