#![no_std]

pub use embedded_hal as hal;
pub use nb;
pub use nb::block;
pub use stm32l4p5_pac as pac;
pub use pac as stm32;
// pub use self::pac::interrupt;

// peripheral exports
pub mod dma2d;
pub mod gpio;
pub mod rcc;
pub mod prelude;
pub mod pwr;
pub mod ltdc;
pub mod flash;
pub mod time;
pub mod traits;